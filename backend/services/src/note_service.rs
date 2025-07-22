use anyhow::Result;
use async_recursion::async_recursion;
use futures::future::join_all;
use serde::{Deserialize, Serialize};

use github_service::{GitHubService, GitHubServiceError};

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum NoteServiceError {
    #[error("Note already exists")]
    NoteAlreadyExists,
    #[error("GitHub API error: {0}")]
    GitHub(String),
    #[error("An internal error occurred: {0}")]
    Anyhow(String),
}

impl From<anyhow::Error> for NoteServiceError {
    fn from(err: anyhow::Error) -> Self {
        NoteServiceError::Anyhow(err.to_string())
    }
}

impl From<GitHubServiceError> for NoteServiceError {
    fn from(err: GitHubServiceError) -> Self {
        match err {
            GitHubServiceError::NoteAlreadyExists => NoteServiceError::NoteAlreadyExists,
            GitHubServiceError::Octocrab(s) => NoteServiceError::GitHub(s),
            GitHubServiceError::Anyhow(s) => NoteServiceError::Anyhow(s),
        }
    }
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

pub struct NoteService {
    github_service: GitHubService,
    app_identifier: String,
}

impl NoteService {
    pub fn new(github_token: String, repo_name: String, app_identifier: String) -> Result<Self, NoteServiceError> {
        let parts: Vec<&str> = repo_name.split('/').collect();
        if parts.len() != 2 {
            return Err(NoteServiceError::Anyhow(
                "Invalid repo_name format. Expected 'owner/name'".to_string(),
            ));
        }
        let owner = parts[0].to_string();
        let repo = parts[1].to_string();

        let github_service = GitHubService::new(github_token, owner, repo);

        Ok(Self {
            github_service,
            app_identifier,
        })
    }

    pub async fn get_all_notes(&self) -> Result<Vec<Note>> {
        self.get_notes_recursive("notes/").await.map_err(|e| e.into())
    }

    #[async_recursion]
    async fn get_notes_recursive(&self, path: &str) -> Result<Vec<Note>, NoteServiceError> {
        let content_items = self.github_service.get_content_items(path).await?;
        let futures = content_items.into_iter().map(|item| async move {
            let id = item.path.clone();
            let name = item.name.clone();

            if item.r#type == "dir" {
                let children = self.get_notes_recursive(&item.path).await?;
                Ok(Note {
                    id,
                    name,
                    content: None,
                    children: Some(children),
                })
            } else {
                Ok(Note {
                    id,
                    name,
                    content: None,
                    children: None,
                })
            }
        });
        let notes: Vec<Result<Note, NoteServiceError>> = join_all(futures).await;
        let notes: Vec<Note> = notes.into_iter().filter_map(Result::ok).collect();
        Ok(notes)
    }

    pub async fn get_note(&self, id: &str) -> Result<Option<Note>> {
        let path = format!("notes/{}", id);
        match self.github_service.get_content_items(&path).await {
            Ok(content) => {
                if let Some(item) = content.into_iter().next() {
                    let note = if item.r#type == "dir" {
                        let children = self.get_notes_recursive(&item.path).await?;
                        Note {
                            id: item.path,
                            name: item.name,
                            content: None,
                            children: Some(children),
                        }
                    } else {
                        let content = self.get_note_content(&item.path).await?;
                        Note {
                            id: item.path,
                            name: item.name,
                            content,
                            children: None,
                        }
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
                Err(e.into())
            }
        }
    }

    async fn get_note_content(&self, path: &str) -> Result<Option<String>> {
        match self.github_service.get_content_items(path).await {
            Ok(content) => Ok(content.into_iter().next().and_then(|item| item.decoded_content())),
            Err(e) => {
                if let Some(octocrab::Error::GitHub { source, .. }) = e.downcast_ref::<octocrab::Error>() {
                    if source.status_code == 404 {
                        return Ok(None);
                    }
                }
                Err(e.into())
            }
        }
    }

    pub async fn create_note(&self, path: &str, content: &str) -> Result<(), NoteServiceError> {
        let full_path = format!("notes/{}", path);
        if self.github_service.note_exists(&full_path).await? {
            return Err(NoteServiceError::NoteAlreadyExists);
        }

        self.ensure_parent_directories_exist(path, content).await?;

        let is_readme = path.ends_with("README.md");
        if !is_readme {
            let commit_message = format!("feat: create new note by {}", self.app_identifier);
            self.github_service
                .create_file(&full_path, &commit_message, content)
                .await?;
        }

        Ok(())
    }

    async fn ensure_parent_directories_exist(&self, path: &str, content: &str) -> Result<(), NoteServiceError> {
        let path_obj = std::path::Path::new(path);
        if let Some(parent_path) = path_obj.parent() {
            let mut cumulative_path = std::path::PathBuf::from("notes");
            for component in parent_path.iter() {
                cumulative_path.push(component);
                let readme_path = cumulative_path.join("README.md");
                let readme_path_str = readme_path.to_str().unwrap();

                if !self.github_service.note_exists(readme_path_str).await? {
                    let dir_name = component.to_str().unwrap().replace('-', " ");
                    let readme_content = if readme_path_str == format!("notes/{}", path) {
                        content.to_string()
                    } else {
                        format!("# {}", dir_name)
                    };
                    let commit_message = format!("feat: create category README by {}", self.app_identifier);
                    self.github_service
                        .create_file(readme_path_str, &commit_message, &readme_content)
                        .await?;
                }
            }
        }
        Ok(())
    }

    pub async fn update_note(&self, id: &str, content: &str) -> Result<(), NoteServiceError> {
        let path = format!("notes/{}", id);
        let sha = self.github_service.get_sha(&path).await?;
        let commit_message = format!("feat: update note by {}", self.app_identifier);
        self.github_service
            .update_file(&path, &commit_message, content, &sha)
            .await?;
        Ok(())
    }

    pub async fn delete_note(&self, id: &str) -> Result<(), NoteServiceError> {
        let path = format!("notes/{}", id);
        let sha = self.github_service.get_sha(&path).await?;
        let commit_message = format!("feat: delete note by {}", self.app_identifier);
        self.github_service.delete_file(&path, &commit_message, &sha).await?;
        Ok(())
    }
}
