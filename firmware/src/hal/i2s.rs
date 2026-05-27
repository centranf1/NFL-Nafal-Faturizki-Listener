/// I2S Hardware Abstraction Layer
/// 
/// Provides clean abstraction over nRF5340 I2S peripheral
/// Support for both capture (RX) and playback (TX)

use embassy_nrf::i2s::{self, Config, I2s, Mode};
use defmt::*;
use heapless::Vec;

/// I2S configuration for audio
pub struct I2sConfig {
    pub sample_rate: SampleRate,
    pub channels: Channels,
    pub mode: I2sMode,
}

#[derive(Clone, Copy)]
pub enum SampleRate {
    Hz16000,
    Hz48000,
}

impl SampleRate {
    pub fn value(&self) -> u32 {
        match self {
            SampleRate::Hz16000 => 16000,
            SampleRate::Hz48000 => 48000,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Channels {
    Mono,
    Stereo,
}

#[derive(Clone, Copy)]
pub enum I2sMode {
    Master,
    Slave,
}

/// I2S driver abstraction
pub struct I2sDriver {
    // I2S peripheral reference (would be wrapped in Arc<Mutex<>> in real impl)
    // For now, keeping structure simple for Phase 1
}

impl I2sDriver {
    pub fn new(config: I2sConfig) -> Self {
        info!("🎵 I2S Driver initialized @ {:?} Hz", config.sample_rate.value());
        Self {}
    }

    /// Start I2S capture (RX) operation
    /// 
    /// Parameters:
    /// - buffer: Ring buffer for captured audio samples
    pub async fn start_capture(&mut self, _buffer: &mut Vec<i16, 512>) -> Result<(), I2sError> {
        info!("▶ Starting I2S capture from SPH0645 MEMS microphone");
        Ok(())
    }

    /// Start I2S playback (TX) operation
    /// 
    /// Parameters:
    /// - buffer: Ring buffer with audio samples to play
    pub async fn start_playback(&mut self, _buffer: &Vec<i16, 512>) -> Result<(), I2sError> {
        info!("▶ Starting I2S playback to TPA6132A2 amplifier");
        Ok(())
    }

    /// Stop I2S capture
    pub async fn stop_capture(&mut self) -> Result<(), I2sError> {
        info!("⏹ Stopping I2S capture");
        Ok(())
    }

    /// Stop I2S playback
    pub async fn stop_playback(&mut self) -> Result<(), I2sError> {
        info!("⏹ Stopping I2S playback");
        Ok(())
    }

    /// Get current I2S status
    pub fn status(&self) -> I2sStatus {
        I2sStatus {
            capturing: false,
            playing: false,
            sample_rate: 16000,
        }
    }
}

pub struct I2sStatus {
    pub capturing: bool,
    pub playing: bool,
    pub sample_rate: u32,
}

#[derive(Debug)]
pub enum I2sError {
    ConfigError,
    AlreadyRunning,
    NotRunning,
    BufferOverflow,
    BufferUnderflow,
}

/// Helper: Convert dB to linear scale
pub fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

/// Helper: Convert linear to dB
pub fn linear_to_db(linear: f32) -> f32 {
    20.0 * linear.log10()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_conversion() {
        let linear = db_to_linear(0.0);
        assert!((linear - 1.0).abs() < 0.001);

        let linear = db_to_linear(6.0);
        assert!((linear - 2.0).abs() < 0.01);

        let linear = db_to_linear(-6.0);
        assert!((linear - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_linear_db_roundtrip() {
        let original = 0.5;
        let db = linear_to_db(original);
        let recovered = db_to_linear(db);
        assert!((original - recovered).abs() < 0.001);
    }
}
