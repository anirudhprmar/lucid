use futures_util::StreamExt;
use std::path::{Path, PathBuf};
use tauri::{Emitter, Manager};
use tokio::io::AsyncWriteExt;

pub fn find_existing_model(app: &tauri::AppHandle, name: &str) -> Option<PathBuf> {
    let candidates = [
        PathBuf::from("models").join(name),
        PathBuf::from("src-tauri/models").join(name),
        app.path().app_data_dir().ok()?.join("models").join(name),
    ];
    candidates.into_iter().find(|p| p.exists())
}

pub async fn download_model(
    app: tauri::AppHandle,
    url: &str,
    dest: &Path,
    model_name: &str,
) -> Result<(), String> {
    let tmp_dest = dest.with_extension("part");
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut file = tokio::fs::File::create(&tmp_dest)
        .await
        .map_err(|e| e.to_string())?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;
        if total_size > 0 {
            let percent = (downloaded as f64 / total_size as f64 * 100.0) as u32;
            let progress_event = format!("model-download-progress-{}", model_name);
            app.emit_to("main", &progress_event, percent).ok();
        }
    }

    if total_size > 0 && downloaded != total_size {
        tokio::fs::remove_file(&tmp_dest).await.ok();
        return Err(format!(
            "download incomplete: got {downloaded} of {total_size} bytes"
        ));
    }

    tokio::fs::rename(&tmp_dest, dest)
        .await
        .map_err(|e| e.to_string())?;

    let complete_event = format!("model-download-complete-{}", model_name);
    app.emit_to("main", &complete_event, ()).ok();
    Ok(())
}

pub async fn resolve_model_path(app: tauri::AppHandle, model: &str) -> Result<PathBuf, String> {
    let model_name = model;

    if let Some(path) = find_existing_model(&app, model_name) {
        return Ok(path);
    }

    if let Some(main) = app.get_webview_window("main") {
        main.show().ok();
        main.set_focus().ok();
    }

    let model_url = format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{}",
        model_name
    );

    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("models");
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| e.to_string())?;
    let dest = dir.join(model_name);

    if !dest.exists() {
        download_model(app.clone(), &model_url, &dest, model_name).await?;
    }

    Ok(dest)
}

pub fn list_downloaded_models(app: &tauri::AppHandle) -> Vec<String> {
    let mut result = Vec::new();
    let candidates = [
        PathBuf::from("models"),
        PathBuf::from("src-tauri/models"),
        app.path()
            .app_data_dir()
            .ok()
            .unwrap_or_default()
            .join("models"),
    ];

    for dir in candidates {
        if dir.exists() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name().to_string_lossy().into_owned();
                    if file_name.ends_with(".bin") && !result.contains(&file_name) {
                        result.push(file_name);
                    }
                }
            }
        }
    }

    result
}

pub async fn download_named_model(app: tauri::AppHandle, name: &str) -> Result<(), String> {
    let filename = match name {
        "tiny" => "ggml-tiny.en.bin".to_string(),
        "base.en" => "ggml-base.en.bin".to_string(),
        "small-q5_1" => "ggml-small-q5_1.bin".to_string(),
        "small.en" => "ggml-small.en.bin".to_string(),
        _ => return Err("unknown model".to_string()),
    };
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("models");
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| e.to_string())?;
    let dest = dir.join(&filename);
    let url = format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{}",
        filename
    );
    if !dest.exists() {
        download_model(app, &url, &dest, name).await?;
    }
    Ok(())
}

pub async fn delete_model(app: tauri::AppHandle, name: &str) -> Result<(), String> {
    let filename = match name {
        "tiny" => "ggml-tiny.en.bin".to_string(),
        "base.en" => "ggml-base.en.bin".to_string(),
        "small-q5_1" => "ggml-small-q5_1.bin".to_string(),
        "small.en" => "ggml-small.en.bin".to_string(),
        _ => return Err("unknown model".to_string()),
    };

    let existing_dir_path = app
        .path()
        .app_data_dir()
        .expect("failed to resolve data dir");
    let full_path = existing_dir_path.join("models");
    let dest = full_path.join(filename);

    if !dest.exists() {
        return Ok(());
    }

    tokio::fs::remove_file(&dest)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
