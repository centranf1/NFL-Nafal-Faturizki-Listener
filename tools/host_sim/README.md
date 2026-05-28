# NFL Firmware Host Simulator

Usage examples:

Generate a 2s tone and write `out.wav` + `stats.csv`:

```bash
cargo run --manifest-path tools/host_sim/Cargo.toml -- --output out.wav --duration 2 --tone-freq 1000 --tone-amp 0.1 --csv stats.csv
```

Play the generated WAV (requires audio device and `playback` feature):

```bash
cargo run --manifest-path tools/host_sim/Cargo.toml --features playback -- --output out.wav --duration 2 --play
```

Plot CSV results (requires Python & matplotlib):

```bash
python3 scripts/plot_stats.py stats.csv
```
