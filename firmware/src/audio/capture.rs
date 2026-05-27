/// Audio Capture Driver for SPH0645 MEMS Microphone
/// 
/// Handles I2S data capture from digital microphone using DMA
/// Uses ring buffers for efficient streaming with minimal CPU overhead
/// 
/// Architecture:
/// - DMA transfers data from I2S peripheral to ring buffer
/// - Frame-based processing (256 samples = 16ms @ 16kHz)
/// - Double buffering to minimize latency

use heapless::Vec;
use defmt::*;
use crate::hal::i2s::{I2sDriver, I2sConfig, SampleRate, Channels, I2sMode, I2sError};

pub const CAPTURE_BUFFER_SIZE: usize = 512; // 2 frames @ 16kHz
pub const FRAME_SIZE: usize = 256; // 16ms @ 16kHz

/// SPH0645 MEMS Microphone Driver with DMA
/// 
/// Specifications:
/// - Interface: I2S digital output
/// - Frequency range: 20Hz - 20kHz
/// - Sensitivity: -26dBFS @ 94dB SPL
/// - Sample rate: 16kHz (Phase 1), expandable to 48kHz
pub struct AudioCapture {
    i2s: I2sDriver,
    buffer: Vec<i16, CAPTURE_BUFFER_SIZE>,
    read_pos: usize,
    
    // Statistics
    frames_captured: u32,
    buffer_overflows: u32,
}

impl AudioCapture {
    /// Create new audio capture driver with DMA
    pub fn new() -> Result<Self, I2sError> {
        let config = I2sConfig {
            sample_rate: SampleRate::Hz16000,
            channels: Channels::Mono,
            mode: I2sMode::Master,
        };

        let i2s = I2sDriver::new(config);

        info!("🎤 AudioCapture initialized (DMA-backed)");
        info!("   Sample rate: 16 kHz");
        info!("   Channels: Mono");
        info!("   Input device: SPH0645 MEMS Microphone");
        info!("   Buffer size: {} samples (2 frames)", CAPTURE_BUFFER_SIZE);
        info!("   Frame size: {} samples (16ms)", FRAME_SIZE);

        Ok(Self {
            i2s,
            buffer: Vec::new(),
            read_pos: 0,
            frames_captured: 0,
            buffer_overflows: 0,
        })
    }

    /// Start capturing audio via I2S DMA
    pub async fn start(&mut self) -> Result<(), I2sError> {
        self.i2s.start_capture().await?;
        
        info!("▶ Audio capture started");
        info!("   DMA transfer active");
        info!("   I2S interface: I2S0 (nRF5340 APP core)");
        
        Ok(())
    }

    /// Stop capturing audio
    pub async fn stop(&mut self) -> Result<(), I2sError> {
        self.i2s.stop_capture().await?;
        info!("⏹ Audio capture stopped");
        info!("   Frames captured: {}", self.frames_captured);
        Ok(())
    }

    /// Get statistics
    pub fn stats(&self) -> CaptureStats {
        CaptureStats {
            frames_captured: self.frames_captured,
            buffer_overflows: self.buffer_overflows,
            buffer_fill: self.buffer.len(),
            available_frames: self.buffer.len() / FRAME_SIZE,
        }
    }

    /// Read a frame of audio data (256 samples)
    /// 
    /// Returns: Reference to frame if available
    pub fn read_frame(&mut self) -> Option<[i16; FRAME_SIZE]> {
        // Check if we have a complete frame
        if self.read_pos + FRAME_SIZE <= self.buffer.len() {
            let mut frame = [0i16; FRAME_SIZE];
            
            // Copy frame data
            for i in 0..FRAME_SIZE {
                frame[i] = self.buffer[self.read_pos + i];
            }
            
            self.read_pos += FRAME_SIZE;
            self.frames_captured += 1;
            
            Some(frame)
        } else {
            None
        }
    }

    /// Read multiple frames
    pub fn read_frames(&mut self, count: usize) -> Option<Vec<i16, 2048>> {
        if count == 0 || count > 8 {
            return None;
        }

        let total_samples = count * FRAME_SIZE;
        if self.read_pos + total_samples > self.buffer.len() {
            return None;
        }

        let mut data = Vec::new();
        for i in 0..total_samples {
            data.push(self.buffer[self.read_pos + i]).ok()?;
        }

        self.read_pos += total_samples;
        self.frames_captured += count as u32;

        Some(data)
    }

    /// Get current microphone input level (RMS in dBFS)
    pub fn input_level(&self) -> f32 {
        if self.buffer.is_empty() {
            return -120.0; // Silence
        }

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
        if self.buffer.is_empty() {
            return -120.0;
        }

        let max_abs = self
            .buffer
            .iter()
            .map(|&s| s.abs() as i32)
            .max()
            .unwrap_or(0);

        if max_abs == 0 {
            -120.0
        } else {
            20.0 * (max_abs as f32 / 32768.0).log10()
        }
    }

    /// Add captured data from DMA (called by I2S DMA handler)
    pub fn push_dma_data(&mut self, samples: &[i16]) -> Result<(), I2sError> {
        // Check buffer space
        if self.buffer.len() + samples.len() > CAPTURE_BUFFER_SIZE {
            self.buffer_overflows += 1;
            return Err(I2sError::BufferOverflow);
        }

        // Add to buffer
        for &sample in samples {
            self.buffer.push(sample)
                .map_err(|_| I2sError::BufferOverflow)?;
        }

        Ok(())
    }

    /// Clear capture buffer
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.read_pos = 0;
    }

    /// Get I2S status
    pub fn i2s_status(&self) -> crate::hal::i2s::I2sStatus {
        self.i2s.status()
    }
}

pub struct CaptureStats {
    pub frames_captured: u32,
    pub buffer_overflows: u32,
    pub buffer_fill: usize,
    pub available_frames: usize,
}

/// Helper: Measure input level from a frame
pub fn measure_frame_level(frame: &[i16]) -> FrameLevel {
    if frame.is_empty() {
        return FrameLevel {
            rms_db: -120.0,
            peak_db: -120.0,
            peak_linear: 0,
            crest_factor: 0.0,
        };
    }

    let sum_sq: i64 = frame.iter().map(|&s| (s as i64) * (s as i64)).sum();
    let rms = ((sum_sq as f32) / (frame.len() as f32)).sqrt();
    let rms_db = 20.0 * (rms / 32768.0).log10();

    let peak_abs = frame.iter().map(|&s| s.abs() as i32).max().unwrap_or(0);
    let peak_db = 20.0 * (peak_abs as f32 / 32768.0).log10();

    // Crest factor = Peak / RMS (in linear scale)
    let crest_factor = if rms > 0.0 {
        (peak_abs as f32) / rms
    } else {
        0.0
    };

    FrameLevel {
        rms_db,
        peak_db,
        peak_linear: peak_abs as i16,
        crest_factor,
    }
}

pub struct FrameLevel {
    pub rms_db: f32,
    pub peak_db: f32,
    pub peak_linear: i16,
    pub crest_factor: f32,
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
    fn test_frame_level_silence() {
        let silence = [0i16; 256];
        let level = measure_frame_level(&silence);

        // Silence should be very low or -inf dB
        assert!(level.rms_db < -100.0);
        assert!(level.peak_db < -100.0);
        assert_eq!(level.peak_linear, 0);
    }

    #[test]
    fn test_frame_level_tone() {
        // Generate 1kHz sine wave at -6dBFS
        let mut frame = [0i16; 256];
        let amplitude = (0.5 * 32768.0) as i16; // -6dBFS

        for (i, sample) in frame.iter_mut().enumerate() {
            let freq = 1000.0 / 16000.0; // 1kHz @ 16kHz
            let phase = 2.0 * std::f32::consts::PI * freq * (i as f32);
            *sample = (amplitude as f32 * phase.sin()) as i16;
        }

        let level = measure_frame_level(&frame);

        // Peak should be close to -6dBFS
        assert!((level.peak_db + 6.0).abs() < 0.5);
        
        // Crest factor for sine wave should be ~√2 ≈ 1.414
        assert!((level.crest_factor - 1.414).abs() < 0.2);
    }

    #[test]
    fn test_capture_buffer_overflow() {
        let mut capture = AudioCapture::new().unwrap();
        
        // Try to push more data than buffer can hold
        let huge_data = vec![100i16; 1000];
        
        // First push should work
        let result1 = capture.push_dma_data(&huge_data[..CAPTURE_BUFFER_SIZE]);
        assert!(result1.is_ok());

        // Second push should overflow
        let result2 = capture.push_dma_data(&huge_data);
        assert!(result2.is_err());
    }

    #[test]
    fn test_frame_reading() {
        let mut capture = AudioCapture::new().unwrap();
        
        // Push a frame of data
        let frame_data: Vec<i16, 256> = (0..256).map(|i| (i as i16) * 10).collect();
        capture.push_dma_data(&frame_data).ok();

        // Read frame
        if let Some(frame) = capture.read_frame() {
            assert_eq!(frame[0], 0);
            assert_eq!(frame[255], 255 * 10);
        } else {
            panic!("Failed to read frame");
        }
    }

    #[test]
    fn test_capture_stats() {
        let capture = AudioCapture::new().unwrap();
        let stats = capture.stats();

        assert_eq!(stats.frames_captured, 0);
        assert_eq!(stats.buffer_overflows, 0);
        assert_eq!(stats.buffer_fill, 0);
    }
}
