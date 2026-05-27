# 🎵 PHASE 1 — Audio Proof of Concept

**Duration:** Minggu 4–8 (4-8 weeks)  
**Target:** Audio passthrough dengan DSP pipeline, latency < 15ms

---

## 🎯 Phase 1 Objectives

### Primary Goals
1. ✅ **Audio Capture** — SPH0645 I2S microphone integration
2. ✅ **Audio Playback** — TPA6132A2 I2S amplifier integration
3. ✅ **Passthrough Pipeline** — Mic → Speaker (no DSP, baseline latency)
4. ✅ **Noise Gate** — Remove ambient noise below threshold
5. ✅ **8-Band Equalizer** — Parametric EQ for hearing correction
6. ✅ **Real Hardware Testing** — Verify with oscilloscope & listening tests

### Success Criteria
- ✅ Audio captures cleanly (no clipping, clean SNR)
- ✅ Audio plays back with minimal distortion (THD < 2%)
- ✅ Latency measured < 15ms (end-to-end)
- ✅ Noise gate reduces ambient noise by 20+ dB
- ✅ Equalizer adjusts ±20dB per band without artifacts
- ✅ Real-time DSP without audio dropouts

---

## 📋 Implementation Roadmap

### Milestone 1: Hardware Abstraction (Week 1-2)
**Goal:** Clean abstraction for I2S peripheral

Files to implement:
- `firmware/src/hal/i2s.rs` — I2S HAL abstraction
- `firmware/src/hal/gpio.rs` — GPIO abstraction (optional, minimal)

Key points:
- I2S master mode for nRF5340 APP core
- Configure for 16kHz, 16-bit, stereo/mono
- Use Embassy + nrf-softdevice
- DMA for efficient audio streaming

### Milestone 2: Audio Drivers (Week 2-3)
**Goal:** Complete I2S capture & playback

Files:
- `firmware/src/audio/capture.rs` — I2S input driver (SPH0645)
- `firmware/src/audio/playback.rs` — I2S output driver (TPA6132A2)

Key specifications:
- **SPH0645:** I2S digital mic, 16kHz/16-bit, -26dBFS sensitivity
- **TPA6132A2:** I2S Class-G amplifier, 25mW/channel
- Use ring buffers for audio flow
- Implement with Embassy async patterns

### Milestone 3: Audio Pipeline (Week 3-4)
**Goal:** Real-time audio passthrough

Files:
- `firmware/src/audio/pipeline.rs` — DSP pipeline coordinator
- Update `firmware/src/main.rs` — Integration

Architecture:
```
Capture (I2S in) 
    ↓
Pipeline processor (per-frame)
    ↓
Playback (I2S out)
```

Frame size: 256 samples @ 16kHz = 16ms latency baseline

### Milestone 4: DSP Algorithms (Week 4-6)
**Goal:** Noise gate + equalizer

Files:
- `firmware/src/audio/dsp/noise_gate.rs` — Adaptive noise gate
- `firmware/src/audio/dsp/filters.rs` — IIR filter implementations
- `firmware/src/audio/dsp/equalizer.rs` — 8-band parametric EQ
- `firmware/src/audio/dsp/compressor.rs` — Dynamic compressor (Phase 2)

Algorithms:
1. **Noise Gate:**
   - Threshold: -50dBFS (adaptive)
   - Release: 100ms
   - Attack: 1ms

2. **High-Pass Filter:**
   - 100Hz cutoff
   - 2nd order Butterworth
   - Remove wind noise & rumble

3. **8-Band EQ:**
   - Frequencies: 250Hz, 500Hz, 1kHz, 2kHz, 3kHz, 4kHz, 6kHz, 8kHz
   - Gain range: -20dB to +40dB
   - Fixed-point arithmetic (no floating point)

4. **Limiter:**
   - Hard limit at -6dBFS
   - Proteksi akhir terhadap clipping

### Milestone 5: Integration & Testing (Week 6-8)
**Goal:** Complete firmware + validation

Files:
- `firmware/src/main.rs` — Full integration
- `firmware/tests/audio_test.rs` — Unit tests

Testing:
- Hardware latency measurement (oscilloscope)
- Audio quality subjective tests
- Real volunteers (5+)

---

## 🔧 Technical Specifications

### Audio Format
- **Sample rate:** 16 kHz
- **Bit depth:** 16-bit signed (i16)
- **Channels:** Mono (can expand to stereo in Phase 2)
- **Frame size:** 256 samples = 16ms
- **Buffer size:** 512 samples (2 frames)

### DSP Processing (per 256-sample frame)
```
Frame processing @ 62.5 fps (16ms per frame)

1. Capture from SPH0645 (I2S in)
2. Noise gate filter
3. High-pass filter (100Hz)
4. 8-band equalizer
5. Hard limiter
6. Playback to TPA6132A2 (I2S out)

Total latency target: < 15ms (3-4 frame buffers)
```

### Fixed-Point Arithmetic
- Use `fixed` crate for DSP calculations
- Q15 format (15 bits fractional) for audio samples
- No floating point in hot path
- Reduces power consumption + ensures deterministic timing

### Memory Budget
- Flash: ~300KB for firmware + DSP code
- RAM: ~100KB for buffers + state variables
- nRF5340: 1MB flash, 512KB RAM → plenty of room

---

## 📊 Expected Performance

| Metric | Target | Notes |
|--------|--------|-------|
| Capture latency | 2-3ms | I2S DMA |
| Processing latency | 5-8ms | DSP pipeline (3-4 frame buffers) |
| Playback latency | 2-3ms | I2S DMA |
| **Total latency** | **< 15ms** | End-to-end |
| Noise floor | < -55dBFS | Clean capture |
| THD | < 2% | Playback distortion |
| Power consumption | 8-10mA | Normal mode (full DSP) |

---

## 🔌 Hardware Connections (nRF5340 DK)

### I2S (Audio Data)
- `P0.04` → SPH0645 SCK (clock)
- `P0.05` → SPH0645 SDI (data in)
- `P0.07` → TPA6132A2 SCK (clock)
- `P0.08` → TPA6132A2 SDO (data out)
- `P0.25` → SPH0645 WS (word select)
- `P0.26` → TPA6132A2 WS (word select)

### Control
- `P0.28` → TPA6132A2 ENABLE (GPIO output, active high)
- `P0.29` → LED indicator (GPIO output)

### Development Tips
- Use oscilloscope on I2S lines to verify timing
- Use logic analyzer to capture I2S frames
- Use audio probe/mic to verify output quality
- Record audio samples for offline analysis

---

## 🧪 Testing Strategy

### Phase 1a: Passthrough Validation
1. Connect oscilloscope to I2S lines
2. Feed known signal to SPH0645
3. Capture output on TPA6132A2
4. Measure latency between input & output
5. Target: Clean audio, < 15ms latency

### Phase 1b: DSP Algorithm Testing
1. Apply noise gate → measure noise reduction
2. Apply EQ → measure frequency response
3. Check for artifacts (pops, clicks)
4. Measure CPU usage / power consumption

### Phase 1c: Real User Testing
1. Deploy on 5+ volunteers
2. Subjective audio quality assessment
3. Measure battery life
4. Collect feedback for Phase 2

---

## 📁 Files to Create/Modify

### New Files (Phase 1)
```
firmware/src/hal/
├── i2s.rs ......................... I2S HAL abstraction
├── gpio.rs ........................ GPIO (if needed)
└── spi.rs ......................... (Keep stub for storage, Phase 2)

firmware/src/audio/
├── capture.rs ..................... SPH0645 I2S driver
├── playback.rs .................... TPA6132A2 I2S driver
├── pipeline.rs .................... Main DSP pipeline
└── dsp/
    ├── noise_gate.rs .............. Noise gate algorithm
    ├── filters.rs ................. IIR filter library
    ├── equalizer.rs ............... 8-band parametric EQ
    └── compressor.rs .............. (Phase 2 placeholder)

firmware/tests/
├── audio_test.rs .................. Audio pipeline tests
└── dsp_test.rs .................... DSP algorithm tests
```

### Modified Files
```
firmware/src/main.rs ............... Integration with audio pipeline
firmware/src/lib.rs ................ Module declarations
firmware/Cargo.toml ................ Additional dependencies
```

---

## 📦 Dependencies to Add

```toml
# In firmware/Cargo.toml

[dependencies]
# Existing
embassy-executor = { workspace = true }
embassy-nrf = { workspace = true }
embassy-time = { workspace = true }
defmt = { workspace = true }

# NEW for Phase 1
embassy-embedded-hal = "0.1"      # Embedded HAL traits
heapless = { workspace = true }   # Fixed-size collections
fixed = { workspace = true }      # Fixed-point math

# Optional but useful
micromath = "0.2"                 # Optimized math functions
```

---

## 🚀 Development Workflow

### Quick Iteration Loop
1. Make code changes
2. `cargo build -p nfl-firmware --release`
3. Connect hardware via USB
4. `cargo run -p nfl-firmware --release` (flashes + monitors logs)
5. Test with oscilloscope / listening
6. Fix issues
7. Repeat

### Debugging Tips
- Use `defmt` logging (lightweight embedded logging)
- `defmt::info!()`, `defmt::debug!()` for troubleshooting
- Use `cargo run` to see RTT output in real-time
- Use probe-rs debugger for breakpoints if needed

### Code Quality
- Run `cargo fmt --all` before commits
- Run `cargo clippy --all` to catch issues
- Keep code modular & testable

---

## ✅ Phase 1 Completion Checklist

### Week 1-2: HAL
- [ ] I2S HAL abstraction implemented
- [ ] Compiles without errors
- [ ] Ready for driver implementation

### Week 2-3: Drivers
- [ ] SPH0645 capture driver complete
- [ ] TPA6132A2 playback driver complete
- [ ] I2S data flowing through both drivers
- [ ] Verified with oscilloscope

### Week 3-4: Pipeline
- [ ] Audio passthrough pipeline working
- [ ] Latency measured (< 15ms target)
- [ ] No audio dropouts
- [ ] Clean audio quality confirmed

### Week 4-6: DSP
- [ ] Noise gate implemented & tested
- [ ] High-pass filter verified
- [ ] 8-band equalizer working
- [ ] Limiter active (preventing clipping)
- [ ] No artifacts or distortion

### Week 6-8: Integration & Testing
- [ ] All modules integrated into firmware
- [ ] Full DSP pipeline tested end-to-end
- [ ] Unit tests passing
- [ ] Real hardware validation (5+ users)
- [ ] Phase 1 documentation complete

---

## 📚 Reference Documentation

### Datasheets
- **SPH0645:** Knowles digital MEMS microphone I2S interface
- **TPA6132A2:** TI Class-G headphone amplifier I2S input
- **nRF5340:** Nordic I2S peripheral (LFCLK/HFCLK timing)

### Embassy Resources
- https://embassy.dev/ — Official Embassy framework docs
- https://github.com/embassy-rs/embassy — Source code examples
- nRF5340 examples in embassy-rs repository

### DSP Theory
- **Noise Gate:** Threshold-based amplitude expansion
- **IIR Filters:** Digital filter design, fixed-point implementation
- **Parametric EQ:** Biquad filter topology (most common)
- **Fixed-Point:** Q15 arithmetic for audio DSP

---

## 🎓 Key Learnings from Phase 1

After completing Phase 1, you'll understand:
1. ✅ Embedded audio systems (I2S, DMA, buffering)
2. ✅ Real-time DSP implementation in Rust
3. ✅ Fixed-point arithmetic for audio processing
4. ✅ Embassy async patterns for embedded
5. ✅ Hardware-firmware integration testing

---

## 🎯 Phase 1 Success = Phase 2 Ready

Phase 1 completion unlocks:
- **Phase 2:** BLE control + profile storage
- **Phase 3:** Hearing test + auto calibration
- **Phase 4:** Hardware miniaturization
- **Phase 5:** Clinical validation

---

**Next:** Start implementation with HAL & drivers!
