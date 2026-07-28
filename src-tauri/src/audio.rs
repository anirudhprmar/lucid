use std::sync::{Arc, Mutex};
use cpal::{SampleFormat, Stream, traits::{DeviceTrait, HostTrait, StreamTrait}};

pub struct AudioRecorderState {
    pub stream: Option<Stream>,
    pub buffer: Arc<Mutex<Vec<f32>>>,
    pub sample_rate: u32,
    pub channels: u16
}

unsafe impl Send for AudioRecorderState {}
unsafe impl Sync for AudioRecorderState {}

impl Default for AudioRecorderState {
    fn default() -> Self {
        Self {
            stream: None,
            buffer: Arc::new(Mutex::new(Vec::new())),
            sample_rate: 16000,
            channels: 1
        }
    }
}

pub fn to_mono(interleaved: &[f32], channels: u16) -> Vec<f32> {
    if channels <= 1 {
        return interleaved.to_vec();
    }
    let ch = channels as usize;
    interleaved.chunks_exact(ch).map(|chunk| chunk.iter().sum::<f32>() / ch as f32).collect()
}

pub fn resample_to_16k(mono_samples: &[f32], src_rate: u32) -> Vec<f32> {
    if src_rate == 16000 || mono_samples.is_empty() {
        return mono_samples.to_vec();
    }

    let ratio = src_rate as f32 / 16000.0;
    let target_len = ((mono_samples.len() as f32) / ratio).floor() as usize;
    let mut output = Vec::with_capacity(target_len);

    for i in 0..target_len {
        let src_index = i as f32 * ratio;
        let index0 = src_index.floor() as usize;
        let index1 = (index0 + 1).min(mono_samples.len() - 1);
        let weight = src_index - index0 as f32;

        let sample = (1.0 - weight) * mono_samples[index0] + weight * mono_samples[index1];
        output.push(sample);
    }

    output
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

        self.sample_rate = config.sample_rate().0;
        self.channels = config.channels();

        println!("Recording started using {} Hz, {} channels(s)", self.sample_rate, self.channels);

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
    
    #[allow(dead_code)]
    pub fn stop_recording(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
            println!("recording stopped");
        }

        if let Ok(buf) = self.buffer.lock() {
            println!("Captured raw audio buffer: {} samples in memory", buf.len());
        }  
    }

    pub fn stop_recording_and_extract_pcm16k(&mut self) -> Vec<f32> {
        if let Some(stream) = self.stream.take() {
            drop(stream);
            println!("Recording stopped");
        }

        let raw_data = if let Ok(mut buf) = self.buffer.lock() {
            std::mem::take(&mut *buf)
        } else {
            Vec::new()
        };

        println!("Captured raw audio: {} samples", raw_data.len());
        
        let mono_samples = to_mono(&raw_data, self.channels);
        let pcm16k_samples = resample_to_16k(&mono_samples, self.sample_rate);

        println!("Extracted {} PCM16k samples", pcm16k_samples.len());

        pcm16k_samples
    }
}