/// NFL Firmware - Main Entry Point
/// nRF5340 Application Core - Phase 1: Audio DSP Pipeline
/// 
/// Architecture:
/// - Embassy async runtime (bare-metal no-std)
/// - Dual-core: App core (DSP) + Net core (BLE in Phase 2)
/// - Audio pipeline: I2S capture → DSP → I2S playback
/// - Target latency: < 15ms end-to-end

#![no_std]
#![no_main]

use nfl_firmware::audio::pipeline::{AudioPipeline, PipelineConfig};
use nfl_firmware::audio::dsp::equalizer::EqProfile;
use defmt::{info, warn, error};
use embassy_executor::Spawner;
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("🦻 NFL Firmware v0.1.0 - Phase 1: Audio DSP Pipeline");
    info!("═══════════════════════════════════════════════════════");
    info!("Target: nRF5340 Application Core (ARM Cortex-M33 @ 120MHz)");
    info!("Mode: Audio capture → DSP processing → Audio playback");
    info!("═══════════════════════════════════════════════════════");

    // Initialize audio pipeline
    let mut pipeline = match AudioPipeline::new() {
        Ok(p) => {
            info!("✅ Audio pipeline created successfully");
            p
        }
        Err(e) => {
            error!("❌ Failed to create audio pipeline: {}", e);
            return;
        }
    };

    // Configure DSP
    let mut config = PipelineConfig::default();
    config.enable_noise_gate = true;
    config.enable_highpass = true;
    config.enable_eq = true;
    config.enable_limiter = true;
    config.output_gain_db = 0.0;

    pipeline.set_config(config);
    info!("✅ DSP pipeline configured");
    info!("   • Noise gate: enabled");
    info!("   • High-pass filter: enabled (100Hz)");
    info!("   • 8-band EQ: enabled");
    info!("   • Limiter: enabled (-6dBFS)");

    // Set EQ profile (flat for Phase 1 baseline)
    let eq_profile = EqProfile::flat();
    pipeline.set_eq_profile(eq_profile);
    info!("✅ EQ profile: FLAT (no adjustment)");

    // Start audio I/O
    match pipeline.start().await {
        Ok(_) => {
            info!("✅ Audio I/O started");
            info!("   • I2S capture (SPH0645 MEMS mic)");
            info!("   • I2S playback (TPA6132A2 amp)");
            info!("   • Sample rate: 16 kHz");
            info!("   • Frame size: 256 samples (16ms)");
        }
        Err(e) => {
            error!("❌ Failed to start audio I/O: {}", e);
            return;
        }
    }

    info!("");
    info!("🎵 Audio processing pipeline is ACTIVE");
    info!("   Listening for incoming audio...");
    info!("");

    // Spawn background task for processing
    spawner.spawn(audio_processing_task(pipeline))
        .expect("Failed to spawn audio task");

    info!("✅ Audio task spawned successfully");
    info!("");
    info!("═══════════════════════════════════════════════════════");
    info!("Phase 1 Initialization Complete!");
    info!("Waiting for audio input...");
    info!("═══════════════════════════════════════════════════════");

    // Keep main loop alive
    loop {
        embassy_time::Timer::after_millis(10000).await;

        // Status update every 10 seconds
        info!("🔄 Pipeline heartbeat - still running...");
    }
}

#[embassy_executor::task]
async fn audio_processing_task(mut pipeline: AudioPipeline) {
    info!("🚀 Audio processing task started");

    let mut error_count = 0;
    let mut success_count = 0;

    loop {
        match pipeline.process_frame().await {
            Ok(_) => {
                success_count += 1;

                // Print status every 100 frames (1.6 seconds @ 16ms/frame)
                if success_count % 100 == 0 {
                    pipeline.print_status();
                }
            }
            Err(e) => {
                error_count += 1;
                warn!("Audio processing error: {:?} (count: {})", e, error_count);

                if error_count > 10 {
                    error!("❌ Too many errors, stopping audio pipeline");
                    break;
                }
            }
        }

        // Yield to other tasks
        embassy_time::Timer::after_micros(10).await;
    }
}

