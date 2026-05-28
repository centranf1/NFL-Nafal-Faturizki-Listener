//! Host-side mock I2S implementation for testing and simulation
//!
//! This module provides a lightweight, std-based implementation of the
//! `hal::i2s` API so the rest of the firmware can be exercised on the
//! development machine without nRF hardware.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread::{self, JoinHandle};
use std::time::Duration;

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

    pub fn mck_ratio(&self) -> u32 {
        match self {
            SampleRate::Hz16000 => 128,
            SampleRate::Hz48000 => 128,
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

pub const DMA_BUFFER_SIZE: usize = 512;
pub const FRAME_SIZE: usize = 256;

pub struct I2sDriver {
    pub config: I2sConfig,
    rx_buffer: Arc<Mutex<VecDeque<i16>>>,
    tx_buffer: Arc<Mutex<VecDeque<i16>>>,
    dma_active: bool,
    last_processed: u32,

    // Simulation control
    capture_thread: Option<JoinHandle<()>>,
    playback_thread: Option<JoinHandle<()>>,
    capture_running: Arc<AtomicBool>,
    playback_running: Arc<AtomicBool>,
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

pub struct I2sStatus {
    pub capturing: bool,
    pub playing: bool,
    pub sample_rate: u32,
    pub rx_buffer_fill: usize,
    pub tx_buffer_fill: usize,
}

impl I2sDriver {
    pub fn new(config: I2sConfig) -> Self {
        println!("[host] I2S driver (mock) initialized: {} Hz", config.sample_rate.value());
        Self {
            config,
            rx_buffer: Arc::new(Mutex::new(VecDeque::with_capacity(DMA_BUFFER_SIZE))),
            tx_buffer: Arc::new(Mutex::new(VecDeque::with_capacity(DMA_BUFFER_SIZE))),
            dma_active: false,
            last_processed: 0,
            capture_thread: None,
            playback_thread: None,
            capture_running: Arc::new(AtomicBool::new(false)),
            playback_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn configure_pins(&mut self) -> Result<(), I2sError> {
        Ok(())
    }

    pub async fn start_capture(&mut self) -> Result<(), I2sError> {
        self.dma_active = true;
        // Spawn simulation thread that periodically injects a sine wave frame
        if !self.capture_running.load(Ordering::SeqCst) {
            self.capture_running.store(true, Ordering::SeqCst);

            let rx_buf = Arc::clone(&self.rx_buffer);
            let running = Arc::clone(&self.capture_running);
            let sample_rate = self.config.sample_rate.value();

            let handle = thread::spawn(move || {
                let frame_interval = Duration::from_micros((FRAME_SIZE as u64 * 1_000_000) / sample_rate as u64);
                let mut phase: f32 = 0.0;
                let freq = 1000.0; // 1kHz test tone
                let two_pi = core::f32::consts::PI * 2.0;

                while running.load(Ordering::SeqCst) {
                    // Generate frame
                    let mut frame = [0i16; FRAME_SIZE];
                    for i in 0..FRAME_SIZE {
                        let t = (i as f32) / (sample_rate as f32);
                        let v = (phase + two_pi * freq * t).sin();
                        frame[i] = (v * 0.1 * 32767.0) as i16; // -20dB tone
                    }

                    // Push into rx buffer
                    if let Ok(mut buf) = rx_buf.lock() {
                        for &s in frame.iter() {
                            if buf.len() < DMA_BUFFER_SIZE {
                                buf.push_back(s);
                            } else {
                                // drop oldest
                                buf.pop_front();
                                buf.push_back(s);
                            }
                        }
                    }

                    // Advance phase
                    phase += two_pi * freq * (FRAME_SIZE as f32) / (sample_rate as f32);

                    thread::sleep(frame_interval);
                }
            });

            self.capture_thread = Some(handle);
        }

        Ok(())
    }

    pub async fn start_playback(&mut self) -> Result<(), I2sError> {
        self.dma_active = true;
        if !self.playback_running.load(Ordering::SeqCst) {
            self.playback_running.store(true, Ordering::SeqCst);

            let tx_buf = Arc::clone(&self.tx_buffer);
            let running = Arc::clone(&self.playback_running);
            let sample_rate = self.config.sample_rate.value();

            let handle = thread::spawn(move || {
                let frame_interval = Duration::from_micros((FRAME_SIZE as u64 * 1_000_000) / sample_rate as u64);

                while running.load(Ordering::SeqCst) {
                    // Simulate DMA consuming a frame
                    if let Ok(mut buf) = tx_buf.lock() {
                        if buf.len() >= FRAME_SIZE {
                            // Pop FRAME_SIZE samples
                            for _ in 0..FRAME_SIZE {
                                buf.pop_front();
                            }
                        }
                    }

                    thread::sleep(frame_interval);
                }
            });

            self.playback_thread = Some(handle);
        }

        Ok(())
    }

    pub async fn stop_capture(&mut self) -> Result<(), I2sError> {
        self.dma_active = false;
        // Stop simulation thread
        self.capture_running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.capture_thread.take() {
            let _ = handle.join();
        }

        Ok(())
    }

    pub async fn stop_playback(&mut self) -> Result<(), I2sError> {
        self.dma_active = false;
        self.playback_running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.playback_thread.take() {
            let _ = handle.join();
        }

        Ok(())
    }

    pub fn rx_dma_complete(&mut self, data: &[i16]) -> Result<(), I2sError> {
        if data.len() != FRAME_SIZE {
            return Err(I2sError::BufferOverflow);
        }
        if let Ok(mut buf) = self.rx_buffer.lock() {
            if buf.len() + data.len() > DMA_BUFFER_SIZE {
                return Err(I2sError::BufferOverflow);
            }
            for &s in data {
                buf.push_back(s);
            }
            self.last_processed = buf.len() as u32;
        }
        Ok(())
    }

    pub fn tx_dma_request(&mut self, output: &mut [i16; FRAME_SIZE]) -> Result<(), I2sError> {
        if let Ok(mut buf) = self.tx_buffer.lock() {
            if buf.len() < FRAME_SIZE {
                return Err(I2sError::BufferUnderflow);
            }
            for i in 0..FRAME_SIZE {
                output[i] = buf.pop_front().unwrap_or(0);
            }
        }
        Ok(())
    }

    pub fn status(&self) -> I2sStatus {
        I2sStatus {
            capturing: self.dma_active,
            playing: self.dma_active,
            sample_rate: self.config.sample_rate.value(),
            rx_buffer_fill: self.rx_buffer.lock().map(|b| b.len()).unwrap_or(0),
            tx_buffer_fill: self.tx_buffer.lock().map(|b| b.len()).unwrap_or(0),
        }
    }

    pub fn rx_available(&self) -> usize {
        self.rx_buffer.lock().map(|b| b.len()).unwrap_or(0)
    }

    pub fn tx_available(&self) -> usize {
        DMA_BUFFER_SIZE - self.tx_buffer.lock().map(|b| b.len()).unwrap_or(0)
    }

    pub fn write_tx(&mut self, sample: i16) -> Result<(), I2sError> {
        if let Ok(mut buf) = self.tx_buffer.lock() {
            if buf.len() >= DMA_BUFFER_SIZE {
                return Err(I2sError::BufferOverflow);
            }
            buf.push_back(sample);
        }
        Ok(())
    }

    pub fn read_rx(&mut self) -> Result<i16, I2sError> {
        if let Ok(mut buf) = self.rx_buffer.lock() {
            if let Some(s) = buf.pop_front() {
                return Ok(s);
            }
        }
        Err(I2sError::BufferUnderflow)
    }
}
