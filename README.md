# Lucid

<p align="center">
  <img src="public/lucid_logo.svg" alt="Lucid logo" width="128" />
</p>

<p align="center">A local, push-to-talk desktop transcription app.</p>

Lucid records while you hold a global shortcut, transcribes the audio locally with Whisper, then pastes the result into the app you were using. A compact, always-on-top indicator shows when Lucid is listening or transcribing.

## Features

- Hold <kbd>Ctrl</kbd>/<kbd>Cmd</kbd> + <kbd>Alt</kbd> + <kbd>Space</kbd> to record.
- Release the shortcut to transcribe and paste the result at the active cursor.
- Runs speech recognition locally with the English Whisper `small` model.
- Downloads the approximately 466 MB model automatically on first launch and stores it in the app data directory.
- Captures the default microphone, converts audio to mono 16 kHz, and briefly preserves then restores the clipboard after pasting.
- Lives in the system tray; open **Settings** or quit from the tray menu.

## Tech stack

- [Tauri 2](https://v2.tauri.app/) and Rust for the desktop runtime
- React, TypeScript, Vite, Tailwind CSS, and Motion for the interface
- `whisper-rs` / whisper.cpp for on-device transcription
- CPAL for microphone capture and Enigo for simulated paste input

## Getting started

### Prerequisites

- [Node.js](https://nodejs.org/) and [pnpm](https://pnpm.io/)
- A current [Rust toolchain](https://www.rust-lang.org/tools/install)
- The platform prerequisites required by [Tauri 2](https://v2.tauri.app/start/prerequisites/)

### Install and run

```bash
pnpm install
pnpm tauri dev
```

On its first run, Lucid opens the setup window while it downloads `ggml-small.en.bin`. Keep the app open until the download completes, grant microphone access if prompted, then use the global push-to-talk shortcut.

## Commands

```bash
pnpm tauri dev  # Run the desktop app in development
pnpm tauri build # Build a distributable desktop app
pnpm lint       # Lint the project
pnpm build      # Type-check and build the frontend
pnpm format     # Format supported source files
```

## Model storage

The Whisper model is searched for in the following order:

1. `models/ggml-small.en.bin`
2. `src-tauri/models/ggml-small.en.bin`
3. The app-data `models` directory

If it is not found, Lucid downloads the English Whisper small model from the `ggerganov/whisper.cpp` Hugging Face repository.

## Project structure

```text
src/                 React interface and the floating notch indicator
src-tauri/src/       Audio capture, model management, transcription, and pasting
public/              App logo and public assets
src-tauri/icons/     Platform application icons
```

## Notes

- Transcription is configured for English (`small.en`).
- The application uses the default system audio-input device.
- The floating indicator ignores mouse input and is centered at the top of the active monitor.

## License

No license has been specified for this repository.
