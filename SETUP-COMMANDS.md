# 🚀 NFL Project - Phase 0 Setup Commands (Siap Copy-Paste)

## Status
✅ **Phase 0 Structure Complete** — Semua folder dan Cargo.toml sudah dibuat sesuai blueprint.

---

## 📋 Ringkasan Apa yang Sudah Dibuat

### 1. Struktur Folder Lengkap ✅
Sudah dibuat sesuai blueprint dengan:
- `firmware/` — nRF5340 bare-metal Rust
- `mobile/rust/` — Rust FFI engine untuk Flutter
- `hardware/` — KiCad PCB & FreeCAD 3D casing
- `docs/` — Dokumentasi
- `tools/` — Utility scripts

### 2. Cargo Workspace ✅
- **Workspace root:** `Cargo.toml` dengan 2 members
- **firmware/Cargo.toml** — Dependencies untuk embedded
- **mobile/rust/Cargo.toml** — Dependencies untuk FFI

### 3. Configuration Files ✅
- `rust-toolchain.toml` — Versi Rust yang consistent
- `firmware/.cargo/config.toml` — Target bare-metal configuration
- `firmware/memory.x` — Linker script untuk nRF5340
- `setup-environment.sh` — Setup script lengkap

### 4. Documentation ✅
- `PHASE-0.md` — Dokumentasi Phase 0 lengkap
- `blueprint_nfl.md` — Source of truth arsitektur

---

## 🎯 COMMAND TERMINAL UNTUK SETUP LENGKAP

### Step 1: Buat script executable
```bash
chmod +x /workspaces/NFL-Nafal-Faturizki-Listener/setup-environment.sh
```

### Step 2: Jalankan setup environment (PILIH SATU)

#### Option A: Automatic setup (RECOMMENDED)
```bash
cd /workspaces/NFL-Nafal-Faturizki-Listener && bash setup-environment.sh
```

#### Option B: Manual setup step-by-step
```bash
# Update rustup
rustup update stable

# Add bare-metal target
rustup target add thumbv8m.main-none-eabihf

# Install probe-rs (untuk flashing firmware)
cargo install probe-rs-tools --locked

# Install useful tools
cargo install cargo-tree cargo-expand cargo-generate

# Verify workspace
cd /workspaces/NFL-Nafal-Faturizki-Listener
cargo tree --depth 0
```

### Step 3: Validasi workspace (setelah setup)
```bash
cd /workspaces/NFL-Nafal-Faturizki-Listener
cargo tree --depth 0
```

**Expected output:**
```
nfl-hearing v0.1.0 (/workspaces/NFL-Nafal-Faturizki-Listener)
├── nfl-firmware v0.1.0 (firmware/)
└── nfl-mobile-engine v0.1.0 (mobile/rust/)
```

### Step 4: Test compile firmware (tanpa hardware)
```bash
cd /workspaces/NFL-Nafal-Faturizki-Listener
cargo check -p nfl-firmware --target thumbv8m.main-none-eabihf
```

### Step 5: Test compile mobile engine
```bash
cd /workspaces/NFL-Nafal-Faturizki-Listener
cargo check -p nfl-mobile-engine
```

---

## 🔍 Struktur File & Folder

```
/workspaces/NFL-Nafal-Faturizki-Listener/
├── Cargo.toml ......................... Workspace root definition
├── rust-toolchain.toml ............... Rust version lock
├── .gitignore ........................ Git ignore rules
├── setup-environment.sh .............. ⭐ RUN THIS FIRST
├── PHASE-0.md ........................ Phase 0 documentation
│
├── firmware/ ......................... nRF5340 Rust firmware
│   ├── Cargo.toml
│   ├── .cargo/config.toml ........... Target: thumbv8m.main-none-eabihf
│   ├── memory.x ..................... Linker script
│   └── src/
│       ├── main.rs ................. Entry point (Embassy executor)
│       ├── lib.rs .................. Module tree
│       ├── audio/ .................. I2S capture/playback + DSP pipeline
│       ├── ble/ .................... GATT server + BLE profiles
│       ├── storage/ ................ Flash management
│       ├── power/ .................. Power management
│       └── hal/ .................... Hardware abstraction layer
│
├── mobile/ ........................... Flutter + Rust FFI
│   ├── rust/ ........................ 🦀 Rust FFI engine
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs .............. FFI bindings untuk Dart
│   │       ├── audiogram.rs ........ Hearing test data structures
│   │       ├── profile_gen.rs ...... NAL-NL2 algorithm
│   │       └── ble_bridge.rs ....... BLE protocol layer
│   └── lib/ ......................... 📱 Flutter UI (Phase 2)
│
├── hardware/ ......................... PCB & 3D design
│   ├── pcb/ ......................... KiCad project
│   ├── casing/ ...................... FreeCAD + STL
│   └── docs/ ........................ Assembly guide
│
├── docs/ ............................ Dokumentasi project
│   └── regulatory/ .................. Compliance docs
│
├── tools/ ........................... Utility scripts
│   ├── flash.sh
│   ├── build-release.sh
│   └── test-audio.py
│
└── blueprint_nfl.md ................. ⭐ Source of truth untuk arsitektur
```

---

## 📊 Workspace Members

| Member | Path | Language | Purpose |
|--------|------|----------|---------|
| **nfl-firmware** | `firmware/` | Rust (no-std) | nRF5340 bare-metal DSP engine |
| **nfl-mobile-engine** | `mobile/rust/` | Rust (std) | FFI untuk Flutter (audiogram, EQ gen) |

---

## 🛠️ Build Commands Setelah Setup

### Compile firmware
```bash
cd /workspaces/NFL-Nafal-Faturizki-Listener
cargo build -p nfl-firmware --release
```

### Compile mobile engine (library)
```bash
cargo build -p nfl-mobile-engine --release
```

### Compile semua members
```bash
cargo build --all --release
```

### Format kode
```bash
cargo fmt --all
```

### Lint kode
```bash
cargo clippy --all
```

---

## ⚡ Quick Debug Tips

### Check workspace structure
```bash
cargo tree --depth 0
```

### Check dependency tree
```bash
cargo tree -p nfl-firmware
```

### Generate documentation
```bash
cargo doc --no-deps --open
```

### Check for unused dependencies
```bash
cargo udeps --all
```

---

## 📌 IMPORTANT NOTES

1. **Rust Toolchain:**
   - Recommended: Stable channel
   - Will install bare-metal target `thumbv8m.main-none-eabihf` automatically

2. **Hardware Needed untuk Phase 1:**
   - nRF5340 DK (Nordic Semiconductor)
   - J-Link debugger (included dengan DK)
   - USB cable

3. **Workspace Layout:**
   - Root Cargo.toml menggunakan workspace feature
   - Setiap member independen tetapi dapat share dependencies via workspace
   - `/target` folder akan dibuat saat `cargo build`

4. **Next Steps (Phase 1):**
   - Setup hardware (nRF5340 DK + J-Link)
   - Implement I2S drivers
   - Implement audio passthrough
   - Test latency

---

## 🆘 Troubleshooting

### Error: "cargo: command not found"
→ Run setup script dulu: `bash setup-environment.sh`

### Error: "target 'thumbv8m.main-none-eabihf' not installed"
→ Run: `rustup target add thumbv8m.main-none-eabihf`

### Error: "probe-rs not found"
→ Run: `cargo install probe-rs-tools --locked`

### Workspace validation error
→ Check: `cargo metadata --format-version 1` (akan error jika Cargo.toml invalid)

---

## 📚 Reference Documentation

- **Blueprint (Source of Truth):** [blueprint_nfl.md](blueprint_nfl.md)
- **Phase 0 Details:** [PHASE-0.md](PHASE-0.md)
- **Embassy Framework:** https://embassy.dev/
- **nRF5340 Docs:** https://infocenter.nordicsemi.com/
- **Rust Embedded:** https://rust-embedded.github.io/

---

**Last Updated:** Phase 0 Complete ✅
**Status:** Ready for Phase 1 (Audio Implementation)
