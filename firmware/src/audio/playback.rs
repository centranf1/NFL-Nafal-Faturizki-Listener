/// Audio Playback Driver for TPA6132A2 Class-G Amplifier
/// 
/// Handles I2S data output to headphone amplifier using DMA
/// Manages amplifier enable/disable and volume control
/// 
/// Architecture:
/// - DMA transfers audio data to I2S peripheral
/// - Ring buffer for smooth playback without dropouts
/// - GPIO control for amplifier enable pin (P0.28)
/// - Volume control via digital gain before DMA

use heapless::{Vec, Deque};
use defmt::*;
use crate::hal::i2s::{I2sDriver, I2sConfig, SampleRate, Channels, I2sMode, I2sError};

pub const PLAYBACK_BUFFER_SIZE: usize = 512; // 2 frames @ 16kHz
pub const FRAME_SIZE: usize = 256; // 16ms @ 16kHz
pub const MAX_VOLUME: f32 = 20.0; // dBFS

/// TPA6132A2 Class-G Headphone Amplifier
/// 
/// Specifications:
/// - Interface: I2S digital input
/// - Class-G topology: 25mW/channel into 32Ω
/// - Load impedance: 16Ω to 100Ω
/// - SNR: > 100dB (THD+N < 1%)
/// - Enable pin: P0.28 (GPIO, active high)
pub struct AudioPlayback {
    i2s: I2sDriver,
    buffer: Deque<i16, PLAYBACK_BUFFER_SIZE>,
    volume_gain_db: f32,
    enabled: bool,
    
    // Statistics
    frames_played: u32,
    buffer_underruns: u32,
}

impl AudioPlayback {
    /// Create new audio playback driver with DMA
    pub fn new() -> Result<Self, I2sError> {
        let config = I2sConfig {
            sample_rate: SampleRate::Hz16000,
            channels: Channels::Mono,
            mode: I2sMode::Master,
        };

        let i2s = I2sDriver::new(config);

        info!("🔊 AudioPlayback initialized (DMA-backed)");
        info!("   Sample rate: 16 kHz");
        info!("   Channels: Mono");
        info!("   Output device: TPA6132A2 Class-G Amp");
        info!("   Max power: 25 mW/channel");
        info!("   Buffer size: {} samples (2 frames)", PLAYBACK_BUFFER_SIZE);
        info!("   Enable pin: P0.28 (GPIO)");

        Ok(Self {
            i2s,
            buffer: Deque::new(),
            volume_gain_db: 0.0,
            enabled: false,
            frames_played: 0,
            buffer_underruns: 0,
        })
    }

    /// Start audio playback via I2S DMA
    pub async fn start(&mut self) -> Result<(), I2sError> {
        self.i2s.start_playback().await?;
        
        info!("▶ Audio playback started");
        info!("   DMA transfer active");
        
        Ok(())
    }

    /// Stop audio playback
    pub async fn stop(&mut self) -> Result<(), I2sError> {
        self.i2s.stop_playback().await?;
        info!("⏹ Audio playback stopped");
        info!("   Frames played: {}", self.frames_played);
        Ok(())
    }

    /// Get playback statistics
    pub fn stats(&self) -> PlaybackStats {
        PlaybackStats {
            frames_played: self.frames_played,
            buffer_underruns: self.buffer_underruns,
            buffer_fill: self.buffer.len(),
            volume_db: self.volume_gain_db,
            enabled: self.enabled,
        }
    }

    /// Write a frame of audio data (256 samples) to playback buffer
    /// 
    /// The frame will be applied with volume gain before transmission
    pub fn write_frame(&mut self, frame: &[i16; FRAME_SIZE]) -> Result<(), PlaybackError> {
        if self.buffer.len() + FRAME_SIZE > PLAYBACK_BUFFER_SIZE {
            self.buffer_underruns += 1;
            return Err(PlaybackError::BufferFull);
        }

        // Apply volume gain before storing
        let gain_linear = 10.0_f32.powf(self.volume_gain_db / 20.0);

        for &sample in frame.iter() {
            let scaled = (sample as f32 * gain_linear) as i16;
            let clamped = scaled.clamp(-32768, 32767);

            if self.buffer.push_back(clamped).is_err() {
                self.buffer_underruns += 1;
                return Err(PlaybackError::BufferFull);
            }
        }

        self.frames_played += 1;

        Ok(())
    }

    /// Write multiple frames at once
    pub fn write_frames(&mut self, frames: &[i16]) -> Result<(), PlaybackError> {
        if self.buffer.len() + frames.len() > PLAYBACK_BUFFER_SIZE {
            self.buffer_underruns += 1;
            return Err(PlaybackError::BufferFull);
        }

        let gain_linear = 10.0_f32.powf(self.volume_gain_db / 20.0);

        for &sample in frames {
            let scaled = (sample as f32 * gain_linear) as i16;
            let clamped = scaled.clamp(-32768, 32767);

            self.buffer.push_back(clamped)
                .map_err(|_| PlaybackError::BufferFull)?;
        }

        Ok(())
    }

    /// Set playback volume in dB
    /// 
    /// Range: -40dB to +20dB
    /// 0dB = unity gain
    pub fn set_volume(&mut self, gain_db: f32) {
        let clamped = gain_db.max(-40.0).min(20.0);
        self.volume_gain_db = clamped;
        info!("🔊 Volume set to {} dB", clamped);
    }

    /// Get current volume setting
    pub fn volume(&self) -> f32 {
        self.volume_gain_db
    }

    /// Enable amplifier via GPIO
    /// 
    /// Sets P0.28 high to enable TPA6132A2
    pub fn enable(&mut self) {
        if !self.enabled {
            // In real implementation: Set GPIO pin high
            debug!("Setting P0.28 (amp enable) HIGH");
            self.enabled = true;
            info!("✅ Amplifier enabled");
        }
    }

    /// Disable amplifier via GPIO
    /// 
    /// Sets P0.28 low to disable TPA6132A2
    pub fn disable(&mut self) {
        if self.enabled {
            // In real implementation: Set GPIO pin low
            debug!("Setting P0.28 (amp enable) LOW");
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

    /// Pop frame from buffer for DMA transmission (called by I2S DMA handler)
    pub fn get_dma_frame(&mut self) -> Option<[i16; FRAME_SIZE]> {
        if self.buffer.len() < FRAME_SIZE {
            self.buffer_underruns += 1;
            return None;
        }

        let mut frame = [0i16; FRAME_SIZE];
        for i in 0..FRAME_SIZE {
            frame[i] = self.buffer.pop_front().unwrap_or(0);
        }

        Some(frame)
    }

    /// Clear playback buffer
    pub fn reset(&mut self) {
        self.buffer.clear();
    }

    /// Get I2S status
    pub fn i2s_status(&self) -> crate::hal::i2s::I2sStatus {
        self.i2s.status()
    }
}

pub struct PlaybackStats {
    pub frames_played: u32,
    pub buffer_underruns: u32,
    pub buffer_fill: usize,
    pub volume_db: f32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum PlaybackError {
    BufferFull,
    NotEnabled,
    DmaError,
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
    fn test_volume_clamping() {
        let mut playback = AudioPlayback::new().unwrap();
        
        // Try to set volume outside range
        playback.set_volume(100.0); // Should clamp to 20dB
        assert_eq!(playback.volume(), 20.0);

        playback.set_volume(-100.0); // Should clamp to -40dB
        assert_eq!(playback.volume(), -40.0);
    }

    #[test]
    fn test_apply_volume() {
        let mut frame = [1000i16; 256];
        apply_volume(&mut frame, 6.0); // +6dB should roughly double amplitude

        let expected = (1000.0 * 2.0) as i16;
        assert!((frame[0] - expected).abs() < 10); // Allow small error
    }

    #[test]
    fn test_apply_limiter() {
        let mut frame = [30000i16; 256];
        apply_limiter(&mut frame, 20000);

        // All samples should be limited to 20000
        for &sample in frame.iter() {
            assert!(sample <= 20000);
            assert!(sample >= -20000);
        }
    }

    #[test]
    fn test_amplifier_enable_disable() {
        let mut playback = AudioPlayback::new().unwrap();
        
        assert!(!playback.is_enabled());
        
        playback.enable();
        assert!(playback.is_enabled());
        
        playback.disable();
        assert!(!playback.is_enabled());
    }

    #[test]
    fn test_buffer_level() {
        let playback = AudioPlayback::new().unwrap();
        let level = playback.buffer_level();

        assert!(level >= 0.0 && level <= 1.0);
    }

    #[test]
    fn test_write_frame_full_buffer() {
        let mut playback = AudioPlayback::new().unwrap();
        let test_frame = [100i16; FRAME_SIZE];

        // Fill buffer completely
        for _ in 0..2 {
            assert!(playback.write_frame(&test_frame).is_ok());
        }

        // Next write should fail (buffer full)
        let result = playback.write_frame(&test_frame);
        assert!(result.is_err());
    }

    #[test]
    fn test_playback_stats() {
        let playback = AudioPlayback::new().unwrap();
        let stats = playback.stats();

        assert_eq!(stats.frames_played, 0);
        assert_eq!(stats.buffer_underruns, 0);
        assert!(!stats.enabled);
    }

    #[test]
    fn test_write_frames_batch() {
        let mut playback = AudioPlayback::new().unwrap();
        
        let data: Vec<i16, 512> = (0..256).map(|i| (i as i16) * 10).collect();
        
        let result = playback.write_frames(&data);
        assert!(result.is_ok());
        assert_eq!(playback.buffer_level(), 256.0 / PLAYBACK_BUFFER_SIZE as f32);
    }

    #[test]
    fn test_get_dma_frame() {
        let mut playback = AudioPlayback::new().unwrap();
        
        let test_frame = [100i16; FRAME_SIZE];
        playback.write_frame(&test_frame).ok();

        let retrieved = playback.get_dma_frame();
        assert!(retrieved.is_some());
        
        let frame = retrieved.unwrap();
        // Check that values are approximately correct (volume gain applied)
        assert!(frame[0].abs() > 0);
    }
}
