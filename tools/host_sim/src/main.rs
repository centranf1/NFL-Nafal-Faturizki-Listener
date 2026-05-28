use futures::executor::block_on;
use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use nfl_firmware::audio::pipeline::{AudioPipeline, PIPELINE_FRAME_SIZE};
use nfl_firmware::audio::pipeline::PipelineConfig;
use std::env;
use std::path::Path;

fn main() {
    println!("NFL Firmware host simulation enriched demo starting...");

    let args: Vec<String> = env::args().collect();
    // Usage: host_sim [input.wav] [output.wav] [duration_seconds]
    let input = args.get(1).map(|s| s.as_str());
    let output = args.get(2).map(|s| s.as_str()).unwrap_or("out.wav");
    let duration_secs: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(2);

    // Create pipeline
    let mut pipeline = AudioPipeline::new().expect("pipeline init failed");

    // Configure pipeline for deterministic behavior
    let mut cfg = PipelineConfig::default();
    cfg.enable_eq = false;
    cfg.enable_noise_gate = false;
    pipeline.set_config(cfg);

    // Prepare WAV writer for output (16kHz, mono, 16-bit)
    let spec = WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create(Path::new(output), spec).expect("failed to create output wav");

    // If input WAV provided, stream frames from it; otherwise generate simple ramp tone
    if let Some(input_path) = input {
        println!("Using input WAV: {}", input_path);
        let mut reader = WavReader::open(input_path).expect("failed to open input wav");
        let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap_or(0)).collect();

        let total_samples = samples.len();
        let mut offset = 0usize;
        let frames_to_process = (duration_secs * 16000) / PIPELINE_FRAME_SIZE;

        for _ in 0..frames_to_process {
            let mut frame = [0i16; PIPELINE_FRAME_SIZE];
            for i in 0..PIPELINE_FRAME_SIZE {
                frame[i] = samples[offset % total_samples];
                offset += 1;
            }

            pipeline.inject_frame(&frame).expect("inject failed");
            let res = block_on(pipeline.process_frame());
            if res.is_err() {
                eprintln!("process_frame error: {:?}", res);
            }

            if let Some(out_frame) = pipeline.drain_playback_frame() {
                for s in out_frame.iter() {
                    writer.write_sample(*s).ok();
                }
            }
        }
    } else {
        println!("No input WAV provided — generating ramp frames for {} seconds", duration_secs);
        let frames_to_process = (duration_secs * 16000) / PIPELINE_FRAME_SIZE;
        for f in 0..frames_to_process {
            let mut frame = [0i16; PIPELINE_FRAME_SIZE];
            for i in 0..PIPELINE_FRAME_SIZE {
                // ramp pattern that varies over frames
                let v = (((f * PIPELINE_FRAME_SIZE + i) % 65536) as i32 - 32768) as i16;
                frame[i] = v / 4; // reduce amplitude
            }

            pipeline.inject_frame(&frame).expect("inject failed");
            let res = block_on(pipeline.process_frame());
            if res.is_err() {
                eprintln!("process_frame error: {:?}", res);
            }

            if let Some(out_frame) = pipeline.drain_playback_frame() {
                for s in out_frame.iter() {
                    writer.write_sample(*s).ok();
                }
            }
        }
    }

    writer.finalize().ok();
    println!("Simulation finished. Output written to {}", output);
}
