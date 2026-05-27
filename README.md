# 🦻 NFL — Nafal Faturizki Listener

> **Open Source Hearing Aid | Zero Vendor Dependency | Fully Offline**
> 
> *Mendengar itu gratis dan hak semua orang*

---

## 🌟 Quick Overview

NFL adalah proyek **hearing aid open-source** yang dirancang untuk:
- **Zero vendor lock-in** — Tidak ada chip proprietary, SDK berbayar, atau cloud wajib
- **Dapat dirakit siapa saja** — Teknisi, pelajar, komunitas, NGO
- **Memory safe** — Ditulis dalam Rust bare-metal (embedded)
- **Fully offline** — Semua proses di device, privasi terjaga
- **Industrial grade** — Komponen automotive/industrial, bukan consumer grade

**Hardware:** nRF5340 + SPH0645 microphone + TPA6132A2 amplifier + W25Q64JV flash
**Software:** Rust firmware + Flutter mobile app + Audiometry engine

---

## ✨ Current Status

**Phase 0 — Project Initialization:** ✅ **COMPLETE**

- ✅ Workspace Cargo structure created (2 members)
- ✅ Bare-metal firmware scaffolding ready
- ✅ Mobile FFI engine skeleton ready
- ✅ Build system configured
- 🎯 Next: Phase 1 (Audio DSP Pipeline)

---

## 🚀 Quick Start

### 1. Setup Rust Environment (First time only)
```bash
cd /workspaces/NFL-Nafal-Faturizki-Listener
chmod +x setup-environment.sh
bash setup-environment.sh
```

This installs:
- Rust toolchain with bare-metal target `thumbv8m.main-none-eabihf`
- Embassy framework
- probe-rs (firmware flasher)
- Useful embedded tools

### 2. Verify Workspace
```bash
cargo tree --depth 0
```

Expected:
```
nfl-hearing v0.1.0
├── nfl-firmware v0.1.0 (firmware/)
└── nfl-mobile-engine v0.1.0 (mobile/rust/)
```

### 3. Test Firmware Build (without hardware)
```bash
cargo check -p nfl-firmware --target thumbv8m.main-none-eabihf
```

### 4. Test Mobile Engine Build
```bash
cargo check -p nfl-mobile-engine
```

---

## 📋 Project Structure

```
nfl-hearing/
├── Cargo.toml ........................ Workspace definition
├── rust-toolchain.toml ............. Rust version lock
│
├── firmware/ ........................ nRF5340 bare-metal Rust
│   ├── src/
│   │   ├── main.rs ................. Entry point
│   │   ├── lib.rs .................. Module tree
│   │   ├── audio/ .................. I2S + DSP pipeline
│   │   ├── ble/ .................... GATT server + BLE
│   │   ├── storage/ ................ Flash management
│   │   ├── power/ .................. Power management
│   │   └── hal/ .................... Hardware abstraction
│   ├── Cargo.toml
│   ├── .cargo/config.toml
│   └── memory.x .................... Linker script
│
├── mobile/ .......................... Flutter + Rust FFI
│   ├── rust/ ........................ Rust engine (cdylib)
│   │   ├── src/
│   │   │   ├── lib.rs .............. FFI bindings
│   │   │   ├── audiogram.rs ........ Hearing test
│   │   │   ├── profile_gen.rs ...... EQ generation
│   │   │   └── ble_bridge.rs ....... BLE protocol
│   │   └── Cargo.toml
│   └── lib/ ......................... Flutter UI (Phase 2+)
│
├── hardware/ ........................ KiCad + FreeCAD
│   ├── pcb/ ......................... Schematic & layout
│   ├── casing/ ...................... 3D design (ITE/BTE)
│   └── docs/ ........................ Assembly guide
│
├── docs/ ............................ Documentation
├── tools/ ........................... Utility scripts
│
├── blueprint_nfl.md ................. ⭐ Source of Truth
├── PHASE-0.md ....................... Phase 0 documentation
├── SETUP-COMMANDS.md ............... Copy-paste commands
└── README.md ........................ This file
```

---

## 📊 Workspace Members

| Member | Path | Language | Purpose |
|--------|------|----------|---------|
| **nfl-firmware** | `firmware/` | Rust (bare-metal, no-std) | nRF5340 DSP engine |
| **nfl-mobile-engine** | `mobile/rust/` | Rust (std) | FFI for Flutter (audiogram, EQ) |

---

## 🛠️ Common Commands

### Build & Check
```bash
cargo build --all --release          # Build everything (release)
cargo check -p nfl-firmware           # Check firmware only
cargo check -p nfl-mobile-engine      # Check mobile engine only
```

### Code Quality
```bash
cargo fmt --all                       # Format code
cargo clippy --all                    # Lint code
cargo fmt --all -- --check            # Check formatting
```

### Documentation
```bash
cargo doc --no-deps --open            # Generate & open docs
```

### Firmware Specific
```bash
cargo build -p nfl-firmware --release                    # Build firmware
cargo run -p nfl-firmware --release                      # Build & flash
cargo tree -p nfl-firmware                               # Dependencies
```

### Mobile Engine
```bash
cargo build -p nfl-mobile-engine --release --target aarch64-linux-android
```

---

## 📚 Documentation

- **[blueprint_nfl.md](blueprint_nfl.md)** — ⭐ Source of Truth
  - Complete architecture design
  - Hardware BOM with specifications
  - DSP pipeline details
  - GATT profile definition
  - Development roadmap (Phase 0-6)

- **[PHASE-0.md](PHASE-0.md)** — Phase 0 Documentation
  - Initialization checklist
  - Folder structure breakdown
  - Setup instructions
  - Hardware preparation

- **[SETUP-COMMANDS.md](SETUP-COMMANDS.md)** — Ready-to-use Commands
  - Copy-paste setup scripts
  - Build commands
  - Debug tips
  - Troubleshooting

---

## 🧪 Testing

### Firmware Tests
```bash
cargo test -p nfl-firmware --lib
```

### Mobile Engine Tests
```bash
cargo test -p nfl-mobile-engine
```

### Format Check
```bash
cargo fmt --all -- --check
```

---

## 🎯 Development Roadmap

### Phase 0 ✅ — Fondasi
- [x] Workspace setup
- [x] Cargo configuration
- [x] Build system ready
- [x] Documentation scaffolding

**Duration:** 1-3 weeks | **Status:** ✅ COMPLETE

### Phase 1 🎯 — Audio Proof of Concept
- [ ] I2S capture driver (SPH0645)
- [ ] I2S playback driver (TPA6132A2)
- [ ] Audio passthrough (mic → speaker)
- [ ] Noise gate
- [ ] 8-band equalizer (fixed-point)
- [ ] Latency measurement

**Duration:** 4-8 weeks | **Estimated latency:** < 15ms

### Phase 2 — BLE & Kalibrasi
- [ ] GATT server implementation
- [ ] BLE characteristic profiles
- [ ] EQ profile send/receive
- [ ] Flash profile storage
- [ ] Mobile app basic UI

### Phase 3 — Hearing Test & Auto Calibration
- [ ] Pure tone audiometry (PTA)
- [ ] Audiogram processing
- [ ] NAL-NL2 EQ generation
- [ ] Environment presets
- [ ] Full mobile app

### Phase 4 — Hardware Miniaturization
- [ ] PCB design (KiCad)
- [ ] 4-layer layout
- [ ] Manufacturing files (gerbers)
- [ ] 3D casing design
- [ ] Assembly procedures

### Phase 5 — Clinical Validation
- [ ] User testing (20+ volunteers)
- [ ] Audiologist feedback
- [ ] Algorithm refinement
- [ ] Durability testing
- [ ] Documentation

### Phase 6 — Open Source Launch
- [ ] Publish to GitHub
- [ ] Assembly videos
- [ ] GitBook documentation
- [ ] Community setup (Discord/Telegram)
- [ ] Manufacturing partnerships

---

## 🔧 Technology Stack

### Firmware
- **Language:** Rust 2021 edition
- **Runtime:** Embassy (async bare-metal)
- **HAL:** embassy-nrf (Nordic support)
- **Target:** ARM Cortex-M33 (`thumbv8m.main-none-eabihf`)
- **DSP:** Fixed-point arithmetic (no FPU in hot path)
- **Math:** `fixed` crate for DSP calculations
- **Logging:** `defmt` (embedded logging)

### Mobile
- **Frontend:** Flutter (Dart)
- **Backend:** Rust FFI (cdylib)
- **BLE:** flutter_blue_plus
- **Storage:** Hive (local, encrypted)
- **State:** Riverpod

### Hardware
- **MCU:** Nordic nRF5340 (dual-core ARM)
- **Microphone:** Knowles SPH0645LM4H-B (I2S digital)
- **Amplifier:** TI TPA6132A2 (Class-G, 25mW)
- **Speaker:** Knowles ED-29689 (Balanced Armature)
- **Battery:** Cellevia LP401730 (180mAh LiPo)
- **Charger:** TI BQ25185 (USB-C PD)
- **Flash:** Winbond W25Q64JV (8MB SPI NOR)
- **PCB:** 4-layer FR-4, ENIG, 22×18mm

---

## 🚨 Important Notes

1. **Memory Layout**
   - nRF5340 Flash: 1MB (0x00000000 - 0x000FFFFF)
   - nRF5340 RAM: 512KB (0x20000000 - 0x2007FFFF)
   - See `firmware/memory.x` for linker script

2. **Bare-Metal Target**
   - Target: `thumbv8m.main-none-eabihf` (ARM Cortex-M33)
   - No OS, no_std
   - Configured in `.cargo/config.toml`

3. **Compilation Profile**
   - Debug: `opt-level = "z"` (size optimization)
   - Release: LTO enabled, aggressive size optimization

4. **No Proprietary Dependencies**
   - All dependencies from crates.io
   - Reproducible builds
   - SHA-256 verification possible

---

## ⚡ Hardware Requirements (for Phase 1+)

To test and develop firmware:
- nRF5340 DK (Nordic Semiconductor dev board)
- J-Link debugger (included with DK)
- USB 3.0 cable
- Computer with Linux/macOS/Windows
- probe-rs (installed via setup-environment.sh)

To test hardware PCB:
- KiCad 8.0+ (for PCB design)
- Soldering equipment (reflow oven or hot air gun)
- FreeCAD 1.0+ (for 3D casing)
- 3D printer (for casing prototypes)

---

## 🤝 Contributing

This project is **100% open source** under:
- **Hardware:** CERN Open Hardware Licence v2 (CERN-OHL-S v2)
- **Software:** GNU General Public Licence v3 (GPL-3.0)

### Development Setup
1. Clone repository
2. Run `bash setup-environment.sh`
3. Create feature branch
4. Make changes
5. Run `cargo fmt && cargo clippy`
6. Submit pull request

### Contribution Guidelines
- All code must be memory-safe Rust or Flutter
- No proprietary blobs or closed-source components
- Fully documented and tested
- Follow existing code style (use `cargo fmt`)
- GPL-3.0 license header on new files

---

## 📞 Support & Community

- **GitHub Issues:** Report bugs and feature requests
- **Discussions:** Ask questions and share ideas
- **Documentation:** Read [blueprint_nfl.md](blueprint_nfl.md) first

---

## 📄 License

- **Firmware & Software:** GPL-3.0
- **Hardware & Designs:** CERN-OHL-S v2
- **Documentation:** CC-BY-4.0

---

## 🎉 Acknowledgments

**Inspired by:**
- Open source hearing aid projects
- Free software philosophy
- Accessibility for all
- Nordic Semiconductor community
- Rust embedded ecosystem

---

## 📅 Last Updated

**Phase 0 Completion:** May 27, 2026
**Next Phase:** Phase 1 (Audio DSP Pipeline)
**Estimated start:** June 2026

---

**Made with ❤️ by Nafal Faturizki**

*"Technology that saves lives shouldn't be a business exclusive."*
