use clap::Parser;
use futures::executor::block_on;
use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use nfl_firmware::audio::pipeline::{AudioPipeline, PIPELINE_FRAME_SIZE};
use nfl_firmware::audio::pipeline::PipelineConfig;
use std::path::Path;
use std::fs::File;

/// Host Simulator for NFL firmware pipeline
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Optional input WAV file (16kHz, mono, 16-bit)
    input: Option<String>,

    /// Output WAV file path (default: out.wav)
    #[arg(short, long, default_value = "out.wav")]
    output: String,

    /// Duration in seconds to simulate (default: 2)
    #[arg(short, long, default_value_t = 2)]
    duration: usize,

    /// If set, generate a tone instead of using input WAV
    #[arg(short = 't', long, default_value_t = 1000.0)]
    tone_freq: f32,

    /// Tone amplitude (0.0..1.0)
    #[arg(short = 'a', long, default_value_t = 0.1)]
    tone_amp: f32,

    /// CSV output path for per-frame stats (optional)
    #[arg(short, long)]
    csv: Option<String>,

    /// Play generated WAV after simulation (requires `--features playback`)
    #[arg(short, long)]
    play: bool,
}

fn rms_db_from_samples(samples: &[i16]) -> f32 {
    if samples.is_empty() {
        return -120.0;
    }
    let sum_sq: f64 = samples.iter().map(|&s| (s as f64) * (s as f64)).sum();
    let rms = (sum_sq / (samples.len() as f64)).sqrt();
    let rms = rms as f32;
    20.0 * (rms / 32768.0).max(1e-12).log10()
}

fn rms_linear(samples: &[i16]) -> f32 {
    if samples.is_empty() { return 0.0; }
    let sum_sq: f64 = samples.iter().map(|&s| (s as f64) * (s as f64)).sum();
    let rms = (sum_sq / (samples.len() as f64)).sqrt();
    rms as f32
}

fn main() {
    let args = Args::parse();

    println!("Simulator args: {:?}", args);

    // Create pipeline
    let mut pipeline = AudioPipeline::new().expect("pipeline init failed");
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

    let mut writer = WavWriter::create(Path::new(&args.output), spec).expect("failed to create output wav");

    // Prepare CSV writer if requested
    let mut csv_writer = if let Some(csv_path) = &args.csv {
        let file = File::create(csv_path).expect("failed to create csv");
        let mut wtr = csv::Writer::from_writer(file);
        wtr.write_record(&["frame_idx","input_rms_db","output_rms_db","noise_rms_db","snr_db"]).ok();
        Some(wtr)
    } else { None };

    let frames_to_process = (args.duration * 16000) / PIPELINE_FRAME_SIZE;

    if let Some(input_path) = args.input.as_deref() {
        println!("Using input WAV: {}", input_path);
        let mut reader = WavReader::open(input_path).expect("failed to open input wav");
        let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap_or(0)).collect();
        let total_samples = samples.len();
        let mut offset = 0usize;

        for frame_idx in 0..frames_to_process {
            let mut in_frame = [0i16; PIPELINE_FRAME_SIZE];
            for i in 0..PIPELINE_FRAME_SIZE {
                in_frame[i] = samples[offset % total_samples];
                offset += 1;
            }

            pipeline.inject_frame(&in_frame).expect("inject failed");
            let res = block_on(pipeline.process_frame());
            if res.is_err() {
                eprintln!("process_frame error: {:?}", res);
            }

            if let Some(out_frame) = pipeline.drain_playback_frame() {
                // write wav
                for s in out_frame.iter() { writer.write_sample(*s).ok(); }

                // stats
                let input_rms_db = rms_db_from_samples(&in_frame);
                let output_rms_db = rms_db_from_samples(&out_frame);
                // compute noise = out - in
                let mut noise = [0i16; PIPELINE_FRAME_SIZE];
                for i in 0..PIPELINE_FRAME_SIZE { noise[i] = out_frame[i].wrapping_sub(in_frame[i]); }
                let noise_rms = rms_linear(&noise);
                let snr_db = if noise_rms < 1e-6 { 120.0 } else {
                    20.0 * (rms_linear(&out_frame) / noise_rms).max(1e-12).log10()
                };

                if let Some(wtr) = csv_writer.as_mut() {
                    wtr.write_record(&[
                        frame_idx.to_string(),
                        format!("{:.2}", input_rms_db),
                        format!("{:.2}", output_rms_db),
                        format!("{:.4}", noise_rms),
                        format!("{:.2}", snr_db),
                    ]).ok();
                }
            }
        }
    } else {
        println!("Generating tone: {} Hz, amp {} for {} seconds", args.tone_freq, args.tone_amp, args.duration);
        let sample_rate = 16000f32;
        for frame_idx in 0..frames_to_process {
            let mut in_frame = [0i16; PIPELINE_FRAME_SIZE];
            for i in 0..PIPELINE_FRAME_SIZE {
                let t = (frame_idx * PIPELINE_FRAME_SIZE + i) as f32 / sample_rate;
                let v = (2.0 * std::f32::consts::PI * args.tone_freq * t).sin();
                in_frame[i] = (v * args.tone_amp * 32767.0) as i16;
            }

            pipeline.inject_frame(&in_frame).expect("inject failed");
            let res = block_on(pipeline.process_frame());
            if res.is_err() {
                eprintln!("process_frame error: {:?}", res);
            }

            if let Some(out_frame) = pipeline.drain_playback_frame() {
                for s in out_frame.iter() { writer.write_sample(*s).ok(); }

                let input_rms_db = rms_db_from_samples(&in_frame);
                let output_rms_db = rms_db_from_samples(&out_frame);
                let mut noise = [0i16; PIPELINE_FRAME_SIZE];
                for i in 0..PIPELINE_FRAME_SIZE { noise[i] = out_frame[i].wrapping_sub(in_frame[i]); }
                let noise_rms = rms_linear(&noise);
                let snr_db = if noise_rms < 1e-6 { 120.0 } else {
                    20.0 * (rms_linear(&out_frame) / noise_rms).max(1e-12).log10()
                };

                if let Some(wtr) = csv_writer.as_mut() {
                    wtr.write_record(&[
                        frame_idx.to_string(),
                        format!("{:.2}", input_rms_db),
                        format!("{:.2}", output_rms_db),
                        format!("{:.4}", noise_rms),
                        format!("{:.2}", snr_db),
                    ]).ok();
                }
            }
        }
    }

    writer.finalize().ok();
    if let Some(wtr) = csv_writer.as_mut() { wtr.flush().ok(); }
    println!("Simulation finished. Output written to {}", args.output);
    if let Some(csv) = args.csv { println!("CSV stats written to {}", csv); }

    if args.play {
        #[cfg(feature = "playback")]
        {
            println!("Playing output WAV...");
            if let Ok(device) = rodio::default_output_device() {
                if let Ok(file) = std::fs::File::open(&args.output) {
                    if let Ok(source) = rodio::Decoder::new(std::io::BufReader::new(file)) {
                        let sink = rodio::Sink::try_new(&device).expect("failed to create sink");
                        sink.append(source);
                        sink.sleep_until_end();
                    }
                }
            } else {
                eprintln!("No default audio output device available");
            }
        }

        #[cfg(not(feature = "playback"))]
        {
            eprintln!("Playback feature not enabled. Rebuild with `--features playback` to enable audio playback.");
        }
    }
}
