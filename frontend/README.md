# Note-Taking App Frontend

This is the frontend for the Note-Taking App. It is a SvelteKit application that provides a user interface for interacting with the backend API.

For more information about the project as a whole, please see the [root-level README.md](../README.md).

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