mod audio;
mod model;
mod paste;
mod transcriber;

use audio::AudioRecorderState;
use std::sync::{Arc, Mutex, RwLock};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use tauri_plugin_store::StoreExt;
use transcriber::WhisperTranscriber;

#[derive(Clone, Default)]
pub struct TranscriberState {
    pub transcriber: Arc<RwLock<Option<WhisperTranscriber>>>,
    pub current_model_path: Arc<RwLock<Option<String>>>,
}

#[tauri::command]
fn check_model_exists(app: tauri::AppHandle) -> bool {
    const MODEL_NAME: &str = "ggml-small-q5_1.bin";
    model::find_existing_model(&app, MODEL_NAME).is_some()
}

#[tauri::command]
fn get_current_model(app: tauri::AppHandle) -> Option<String> {
    if let Some(state) = app.try_state::<TranscriberState>() {
        if let Ok(lock) = state.current_model_path.read() {
            return lock.clone();
        }
    }
    None
}

#[tauri::command]
fn list_downloaded_models(app: tauri::AppHandle) -> Vec<String> {
    model::list_downloaded_models(&app)
}

#[tauri::command]
async fn download_named_model(app: tauri::AppHandle, name: String) -> Result<(), String> {
    model::download_named_model(app, &name).await
}

#[tauri::command]
async fn delete_model(app: tauri::AppHandle, name: String) -> Result<(), String> {
    model::delete_model(app, &name).await
}

#[tauri::command]
async fn switch_active_model(app: tauri::AppHandle, name: String) -> Result<(), String> {
    model::switch_model(app, &name).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    let ptt_shortcut: tauri_plugin_global_shortcut::Shortcut =
                        "CmdOrCtrl+Alt+Space".parse().unwrap();
                    let notch_toggle_shortcut: tauri_plugin_global_shortcut::Shortcut =
                        "CmdOrCtrl+Alt+N".parse().unwrap();

                    if *shortcut == ptt_shortcut {
                        // --- Push-to-Talk handler ---
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

                                let state = app.try_state::<TranscriberState>();
                                if state.is_none() {
                                    app.emit("notch-state", "not-ready").ok();
                                    return;
                                }

                                if let Some(trans_state) = app.try_state::<TranscriberState>() {
                                    let trans_state = trans_state.inner().clone();
                                    let app_handle = app.clone();
                                    std::thread::spawn(move || {
                                        let transcribe_result = {
                                            if let Ok(lock) = trans_state.transcriber.read() {
                                                if let Some(ref transcriber) = *lock {
                                                    transcriber.transcribe(&pcm_data)
                                                } else {
                                                    Err("Transcriber model is not loaded".into())
                                                }
                                            } else {
                                                Err("Failed to acquire transcriber read lock"
                                                    .into())
                                            }
                                        };

                                        match transcribe_result {
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
                                    eprintln!("TranscriberState is not available.");
                                    if let Err(e) = app.emit("notch-state", "idle") {
                                        eprintln!("Failed to emit notch state idle: {}", e);
                                    }
                                }
                            }
                        }
                    } else if *shortcut == notch_toggle_shortcut {
                        if event.state() == ShortcutState::Pressed {
                            if let Some(notch) = app.get_webview_window("notch") {
                                let is_visible = notch.is_visible().unwrap_or(true);
                                if is_visible {
                                    notch.hide().ok();
                                } else {
                                    notch.show().ok();
                                }

                                if let Ok(store) = app.store("settings.json") {
                                    store.set("notch_visible", serde_json::json!(!is_visible));
                                }
                            }
                        }
                    }
                })
                .build(),
        )
        .manage(Mutex::new(AudioRecorderState::default()))
        .manage(TranscriberState::default())
        .setup(|app| {
            let notch = app.get_webview_window("notch").unwrap();
            if let Ok(Some(monitor)) = notch.current_monitor() {
                let scale_factor = monitor.scale_factor();
                let logical_width = monitor.size().width as f64 / scale_factor;
                let x = (logical_width - 300.0) / 2.0;
                notch.set_position(tauri::Position::Logical(tauri::LogicalPosition {
                    x,
                    y: 0.0,
                }))?;
            }
            notch.set_ignore_cursor_events(true)?;

            let main = app.get_webview_window("main").unwrap();
            main.hide()?;

            app.store("usage.json")?;
            app.store("settings.json")?;

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
                let saved_model = {
                    if let Ok(store) = handle.store("settings.json") {
                        store
                            .get("active_model")
                            .and_then(|v| v.as_str().map(|s| s.to_string()))
                    } else {
                        None
                    }
                };

                let active_name = saved_model.as_deref().unwrap_or("small-q5_1");
                let filename =
                    model::model_name_to_filename(active_name).unwrap_or("ggml-small-q5_1.bin");

                match model::resolve_model_path(handle.clone(), filename).await {
                    Ok(model_path) => match WhisperTranscriber::new(&model_path) {
                        Ok(transcriber) => {
                            println!("Successfully loaded Whisper model from: {:?}", model_path);
                            if let Some(state) = handle.try_state::<TranscriberState>() {
                                if let Ok(mut lock) = state.transcriber.write() {
                                    *lock = Some(transcriber);
                                }
                                if let Ok(mut path_lock) = state.current_model_path.write() {
                                    *path_lock = Some(model_path.display().to_string());
                                }
                            }
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

            let ptt_hotkey = "CmdOrCtrl+Alt+Space";
            if let Err(err) = app.global_shortcut().register(ptt_hotkey) {
                eprintln!(
                    "Warning: Failed to register shortcut '{}': {:?}",
                    ptt_hotkey, err
                );
            } else {
                println!(
                    "Successfully registered Push-To-Talk shortcut ({})",
                    ptt_hotkey
                );
            }

            let notch_hotkey = "CmdOrCtrl+Alt+N";
            if let Err(err) = app.global_shortcut().register(notch_hotkey) {
                eprintln!(
                    "Warning: Failed to register shortcut '{}': {:?}",
                    notch_hotkey, err
                );
            } else {
                println!(
                    "Successfully registered Toggle Notch shortcut ({})",
                    notch_hotkey
                );
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_model_exists,
            get_current_model,
            list_downloaded_models,
            download_named_model,
            delete_model,
            switch_active_model
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
