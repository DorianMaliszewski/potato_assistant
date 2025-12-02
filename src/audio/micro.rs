use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::fmt;
use std::sync::{Arc, Mutex};

// 1. Structure to manage the microphone state
pub struct AudioRecorder {
    stream: Option<cpal::Stream>,
    // Thread-safe buffer to store audio samples
    // Arc = Shared ownership, Mutex = Safe access from multiple threads
    audio_buffer: Arc<Mutex<Vec<f32>>>,
    config: cpal::StreamConfig,
    pub recording: bool,
}

impl fmt::Debug for AudioRecorder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioRecorder")
            .field("config", &self.config)
            .field(
                "audio_buffer",
                &format!(
                    "Buffer holding {} samples",
                    self.audio_buffer.lock().unwrap().len()
                ),
            )
            // C'est ici l'astuce :
            // On vérifie si le stream existe, et on écrit un texte à la place de l'objet
            .field(
                "stream",
                &if self.stream.is_some() {
                    "Recording (Active)"
                } else {
                    "Idle (None)"
                },
            )
            .finish()
    }
}

impl AudioRecorder {
    pub fn new() -> Result<Self, String> {
        let host = cpal::default_host();

        // Get default input device (Microphone)
        let device = host
            .default_input_device()
            .ok_or("No input device available")?;

        // Get default config (Sample rate, channels...)
        let config: cpal::StreamConfig = device
            .default_input_config()
            .map_err(|e| e.to_string())?
            .into();

        Ok(Self {
            stream: None,
            audio_buffer: Arc::new(Mutex::new(Vec::new())),
            config,
            recording: false,
        })
    }

    // 2. Start Recording
    pub fn start(&mut self) -> Result<(), String> {
        let host = cpal::default_host();
        let device = host.default_input_device().ok_or("No input device")?;

        // Clone the arc to pass it into the audio thread closure
        let buffer_clone = self.audio_buffer.clone();

        // Reset buffer
        buffer_clone.lock().unwrap().clear();

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        // Build the input stream
        // This closure runs continuously on a separate thread!
        let stream = device
            .build_input_stream(
                &self.config,
                move |data: &[f32], _: &_| {
                    // Determine silence/noise threshold (simple gate)
                    // This prevents recording pure silence, but optional.

                    // Write data to the shared buffer
                    if let Ok(mut buffer) = buffer_clone.lock() {
                        buffer.extend_from_slice(data);
                    }
                },
                err_fn,
                None, // None = blocking timeout
            )
            .map_err(|e| e.to_string())?;

        stream.play().map_err(|e| e.to_string())?;
        self.stream = Some(stream);
        self.recording = true;

        println!("🎙️ Recording started...");
        Ok(())
    }

    // 3. Stop Recording and return WAV data
    pub fn stop(&mut self) -> Result<Vec<u8>, String> {
        // Drop the stream to stop recording hardware access
        self.stream = None;
        self.recording = false;
        println!("🛑 Recording stopped.");

        // Retrieve the raw data
        let raw_samples = self.audio_buffer.lock().unwrap().clone();

        if raw_samples.is_empty() {
            return Err("No audio recorded".to_string());
        }

        // Convert raw f32 samples to WAV format (bytes)
        // OpenAI API requires a valid file format (WAV, MP3), not raw PCM
        self.create_wav_in_memory(&raw_samples)
    }

    // Helper: Converts raw f32 samples to a WAV byte array
    fn create_wav_in_memory(&self, samples: &[f32]) -> Result<Vec<u8>, String> {
        let spec = hound::WavSpec {
            channels: self.config.channels,
            sample_rate: self.config.sample_rate.0,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        // Create a cursor (in-memory file)
        let mut cursor = std::io::Cursor::new(Vec::new());

        {
            let mut writer = hound::WavWriter::new(&mut cursor, spec).map_err(|e| e.to_string())?;

            for &sample in samples {
                // Convert f32 (-1.0 to 1.0) to i16 (-32768 to 32767)
                let amplitude = i16::MAX as f32;
                let sample_i16 = (sample * amplitude) as i16;
                writer.write_sample(sample_i16).map_err(|e| e.to_string())?;
            }
            writer.finalize().map_err(|e| e.to_string())?;
        }

        // Return the inner buffer (the WAV file bytes)
        Ok(cursor.into_inner())
    }

    // Dans audio/micro.rs
    // Ajoute cette fonction helper pour convertir de InputRate -> 16000Hz
    fn resample_to_16k(input_data: &[f32], input_rate: u32) -> Vec<f32> {
        if input_rate == 16000 {
            return input_data.to_vec();
        }

        // Calcul simple ou utilisation de la crate 'rubato' pour la qualité pro.
        // Pour un "Potato Assistant", une méthode naïve (sauter des échantillons)
        // peut marcher si la qualité est bonne, mais l'interpolation linéaire est mieux.

        let ratio = input_rate as f32 / 16000.0;
        let new_len = (input_data.len() as f32 / ratio) as usize;
        let mut output = Vec::with_capacity(new_len);

        for i in 0..new_len {
            let index = (i as f32 * ratio) as usize;
            if index < input_data.len() {
                output.push(input_data[index]);
            }
        }

        output
    }
}
