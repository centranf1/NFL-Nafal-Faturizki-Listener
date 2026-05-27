/// NFL Firmware Library
/// 
/// Phase 1: Audio DSP Pipeline for nRF5340 Hearing Aid
/// 
/// Modular organization:
/// - audio/     → I2S drivers + audio processing pipeline
/// - ble/       → GATT server + BLE profiles (Phase 2)
/// - storage/   → Flash management (Phase 2)
/// - power/     → Power management (Phase 2)
/// - hal/       → Hardware abstraction layer

#![no_std]

pub mod audio {
    pub mod capture;
    pub mod playback;
    pub mod pipeline;

    pub mod dsp {
        pub mod noise_gate;
        pub mod filters;
        pub mod equalizer;
        pub mod compressor;
    }
}

pub mod ble {
    pub mod gatt_server {
        // Placeholder for Phase 2: GATT server definition
    }
    pub mod profiles {
        pub mod calibration {
            // Placeholder for Phase 2: EQ calibration service
        }
        pub mod battery {
            // Placeholder for Phase 2: Battery service
        }
        pub mod ota {
            // Placeholder for Phase 2: OTA update service
        }
    }
    pub mod advertising {
        // Placeholder for Phase 2: BLE advertisement setup
    }
}

pub mod storage {
    pub mod flash {
        // Placeholder for Phase 2: Flash driver
    }
    pub mod profile {
        // Placeholder for Phase 2: User profile management
    }
    pub mod config {
        // Placeholder for Phase 2: Device configuration
    }
}

pub mod power {
    pub mod manager {
        // Placeholder for Phase 2: Power state management
    }
    pub mod battery {
        // Placeholder for Phase 2: Battery monitoring
    }
}

pub mod hal {
    pub mod i2s;
    pub mod spi {
        // Placeholder for Phase 2+: SPI for flash
    }
    pub mod gpio {
        // Placeholder for Phase 2+: GPIO for amplifier enable
    }
}

