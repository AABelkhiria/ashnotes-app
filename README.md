# Note-Taking App Backend

This project is a Rust-based backend for a note-taking application that uses a GitHub repository as its storage. It provides a Notion-like experience with hierarchical notes, where notes can be categorized into directories and sub-directories.

## Architecture

The application is built with Axum, a popular web framework for Rust, and uses the `octocrab` library to interact with the GitHub API. Notes are stored in a dedicated GitHub repository, which is configured through environment variables.

### Hierarchical Note Structure

Notes are organized in a tree-like structure, where:

- **Categories** are represented as directories.
- **Notes** are represented as files within those directories.

When a new category (directory) is created, a `README.md` file is automatically generated within it to serve as a default landing page.

## API Endpoints

### List All Notes

- **Endpoint:** `GET /api/notes`
- **Description:** Fetches the entire tree of notes and categories.

### Get a Specific Note or Category

- **Endpoint:** `GET /api/notes/*path`
- **Description:** Retrieves a single note or the contents of a category.
- **Example:** `GET /api/notes/work/project-alpha/meeting-notes.md`

### Create a New Note

- **Endpoint:** `POST /api/notes`
- **Description:** Creates a new note. If the parent directories do not exist, they will be created automatically, each with a `README.md` file.
- **Request Body:**
  ```json
  {
    "path": "work/project-beta/initial-ideas.md",
    "content": "# Ideas for Project Beta"
  }
  ```

### Update a Note's Content

- **Endpoint:** `PUT /api/notes/*path`
- **Description:** Updates the content of an existing note.
- **Request Body:**
  ```json
  {
    "content": "This is the updated content."
  }
  ```

### Delete a Note

- **Endpoint:** `DELETE /api/notes/*path`
- **Description:** Deletes a specific note file.

## Setup and Installation

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd note_app_backend
   ```

2. **Create a `.env` file:**

   Copy the example file and fill in your details:
   ```bash
   cp .env.example .env
   ```

3. **Set environment variables in `.env`:**

   - `GITHUB_TOKEN`: Your GitHub personal access token.
   - `NOTES_REPO_OWNER`: The owner of the repository for storing notes.
   - `NOTES_REPO_NAME`: The name of the notes repository.

4. **Run the application:**
   ```bash
   cargo run
   ```

The application will be available at `http://0.0.0.0:3000`.

## Testing with cURL

Here are some `curl` commands to test the application's behavior.

### 1. Create a Top-Level Note

```bash
curl -X POST http://localhost:3000/api/notes \
-H "Content-Type: application/json" \
-d '{
  "path": "journal.md",
  "content": "# Personal Journal\n\nToday was a good day."
}'
```

### 2. Create a Nested Note in a New Category

This command creates a note inside a new directory structure (`work/project-alpha/`). It will automatically create the `work` and `project-alpha` directories, each with its own `README.md` file.

```bash
curl -X POST http://localhost:3000/api/notes \
-H "Content-Type: application/json" \
-d '{
  "path": "work/project-alpha/requirements.md",
  "content": "# Project Alpha Requirements\n\n- Feature A: Must do X.\n- Feature B: Must do Y."
}'
```

### 3. List All Notes

```bash
curl http://localhost:3000/api/notes
```

### 4. Get a Specific Note

```bash
curl http://localhost:3000/api/notes/work/project-alpha/requirements.md
```

### 5. Get a Specific Category

```bash
curl http://localhost:3000/api/notes/work/project-alpha
```

### 6. Update a Note

```bash
curl -X PUT http://localhost:3000/api/notes/journal.md \
-H "Content-Type: application/json" \
-d '{
  "content": "# Personal Journal (Updated)\n\nI accomplished a lot today."
}'
```

### 7. Delete a Note

```bash
curl -X DELETE http://localhost:3000/api/notes/journal.md
```
