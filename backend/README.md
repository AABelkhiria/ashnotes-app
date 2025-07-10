# Note-Taking App Backend

This is the backend for the Note-Taking App. It is a Rust-based application that uses a GitHub repository as its storage and serves an embedded SvelteKit frontend.

For more information about the project as a whole, please see the [root-level README.md](../README.md).

## Architecture

The application is built with Axum, a popular web framework for Rust, and uses the `octocrab` library to interact with the GitHub API. The frontend is embedded using `rust-embed`.

### Hierarchical Note Structure

Notes are organized in a tree-like structure, where:

-   **Categories** are represented as directories.
-   **Notes** are represented as files within those directories.

When a new category (directory) is created, a `README.md` file is automatically generated within it to serve as a default landing page.

## API Endpoints

All API endpoints require the following headers:

-   `GITHUB_TOKEN`: Your GitHub personal access token.
-   `NOTES_REPO`: The owner and name of the repository for storing notes (e.g., `owner/repo`).
-   `APP_IDENTIFIER`: An identifier for the application making the changes.

### List All Notes

-   **Endpoint:** `GET /api/notes`
-   **Description:** Fetches the entire tree of notes and categories.

### Get a Specific Note or Category

-   **Endpoint:** `GET /api/notes/*path`
-   **Description:** Retrieves a single note or the contents of a category.
-   **Example:** `GET /api/notes/work/project-alpha/meeting-notes.md`

### Create a New Note

-   **Endpoint:** `POST /api/notes`
-   **Description:** Creates a new note. If the parent directories do not exist, they will be created automatically, each with a `README.md` file.
-   **Request Body:**
    ```json
    {
      "path": "work/project-beta/initial-ideas.md",
      "content": "# Ideas for Project Beta"
    }
    ```

### Update a Note's Content

-   **Endpoint:** `PUT /api/notes/*path`
-   **Description:** Updates the content of an existing note.
-   **Request Body:**
    ```json
    {
      "content": "This is the updated content."
    }
    ```

### Delete a Note

-   **Endpoint:** `DELETE /api/notes/*path`
-   **Description:** Deletes a specific note file.

## Setup and Installation

1.  **Run the application:**
    ```bash
    make run-backend
    ```

The application will be available at `http://0.0.0.0:3000`.