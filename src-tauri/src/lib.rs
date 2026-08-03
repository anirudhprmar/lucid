mod audio;
mod model;
mod paste;
mod transcriber;

use audio::AudioRecorderState;
use std::sync::{Arc, Mutex};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use transcriber::WhisperTranscriber;

#[tauri::command]
fn check_model_exists(app: tauri::AppHandle) -> bool {
    const MODEL_NAME: &str = "ggml-small.en.bin";
    model::find_existing_model(&app, MODEL_NAME).is_some()
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
                            if let Err(e) = app.emit("notch-state", "listening") {
                                eprintln!("Failed to emit notch state listening: {}", e);
                            }

                            if let Err(e) = recorder.start_recording() {
                                eprintln!("Failed to start recording: {}", e);
                            }
                        }
                        ShortcutState::Released => {
                            let pcm_data = recorder.stop_recording_and_extract_pcm16k();
                            if pcm_data.is_empty() {
                                println!("No audio recorded.");
                                if let Err(e) = app.emit("notch-state", "idle") {
                                    eprintln!("Failed to emit notch state idle: {}", e);
                                }
                                return;
                            }

                            if let Err(e) = app.emit("notch-state", "transcribing") {
                                eprintln!("Failed to emit notch state transcribing: {}", e);
                            }

                            let state = app.try_state::<Arc<WhisperTranscriber>>();
                            if state.is_none() {
                                app.emit("notch-state", "not-ready").ok();
                                return;
                            }

                            if let Some(transcriber) = app.try_state::<Arc<WhisperTranscriber>>() {
                                let transcriber = transcriber.inner().clone();
                                let app_handle = app.clone();
                                std::thread::spawn(move || {
                                    match transcriber.transcribe(&pcm_data) {
                                        Ok(text) => {
                                            if !text.is_empty() {
                                                if let Err(e) = paste::paste_text(&text) {
                                                    eprintln!("Failed to paste: {}", e);
                                                }
                                            }
                                        }
                                        Err(e) => eprintln!("Transcription error: {}", e),
                                    }

                                    if let Err(e) = app_handle.emit("notch-state", "idle") {
                                        eprintln!("Failed to emit notch state idle: {}", e);
                                    }
                                });
                            } else {
                                eprintln!("WhisperTranscriber state is not available.");
                                if let Err(e) = app.emit("notch-state", "idle") {
                                    eprintln!("Failed to emit notch state idle: {}", e);
                                }
                            }
                        }
                    }
                })
                .build(),
        )
        .manage(Mutex::new(AudioRecorderState::default()))
        .setup(|app| {
            let notch = app.get_webview_window("notch").unwrap();
            let monitor = notch.current_monitor()?.unwrap();
            let screen_width = monitor.size().width;
            let x = (screen_width - 300) / 2;
            notch.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                x: x as i32,
                y: 0,
            }))?;
            notch.set_ignore_cursor_events(true)?;

            let main = app.get_webview_window("main").unwrap();
            main.hide()?;

            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit, &settings_item])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| {
                    if event.id() == "quit" {
                        app.exit(0);
                    } else if event.id() == "settings" {
                        if let Some(main) = app.get_webview_window("main") {
                            main.show().ok();
                            main.set_focus().ok();
                        }
                    }
                })
                .build(app)?;

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match model::resolve_model_path(handle.clone()).await {
                    Ok(model_path) => match WhisperTranscriber::new(&model_path) {
                        Ok(transcriber) => {
                            println!("Successfully loaded Whisper model from: {:?}", model_path);
                            handle.manage(Arc::new(transcriber));
                        }
                        Err(e) => {
                            eprintln!("Failed to initialize Whisper model: {}", e);
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to resolve model path: {}", e);
                    }
                }
            });

            let hotkey = "CmdOrCtrl+Alt+Space";
            if let Err(err) = app.global_shortcut().register(hotkey) {
                eprintln!(
                    "Warning: Failed to register shortcut '{}': {:?}",
                    hotkey, err
                );
            } else {
                println!("Successfully registered Push-To-Talk shortcut ({})", hotkey);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![check_model_exists])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
