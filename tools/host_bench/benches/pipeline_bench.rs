use criterion::{criterion_group, criterion_main, Criterion};
use nfl_firmware::audio::pipeline::AudioPipeline;
use futures::executor::block_on;

fn bench_pipeline_frame(c: &mut Criterion) {
    // Initialize pipeline once
    let mut pipeline = AudioPipeline::new().expect("pipeline init failed");

    c.bench_function("process_frame", |b| {
        b.iter(|| {
            // Inject a synthetic frame
            let mut frame = [0i16; 256];
            for i in 0..256 {
                frame[i] = ((i as i16) % 256) as i16;
            }
            pipeline.inject_frame(&frame).expect("inject");
            // Process one frame
            let _ = block_on(pipeline.process_frame());
        })
    });
}

criterion_group!(benches, bench_pipeline_frame);
criterion_main!(benches);
