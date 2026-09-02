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

use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
pub struct ActiveRecordingSession(pub Mutex<Option<Arc<AtomicBool>>>);

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
                        match event.state() {
                            ShortcutState::Pressed => {
                                let has_model = app.try_state::<TranscriberState>().map_or(false, |s| {
                                    s.transcriber.read().map(|l| l.is_some()).unwrap_or(false)
                                });

                                if !has_model {
                                    let _ = app.emit("notch-state", "not-ready");
                                    return;
                                }

                                let flag = Arc::new(AtomicBool::new(true));
                                {
                                    let session = app.state::<ActiveRecordingSession>();
                                    let mut current = session.0.lock().unwrap();
                                    if current.is_some() {
                                        // Already recording — ignore OS auto-repeat events while holding shortcut
                                        return;
                                    }
                                    *current = Some(flag.clone());
                                }

                                let audio_state = app.state::<Mutex<AudioRecorderState>>();
                                if let Ok(mut recorder) = audio_state.lock() {
                                    if let Err(e) = recorder.start_recording() {
                                        eprintln!("Failed to start recording: {}", e);
                                        let _ = app.emit("notch-state", "idle");
                                        let session = app.state::<ActiveRecordingSession>();
                                        if let Ok(mut current) = session.0.lock() {
                                            current.take();
                                        }
                                        return;
                                    }
                                }

                                if let Err(e) = app.emit("notch-state", "listening") {
                                    eprintln!("Failed to emit notch state listening: {}", e);
                                }

                                    let app_handle = app.clone();
                                    let is_rec = flag.clone();
                                    std::thread::spawn(move || {
                                        let mut local_pcm_buffer: Vec<f32> = Vec::new();
                                        let mut chunk_index: usize = 0;
                                        // 2.0 seconds chunk @ 16kHz
                                        const CHUNK_SAMPLES: usize = 32000;

                                        while is_rec.load(Ordering::SeqCst) {
                                            std::thread::sleep(std::time::Duration::from_millis(150));

                                            if !is_rec.load(Ordering::SeqCst) {
                                                break;
                                            }

                                            let new_samples = {
                                                if let Some(state) = app_handle.try_state::<Mutex<AudioRecorderState>>() {
                                                    if let Ok(mut rec) = state.lock() {
                                                        rec.extract_available_pcm16k()
                                                    } else {
                                                        Vec::new()
                                                    }
                                                } else {
                                                    Vec::new()
                                                }
                                            };

                                            if !new_samples.is_empty() {
                                                local_pcm_buffer.extend(new_samples);
                                            }

                                            if local_pcm_buffer.len() >= CHUNK_SAMPLES {
                                                let chunk: Vec<f32> = local_pcm_buffer.drain(..CHUNK_SAMPLES).collect();
                                                if let Some(trans_state) = app_handle.try_state::<TranscriberState>() {
                                                    if let Ok(lock) = trans_state.transcriber.read() {
                                                        if let Some(ref transcriber) = *lock {
                                                            if let Ok(text) = transcriber.transcribe(&chunk) {
                                                                let trimmed = text.trim();
                                                                if !trimmed.is_empty() && !trimmed.starts_with('[') && !trimmed.starts_with('(') {
                                                                    let formatted = if chunk_index == 0 {
                                                                        format!("{}", trimmed)
                                                                    } else {
                                                                        format!(" {}", trimmed)
                                                                    };
                                                                    chunk_index += 1;
                                                                    if let Err(e) = paste::paste_text(&formatted) {
                                                                        eprintln!("Failed to stream paste: {}", e);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // Key released -> drain final audio samples
                                        let final_samples = {
                                            if let Some(state) = app_handle.try_state::<Mutex<AudioRecorderState>>() {
                                                if let Ok(mut rec) = state.lock() {
                                                    rec.stop_recording_and_extract_pcm16k()
                                                } else {
                                                    Vec::new()
                                                }
                                            } else {
                                                Vec::new()
                                            }
                                        };

                                        if !final_samples.is_empty() {
                                            local_pcm_buffer.extend(final_samples);
                                        }

                                        if local_pcm_buffer.len() >= 4000 {
                                            if let Some(trans_state) = app_handle.try_state::<TranscriberState>() {
                                                if let Ok(lock) = trans_state.transcriber.read() {
                                                    if let Some(ref transcriber) = *lock {
                                                        if let Ok(text) = transcriber.transcribe(&local_pcm_buffer) {
                                                            let trimmed = text.trim();
                                                            if !trimmed.is_empty() && !trimmed.starts_with('[') && !trimmed.starts_with('(') {
                                                                let formatted = if chunk_index > 0 {
                                                                    format!(" {}", trimmed)
                                                                } else {
                                                                    format!("{}", trimmed)
                                                                };
                                                                if let Err(e) = paste::paste_text(&formatted) {
                                                                    eprintln!("Failed to paste tail chunk: {}", e);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        if let Err(e) = app_handle.emit("notch-state", "idle") {
                                            eprintln!("Failed to emit notch state idle: {}", e);
                                        }
                                    });
                                }
                            ShortcutState::Released => {
                                if let Some(session) = app.try_state::<ActiveRecordingSession>() {
                                    if let Ok(mut current) = session.0.lock() {
                                        if let Some(flag) = current.take() {
                                            flag.store(false, Ordering::SeqCst);
                                        }
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
        .manage(ActiveRecordingSession::default())
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
