/// Audio Playback Driver for TPA6132A2 Class-G Amplifier
/// 
/// Handles I2S data output to headphone amplifier
/// Manages amplifier enable/disable and volume control

use heapless::Vec;
use defmt::*;
use crate::hal::i2s::{I2sDriver, I2sConfig, SampleRate, Channels, I2sMode, I2sError};

pub const PLAYBACK_BUFFER_SIZE: usize = 512; // 2 frames @ 16kHz
pub const FRAME_SIZE: usize = 256; // 16ms @ 16kHz
pub const MAX_VOLUME: i8 = 20; // dBFS

/// TPA6132A2 Headphone Amplifier Driver
pub struct AudioPlayback {
    i2s: I2sDriver,
    buffer: Vec<i16, PLAYBACK_BUFFER_SIZE>,
    read_pos: usize,
    write_pos: usize,
    volume_gain_db: f32,
    enabled: bool,
}

impl AudioPlayback {
    /// Create new audio playback driver
    pub fn new() -> Result<Self, I2sError> {
        let config = I2sConfig {
            sample_rate: SampleRate::Hz16000,
            channels: Channels::Mono,
            mode: I2sMode::Master,
        };

        let i2s = I2sDriver::new(config);

        info!("🔊 AudioPlayback initialized");
        info!("   Sample rate: 16 kHz");
        info!("   Channels: Mono");
        info!("   Max output: 25 mW/channel");
        info!("   Buffer size: {} samples", PLAYBACK_BUFFER_SIZE);

        Ok(Self {
            i2s,
            buffer: Vec::new(),
            read_pos: 0,
            write_pos: 0,
            volume_gain_db: 0.0,
            enabled: false,
        })
    }

    /// Start audio playback
    pub async fn start(&mut self) -> Result<(), I2sError> {
        self.i2s.start_playback(&self.buffer).await?;
        self.enabled = true;
        info!("▶ Audio playback started");
        Ok(())
    }

    /// Stop audio playback
    pub async fn stop(&mut self) -> Result<(), I2sError> {
        self.i2s.stop_playback().await?;
        self.enabled = false;
        info!("⏹ Audio playback stopped");
        Ok(())
    }

    /// Write a frame of audio data (256 samples) to playback buffer
    pub fn write_frame(&mut self, frame: &[i16; FRAME_SIZE]) -> Result<(), PlaybackError> {
        if self.write_pos + FRAME_SIZE > PLAYBACK_BUFFER_SIZE {
            return Err(PlaybackError::BufferFull);
        }

        // Apply volume gain before writing
        for &sample in frame.iter() {
            let gain_linear = 10.0_f32.powf(self.volume_gain_db / 20.0);
            let scaled = (sample as f32 * gain_linear) as i16;

            if self.buffer.push(scaled).is_err() {
                return Err(PlaybackError::BufferFull);
            }
        }

        self.write_pos += FRAME_SIZE;
        Ok(())
    }

    /// Set playback volume in dB (0dB = unity, -20dB = quiet, +20dB = loud)
    pub fn set_volume(&mut self, gain_db: f32) {
        let clamped = gain_db.max(-40.0).min(20.0);
        self.volume_gain_db = clamped;
        info!("🔊 Volume set to {} dB", clamped);
    }

    /// Get current volume setting
    pub fn volume(&self) -> f32 {
        self.volume_gain_db
    }

    /// Enable amplifier (GPIO control)
    pub fn enable(&mut self) {
        if !self.enabled {
            // In real implementation: Set GPIO pin high to enable TPA6132A2
            self.enabled = true;
            info!("✅ Amplifier enabled");
        }
    }

    /// Disable amplifier (GPIO control)
    pub fn disable(&mut self) {
        if self.enabled {
            // In real implementation: Set GPIO pin low to disable TPA6132A2
            self.enabled = false;
            info!("❌ Amplifier disabled");
        }
    }

    /// Check if amplifier is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get remaining space in buffer (in samples)
    pub fn buffer_available(&self) -> usize {
        PLAYBACK_BUFFER_SIZE - self.buffer.len()
    }

    /// Get current buffer fill level (0.0 = empty, 1.0 = full)
    pub fn buffer_level(&self) -> f32 {
        self.buffer.len() as f32 / PLAYBACK_BUFFER_SIZE as f32
    }
}

#[derive(Debug)]
pub enum PlaybackError {
    BufferFull,
    NotEnabled,
}

/// Helper: Apply volume to a frame in-place
pub fn apply_volume(frame: &mut [i16], gain_db: f32) {
    let gain_linear = 10.0_f32.powf(gain_db / 20.0);

    for sample in frame.iter_mut() {
        let scaled = (*sample as f32 * gain_linear) as i16;
        *sample = scaled.max(-32768).min(32767); // Clamp to i16 range
    }
}

/// Helper: Hard limiter to prevent clipping
pub fn apply_limiter(frame: &mut [i16], threshold: i16) {
    for sample in frame.iter_mut() {
        if *sample > threshold {
            *sample = threshold;
        } else if *sample < -threshold {
            *sample = -threshold;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback_creation() {
        let result = AudioPlayback::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_volume_setting() {
        let mut playback = AudioPlayback::new().unwrap();
        playback.set_volume(-6.0);
        assert!((playback.volume() + 6.0).abs() < 0.01);
    }

    #[test]
    fn test_apply_volume() {
        let mut frame = [1000i16; 256];
        apply_volume(&mut frame, 6.0); // +6dB should roughly double amplitude

        let expected = (1000.0 * 2.0) as i16;
        assert!((frame[0] - expected).abs() < 10); // Allow small error
    }

    #[test]
    fn test_limiter() {
        let mut frame = [30000i16; 256];
        apply_limiter(&mut frame, 20000);

        // All samples should be limited to 20000
        for &sample in frame.iter() {
            assert!(sample <= 20000);
            assert!(sample >= -20000);
        }
    }

    #[test]
    fn test_buffer_level() {
        let playback = AudioPlayback::new().unwrap();
        let level = playback.buffer_level();

        assert!(level >= 0.0 && level <= 1.0);
    }
}
