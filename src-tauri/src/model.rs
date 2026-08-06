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

pub async fn download_model(app: tauri::AppHandle, url: &str, dest: &Path) -> Result<(), String> {
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
            app.emit_to("main", "model-download-progress", percent).ok();
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

    app.emit_to("main", "model-download-complete", ()).ok();
    Ok(())
}

pub async fn resolve_model_path(app: tauri::AppHandle) -> Result<PathBuf, String> {
    const MODEL_NAME: &str = "ggml-small-q5_1.bin";

    if let Some(path) = find_existing_model(&app, MODEL_NAME) {
        return Ok(path);
    }

    if let Some(main) = app.get_webview_window("main") {
        main.show().ok();
        main.set_focus().ok();
    }

    const MODEL_URL: &str =
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small-q5_1.bin";

    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("models");
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| e.to_string())?;
    let dest = dir.join(MODEL_NAME);

    if !dest.exists() {
        download_model(app.clone(), MODEL_URL, &dest).await?;
    }

    Ok(dest)
}
