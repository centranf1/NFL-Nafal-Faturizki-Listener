/// I2S Hardware Abstraction Layer with DMA Support
/// 
/// Provides clean abstraction over nRF5340 I2S peripheral with DMA
/// Support for both capture (RX) and playback (TX) with ring buffers
/// 
/// Architecture:
/// - I2S peripheral configured for master mode
/// - DMA streams for efficient data transfer
/// - Ring buffers for double-buffering (minimize latency)
/// - Per-frame processing (256 samples @ 16kHz = 16ms)

use embassy_nrf::i2s::{self, Config, I2s, Mode};
use embassy_nrf::dma::Config as DmaConfig;
use defmt::*;
use heapless::Vec;

/// I2S configuration for audio
#[derive(Clone, Copy)]
pub struct I2sConfig {
    pub sample_rate: SampleRate,
    pub channels: Channels,
    pub mode: I2sMode,
}

#[derive(Clone, Copy, Debug)]
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

    /// Get MCK (master clock) value for nRF5340
    /// Formula: MCK = Sample Rate * 32 (for 32-bit words) * 2 (stereo) or 1 (mono)
    pub fn mck_value(&self, channels: Channels) -> u32 {
        let base = self.value() * 32;
        match channels {
            Channels::Mono => base,
            Channels::Stereo => base * 2,
        }
    }

    /// Convert to nRF MCK ratio for CLOCK config
    pub fn mck_ratio(&self) -> u32 {
        match self {
            SampleRate::Hz16000 => 128,  // 16kHz * 128 = 2.048 MHz
            SampleRate::Hz48000 => 128,  // 48kHz * 128 = 6.144 MHz
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Channels {
    Mono,
    Stereo,
}

impl Channels {
    pub fn count(&self) -> usize {
        match self {
            Channels::Mono => 1,
            Channels::Stereo => 2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum I2sMode {
    Master,
    Slave,
}

/// I2S DMA buffer configuration
pub const DMA_BUFFER_SIZE: usize = 512; // 2 frames of 256 samples
pub const FRAME_SIZE: usize = 256;      // 16ms @ 16kHz

/// I2S driver with DMA support
pub struct I2sDriver {
    // I2S peripheral (actual nrf-hal I2S driver would be here)
    // For now, keeping simplified structure
    config: I2sConfig,
    
    // DMA buffer state
    rx_buffer: Vec<i16, DMA_BUFFER_SIZE>,
    tx_buffer: Vec<i16, DMA_BUFFER_SIZE>,
    
    // State tracking
    dma_active: bool,
    last_processed: u32,
}

impl I2sDriver {
    /// Create new I2S driver with DMA
    /// 
    /// Parameters:
    /// - config: I2S configuration
    pub fn new(config: I2sConfig) -> Self {
        info!("🎵 I2S Driver initialized with DMA");
        info!("   Sample rate: {} Hz", config.sample_rate.value());
        info!("   Channels: {:?}", config.channels);
        info!("   Mode: {:?}", config.mode);
        info!("   MCK ratio: {}", config.sample_rate.mck_ratio());
        info!("   DMA buffer: {} samples", DMA_BUFFER_SIZE);

        Self {
            config,
            rx_buffer: Vec::new(),
            tx_buffer: Vec::new(),
            dma_active: false,
            last_processed: 0,
        }
    }

    /// Configure I2S pins and DMA
    /// 
    /// nRF5340 DK pin configuration:
    /// - P0.04: SCK (serial clock)
    /// - P0.05: SDI (serial data in / microphone)
    /// - P0.07: SCK (TX)
    /// - P0.08: SDO (serial data out / amplifier)
    /// - P0.25: WS (word select / frame sync)
    /// - P0.26: WS (TX word select)
    pub fn configure_pins(&mut self) -> Result<(), I2sError> {
        // In real implementation, would configure GPIO via HAL
        // Using struct pattern to keep abstraction layer clean
        
        debug!("Configuring I2S GPIO pins");
        debug!("  RX: P0.04 (SCK), P0.05 (SDI), P0.25 (WS)");
        debug!("  TX: P0.07 (SCK), P0.08 (SDO), P0.26 (WS)");
        
        Ok(())
    }

    /// Start I2S capture (RX) with DMA
    pub async fn start_capture(&mut self) -> Result<(), I2sError> {
        if self.dma_active {
            return Err(I2sError::AlreadyRunning);
        }

        debug!("Starting I2S capture via DMA");
        debug!("  Buffer: {} samples", DMA_BUFFER_SIZE);
        debug!("  Frame size: {} samples", FRAME_SIZE);
        
        // Clear buffer
        self.rx_buffer.clear();
        
        // In real implementation, would:
        // 1. Initialize PSEL registers for I2S pins
        // 2. Configure MCK divider (CONFIG.MCK register)
        // 3. Set RATIO and ALIGN fields
        // 4. Enable I2S RX engine (CONFIG.RXEN)
        // 5. Start DMA on I2S RXD.PTR + AMOUNT
        // 6. Set TASKS_START
        
        self.dma_active = true;
        info!("▶ I2S capture started (DMA)");
        
        Ok(())
    }

    /// Start I2S playback (TX) with DMA
    pub async fn start_playback(&mut self) -> Result<(), I2sError> {
        if self.dma_active {
            return Err(I2sError::AlreadyRunning);
        }

        debug!("Starting I2S playback via DMA");
        debug!("  Buffer: {} samples", DMA_BUFFER_SIZE);
        
        // Clear buffer
        self.tx_buffer.clear();
        
        // In real implementation, would:
        // 1. Initialize PSEL registers for I2S pins
        // 2. Configure MCK divider (CONFIG.MCK register)
        // 3. Set RATIO and ALIGN fields
        // 4. Enable I2S TX engine (CONFIG.TXEN)
        // 5. Start DMA on I2S TXD.PTR + AMOUNT
        // 6. Set TASKS_START
        
        self.dma_active = true;
        info!("▶ I2S playback started (DMA)");
        
        Ok(())
    }

    /// Stop I2S capture
    pub async fn stop_capture(&mut self) -> Result<(), I2sError> {
        if !self.dma_active {
            return Err(I2sError::NotRunning);
        }

        debug!("Stopping I2S capture");
        
        // In real implementation, would:
        // 1. Set TASKS_STOP
        // 2. Wait for EVENTS_STOPPED
        // 3. Disable I2S RX (CONFIG.RXEN = 0)
        // 4. Stop DMA
        
        self.dma_active = false;
        info!("⏹ I2S capture stopped");
        
        Ok(())
    }

    /// Stop I2S playback
    pub async fn stop_playback(&mut self) -> Result<(), I2sError> {
        if !self.dma_active {
            return Err(I2sError::NotRunning);
        }

        debug!("Stopping I2S playback");
        
        // In real implementation, would:
        // 1. Set TASKS_STOP
        // 2. Wait for EVENTS_STOPPED
        // 3. Disable I2S TX (CONFIG.TXEN = 0)
        // 4. Stop DMA
        
        self.dma_active = false;
        info!("⏹ I2S playback stopped");
        
        Ok(())
    }

    /// Push data to RX buffer (called by DMA interrupt)
    pub fn rx_dma_complete(&mut self, data: &[i16]) -> Result<(), I2sError> {
        // Validate frame size
        if data.len() != FRAME_SIZE {
            return Err(I2sError::BufferOverflow);
        }

        // Check buffer space
        if self.rx_buffer.len() + FRAME_SIZE > DMA_BUFFER_SIZE {
            return Err(I2sError::BufferOverflow);
        }

        // Add data to ring buffer
        for &sample in data {
            self.rx_buffer.push(sample)
                .map_err(|_| I2sError::BufferOverflow)?;
        }

        self.last_processed = self.rx_buffer.len() as u32;
        
        Ok(())
    }

    /// Pop data from TX buffer (called by DMA handler)
    pub fn tx_dma_request(&mut self, output: &mut [i16; FRAME_SIZE]) -> Result<(), I2sError> {
        // Need at least one frame in buffer
        if self.tx_buffer.len() < FRAME_SIZE {
            return Err(I2sError::BufferUnderflow);
        }

        // Pop frame from buffer
        for i in 0..FRAME_SIZE {
            output[i] = self.tx_buffer.remove(0);
        }

        Ok(())
    }

    /// Get current I2S status
    pub fn status(&self) -> I2sStatus {
        I2sStatus {
            capturing: self.dma_active,
            playing: self.dma_active,
            sample_rate: self.config.sample_rate.value(),
            rx_buffer_fill: self.rx_buffer.len(),
            tx_buffer_fill: self.tx_buffer.len(),
        }
    }

    /// Get number of samples available in RX buffer
    pub fn rx_available(&self) -> usize {
        self.rx_buffer.len()
    }

    /// Get free space in TX buffer
    pub fn tx_available(&self) -> usize {
        DMA_BUFFER_SIZE - self.tx_buffer.len()
    }

    /// Write sample to TX buffer
    pub fn write_tx(&mut self, sample: i16) -> Result<(), I2sError> {
        self.tx_buffer.push(sample)
            .map_err(|_| I2sError::BufferOverflow)
    }

    /// Read sample from RX buffer
    pub fn read_rx(&mut self) -> Result<i16, I2sError> {
        if self.rx_buffer.is_empty() {
            Err(I2sError::BufferUnderflow)
        } else {
            Ok(self.rx_buffer.remove(0))
        }
    }
}

pub struct I2sStatus {
    pub capturing: bool,
    pub playing: bool,
    pub sample_rate: u32,
    pub rx_buffer_fill: usize,
    pub tx_buffer_fill: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum I2sError {
    ConfigError,
    AlreadyRunning,
    NotRunning,
    BufferOverflow,
    BufferUnderflow,
    DmaError,
    PinConfigError,
}

/// Helper: Convert dB to linear scale
pub fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

/// Helper: Convert linear to dB
pub fn linear_to_db(linear: f32) -> f32 {
    20.0 * linear.max(1e-9).log10()
}

/// DMA Transfer Configuration Helper
pub struct DmaTransferConfig {
    pub ptr: *const u8,
    pub amount: usize,
}

impl DmaTransferConfig {
    pub fn new(buffer: &[i16]) -> Self {
        Self {
            ptr: buffer.as_ptr() as *const u8,
            amount: buffer.len() * 2, // bytes (i16 = 2 bytes)
        }
    }
}

/// nRF5340 I2S Register Map Reference
/// 
/// Key registers:
/// - TASKS_START (0x000)       - Start I2S transfer
/// - TASKS_STOP (0x004)        - Stop I2S transfer
/// - EVENTS_RXPTRUPD (0x104)   - RX DMA pointer updated
/// - EVENTS_TXPTRUPD (0x108)   - TX DMA pointer updated
/// - CONFIG (0x500)            - I2S configuration register
/// - CONFIG.MCK (0x500-0x504)  - Master clock config
/// - CONFIG.RATIO (4 bits)     - Master clock ratio
/// - RXD.PTR (0x538)           - RX DMA pointer
/// - RXD.AMOUNT (0x53C)        - RX DMA amount
/// - TXD.PTR (0x544)           - TX DMA pointer
/// - TXD.AMOUNT (0x548)        - TX DMA amount
pub mod nrf5340_i2s {
    use defmt::info;

    pub fn print_config_reference() {
        info!("═══ nRF5340 I2S DMA Configuration ═══");
        info!("TASKS_START:    Set bit 0 to start transfer");
        info!("CONFIG.MCK.EN:  Bit 0 - Enable master clock");
        info!("CONFIG.RATIO:   Bits 2-5 - MCK divider");
        info!("  0: /32  | 1: /48  | 2: /64  | 3: /96");
        info!("  4: /128 | 5: /192 | 6: /256 | 7: /384");
        info!("RXD.PTR:        DMA source pointer (I2S peripheral)");
        info!("RXD.AMOUNT:     Number of samples to transfer");
        info!("TXD.PTR:        DMA dest pointer (I2S peripheral)");
        info!("TXD.AMOUNT:     Number of samples to transfer");
        info!("════════════════════════════════════");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_rate_config() {
        let sr_16k = SampleRate::Hz16000;
        assert_eq!(sr_16k.value(), 16000);
        assert_eq!(sr_16k.mck_ratio(), 128);

        let sr_48k = SampleRate::Hz48000;
        assert_eq!(sr_48k.value(), 48000);
        assert_eq!(sr_48k.mck_ratio(), 128);
    }

    #[test]
    fn test_mck_value() {
        let sr = SampleRate::Hz16000;
        let mck_mono = sr.mck_value(Channels::Mono);
        let mck_stereo = sr.mck_value(Channels::Stereo);

        assert_eq!(mck_mono, 16000 * 32);    // 512 kHz
        assert_eq!(mck_stereo, 16000 * 64);  // 1.024 MHz
    }

    #[test]
    fn test_i2s_driver_creation() {
        let config = I2sConfig {
            sample_rate: SampleRate::Hz16000,
            channels: Channels::Mono,
            mode: I2sMode::Master,
        };

        let driver = I2sDriver::new(config);
        let status = driver.status();

        assert_eq!(status.sample_rate, 16000);
        assert!(!status.capturing);
        assert!(!status.playing);
    }

    #[test]
    fn test_buffer_management() {
        let config = I2sConfig {
            sample_rate: SampleRate::Hz16000,
            channels: Channels::Mono,
            mode: I2sMode::Master,
        };

        let mut driver = I2sDriver::new(config);

        // Write some samples
        for i in 0..10 {
            assert!(driver.write_tx((i * 100) as i16).is_ok());
        }

        assert_eq!(driver.tx_available(), DMA_BUFFER_SIZE - 10);
    }

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

    #[test]
    fn test_dma_transfer_config() {
        let buffer = [100i16, 200, 300, 400];
        let config = DmaTransferConfig::new(&buffer);

        assert_eq!(config.ptr, buffer.as_ptr() as *const u8);
        assert_eq!(config.amount, 8); // 4 samples * 2 bytes
    }
}
