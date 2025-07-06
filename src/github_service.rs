use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug)]
pub enum GitHubServiceError {
    #[allow(dead_code)]
    OctocrabError(octocrab::Error),
    NoteAlreadyExists,
}

impl From<octocrab::Error> for GitHubServiceError {
    fn from(err: octocrab::Error) -> Self {
        GitHubServiceError::OctocrabError(err)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub name: String,
    pub content: Option<String>,
    pub children: Option<Vec<Note>>,
}

pub struct GitHubService {
    client: Octocrab,
    repo_owner: String,
    repo_name: String,
    app_identifier: String,
}

impl GitHubService {
    pub fn new(token: String, repo_owner: String, repo_name: String, app_identifier: String) -> Self {
        let client = Octocrab::builder().personal_token(token).build().unwrap();
        Self { client, repo_owner, repo_name, app_identifier }
    }

    pub async fn get_all_notes(&self) -> octocrab::Result<Vec<Note>> {
        self.get_notes_recursive("notes/").await
    }

    fn get_notes_recursive<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = octocrab::Result<Vec<Note>>> + Send + 'a>> {
        Box::pin(async move {
            let content_items = self.client
                .repos(&self.repo_owner, &self.repo_name)
                .get_content()
                .path(path)
                .send()
                .await?;

            let mut notes = Vec::new();
            for item in content_items.items {
                let name = item.name.clone();
                let id = item.path.clone();

                if item.r#type == "dir" {
                    let children = self.get_notes_recursive(&item.path).await?;
                    notes.push(Note {
                        id,
                        name,
                        content: None,
                        children: Some(children),
                    });
                } else {
                    notes.push(Note {
                        id,
                        name,
                        content: None,
                        children: None,
                    });
                }
            }
            Ok(notes)
        })
    }

    pub async fn get_note(&self, id: &str) -> octocrab::Result<Option<Note>> {
        let path = format!("notes/{}", id);
        let result = self.client
            .repos(&self.repo_owner, &self.repo_name)
            .get_content()
            .path(&path)
            .send()
            .await;

        match result {
            Ok(content) => {
                if let Some(item) = content.items.into_iter().next() {
                    let name = item.name.clone();
                    let id = item.path.clone();

                    if item.r#type == "dir" {
                        let children = self.get_notes_recursive(&item.path).await?;
                        Ok(Some(Note {
                            id,
                            name,
                            content: None,
                            children: Some(children),
                        }))
                    } else {
                        let note_content = self.get_note_content(&item.path).await?;
                        Ok(Some(Note {
                            id,
                            name,
                            content: note_content,
                            children: None,
                        }))
                    }
                } else {
                    Ok(None)
                }
            },
            Err(octocrab::Error::GitHub { source, .. }) if source.status_code == 404 => {
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    async fn get_note_content(&self, path: &str) -> octocrab::Result<Option<String>> {
        let result = self.client
            .repos(&self.repo_owner, &self.repo_name)
            .get_content()
            .path(path)
            .send()
            .await;

        match result {
            Ok(content) => {
                if let Some(item) = content.items.into_iter().next() {
                    Ok(item.decoded_content())
                } else {
                    Ok(None)
                }
            },
            Err(octocrab::Error::GitHub { source, .. }) if source.status_code == 404 => {
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

pub async fn create_note(&self, path: &str, content: &str) -> Result<(), GitHubServiceError> {
    let full_path = format!("notes/{}", path);
    let path_obj = std::path::Path::new(path);

    // Check if the file already exists
    match self.client
        .repos(&self.repo_owner, &self.repo_name)
        .get_content()
        .path(&full_path)
        .send()
        .await {
        Ok(_) => return Err(GitHubServiceError::NoteAlreadyExists),
        Err(octocrab::Error::GitHub { source, .. }) if source.status_code == 404 => { /* File does not exist, proceed */ },
        Err(e) => return Err(e.into()),
    }

    // A flag to check if the loop created our target file.
    let mut target_file_was_created_in_loop = false;

    if let Some(parent_path) = path_obj.parent() {
        if !parent_path.as_os_str().is_empty() {
            let mut cumulative_path = std::path::PathBuf::from("notes");
            for component in parent_path.iter() {
                cumulative_path.push(component);
                let readme_path = cumulative_path.join("README.md");
                let readme_path_str = readme_path.to_str().unwrap();

                // Check if the README we are about to create is our actual target file
                if readme_path_str == full_path {
                    target_file_was_created_in_loop = true;
                }
                
                let result = self.client
                    .repos(&self.repo_owner, &self.repo_name)
                    .get_content()
                    .path(readme_path_str)
                    .send()
                    .await;

                match result {
                    Ok(_) => { /* README.md exists, do nothing */ },
                    Err(octocrab::Error::GitHub { source, .. }) if source.status_code == 404 => {
                        // README.md does not exist, create it
                        let dir_name = component.to_str().unwrap().replace("-", " ");
                        
                        // If this README is our target file, use the user-provided content.
                        // Otherwise, use the generated title.
                        let readme_content = if target_file_was_created_in_loop {
                            content
                        } else {
                            &format!("# {}", dir_name)
                        };

                        let commit_message = format!("feat: create category README by {}", self.app_identifier);
                        let result = self.client
                            .repos(&self.repo_owner, &self.repo_name)
                            .create_file(readme_path_str, &commit_message, readme_content)
                            .send()
                            .await;

                        if let Err(e) = result {
                            // Ignore 422 in case of a race condition, but fail on other errors.
                            if let octocrab::Error::GitHub { source, .. } = &e {
                                if source.status_code != 422 {
                                    return Err(e.into());
                                }
                            } else {
                                return Err(e.into());
                            }
                        }
                    },
                    Err(e) => return Err(e.into()),
                }
            }
        }
    }

    // Only create the file if it wasn't already created by the directory loop.
    if !target_file_was_created_in_loop {
        let commit_message = format!("feat: create new note by {}", self.app_identifier);
        self.client
            .repos(&self.repo_owner, &self.repo_name)
            .create_file(&full_path, &commit_message, content)
            .send()
            .await?;
    }

    Ok(())
}

    pub async fn update_note(&self, id: &str, content: &str) -> octocrab::Result<()> {
        let path = format!("notes/{}", id);
        let get_content_result = self.client
            .repos(&self.repo_owner, &self.repo_name)
            .get_content()
            .path(&path)
            .send()
            .await?;

        let sha = get_content_result.items.first().unwrap().sha.clone();

        let commit_message = format!("feat: update note by {}", self.app_identifier);
        self.client
            .repos(&self.repo_owner, &self.repo_name)
            .update_file(&path, &commit_message, content, sha)
            .send()
            .await?;
        Ok(())
    }

    pub async fn delete_note(&self, id: &str) -> octocrab::Result<()> {
        let path = format!("notes/{}", id);
        let get_content_result = self.client
            .repos(&self.repo_owner, &self.repo_name)
            .get_content()
            .path(&path)
            .send()
            .await?;

        let sha = get_content_result.items.first().unwrap().sha.clone();

        let commit_message = format!("feat: delete note by {}", self.app_identifier);
        self.client
            .repos(&self.repo_owner, &self.repo_name)
            .delete_file(&path, &commit_message, sha)
            .send()
            .await?;
        Ok(())
    }
}