# Note Taker API Documentation

This document provides instructions on how to use the Note Taker API.

## Authentication

All API endpoints require the following headers for authentication and configuration:

-   `GITHUB_TOKEN`: Your GitHub personal access token. This is used to authenticate with the GitHub API.
-   `NOTES_REPO`: The repository where your notes are stored, in the format `owner/repo-name`.
-   `APP_IDENTIFIER` (Optional): An identifier for your application. Defaults to "NoteApp".

## Endpoints

### Notes

#### List Notes

-   **GET** `/api/notes`

    Retrieves a list of all notes.

    **Response:**

    -   `200 OK`: A JSON array of note objects.
    -   `500 Internal Server Error`: If there is an error fetching the notes.

#### Create Note

-   **POST** `/api/notes`

    Creates a new note.

    **Request Body:**

    ```json
    {
        "path": "path/to/your/note.md",
        "content": "This is the content of the note."
    }
    ```

    **Responses:**

    -   `201 Created`: If the note is created successfully.
    -   `409 Conflict`: If a note with the same path already exists.
    -   `500 Internal Server Error`: If there is an error creating the note.

#### Get Note

-   **GET** `/api/notes/*path`

    Retrieves a single note by its path.

    **Example:**

    `GET /api/notes/path/to/your/note.md`

    **Responses:**

    -   `200 OK`: A JSON object representing the note.
    -   `404 Not Found`: If the note is not found.
    -   `500 Internal Server Error`: If there is an error fetching the note.

#### Update Note

-   **PUT** `/api/notes/*path`

    Updates the content of an existing note.

    **Example:**

    `PUT /api/notes/path/to/your/note.md`

    **Request Body:**

    ```json
    {
        "content": "This is the updated content of the note."
    }
    ```

    **Responses:**

    -   `200 OK`: If the note is updated successfully.
    -   `500 Internal Server Error`: If there is an error updating the note.

#### Delete Note

-   **DELETE** `/api/notes/*path`

    Deletes a note.

    **Example:**

    `DELETE /api/notes/path/to/your/note.md`

    **Responses:**

    -   `200 OK`: If the note is deleted successfully.
    -   `500 Internal Server Error`: If there is an error deleting the note.
