/// Audio Filter Implementations for DSP Pipeline
/// 
/// Implements:
/// - Biquad IIR filters (2nd-order)
/// - Butterworth high-pass filter
/// - Parametric EQ filters
/// 
/// Using fixed-point arithmetic for deterministic real-time performance

use defmt::*;

/// Biquad IIR Filter Coefficient
/// 
/// Standard Direct Form II realization:
/// y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2] - a1*y[n-1] - a2*y[n-2]
#[derive(Clone, Copy)]
pub struct BiquadCoeff {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
}

impl BiquadCoeff {
    /// Create Butterworth high-pass filter
    /// 
    /// Parameters:
    /// - cutoff_hz: Cutoff frequency in Hz
    /// - sample_rate: Sample rate in Hz
    pub fn butterworth_highpass(cutoff_hz: f32, sample_rate: f32) -> Self {
        let w0 = 2.0 * std::f32::consts::PI * cutoff_hz / sample_rate;
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();

        // Butterworth Q = 0.707 (maximum flatness)
        let q = 1.0 / std::f32::consts::SQRT_2;
        let alpha = sin_w0 / (2.0 * q);

        let b0 = (1.0 + cos_w0) / 2.0;
        let b1 = -(1.0 + cos_w0);
        let b2 = (1.0 + cos_w0) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }

    /// Create parametric EQ filter
    /// 
    /// Parameters:
    /// - center_hz: Center frequency in Hz
    /// - sample_rate: Sample rate in Hz
    /// - gain_db: Gain adjustment in dB
    /// - q: Quality factor (higher = narrower)
    pub fn peaking_eq(center_hz: f32, sample_rate: f32, gain_db: f32, q: f32) -> Self {
        let w0 = 2.0 * std::f32::consts::PI * center_hz / sample_rate;
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();
        let alpha = sin_w0 / (2.0 * q);

        let a = 10.0_f32.powf(gain_db / 40.0);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos_w0;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha / a;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }

    /// Create low-pass filter
    pub fn lowpass(cutoff_hz: f32, sample_rate: f32) -> Self {
        let w0 = 2.0 * std::f32::consts::PI * cutoff_hz / sample_rate;
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();
        let q = 1.0 / std::f32::consts::SQRT_2;
        let alpha = sin_w0 / (2.0 * q);

        let b0 = (1.0 - cos_w0) / 2.0;
        let b1 = 1.0 - cos_w0;
        let b2 = (1.0 - cos_w0) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }
}

/// Biquad IIR Filter State
/// 
/// Holds filter state variables for Direct Form II realization
#[derive(Clone, Copy)]
pub struct BiquadState {
    x1: f32, // x[n-1]
    x2: f32, // x[n-2]
    y1: f32, // y[n-1]
    y2: f32, // y[n-2]
}

impl Default for BiquadState {
    fn default() -> Self {
        Self {
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }
}

impl BiquadState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Process single audio sample through filter
    /// Returns filtered sample
    pub fn process(&mut self, x0: f32, coeff: &BiquadCoeff) -> f32 {
        let y0 = coeff.b0 * x0 + coeff.b1 * self.x1 + coeff.b2 * self.x2
            - coeff.a1 * self.y1
            - coeff.a2 * self.y2;

        self.x2 = self.x1;
        self.x1 = x0;
        self.y2 = self.y1;
        self.y1 = y0;

        y0
    }

    /// Process entire frame through filter
    pub fn process_frame(&mut self, frame: &[i16], coeff: &BiquadCoeff, output: &mut [i16]) {
        for (i, &sample) in frame.iter().enumerate() {
            let input = sample as f32 / 32768.0;
            let filtered = self.process(input, coeff);
            output[i] = (filtered * 32768.0) as i16;
        }
    }

    /// Reset filter state
    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

/// Cascaded 2nd-order biquad filters (for higher orders)
pub struct CascadedBiquads {
    stages: heapless::Vec<BiquadState, 4>, // Up to 4 stages (8th order)
    coeffs: heapless::Vec<BiquadCoeff, 4>,
}

impl CascadedBiquads {
    pub fn new() -> Self {
        Self {
            stages: heapless::Vec::new(),
            coeffs: heapless::Vec::new(),
        }
    }

    pub fn add_stage(&mut self, coeff: BiquadCoeff) -> Result<(), &'static str> {
        self.stages.push(BiquadState::new()).map_err(|_| "Too many stages")?;
        self.coeffs.push(coeff).map_err(|_| "Too many stages")?;
        Ok(())
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        let mut result = sample;

        for i in 0..self.stages.len() {
            let coeff = self.coeffs[i];
            result = self.stages[i].process(result, &coeff);
        }

        result
    }

    pub fn process_frame(&mut self, frame: &[i16], output: &mut [i16]) {
        for (i, &sample) in frame.iter().enumerate() {
            let input = sample as f32 / 32768.0;
            let filtered = self.process(input);
            output[i] = (filtered.max(-1.0).min(1.0) * 32768.0) as i16;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highpass_creation() {
        let coeff = BiquadCoeff::butterworth_highpass(100.0, 16000.0);
        assert!(!coeff.b0.is_nan());
        assert!(!coeff.a1.is_nan());
    }

    #[test]
    fn test_eq_creation() {
        let coeff = BiquadCoeff::peaking_eq(1000.0, 16000.0, 6.0, 0.7);
        assert!(!coeff.b0.is_nan());
    }

    #[test]
    fn test_filter_passthrough() {
        let mut state = BiquadState::new();
        let coeff = BiquadCoeff::peaking_eq(1000.0, 16000.0, 0.0, 0.7);

        let input = 0.5;
        let output = state.process(input, &coeff);

        // With 0dB gain, output should be close to input
        assert!((output - input).abs() < 0.1);
    }

    #[test]
    fn test_cascaded_biquads() {
        let mut cascade = CascadedBiquads::new();
        let coeff = BiquadCoeff::lowpass(5000.0, 16000.0);

        let _ = cascade.add_stage(coeff);
        let _ = cascade.add_stage(coeff);

        let output = cascade.process(0.5);
        assert!(!output.is_nan());
    }
}
