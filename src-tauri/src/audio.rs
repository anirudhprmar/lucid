use std::sync::{Arc, Mutex};
use cpal::{SampleFormat, Stream, traits::{DeviceTrait, HostTrait, StreamTrait}};

pub struct AudioRecorderState {
    pub stream: Option<Stream>,
    pub buffer: Arc<Mutex<Vec<f32>>>,
}

unsafe impl Send for AudioRecorderState {}
unsafe impl Sync for AudioRecorderState {}

impl Default for AudioRecorderState {
    fn default() -> Self {
        Self {
            stream: None,
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl AudioRecorderState {
    pub fn start_recording(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.stream.is_some() {
            return Ok(());
        }

        if let Ok(mut buf) = self.buffer.lock() {
            buf.clear();
        }

        let host = cpal::default_host();
        let device = host.default_input_device().ok_or("Failed to get default audio input")?;

        let config = device.default_input_config()?;

        let err_fn = |err| eprintln!("Audio Stream error: {}", err);

        let buffer_clone = self.buffer.clone();

        let stream = match config.sample_format() {
            SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &_| {
                    if let Ok(mut buf) = buffer_clone.lock() {
                        buf.extend_from_slice(data);
                    }
                },
                err_fn,
                None,
            )?,
            SampleFormat::I16 => device.build_input_stream(
                &config.into(), 
                move |data: &[i16], _: &_| {
                    if let Ok(mut buf) = buffer_clone.lock() {
                        buf.extend(data.iter().map(|&s| s as f32 / i16::MAX as f32));
                    }
                },
                err_fn,
                None,
            )?,
            SampleFormat::U16 => device.build_input_stream(
                &config.into(), 
                move |data: &[u16], _: &_| {
                    if let Ok(mut buf) = buffer_clone.lock() {
                        buf.extend(data.iter().map(|&s| (s as f32 - 32768.0) / 32768.0));
                    }
                },
                err_fn,
                None,
            )?,
            format => return Err(format!("Unsupported sample format: {:?}", format).into()),
        };

        stream.play()?;
        self.stream = Some(stream);
        println!("recording started");
        Ok(())
    }
    
    pub fn stop_recording(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
            println!("recording stopped");
        }

        if let Ok(buf) = self.buffer.lock() {
            println!("Captured raw audio buffer: {} samples in memory", buf.len());
        }  
    }
}