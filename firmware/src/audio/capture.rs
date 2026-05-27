/// Audio Capture Driver for SPH0645 MEMS Microphone
/// 
/// Handles I2S data capture from digital microphone
/// Uses DMA for efficient streaming with minimal CPU overhead

use heapless::Vec;
use defmt::*;
use crate::hal::i2s::{I2sDriver, I2sConfig, SampleRate, Channels, I2sMode, I2sError};

pub const CAPTURE_BUFFER_SIZE: usize = 512; // 2 frames @ 16kHz
pub const FRAME_SIZE: usize = 256; // 16ms @ 16kHz

/// SPH0645 Microphone Driver
pub struct AudioCapture {
    i2s: I2sDriver,
    buffer: Vec<i16, CAPTURE_BUFFER_SIZE>,
    read_pos: usize,
    write_pos: usize,
}

impl AudioCapture {
    /// Create new audio capture driver
    pub fn new() -> Result<Self, I2sError> {
        let config = I2sConfig {
            sample_rate: SampleRate::Hz16000,
            channels: Channels::Mono,
            mode: I2sMode::Master,
        };

        let i2s = I2sDriver::new(config);

        info!("🎤 AudioCapture initialized");
        info!("   Sample rate: 16 kHz");
        info!("   Channels: Mono");
        info!("   Buffer size: {} samples", CAPTURE_BUFFER_SIZE);

        Ok(Self {
            i2s,
            buffer: Vec::new(),
            read_pos: 0,
            write_pos: 0,
        })
    }

    /// Start capturing audio
    pub async fn start(&mut self) -> Result<(), I2sError> {
        self.i2s.start_capture(&mut self.buffer).await?;
        info!("▶ Audio capture started");
        Ok(())
    }

    /// Stop capturing audio
    pub async fn stop(&mut self) -> Result<(), I2sError> {
        self.i2s.stop_capture().await?;
        info!("⏹ Audio capture stopped");
        Ok(())
    }

    /// Read a frame of audio data (256 samples)
    /// Blocks until frame is available
    pub fn read_frame(&mut self) -> Option<&[i16; FRAME_SIZE]> {
        // Simplified for Phase 1 — actual implementation would use DMA interrupts
        if self.read_pos + FRAME_SIZE <= self.buffer.len() {
            let frame_start = self.read_pos;
            self.read_pos += FRAME_SIZE;

            // Safety: We know we have FRAME_SIZE samples available
            if let Some(frame_ptr) = self.buffer.as_ptr().wrapping_add(frame_start) as *const [i16; FRAME_SIZE] {
                unsafe { Some(&*frame_ptr) }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get current microphone input level (RMS in dBFS)
    pub fn input_level(&self) -> f32 {
        let sum_sq: i64 = self
            .buffer
            .iter()
            .map(|&s| (s as i64) * (s as i64))
            .sum();

        let rms = ((sum_sq as f32) / (self.buffer.len() as f32)).sqrt();

        // Convert to dBFS (Full Scale = 32768)
        20.0 * (rms / 32768.0).log10()
    }

    /// Get peak level (maximum absolute value in dBFS)
    pub fn peak_level(&self) -> f32 {
        let max_abs = self
            .buffer
            .iter()
            .map(|&s| s.abs() as i32)
            .max()
            .unwrap_or(0);

        20.0 * (max_abs as f32 / 32768.0).log10()
    }
}

/// Helper: Measure input level from a frame
pub fn measure_frame_level(frame: &[i16]) -> FrameLevel {
    let sum_sq: i64 = frame.iter().map(|&s| (s as i64) * (s as i64)).sum();
    let rms = ((sum_sq as f32) / (frame.len() as f32)).sqrt();
    let rms_db = 20.0 * (rms / 32768.0).log10();

    let peak_abs = frame.iter().map(|&s| s.abs() as i32).max().unwrap_or(0);
    let peak_db = 20.0 * (peak_abs as f32 / 32768.0).log10();

    FrameLevel {
        rms_db,
        peak_db,
        peak_linear: peak_abs as i16,
    }
}

pub struct FrameLevel {
    pub rms_db: f32,
    pub peak_db: f32,
    pub peak_linear: i16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_creation() {
        let result = AudioCapture::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_measure_silence() {
        let silence = [0i16; 256];
        let level = measure_frame_level(&silence);

        // Silence should be very low or -inf dB
        assert!(level.rms_db < -100.0);
        assert!(level.peak_db < -100.0);
    }

    #[test]
    fn test_measure_tone() {
        // Generate 1kHz sine wave at -6dBFS
        let mut frame = [0i16; 256];
        let amplitude = (0.5 * 32768.0) as i16; // -6dBFS
        
        for (i, sample) in frame.iter_mut().enumerate() {
            let freq = 1000.0 / 16000.0; // 1kHz @ 16kHz
            let phase = 2.0 * std::f32::consts::PI * freq * (i as f32);
            *sample = (amplitude as f32 * phase.sin()) as i16;
        }

        let level = measure_frame_level(&frame);
        
        // Should be close to -6dBFS
        assert!((level.peak_db + 6.0).abs() < 0.5);
    }
}
