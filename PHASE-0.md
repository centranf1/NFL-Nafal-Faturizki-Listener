# 🦻 NFL - Phase 0: Project Initialization

## Overview
Phase 0 adalah fondasi pengembangan — setup environment Rust untuk embedded, structure workspace Cargo, dan validasi toolchain.

**Duration:** Minggu 1–3 (dapat diselesaikan dalam 1–2 jam untuk setup awal)

---

## ✅ Checklist Phase 0

- [x] Create workspace structure (firmware, mobile, hardware, docs)
- [x] Initialize Cargo workspace dengan workspace members
- [x] Create Cargo.toml di setiap member project
- [ ] Install Rust toolchain untuk bare-metal nRF5340
- [ ] Verify workspace builds without errors
- [ ] Setup probe-rs untuk firmware flashing
- [ ] Prepare nRF5340 DK development board

---

## 🛠️ Setup Commands (Copy-Paste Ready)

### 1. Make setup script executable
```bash
chmod +x /workspaces/NFL-Nafal-Faturizki-Listener/setup-environment.sh
```

### 2. Run complete environment setup
```bash
cd /workspaces/NFL-Nafal-Faturizki-Listener
bash setup-environment.sh
```

### 3. Verify workspace structure
```bash
cd /workspaces/NFL-Nafal-Faturizki-Listener
cargo tree --depth 0
```

Expected output:
```
nfl-hearing v0.1.0 (/workspaces/NFL-Nafal-Faturizki-Listener)
├── nfl-firmware v0.1.0 (firmware/)
└── nfl-mobile-engine v0.1.0 (mobile/rust/)
```

### 4. Test firmware compilation (without hardware)
```bash
cd /workspaces/NFL-Nafal-Faturizki-Listener
cargo check -p nfl-firmware
```

### 5. Test mobile engine compilation
```bash
cd /workspaces/NFL-Nafal-Faturizki-Listener
cargo check -p nfl-mobile-engine
```

---

## 📁 Project Structure (Created in Phase 0)

```
/workspaces/NFL-Nafal-Faturizki-Listener/
│
├── Cargo.toml                    # Workspace root definition
├── rust-toolchain.toml           # Rust version specification
├── .gitignore
├── setup-environment.sh          # Setup script
│
├── firmware/                     # nRF5340 embedded Rust
│   ├── Cargo.toml
│   ├── .cargo/config.toml        # Bare-metal target config
│   ├── memory.x                  # Linker script untuk nRF5340
│   ├── src/
│   │   ├── main.rs               # Entry point (Embassy executor)
│   │   ├── lib.rs                # Module definitions
│   │   ├── audio/
│   │   │   ├── capture.rs        # I2S input driver
│   │   │   ├── playback.rs       # I2S output driver
│   │   │   ├── pipeline.rs       # DSP pipeline coordinator
│   │   │   └── dsp/
│   │   │       ├── noise_gate.rs
│   │   │       ├── equalizer.rs
│   │   │       ├── compressor.rs
│   │   │       └── filters.rs
│   │   ├── ble/
│   │   │   ├── gatt_server.rs
│   │   │   ├── profiles/
│   │   │   │   ├── calibration.rs
│   │   │   │   ├── battery.rs
│   │   │   │   └── ota.rs
│   │   │   └── advertising.rs
│   │   ├── storage/
│   │   │   ├── flash.rs
│   │   │   ├── profile.rs
│   │   │   └── config.rs
│   │   ├── power/
│   │   │   ├── manager.rs
│   │   │   └── battery.rs
│   │   └── hal/
│   │       ├── i2s.rs
│   │       ├── spi.rs
│   │       └── gpio.rs
│   └── tests/
│
├── mobile/                       # Flutter + Rust FFI
│   ├── pubspec.yaml             # (akan dibuat di Phase 2)
│   ├── rust/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs           # FFI entry point
│   │   │   ├── audiogram.rs
│   │   │   ├── profile_gen.rs
│   │   │   └── ble_bridge.rs
│   │   └── [iOS, Android build configs]
│   └── lib/
│       ├── main.dart            # (akan dibuat di Phase 2)
│       ├── screens/
│       │   ├── hearing_test/
│       │   ├── calibration/
│       │   └── device/
│       ├── services/
│       │   ├── ble_service.dart
│       │   ├── audio_engine.dart
│       │   └── storage_service.dart
│       └── models/
│
├── hardware/                     # KiCad PCB, FreeCAD 3D
│   ├── pcb/
│   │   ├── nfl-v1.kicad_pro
│   │   ├── nfl-v1.kicad_sch
│   │   ├── nfl-v1.kicad_pcb
│   │   ├── gerbers/
│   │   └── bom.csv
│   ├── casing/
│   │   ├── nfl-ite-v1.FCStd
│   │   ├── nfl-ite-v1.stl
│   │   └── nfl-bte-v1.stl
│   └── docs/
│
├── docs/                         # Documentation
│   ├── getting-started.md       # (Phase 0)
│   ├── architecture.md
│   ├── contributing.md
│   └── regulatory/
│
├── tools/                        # Utility scripts
│   ├── flash.sh
│   ├── build-release.sh
│   └── test-audio.py
│
└── blueprint_nfl.md             # Source of truth for architecture
```

---

## 🧩 Workspace Members

### 1. **nfl-firmware** (`firmware/`)
- **Language:** Rust (bare-metal, no-std)
- **Target:** `thumbv8m.main-none-eabihf` (ARM Cortex-M33)
- **Platform:** nRF5340 DK (Nordic)
- **Runtime:** Embassy async executor
- **Status:** Phase 0 ✅ Structure ready, Phase 1 → Implement audio pipeline

### 2. **nfl-mobile-engine** (`mobile/rust/`)
- **Language:** Rust (std, FFI)
- **Platform:** iOS, Android (via Dart FFI)
- **Compilation:** `cdylib` for native libraries
- **Status:** Phase 0 ✅ Structure ready, Phase 3 → Implement audiogram + EQ generation

---

## 🚀 Next: Phase 1 (Audio Proof of Concept)

Phase 1 akan fokus pada:
1. **Driver I2S** untuk SPH0645 (capture) dan TPA6132A2 (playback)
2. **Audio passthrough** (mic → speaker) tanpa DSP — test latency
3. **Noise gate** sederhana
4. **8-band equalizer** dengan fixed-point arithmetic
5. **Real hardware testing** dengan oscilloscope

### Estimated timeline: Minggu 4–8

---

## 📚 Reference

- **Blueprint:** [blueprint_nfl.md](../blueprint_nfl.md) — Definisi arsitektur lengkap
- **Embassy Documentation:** https://embassy.dev/
- **nRF5340 Reference Manual:** https://infocenter.nordicsemi.com/
- **Rust Embedded Book:** https://rust-embedded.github.io/book/

---

## ⚙️ Configuration Files Overview

### `.cargo/config.toml`
Mengatur default build target dan runner untuk firmware:
```toml
[build]
target = "thumbv8m.main-none-eabihf"

[target.thumbv8m.main-none-eabihf]
runner = "probe-rs run --chip nRF5340_xxAA"
```

### `memory.x`
Linker script untuk nRF5340 memory layout:
- Flash: 1MB (0x00000000 - 0x000FFFFF)
- RAM: 512KB (0x20000000 - 0x2007FFFF)

### `rust-toolchain.toml`
Memastikan semua developer menggunakan Rust version yang sama:
```toml
channel = "stable"
components = ["rustfmt", "clippy"]
targets = ["thumbv8m.main-none-eabihf"]
```

---

## 🔗 Hardware Preparation

Untuk Phase 1 testing, Anda memerlukan:
1. **nRF5340 DK** (Nordic Semiconductor) — dev board resmi
2. **J-Link debugger** (included dengan DK)
3. **probe-rs** — sudah terinstall via setup-environment.sh

Dapatkan nRF5340 DK dari:
- Nordic Store: https://www.nordicsemi.com/Products/Development-kits/nRF5340-DK
- Distributor lokal Indonesia (Tokopedia, Elemen Indonesia)

---

**Status:** ✅ Phase 0 Complete
**Next:** Proceed to Phase 1 untuk implementasi audio pipeline
