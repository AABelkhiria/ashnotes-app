use octocrab::Octocrab;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub content: String,
}

pub struct GitHubService {
    client: Octocrab,
    repo_owner: String,
    repo_name: String,
}

impl GitHubService {
    pub fn new(token: String, repo_owner: String, repo_name: String) -> Self {
        let client = Octocrab::builder().personal_token(token).build().unwrap();
        Self { client, repo_owner, repo_name }
    }

    pub async fn get_all_notes(&self) -> octocrab::Result<Vec<Note>> {
        let content_items = self.client
            .repos(&self.repo_owner, &self.repo_name)
            .get_content()
            .path("notes/")
            .send()
            .await?;

        let mut notes = Vec::new();
        for item in content_items.items {
            if let Some(note_content) = self.get_note_content(&item.path).await? {
                notes.push(Note {
                    id: item.name,
                    content: note_content,
                });
            }
        }
        Ok(notes)
    }

    pub async fn get_note(&self, id: &str) -> octocrab::Result<Option<Note>> {
        if let Some(content) = self.get_note_content(&format!("notes/{}", id)).await? {
            Ok(Some(Note {
                id: id.to_string(),
                content,
            }))
        } else {
            Ok(None)
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

    pub async fn create_note(&self, filename: &str, content: &str) -> octocrab::Result<()> {
        self.client
            .repos(&self.repo_owner, &self.repo_name)
            .create_file(&format!("notes/{}", filename), "feat: create new note", content)
            .send()
            .await?;
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

        self.client
            .repos(&self.repo_owner, &self.repo_name)
            .update_file(&path, "feat: update note", content, sha)
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

        self.client
            .repos(&self.repo_owner, &self.repo_name)
            .delete_file(&path, "feat: delete note", sha)
            .send()
            .await?;
        Ok(())
    }
}
