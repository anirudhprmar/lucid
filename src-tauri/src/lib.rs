mod audio;

use std::sync::Mutex;
use audio::AudioRecorderState;
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
                            recorder.stop_recording();
                        }
                    }
                })
                .build(),
        )
        .manage(Mutex::new(AudioRecorderState::default()))
        .setup(|app| {
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

