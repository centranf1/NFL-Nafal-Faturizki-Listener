/// Noise Gate (Amplitude Expansion) Algorithm
/// 
/// Reduces background noise by attenuating signals below a threshold
/// Implements adaptive gate with smooth attack/release to avoid clicks

use defmt::*;

/// Noise Gate Configuration
#[derive(Clone, Copy)]
pub struct NoiseGateConfig {
    /// Gate threshold in dBFS (-80 to 0)
    pub threshold_dbfs: f32,
    
    /// Attack time in milliseconds (when signal exceeds threshold)
    pub attack_ms: f32,
    
    /// Release time in milliseconds (when signal falls below threshold)
    pub release_ms: f32,
    
    /// Gate range / expansion ratio (0.0 = mute, 1.0 = no gating)
    pub range: f32,
}

impl Default for NoiseGateConfig {
    fn default() -> Self {
        Self {
            threshold_dbfs: -50.0,
            attack_ms: 1.0,
            release_ms: 100.0,
            range: 1.0,
        }
    }
}

/// Noise Gate State Machine
pub struct NoiseGate {
    config: NoiseGateConfig,
    envelope: f32, // Current gate envelope (0.0 = fully closed, 1.0 = fully open)
    
    // Coefficient for attack/release smoothing
    attack_coeff: f32,
    release_coeff: f32,
}

impl NoiseGate {
    /// Create new noise gate with configuration
    /// 
    /// Parameters:
    /// - config: Gate configuration
    /// - sample_rate: Audio sample rate in Hz
    pub fn new(config: NoiseGateConfig, sample_rate: f32) -> Self {
        // Convert time constants to coefficients
        // Using exponential decay: coeff = exp(-2.2 / (time_ms * sample_rate / 1000))
        let attack_coeff = (-2.2 / (config.attack_ms * sample_rate / 1000.0)).exp();
        let release_coeff = (-2.2 / (config.release_ms * sample_rate / 1000.0)).exp();

        info!("🔇 Noise Gate initialized");
        info!("   Threshold: {} dBFS", config.threshold_dbfs);
        info!("   Attack: {} ms", config.attack_ms);
        info!("   Release: {} ms", config.release_ms);
        info!("   Range: {:.1}%", config.range * 100.0);

        Self {
            config,
            envelope: 1.0, // Start open
            attack_coeff,
            release_coeff,
        }
    }

    /// Update configuration
    pub fn set_config(&mut self, config: NoiseGateConfig, sample_rate: f32) {
        self.config = config;
        self.attack_coeff = (-2.2 / (config.attack_ms * sample_rate / 1000.0)).exp();
        self.release_coeff = (-2.2 / (config.release_ms * sample_rate / 1000.0)).exp();
    }

    /// Process single sample through gate
    pub fn process(&mut self, sample: i16) -> i16 {
        // Measure input level
        let sample_f = sample as f32 / 32768.0;
        let level_dbfs = 20.0 * sample_f.abs().log10();

        // Determine target envelope (0.0 = closed, 1.0 = open)
        let target_envelope = if level_dbfs > self.config.threshold_dbfs {
            1.0 // Above threshold → open
        } else {
            self.config.range // Below threshold → partially close
        };

        // Smooth envelope with attack/release
        if target_envelope > self.envelope {
            // Attack (fast)
            self.envelope = self.attack_coeff * self.envelope
                + (1.0 - self.attack_coeff) * target_envelope;
        } else {
            // Release (slower)
            self.envelope = self.release_coeff * self.envelope
                + (1.0 - self.release_coeff) * target_envelope;
        }

        // Apply gate to sample
        let gated = sample_f * self.envelope;

        (gated * 32768.0).clamp(-32768.0, 32767.0) as i16
    }

    /// Process entire frame through gate
    pub fn process_frame(&mut self, frame: &[i16], output: &mut [i16]) {
        for (i, &sample) in frame.iter().enumerate() {
            output[i] = self.process(sample);
        }
    }

    /// Get current gate envelope (0.0 = closed, 1.0 = open)
    pub fn envelope(&self) -> f32 {
        self.envelope
    }

    /// Reset gate state
    pub fn reset(&mut self) {
        self.envelope = 1.0;
    }
}

/// Helper: Measure noise reduction ratio
/// 
/// Returns: Attenuation in dB
pub fn measure_attenuation(input_dbfs: f32, output_dbfs: f32) -> f32 {
    input_dbfs - output_dbfs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gate_creation() {
        let config = NoiseGateConfig::default();
        let gate = NoiseGate::new(config, 16000.0);

        assert!(gate.envelope >= 0.0 && gate.envelope <= 1.0);
    }

    #[test]
    fn test_gate_silence() {
        let config = NoiseGateConfig {
            threshold_dbfs: -50.0,
            range: 0.0, // Mute below threshold
            ..Default::default()
        };
        let mut gate = NoiseGate::new(config, 16000.0);

        // Process silence (very quiet signal)
        let silent = [100i16; 256]; // ~-50dBFS
        let mut output = [0i16; 256];
        
        for _ in 0..10 {
            // Process multiple times to let gate settle
            gate.process_frame(&silent, &mut output);
        }

        // After settling, very quiet signals should be heavily attenuated
        let max_output = output.iter().map(|&s| s.abs()).max().unwrap_or(0);
        assert!(max_output < 50); // Should be nearly silent
    }

    #[test]
    fn test_gate_signal() {
        let config = NoiseGateConfig::default();
        let mut gate = NoiseGate::new(config, 16000.0);

        // Loud signal (above threshold)
        let loud = [20000i16; 256]; // Strong signal
        let mut output = [0i16; 256];
        
        gate.process_frame(&loud, &mut output);

        // Gate should pass strong signals with minimal attenuation
        let input_energy: i64 = loud.iter().map(|&s| (s as i64) * (s as i64)).sum();
        let output_energy: i64 = output.iter().map(|&s| (s as i64) * (s as i64)).sum();

        let ratio = output_energy as f32 / input_energy as f32;
        assert!(ratio > 0.9); // Less than -0.9dB loss
    }

    #[test]
    fn test_envelope_smoothing() {
        let config = NoiseGateConfig::default();
        let mut gate = NoiseGate::new(config, 16000.0);

        // Should smoothly transition from closed to open
        gate.envelope = 0.0;

        // Process loud signal to trigger opening
        let loud = [20000i16; 256];
        let mut output = [0i16; 256];

        for i in 0..100 {
            gate.process_frame(&loud, &mut output);
            
            // Envelope should increase over time (exponential approach to 1.0)
            let expected_direction = gate.envelope >= 0.5; // Should trend upward
            assert!(expected_direction || i < 5); // Allow initial settling
        }

        assert!(gate.envelope > 0.95); // Should be nearly open
    }
}
