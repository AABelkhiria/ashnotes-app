# Note-Taking App

This project is a full-stack note-taking application that uses a GitHub repository as its storage, providing a Notion-like experience with hierarchical notes.

## Project Structure

The project is a monorepo with the following structure:

-   **/frontend**: Contains the SvelteKit frontend application.
-   **/backend**: Contains the backend application, written in Rust with Axum.
-   **/docs**: Contains additional documentation for the project.

## Getting Started

To get started with this project, you'll need to have Node.js, npm, and Rust installed.

### Installation

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd note-app
    ```

2.  **Install frontend dependencies:**
    ```bash
    make install
    ```

### Running the Application

To run both the frontend and backend development servers concurrently, use the following command:

```bash
make run
```

The frontend will be available at `http://localhost:5173`, and the backend will be running on port `3000`.

## Available Commands

This project uses a `Makefile` to manage common tasks. Here are some of the most useful commands:

-   `make all`: Build both the frontend and backend.
-   `make run`: Run both the frontend and backend development servers.
-   `make install`: Install frontend dependencies.
-   `make build`: Build both the frontend and backend for production.
-   `make build-frontend`: Build the frontend only.
-   `make build-backend`: Build the backend only.
-   `make run-frontend`: Run the frontend development server.
-   `make run-backend`: Run the backend development server.
-   `make check`: Run static checks on the frontend code.
-   `make test`: Run tests for the backend.
-   `make clean`: Clean the build artifacts for both frontend and backend.
-   `make help`: Show all available commands.

## Documentation

For more detailed documentation, please see the [docs](./docs) directory.

## Continuous Deployment

This project uses a GitHub Actions workflow to automate the release process. When a commit is pushed to the `main` branch, the following steps are executed:

1.  **Versioning**: The workflow determines the new version number based on the current date and the latest git tag. The versioning scheme is `YY.MM.minor`, where `YY` is the year, `MM` is the month, and `minor` is an incremental number that resets to `1` each month.
2.  **Build**: The desktop application is built, generating a `.deb` package.
3.  **Release**: A new GitHub Release is created with the new version number as the tag.
4.  **Upload**: The generated `.deb` package is uploaded as an asset to the new release.
