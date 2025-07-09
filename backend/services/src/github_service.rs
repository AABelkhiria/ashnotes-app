use anyhow::{Context, Result};
use async_recursion::async_recursion;
use futures::future::join_all;
use octocrab::models::repos::Content;
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum GitHubServiceError {
    #[error("Note already exists")]
    NoteAlreadyExists,
    #[error("GitHub API error: {0}")]
    Octocrab(String),
    #[error("An internal error occurred: {0}")]
    Anyhow(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Note>>,
}

impl From<anyhow::Error> for GitHubServiceError {
    fn from(err: anyhow::Error) -> Self {
        GitHubServiceError::Anyhow(err.to_string())
    }
}

impl From<octocrab::Error> for GitHubServiceError {
    fn from(err: octocrab::Error) -> Self {
        GitHubServiceError::Octocrab(err.to_string())
    }
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
        Self {
            client,
            repo_owner,
            repo_name,
            app_identifier,
        }
    }

    pub async fn get_all_notes(&self) -> Result<Vec<Note>> {
        self.get_notes_recursive("notes/").await
    }

    #[async_recursion]
    async fn get_notes_recursive(&self, path: &str) -> Result<Vec<Note>> {
        let content_items = self.get_content_items(path).await?;
        let futures = content_items.into_iter().map(|item| self.process_content_item(item));
        let notes = join_all(futures).await.into_iter().filter_map(Result::ok).collect();
        Ok(notes)
    }

    async fn process_content_item(&self, item: Content) -> Result<Note> {
        let id = item.path.clone();
        let name = item.name.clone();

        if item.r#type == "dir" {
            let children = self.get_notes_recursive(&item.path).await?;
            Ok(Note { id, name, content: None, children: Some(children) })
        } else {
            Ok(Note { id, name, content: None, children: None })
        }
    }

    pub async fn get_note(&self, id: &str) -> Result<Option<Note>> {
        let path = format!("notes/{}", id);
        match self.get_content_items(&path).await {
            Ok(content) => {
                if let Some(item) = content.into_iter().next() {
                    let note = if item.r#type == "dir" {
                        let children = self.get_notes_recursive(&item.path).await?;
                        Note { id: item.path, name: item.name, content: None, children: Some(children) }
                    } else {
                        let content = self.get_note_content(&item.path).await?;
                        Note { id: item.path, name: item.name, content, children: None }
                    };
                    Ok(Some(note))
                } else {
                    Ok(None)
                }
            }
            Err(e) => {
                if let Some(octocrab::Error::GitHub { source, .. }) = e.downcast_ref::<octocrab::Error>() {
                    if source.status_code == 404 {
                        return Ok(None);
                    }
                }
                Err(e)
            }
        }
    }

    async fn get_note_content(&self, path: &str) -> Result<Option<String>> {
        match self.get_content_items(path).await {
            Ok(content) => Ok(content.into_iter().next().and_then(|item| item.decoded_content())),
            Err(e) => {
                if let Some(octocrab::Error::GitHub { source, .. }) = e.downcast_ref::<octocrab::Error>() {
                    if source.status_code == 404 {
                        return Ok(None);
                    }
                }
                Err(e)
            }
        }
    }

    pub async fn create_note(&self, path: &str, content: &str) -> Result<(), GitHubServiceError> {
        let full_path = format!("notes/{}", path);
        if self.note_exists(&full_path).await? {
            return Err(GitHubServiceError::NoteAlreadyExists);
        }

        self.ensure_parent_directories_exist(path, content).await?;
        
        let is_readme = path.ends_with("README.md");
        if !is_readme {
            let commit_message = format!("feat: create new note by {}", self.app_identifier);
            self.create_file(&full_path, &commit_message, content).await?;
        }

        Ok(())
    }

    async fn ensure_parent_directories_exist(&self, path: &str, content: &str) -> Result<(), GitHubServiceError> {
        let path_obj = std::path::Path::new(path);
        if let Some(parent_path) = path_obj.parent() {
            let mut cumulative_path = std::path::PathBuf::from("notes");
            for component in parent_path.iter() {
                cumulative_path.push(component);
                let readme_path = cumulative_path.join("README.md");
                let readme_path_str = readme_path.to_str().unwrap();

                if !self.note_exists(readme_path_str).await? {
                    let dir_name = component.to_str().unwrap().replace('-', " ");
                    let readme_content = if readme_path_str == format!("notes/{}", path) {
                        content.to_string()
                    } else {
                        format!("# {}", dir_name)
                    };
                    let commit_message = format!("feat: create category README by {}", self.app_identifier);
                    self.create_file(readme_path_str, &commit_message, &readme_content).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn update_note(&self, id: &str, content: &str) -> Result<()> {
        let path = format!("notes/{}", id);
        let sha = self.get_sha(&path).await?;
        let commit_message = format!("feat: update note by {}", self.app_identifier);
        self.client
            .repos(&self.repo_owner, &self.repo_name)
            .update_file(&path, &commit_message, content, sha)
            .send()
            .await?;
        Ok(())
    }

    pub async fn delete_note(&self, id: &str) -> Result<()> {
        let path = format!("notes/{}", id);
        let sha = self.get_sha(&path).await?;
        let commit_message = format!("feat: delete note by {}", self.app_identifier);
        self.client
            .repos(&self.repo_owner, &self.repo_name)
            .delete_file(&path, &commit_message, sha)
            .send()
            .await?;
        Ok(())
    }

    async fn get_content_items(&self, path: &str) -> Result<Vec<Content>> {
        Ok(self.client
            .repos(&self.repo_owner, &self.repo_name)
            .get_content()
            .path(path)
            .send()
            .await?
            .items)
    }

    async fn note_exists(&self, path: &str) -> Result<bool> {
        match self.client.repos(&self.repo_owner, &self.repo_name).get_content().path(path).send().await {
            Ok(_) => Ok(true),
            Err(octocrab::Error::GitHub { source, .. }) if source.status_code == 404 => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    async fn create_file(&self, path: &str, message: &str, content: &str) -> Result<(), GitHubServiceError> {
        match self.client.repos(&self.repo_owner, &self.repo_name).create_file(path, message, content).send().await {
            Ok(_) => Ok(()),
            Err(octocrab::Error::GitHub { source, .. }) if source.status_code == 422 => Ok(()), // Race condition
            Err(e) => Err(e.into()),
        }
    }

    async fn get_sha(&self, path: &str) -> Result<String> {
        let content = self.get_content_items(path).await?;
        let sha = content.first().context("No content found")?.sha.clone();
        Ok(sha)
    }
}
