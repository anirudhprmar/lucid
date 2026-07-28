mod audio;
mod transcriber;

use std::sync::{Arc, Mutex};
use audio::AudioRecorderState;
use transcriber::WhisperTranscriber;
use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    let audio_state = app.state::<Mutex<AudioRecorderState>>();
                    let mut recorder = audio_state.lock().unwrap();

                    match event.state() {
                        ShortcutState::Pressed => {
                            if let Err(e) = recorder.start_recording() {
                                eprintln!("Failed to start recording: {}", e);
                            }
                        }
                        ShortcutState::Released => {
                            let pcm_data = recorder.stop_recording_and_extract_pcm16k();
                            if pcm_data.is_empty() {
                                println!("No audio recorded.");
                                return;
                            }

                            if let Some(transcriber) = app.try_state::<Arc<WhisperTranscriber>>() {
                                let transcriber = transcriber.inner().clone();
                                std::thread::spawn(move || {
                                    println!("Transcribing audio...");
                                    match transcriber.transcribe(&pcm_data) {
                                        Ok(text) => {
                                            if text.is_empty() {
                                                println!("\n=== TRANSCRIPTION ===\n[No speech detected]\n=====================\n");
                                            } else {
                                                println!("\n=== TRANSCRIPTION ===\n{}\n=====================\n", text);
                                            }
                                        }
                                        Err(e) => eprintln!("Transcription error: {}", e),
                                    }
                                });
                            } else {
                                eprintln!("WhisperTranscriber state is not available.");
                            }
                        }
                    }
                })
                .build(),
        )
        .manage(Mutex::new(AudioRecorderState::default()))
        .setup(|app| {
            let model_path = if std::path::Path::new("models/ggml-small.en.bin").exists() {
                "models/ggml-small.en.bin"
            } else if std::path::Path::new("src-tauri/models/ggml-small.en.bin").exists() {
                "src-tauri/models/ggml-small.en.bin"
            } else {
                eprintln!("Warning: GGML model not found at models/ggml-base.en.bin!");
                "models/ggml-base.en.bin"
            };

            match WhisperTranscriber::new(model_path) {
                Ok(transcriber) => {
                    println!("Successfully loaded Whisper model from: {}", model_path);
                    app.manage(Arc::new(transcriber));
                }
                Err(e) => {
                    eprintln!("Failed to initialize Whisper model: {}", e);
                }
            }

            let hotkey = "CmdOrCtrl+Alt+Space";
            if let Err(err) = app.global_shortcut().register(hotkey) {
                eprintln!("Warning: Failed to register shortcut '{}': {:?}", hotkey, err);
            } else {
                println!("Successfully registered Push-To-Talk shortcut ({})", hotkey);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
