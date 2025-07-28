# Desktop App

This is the Tauri-based desktop application for the note-taking app.

## Development

To run the desktop app in development mode, use the following command:

```bash
cargo tauri dev
```

## Building

To build the desktop app for production, use the following command:

```bash
cargo tauri build
```

### System Dependencies

To build the desktop application, you will need to have the following system dependencies installed:

- `libsoup2.4-dev`
- `libjavascriptcoregtk-4.0-dev`
- `libappindicator3-dev`

You can install these on Debian-based systems using the following command:

```bash
sudo apt update && sudo apt install -y libsoup2.4-dev libjavascriptcoregtk-4.0-dev libappindicator3-dev
```

## System Tray

The application uses a system tray icon for background operation. The tray menu includes a "Quit" option to exit the application.
