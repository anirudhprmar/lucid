use std::path::Path;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct WhisperTranscriber {
    context: WhisperContext,
}

unsafe impl Send for WhisperTranscriber {}
unsafe impl Sync for WhisperTranscriber {}

impl WhisperTranscriber {
    pub fn new<P: AsRef<Path>>(model_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = model_path.as_ref();
        if !path.exists() {
            return Err(format!("Whisper model file not found at: {:?}", path).into());
        }

        let path_str = path
            .to_str()
            .ok_or_else(|| "Invalid UTF-8 string in model path".to_string())?;

        let context = WhisperContext::new_with_params(path_str, WhisperContextParameters::default())?;

        Ok(Self { context })
    }

    pub fn transcribe(&self, pcm_16k_mono: &[f32]) -> Result<String, Box<dyn std::error::Error>> {
        if pcm_16k_mono.is_empty() {
            return Ok(String::new());
        }

        let mut state = self.context.create_state()?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        let threads = std::thread::available_parallelism()
            .map(|n| n.get() as i32)
            .unwrap_or(4);
        params.set_n_threads(threads);

        state.full(params, pcm_16k_mono)?;

        let mut text = String::new();
        for segment in state.as_iter() {
            text.push_str(&segment.to_string());
        }

        Ok(text.trim().to_string())
    }
}
