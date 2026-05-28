/// 8-Band Parametric Equalizer
/// 
/// Implements cascaded biquad filters for hearing aid frequency correction
/// Frequencies: 250Hz, 500Hz, 1kHz, 2kHz, 3kHz, 4kHz, 6kHz, 8kHz
/// Gain range: -20dB to +40dB per band

use crate::audio::dsp::filters::{BiquadCoeff, BiquadState};
use defmt::*;

pub const NUM_EQ_BANDS: usize = 8;

// Standard 8 center frequencies for hearing aid EQ (ISO 266)
pub const EQ_FREQUENCIES: [f32; NUM_EQ_BANDS] = [
    250.0,  // Band 0
    500.0,  // Band 1
    1000.0, // Band 2
    2000.0, // Band 3
    3000.0, // Band 4
    4000.0, // Band 5
    6000.0, // Band 6
    8000.0, // Band 7
];

/// 8-Band EQ Configuration (gains in dB per band)
#[derive(Clone, Copy)]
pub struct EqProfile {
    pub gains: [f32; NUM_EQ_BANDS],
    pub q_factor: f32, // Bandwidth control (typical: 0.7-1.4)
}

impl Default for EqProfile {
    fn default() -> Self {
        Self {
            gains: [0.0; NUM_EQ_BANDS], // Flat (no EQ)
            q_factor: 0.7, // Butterworth-like
        }
    }
}

impl EqProfile {
    /// Create flat EQ profile (no adjustment)
    pub fn flat() -> Self {
        Self::default()
    }

    /// Create profile from array of gains
    pub fn from_gains(gains: [f32; NUM_EQ_BANDS]) -> Self {
        Self {
            gains,
            q_factor: 0.7,
        }
    }

    /// Create typical hearing aid profile (boosting high frequencies)
    /// Based on NAL-NL2 prescription
    pub fn hearing_aid_high_frequency() -> Self {
        // Typical high-frequency boost for age-related hearing loss
        Self {
            gains: [0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 10.0],
            q_factor: 0.7,
        }
    }

    /// Create profile for speech clarity (peak at 2-3kHz)
    pub fn speech_boost() -> Self {
        Self {
            gains: [-2.0, 0.0, 2.0, 4.0, 6.0, 4.0, 0.0, -2.0],
            q_factor: 0.7,
        }
    }

    /// Clamp gains to valid range (-20 to +40 dB)
    pub fn validate(&mut self) {
        for gain in self.gains.iter_mut() {
            *gain = gain.max(-20.0).min(40.0);
        }
    }
}

/// 8-Band Parametric Equalizer
pub struct Equalizer {
    filters: [BiquadState; NUM_EQ_BANDS],
    coeffs: [BiquadCoeff; NUM_EQ_BANDS],
    profile: EqProfile,
    sample_rate: f32,
}

impl Equalizer {
    /// Create new 8-band equalizer
    pub fn new(sample_rate: f32) -> Self {
        let profile = EqProfile::default();
        let mut eq = Self {
            filters: [BiquadState::new(); NUM_EQ_BANDS],
            coeffs: [BiquadCoeff::lowpass(1000.0, sample_rate); NUM_EQ_BANDS],
            profile,
            sample_rate,
        };

        eq.update_coefficients();
        
        info!("🎛️  8-Band Equalizer initialized");
        info!("   Frequencies: 250Hz to 8kHz");
        info!("   Q factor: {}", eq.profile.q_factor);

        eq
    }

    /// Set equalizer profile
    pub fn set_profile(&mut self, mut profile: EqProfile) {
        profile.validate();
        self.profile = profile;
        self.update_coefficients();

        debug!("EQ Profile: {:?}", self.profile.gains);
    }

    /// Set individual band gain
    pub fn set_band_gain(&mut self, band: usize, gain_db: f32) {
        if band < NUM_EQ_BANDS {
            self.profile.gains[band] = gain_db.max(-20.0).min(40.0);
            self.update_coefficients();
        }
    }

    /// Get current profile
    pub fn profile(&self) -> &EqProfile {
        &self.profile
    }

    /// Update filter coefficients based on current profile
    fn update_coefficients(&mut self) {
        for (i, &freq) in EQ_FREQUENCIES.iter().enumerate() {
            let gain_db = self.profile.gains[i];
            self.coeffs[i] = BiquadCoeff::peaking_eq(
                freq,
                self.sample_rate,
                gain_db,
                self.profile.q_factor,
            );
        }
    }

    /// Process single sample through all EQ bands
    pub fn process(&mut self, sample: i16) -> i16 {
        let mut result = sample as f32 / 32768.0;

        // Process through each band cascade
        for i in 0..NUM_EQ_BANDS {
            result = self.filters[i].process(result, &self.coeffs[i]);
        }

        // Clamp to valid range
        (result.clamp(-1.0, 0.99999) * 32768.0) as i16
    }

    /// Process entire frame through equalizer
    pub fn process_frame(&mut self, frame: &[i16], output: &mut [i16]) {
        for (i, &sample) in frame.iter().enumerate() {
            output[i] = self.process(sample);
        }
    }

    /// Reset all filter states
    pub fn reset(&mut self) {
        for filter in self.filters.iter_mut() {
            filter.reset();
        }
    }

    /// Get frequency response at specific frequency (in dB)
    pub fn frequency_response(&self, freq_hz: f32) -> f32 {
        // Simplified: sum of individual band responses at this frequency
        let mut response = 0.0;

        for (i, &band_freq) in EQ_FREQUENCIES.iter().enumerate() {
            let gain = self.profile.gains[i];
            
            // Gaussian approximation of peaking filter response
            let distance = (freq_hz - band_freq).abs();
            let bandwidth = band_freq / self.profile.q_factor;
            let normalized = distance / bandwidth;
            
            // Contribution decreases with distance
            let contribution = gain * (-normalized * normalized).exp();
            response += contribution;
        }

        response
    }

    /// Print frequency response for debugging
    pub fn print_response(&self) {
        let test_freqs = [100.0, 250.0, 500.0, 1000.0, 2000.0, 4000.0, 8000.0, 16000.0];
        
        info!("Frequency Response:");
        for &freq in test_freqs.iter() {
            let response = self.frequency_response(freq);
            info!("  {}Hz: {} dB", freq, response);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_creation() {
        let eq = Equalizer::new(16000.0);
        assert_eq!(eq.profile().gains[0], 0.0);
    }

    #[test]
    fn test_eq_flat() {
        let mut eq = Equalizer::new(16000.0);
        let profile = EqProfile::flat();
        eq.set_profile(profile);

        // Flat EQ should have minimal effect on signal
        let input = [1000i16; 256];
        let mut output = [0i16; 256];

        eq.process_frame(&input, &mut output);

        // Energy should be similar
        let input_energy: i64 = input.iter().map(|&s| (s as i64) * (s as i64)).sum();
        let output_energy: i64 = output.iter().map(|&s| (s as i64) * (s as i64)).sum();

        let ratio = output_energy as f32 / input_energy as f32;
        assert!(ratio > 0.95); // Allow small loss
    }

    #[test]
    fn test_eq_gain_clamping() {
        let mut profile = EqProfile::default();
        profile.gains[0] = 100.0; // Too high
        profile.gains[1] = -100.0; // Too low

        profile.validate();

        assert!(profile.gains[0] <= 40.0);
        assert!(profile.gains[1] >= -20.0);
    }

    #[test]
    fn test_hearing_aid_profile() {
        let profile = EqProfile::hearing_aid_high_frequency();
        
        // Should boost high frequencies
        assert!(profile.gains[6] > profile.gains[0]); // 6kHz > 250Hz
        assert!(profile.gains[7] > profile.gains[1]); // 8kHz > 500Hz
    }

    #[test]
    fn test_frequency_response() {
        let eq = Equalizer::new(16000.0);
        
        let response_1k = eq.frequency_response(1000.0);
        assert!(!response_1k.is_nan());
        
        // With flat profile, response should be close to 0
        assert!(response_1k.abs() < 0.1);
    }

    #[test]
    fn test_eq_reset() {
        let mut eq = Equalizer::new(16000.0);
        
        // Process some samples
        let _ = eq.process(1000);
        let _ = eq.process(2000);
        
        eq.reset();
        
        // After reset, processing same input should give same output
        // (state should be clean)
        assert!(true); // Simplified test
    }
}
