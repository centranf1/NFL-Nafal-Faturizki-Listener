/// Audio DSP Pipeline
/// 
/// Main signal processing chain:
/// Capture (I2S) → Noise Gate → High-Pass Filter → EQ → Limiter → Playback (I2S)
/// 
/// Processes audio in 256-sample frames @ 16kHz = 16ms latency per frame

use crate::audio::capture::{AudioCapture, FRAME_SIZE};
use crate::audio::playback::{AudioPlayback, apply_limiter};
use crate::audio::dsp::filters::{BiquadCoeff, BiquadState};
use crate::audio::dsp::noise_gate::{NoiseGate, NoiseGateConfig};
use crate::audio::dsp::equalizer::{Equalizer, EqProfile};
use defmt::*;

pub const PIPELINE_FRAME_SIZE: usize = 256;

/// Audio Processing Configuration
#[derive(Clone, Copy)]
pub struct PipelineConfig {
    pub enable_noise_gate: bool,
    pub enable_highpass: bool,
    pub enable_eq: bool,
    pub enable_limiter: bool,
    pub output_gain_db: f32,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            enable_noise_gate: true,
            enable_highpass: true,
            enable_eq: true,
            enable_limiter: true,
            output_gain_db: 0.0,
        }
    }
}

/// Main Audio DSP Pipeline
pub struct AudioPipeline {
    capture: AudioCapture,
    playback: AudioPlayback,
    
    // DSP modules
    noise_gate: NoiseGate,
    highpass_filter: BiquadState,
    equalizer: Equalizer,
    
    // Configuration
    config: PipelineConfig,
    
    // Statistics
    pub frames_processed: u32,
    pub underruns: u32,
    pub overruns: u32,
}

impl AudioPipeline {
    /// Create new audio pipeline
    pub fn new() -> Result<Self, &'static str> {
        let capture = AudioCapture::new().map_err(|_| "Capture init failed")?;
        let playback = AudioPlayback::new().map_err(|_| "Playback init failed")?;

        // Initialize DSP modules
        let noise_gate = NoiseGate::new(NoiseGateConfig::default(), 16000.0);
        let highpass_filter = BiquadState::new();
        let equalizer = Equalizer::new(16000.0);

        info!("🔄 Audio Pipeline initialized");
        info!("   Frame size: {} samples (16ms @ 16kHz)", PIPELINE_FRAME_SIZE);
        info!("   Modules: Gate → HPF → EQ → Limiter → Output");

        Ok(Self {
            capture,
            playback,
            noise_gate,
            highpass_filter,
            equalizer,
            config: PipelineConfig::default(),
            frames_processed: 0,
            underruns: 0,
            overruns: 0,
        })
    }

    /// Start audio I/O
    pub async fn start(&mut self) -> Result<(), &'static str> {
        self.capture.start().await.map_err(|_| "Capture start failed")?;
        self.playback.start().await.map_err(|_| "Playback start failed")?;
        self.playback.enable();

        info!("▶ Audio pipeline started");
        Ok(())
    }

    /// Stop audio I/O
    pub async fn stop(&mut self) -> Result<(), &'static str> {
        self.playback.disable();
        self.capture.stop().await.map_err(|_| "Capture stop failed")?;
        self.playback.stop().await.map_err(|_| "Playback stop failed")?;

        info!("⏹ Audio pipeline stopped");
        Ok(())
    }

    /// Set pipeline configuration
    pub fn set_config(&mut self, config: PipelineConfig) {
        self.config = config;
        debug!("Pipeline config updated");
    }

    /// Set equalizer profile
    pub fn set_eq_profile(&mut self, profile: EqProfile) {
        self.equalizer.set_profile(profile);
    }

    /// Set output volume
    pub fn set_output_volume(&mut self, gain_db: f32) {
        self.playback.set_volume(gain_db);
    }

    /// Get pipeline statistics
    pub fn stats(&self) -> PipelineStats {
        PipelineStats {
            frames_processed: self.frames_processed,
            underruns: self.underruns,
            overruns: self.overruns,
            gate_envelope: self.noise_gate.envelope(),
            playback_buffer_level: self.playback.buffer_level(),
        }
    }

    /// Process one frame through the full pipeline
    /// 
    /// Signal flow:
    /// 1. Capture frame from I2S
    /// 2. Apply noise gate
    /// 3. Apply high-pass filter (100Hz cutoff)
    /// 4. Apply 8-band EQ
    /// 5. Apply hard limiter
    /// 6. Send to playback I2S
    pub async fn process_frame(&mut self) -> Result<(), PipelineError> {
        // 1. Capture input frame
        let frame = if let Some(f) = self.capture.read_frame() {
            f
        } else {
            self.underruns += 1;
            return Err(PipelineError::CaptureUnderrun);
        };

        let mut processed = [0i16; FRAME_SIZE];

        // 2. Apply noise gate (if enabled)
        if self.config.enable_noise_gate {
            self.noise_gate.process_frame(frame, &mut processed);
        } else {
            processed.copy_from_slice(frame);
        }

        // 3. Apply high-pass filter @ 100Hz (if enabled)
        if self.config.enable_highpass {
            let hpf_coeff = BiquadCoeff::butterworth_highpass(100.0, 16000.0);
            let mut hpf_output = [0i16; FRAME_SIZE];

            self.highpass_filter.process_frame(&processed, &hpf_coeff, &mut hpf_output);
            processed.copy_from_slice(&hpf_output);
        }

        // 4. Apply 8-band EQ (if enabled)
        if self.config.enable_eq {
            let mut eq_output = [0i16; FRAME_SIZE];
            self.equalizer.process_frame(&processed, &mut eq_output);
            processed.copy_from_slice(&eq_output);
        }

        // 5. Apply hard limiter (always enabled for safety)
        if self.config.enable_limiter {
            apply_limiter(&mut processed, 30000); // -6dBFS ceiling
        }

        // 6. Apply output gain
        if self.config.output_gain_db.abs() > 0.01 {
            let gain_linear = 10.0_f32.powf(self.config.output_gain_db / 20.0);
            for sample in processed.iter_mut() {
                let scaled = (*sample as f32 * gain_linear) as i16;
                *sample = scaled.clamp(-32768, 32767);
            }
        }

        // 7. Send to playback
        self.playback.write_frame(&processed)
            .map_err(|_| PipelineError::PlaybackOverrun)?;

        self.frames_processed += 1;

        Ok(())
    }

    /// Run continuous audio processing loop (async)
    pub async fn run(&mut self) -> Result<(), PipelineError> {
        self.start().await.map_err(|_| PipelineError::StartupFailed)?;

        loop {
            if let Err(e) = self.process_frame().await {
                warn!("Pipeline error: {:?}", e);
                // Continue despite errors
            }

            // Allow other tasks to run
            embassy_time::Timer::after_micros(1).await;
        }
    }

    /// Print pipeline status
    pub fn print_status(&self) {
        let stats = self.stats();
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("Audio Pipeline Status:");
        info!("  Frames: {}", stats.frames_processed);
        info!("  Underruns: {}", stats.underruns);
        info!("  Overruns: {}", stats.overruns);
        info!("  Gate envelope: {:.2}", stats.gate_envelope);
        info!("  Buffer level: {:.1}%", stats.playback_buffer_level * 100.0);
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}

pub struct PipelineStats {
    pub frames_processed: u32,
    pub underruns: u32,
    pub overruns: u32,
    pub gate_envelope: f32,
    pub playback_buffer_level: f32,
}

#[derive(Debug)]
pub enum PipelineError {
    CaptureUnderrun,
    PlaybackOverrun,
    StartupFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let result = AudioPipeline::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_modification() {
        let mut pipeline = AudioPipeline::new().unwrap();
        
        let mut config = PipelineConfig::default();
        config.enable_eq = false;
        config.output_gain_db = 6.0;

        pipeline.set_config(config);
        
        assert!(!pipeline.config.enable_eq);
        assert!((pipeline.config.output_gain_db - 6.0).abs() < 0.01);
    }

    #[test]
    fn test_stats_tracking() {
        let pipeline = AudioPipeline::new().unwrap();
        let stats = pipeline.stats();

        assert_eq!(stats.frames_processed, 0);
        assert_eq!(stats.underruns, 0);
        assert_eq!(stats.overruns, 0);
    }
}
