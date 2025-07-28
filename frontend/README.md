# Frontend

This is the frontend for the Note-Taking App. It is a SvelteKit application that provides a user interface for interacting with the backend API.

## Getting Started

To get started with the frontend, you'll need to have Node.js and npm installed.

1.  **Install dependencies:**
    ```bash
    make install
    ```

2.  **Run the development server:**
    ```bash
    make run-frontend
    ```

The application will be available at `http://localhost:5173`.

## Available Commands

-   `make build-frontend`: Build the frontend for production.
-   `make run-frontend`: Run the frontend development server.
-   `make check`: Run static checks on the frontend code.
-   `make clean`: Clean the build artifacts.

## Features

-   **Collapsible Sidebar:** The sidebar can be collapsed to save space.
    -   Click the arrow icon to toggle the sidebar.
    -   When collapsed, hover over the sidebar for more than one second to expand it temporarily.
    -   The settings icon is hidden when the sidebar is collapsed.
-   **Settings Menu:**
    -   A divider separates the repository settings from the theme settings.
    -   The theme can be toggled with a sun/moon icon.
    -   The save button is right-aligned.
-   **Responsive Design:** The sidebar automatically collapses on smaller screens (less than 600px wide) to improve usability on mobile devices.

## Important Notes

-   **Desktop App Initialization:** A race condition where the application attempted to fetch notes before settings were fully loaded on first launch of the desktop app has been resolved. The note page now waits for the application to be fully initialized before fetching note data.

## API Interaction

The frontend communicates with a backend API to perform all data operations. The API is expected to be running on `http://localhost:3000` and properly configured for CORS to allow requests from the frontend's origin (`http://localhost:5173`).

### API Headers

All API requests must include the following headers:

-   `NOTES_REPO`: The owner and name of the repository for storing notes (e.g., `owner/repo`).
-   `APP_IDENTIFIER`: An identifier for the application making the changes.

## Theming

The application supports both light and dark themes. The theme can be toggled from the settings menu. The current theme is stored in local storage and applied on page load.

## State Management

The application uses Svelte stores to manage global state, including:

-   **`noteStore`**: Manages the list of notes and triggers refreshes when notes are created, updated, or deleted.
-   **`themeStore`**: Manages the current theme.
-   **`settingsStore`**: Manages the user's settings, including the GitHub token and repository.
-   **`sidebarStore`**: Manages the state of the sidebar (collapsed or expanded).
-   **`logStore`**: Manages debug logs.

## Components

-   **`NoteTree`**: Displays the hierarchical list of notes.
-   **`NoteEditor`**: Provides an interface for editing and viewing notes.
-   **`Settings`**: Allows the user to configure their GitHub token and repository.
-   **`DebugPanel`**: Displays debug logs.
-   **`DeletionProgress`**: Shows the progress of deleting notes and directories.
-   **`Icon`**: A reusable component for displaying SVG icons.
-   **`NoteTreeItem`**: Represents a single item in the note tree.
-   **`Layout`**: The main layout of the application, including the sidebar and main content area.
-   **`Page`**: The main page of the application, which displays the note editor.
-   **`[...path]`**: A dynamic route that displays the content of a specific note.
