# 🦻 NFL — Nafal Faturizki Listener
### *"Mendengar itu gratis dan hak semua orang"*

> **by Nafal Faturizki**
> Open Source Hearing Aid — Zero Vendor Dependency
> Lisensi: CERN-OHL-S v2 (Hardware) + GPL-3.0 (Software)

---

## 🌟 Visi & Filosofi

Pendengaran bukan kemewahan. Di dunia yang penuh suara, jutaan manusia terpaksa bisu dari kehidupan sekitar mereka — bukan karena takdir, tapi karena harga. Proyek NFL lahir dari satu keyakinan sederhana:

> **Teknologi yang menyelamatkan tidak seharusnya menjadi bisnis eksklusif.**

NFL dirancang dari awal untuk:
- **Zero vendor lock-in** — tidak ada chip proprietary, tidak ada SDK berbayar, tidak ada cloud wajib
- **Dapat dirakit siapa saja** — teknisi, pelajar, komunitas desa, NGO
- **Bertahan lama** — komponen industrial grade, bukan consumer grade
- **Fully offline** — semua proses di device, privasi pengguna terjaga
- **Bahasa Rust** — memory safe, bare-metal capable, tanpa garbage collector

---

## 📐 Arsitektur Sistem Keseluruhan

```
┌─────────────────────────────────────────────────────────────┐
│                     PERANGKAT NFL UNIT                      │
│                                                             │
│  [MEMS Mic] ──► [ADC] ──► [nRF5340 Core] ──► [DAC] ──► [Speaker BA]
│                               │                             │
│                        [DSP Pipeline]                       │
│                        (Rust firmware)                      │
│                               │                             │
│                          [BLE 5.2]                          │
│                               │                             │
└───────────────────────────────┼─────────────────────────────┘
                                │
                    ┌───────────▼───────────┐
                    │   APLIKASI NFL Mobile  │
                    │   (Flutter + Rust FFI) │
                    │                        │
                    │  • Hearing Test        │
                    │  • Kalibrasi Personal  │
                    │  • Firmware OTA        │
                    │  • Profil Lingkungan   │
                    └────────────────────────┘
```

---

## 🔩 BLUEPRINT HARDWARE — Versi 1.0

### Filosofi Pemilihan Komponen
- **Industrial / automotive grade** diutamakan → tahan suhu, getaran, debu
- **Komponen dengan datasheet publik lengkap** → tidak bergantung vendor
- **Tersedia di pasar lokal dan global** (Tokopedia, Digikey, LCSC)
- **Bukan chip dengan binary blob** — semua dapat dikompilasi dari source

---

### 📋 Bill of Materials (BOM) Lengkap

| # | Komponen | Part Number | Spesifikasi | Harga Est. | Alasan Pilih |
|---|---|---|---|---|---|
| 1 | **Mikrofon MEMS** | Knowles SPH0645LM4H-B | I2S digital, SNR 65dB, -26dBFS | Rp 22.000 | Digital output (tidak noise seperti analog), I2S langsung ke MCU |
| 2 | **Microcontroller** | Nordic nRF5340 (SiP) | Dual-core ARM, BLE 5.2, 1MB Flash, 512KB RAM | Rp 85.000 | Open SDK (nRF Connect SDK), Rust support penuh via Embassy |
| 3 | **Amplifier Headphone** | TI TPA6132A2 | 25mW/channel, Class-G, THD < 0.01% | Rp 28.000 | Efisiensi tinggi, distorsi sangat rendah untuk kualitas audio |
| 4 | **Speaker Balanced Armature** | Knowles ED-29689 | Respon 200Hz–10kHz, impedansi 32Ω | Rp 55.000 | Jauh lebih jernih dari speaker dinamis biasa, ukuran kecil |
| 5 | **Baterai LiPo** | Cellevia LP401730 | 180mAh, siklus >500x, -20°C~60°C | Rp 45.000 | Kapasitas optimal, sertifikasi UN38.3 (aman pengiriman) |
| 6 | **IC Pengisi Daya** | TI BQ25185 | USB-C PD, 1A charge, proteksi lengkap | Rp 32.000 | Mendukung USB-C, tidak butuh konfigurasi chip eksternal |
| 7 | **Regulator Tegangan** | TI TPS62840 | LDO 1.8V/3.3V, 750mA, quiescent 60nA | Rp 18.000 | Konsumsi idle sangat rendah → hemat baterai ekstrem |
| 8 | **Konektor USB-C** | Amphenol 12401610E4#2A | 2.0, through-hole reinforced | Rp 8.000 | Mekanis kuat, mudah disolder ulang jika rusak |
| 9 | **Flash Eksternal** | Winbond W25Q64JV | 64Mb SPI NOR, -40°C~85°C | Rp 15.000 | Simpan profil audio, firmware backup, log kalibrasi |
| 10 | **PCB** | 4-layer, FR-4, ENIG | 22mm × 18mm, 1oz copper | Rp 35.000 | 4-layer untuk EMI shielding audio yang baik |
| 11 | **Casing** | Resin PETG print | Desain ITE (In-The-Ear) | Rp 40.000 | PETG tahan panas & keringat lebih baik dari PLA |
| 12 | **Kabel + Konektor** | Molex Pico-Lock 0.8mm | 4-pin untuk speaker | Rp 12.000 | Konektor kecil tapi bisa dicabut untuk servis |
| 13 | **Komponen Pasif** | Vishay / Yageo / Murata | Cap, Res, Induktor SMD | Rp 25.000 | Merk ternama, tersedia luas, tidak mudah palsu |
| | **TOTAL** | | | **~Rp 420.000** | |

> 💡 **Catatan:** Harga Rp 420.000 untuk unit rakitan mandiri. Jika diproduksi batch 1000 unit, estimasi turun ke **Rp 180.000–220.000/unit**.

---

### 🗺️ Skema Blok Hardware

```
                    ┌──────────────────────────────┐
    USB-C ──────►  │ BQ25185 Charger IC            │
                    │ • USB-C PD                    │
                    │ • Overcharge protection       │
                    └──────────┬───────────────────┘
                               │ VBAT
                    ┌──────────▼───────────────────┐
                    │ LP401730 LiPo 180mAh          │
                    └──────────┬───────────────────┘
                               │
                    ┌──────────▼───────────────────┐
                    │ TPS62840 Regulator            │
                    │ → 3.3V (MCU, Flash)           │
                    │ → 1.8V (Mic, DAC)             │
                    └──────┬──────────┬────────────┘
                           │          │
               ┌───────────▼──┐  ┌───▼──────────────┐
               │ nRF5340      │  │ SPH0645 MEMS Mic  │
               │ (Dual-Core)  │  │ I2S Output        │
               │              │◄─┘                   │
               │ • App Core   │  ┌──────────────────┐│
               │ • Net Core   │  │ W25Q64JV Flash   ││
               │ (BLE 5.2)    │◄─┤ 8MB SPI NOR      ││
               │              │  └──────────────────┘│
               │              │                      │
               └──────┬───────┘                      │
                      │ I2S Output                   │
               ┌──────▼───────┐                      │
               │ TPA6132A2    │                      │
               │ Amp Class-G  │                      │
               └──────┬───────┘                      │
                      │                              │
               ┌──────▼───────┐                      │
               │ Knowles      │                      │
               │ ED-29689 BA  │                      │
               │ Speaker      │                      │
               └──────────────┘                      │
```

---

### ⚡ Estimasi Konsumsi Daya

| Mode | Konsumsi | Daya Tahan (180mAh) |
|---|---|---|
| Normal (BLE aktif) | ~8.5 mA | ~21 jam |
| Normal (BLE standby) | ~5.2 mA | ~34 jam |
| Deep sleep (deteksi suara saja) | ~0.8 mA | ~9 hari |
| Pengisian penuh | ~1A (via USB-C) | ~15 menit |

---

## 💻 BLUEPRINT SOFTWARE — Stack Rust Murni

### Prinsip Software NFL
- **No binary blobs** — semua dikompilasi dari source
- **No OS dependency di firmware** — bare-metal dengan Embassy async runtime
- **No cloud** — semua kalkulasi di device atau HP lokal
- **Reproducible build** — siapapun bisa rebuild biner yang identik
- **Audit trail** — setiap release ada hash SHA-256 terverifikasi

---

### 🏗️ Struktur Repositori

```
nfl-hearing/
├── firmware/                    # Kode untuk nRF5340
│   ├── Cargo.toml
│   ├── .cargo/config.toml       # Target: thumbv8m.main-none-eabihf
│   ├── memory.x                 # Linker script nRF5340
│   ├── src/
│   │   ├── main.rs              # Entry point, Embassy executor
│   │   ├── audio/
│   │   │   ├── capture.rs       # Driver I2S untuk SPH0645
│   │   │   ├── pipeline.rs      # DSP pipeline utama
│   │   │   ├── playback.rs      # Output ke TPA6132A2
│   │   │   └── dsp/
│   │   │       ├── noise_gate.rs
│   │   │       ├── equalizer.rs
│   │   │       ├── compressor.rs
│   │   │       └── filters.rs
│   │   ├── ble/
│   │   │   ├── gatt_server.rs   # GATT profile custom NFL
│   │   │   ├── profiles/
│   │   │   │   ├── calibration.rs  # Service kalibrasi
│   │   │   │   ├── battery.rs      # Standard BLE battery
│   │   │   │   └── ota.rs          # Firmware update
│   │   │   └── advertising.rs
│   │   ├── storage/
│   │   │   ├── flash.rs         # Driver W25Q64JV SPI NOR
│   │   │   ├── profile.rs       # Simpan/load profil pengguna
│   │   │   └── config.rs
│   │   ├── power/
│   │   │   ├── manager.rs       # Manajemen daya & sleep mode
│   │   │   └── battery.rs       # Monitor level baterai
│   │   └── hal/
│   │       ├── i2s.rs           # Abstraksi I2S
│   │       ├── spi.rs           # Abstraksi SPI
│   │       └── gpio.rs
│   └── tests/                   # Unit test firmware
│
├── mobile/                      # Aplikasi Flutter + Rust FFI
│   ├── pubspec.yaml
│   ├── rust/                    # Logic engine dalam Rust
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── audiogram.rs     # Algoritma hearing test
│   │       ├── profile_gen.rs   # Generate profil EQ dari audiogram
│   │       └── ble_bridge.rs    # BLE protocol layer
│   └── lib/
│       ├── main.dart
│       ├── screens/
│       │   ├── home_screen.dart
│       │   ├── hearing_test/
│       │   │   ├── pure_tone_test.dart   # Test frekuensi standar
│       │   │   └── results_screen.dart
│       │   ├── calibration/
│       │   │   ├── eq_calibration.dart
│       │   │   └── environment_preset.dart
│       │   └── device/
│       │       ├── device_scan.dart
│       │       ├── battery_status.dart
│       │       └── firmware_update.dart
│       ├── services/
│       │   ├── ble_service.dart         # flutter_blue_plus
│       │   ├── audio_engine.dart        # FFI ke Rust
│       │   └── storage_service.dart     # Hive (local, no cloud)
│       └── models/
│           ├── audiogram.dart
│           ├── eq_profile.dart
│           └── device.dart
│
├── hardware/                    # Desain hardware open source
│   ├── pcb/
│   │   ├── nfl-v1.kicad_pro     # KiCad project
│   │   ├── nfl-v1.kicad_sch     # Skema rangkaian
│   │   ├── nfl-v1.kicad_pcb     # Layout PCB
│   │   ├── gerbers/             # File produksi
│   │   └── bom.csv              # Bill of Materials mesin
│   ├── casing/
│   │   ├── nfl-ite-v1.FCStd     # FreeCAD source
│   │   ├── nfl-ite-v1.stl       # Untuk print 3D
│   │   └── nfl-bte-v1.stl       # Varian Behind-the-Ear
│   └── docs/
│       ├── assembly-guide.md    # Panduan rakitan
│       └── test-procedure.md    # Prosedur QC
│
├── docs/                        # Dokumentasi lengkap
│   ├── getting-started.md
│   ├── architecture.md
│   ├── contributing.md
│   └── regulatory/
│       └── assistive-device-indonesia.md
│
├── tools/                       # Script utilitas
│   ├── flash.sh                 # Flash firmware via probe
│   ├── build-release.sh         # Build semua komponen
│   └── test-audio.py            # Test audio pipeline di host
│
└── README.md
```

---

### 🔧 Komponen Software Detail

---

#### 1. FIRMWARE CORE (`firmware/src/`)

**Bahasa:** Rust (edition 2021)
**Runtime:** Embassy (async bare-metal)
**Target:** `thumbv8m.main-none-eabihf`
**HAL:** `embassy-nrf` (official Embassy support untuk nRF5340)

**Dependencies Utama (Cargo.toml):**
```toml
[dependencies]
embassy-executor   = { version = "0.5", features = ["arch-cortex-m"] }
embassy-nrf        = { version = "0.1", features = ["nrf5340-app-s"] }
embassy-time       = { version = "0.3" }
embassy-sync       = { version = "0.5" }
nrf-softdevice    = { version = "0.1", features = ["ble-peripheral"] }
heapless          = "0.8"          # Fixed-size collections, no heap
fixed             = "1.23"         # Fixed-point arithmetic untuk DSP
defmt             = "0.3"          # Logging efisien untuk embedded
defmt-rtt         = "0.4"
panic-probe       = "0.3"
```

> **Tidak ada dependency proprietary.** Semua crate dari crates.io, open source.

---

#### 2. DSP PIPELINE (`firmware/src/audio/dsp/`)

Pipeline audio berjalan di **App Core** nRF5340 (120MHz), **Net Core** khusus untuk BLE.

**Alur Pemrosesan (per frame 256 sampel @ 16kHz = 16ms latency):**

```
Input I2S (SPH0645)
        │
        ▼
┌───────────────┐
│  NOISE GATE   │  Buang suara di bawah -50dBFS
│  noise_gate.rs│  Adaptive threshold berdasarkan ambient
└───────┬───────┘
        │
        ▼
┌───────────────┐
│  HIGH-PASS    │  Filter < 100Hz (buang angin, gemuruh)
│  FILTER       │  IIR Butterworth order 2
│  filters.rs   │
└───────┬───────┘
        │
        ▼
┌───────────────┐
│  BAND EQ      │  8 band parametric EQ
│  equalizer.rs │  Frekuensi: 250Hz, 500Hz, 1kHz, 2kHz,
│               │  3kHz, 4kHz, 6kHz, 8kHz
│               │  Gain: -20dB hingga +40dB per band
│               │  Sesuai profil audiogram pengguna
└───────┬───────┘
        │
        ▼
┌───────────────┐
│  DYNAMIC      │  Soft-knee compression 4:1
│  COMPRESSOR   │  Attack: 10ms, Release: 100ms
│  compressor.rs│  Cegah suara keras melukai telinga
└───────┬───────┘
        │
        ▼
┌───────────────┐
│  LIMITER      │  Hard limit -6dBFS
│  filters.rs   │  Proteksi absolut terakhir
└───────┬───────┘
        │
        ▼
Output I2S (TPA6132A2 → Speaker BA)
```

**Implementasi menggunakan fixed-point arithmetic** (`fixed` crate) — tidak ada floating point di hot path untuk latency stabil.

---

#### 3. BLE GATT PROFILE CUSTOM (`firmware/src/ble/`)

NFL menggunakan GATT profile sendiri (bukan bergantung standar vendor):

```
NFL GATT Profile
UUID Base: 4e464c00-xxxx-xxxx-xxxx-xxxxxxxxxxxx (NFL = 4e,46,4c)

Service: NFL Audio Config (4e464c01-...)
  ├── Characteristic: EQ Profile Write (4e464c02-...)
  │   Properties: Write, WriteWithoutResponse
  │   Format: [band0_gain_i8, band1_gain_i8, ... band7_gain_i8]
  │
  ├── Characteristic: Environment Preset (4e464c03-...)
  │   Properties: Read, Write
  │   Values: 0=Calm, 1=Indoor, 2=Outdoor, 3=Noisy, 4=Custom
  │
  ├── Characteristic: Noise Gate Threshold (4e464c04-...)
  │   Properties: Read, Write
  │   Format: i8 (-80 to 0 dBFS)
  │
  └── Characteristic: Compression Ratio (4e464c05-...)
      Properties: Read, Write
      Format: u8 (10 = 1.0x, 40 = 4.0x, etc.)

Service: NFL Device Info (4e464c10-...)
  ├── Characteristic: Firmware Version (4e464c11-...)
  ├── Characteristic: Battery Level (4e464c12-...)
  │   Notify setiap 1 menit
  └── Characteristic: Device Stats (4e464c13-...)
      Uptime, error count, temperature MCU

Service: NFL OTA Update (4e464c20-...)
  ├── Characteristic: OTA Control (4e464c21-...)
  │   Write: 0x01 = Start, 0x02 = Confirm, 0x03 = Cancel
  ├── Characteristic: OTA Data (4e464c22-...)
  │   Write: chunks firmware baru (512 bytes/chunk)
  └── Characteristic: OTA Status (4e464c23-...)
      Notify: progress %, error code
```

---

#### 4. PROFIL PENGGUNA & STORAGE (`firmware/src/storage/`)

Semua data pengguna disimpan **lokal di flash W25Q64JV**, tidak pernah ke cloud.

**Format profil (binary, 64 bytes per profil, muat 1000+ profil di flash):**
```rust
#[repr(C, packed)]
struct UserProfile {
    magic: u16,           // 0xNF4C — validasi
    version: u8,
    profile_id: u8,
    name: [u8; 16],       // Nama profil UTF-8
    eq_gains: [i8; 8],    // Gain per band dalam dB
    noise_gate_db: i8,    // Threshold noise gate
    compression: u8,      // Rasio kompresi x10
    environment: u8,      // Preset lingkungan
    reserved: [u8; 31],   // Untuk ekspansi fitur
    checksum: u16,        // CRC-16
}
```

---

#### 5. APLIKASI MOBILE (`mobile/`)

**Bahasa:** Flutter (Dart) + Rust FFI untuk logic berat
**Database lokal:** Hive (NoSQL, tanpa server, tanpa izin internet)
**BLE:** flutter_blue_plus (aktif maintained, cross-platform)
**State management:** Riverpod

**Tidak ada:**
- Firebase / analytics
- Crash reporting ke server eksternal
- Akun wajib
- Internet untuk fungsi utama

---

#### 5a. MODUL HEARING TEST (`mobile/lib/screens/hearing_test/`)

Hearing test berbasis **Pure Tone Audiometry (PTA)** standar ISO 8253-1:

```
Frekuensi yang diuji: 250Hz, 500Hz, 1kHz, 2kHz, 3kHz, 4kHz, 6kHz, 8kHz
Metode: Modified Hughson-Westlake (modified ascending method)

Prosedur per frekuensi:
1. Mulai dari 40dB HL
2. Jika terdengar → turunkan 10dB
3. Jika tidak terdengar → naikkan 5dB
4. Threshold = level terendah yang terdengar 2 dari 3 percobaan
5. Rekam threshold dalam dB HL

Output: audiogram lengkap kanan & kiri
        → dikirim ke Rust engine untuk konversi ke profil EQ
```

**Kalibrasi earphone:** Aplikasi menyertakan file kalibrasi untuk earphone umum (Apple EarPods, Samsung AKG, generic 3.5mm) sehingga hasil test akurat tanpa perlu audiometer mahal.

---

#### 5b. ENGINE KONVERSI AUDIOGRAM → EQ PROFILE (`mobile/rust/src/profile_gen.rs`)

Ini inti teknologi NFL — ditulis dalam Rust, dikompilasi ke native via FFI:

```rust
/// Konversi hasil audiogram ke parameter EQ yang optimal
/// berdasarkan metode NAL-NL2 (National Acoustic Laboratories)
/// yang telah dipublikasikan secara open-source
pub fn audiogram_to_eq_profile(audiogram: &Audiogram) -> EqProfile {
    // 8 band center frequencies (Hz)
    const BANDS: [f32; 8] = [250.0, 500.0, 1000.0, 2000.0, 3000.0, 4000.0, 6000.0, 8000.0];
    
    let mut gains = [0i8; 8];
    
    for (i, &freq) in BANDS.iter().enumerate() {
        let hl = audiogram.interpolate_threshold(freq);
        
        // Formula prescriptive gain (simplified NAL-NL2)
        // Target: restore audibility tanpa over-amplification
        let prescribed_gain = if hl < 20.0 {
            0.0  // Normal hearing, no amplification needed
        } else if hl < 40.0 {
            hl * 0.36  // Mild loss
        } else if hl < 60.0 {
            hl * 0.46 - 4.0  // Moderate loss
        } else {
            hl * 0.52 - 7.2  // Severe loss (cap at hardware limit)
        };
        
        gains[i] = (prescribed_gain.min(40.0).max(-20.0)) as i8;
    }
    
    EqProfile {
        gains,
        name: "Auto dari Audiogram".into(),
        ..Default::default()
    }
}
```

---

#### 5c. FIRMWARE UPDATE OTA (`firmware/src/ble/profiles/ota.rs`)

Update firmware tanpa kabel, via BLE dari aplikasi:

```
Proses OTA NFL:
1. App kirim perintah START_OTA via BLE
2. Firmware aktifkan slot update di flash W25Q64JV
3. App kirim firmware.bin dalam chunks 512 bytes
4. Tiap chunk: verifikasi CRC-32
5. Setelah semua chunk → App kirim SHA-256 hash final
6. Firmware verifikasi hash lengkap
7. Firmware kirim CONFIRM ke App
8. App kirim APPLY → Firmware reboot ke slot baru
9. Jika boot baru gagal → auto rollback ke versi lama
```

**Tidak ada server update wajib.** File `.bin` dapat didistribusi via GitHub Releases, USB, atau bahkan dikirim dari HP ke HP via BLE mesh di masa depan.

---

## 🗺️ ROADMAP PENGEMBANGAN

### Phase 0 — Fondasi (Minggu 1–3)
**Target: Lingkungan development siap**

- [ ] Setup Rust toolchain untuk nRF5340 (`rustup target add thumbv8m.main-none-eabihf`)
- [ ] Setup Embassy + probe-rs untuk flash & debug
- [ ] Beli komponen dan rakit di breadboard/dev board nRF5340-DK
- [ ] Hello world: LED berkedip dari Embassy, konfirmasi toolchain kerja
- [ ] Buka KiCad, buat project PCB kosong, import footprint komponen

---

### Phase 1 — Audio Proof of Concept (Minggu 4–8)
**Target: Suara masuk, proses, keluar. Latency < 20ms.**

- [ ] Driver I2S untuk SPH0645 (capture audio dari mic)
- [ ] Driver I2S output untuk TPA6132A2 (putar ke speaker)
- [ ] Pipeline passthrough (mic langsung ke speaker, tanpa DSP)
- [ ] Ukur latency dengan oscilloscope
- [ ] Implement noise gate dasar
- [ ] Implement 8-band IIR equalizer (fixed-point)
- [ ] Test dengan 5 sukarelawan, ukur kualitas subjektif

---

### Phase 2 — BLE & Kalibrasi (Minggu 9–14)
**Target: Profil EQ bisa dikirim dari HP ke device**

- [ ] GATT server di firmware (service NFL Audio Config)
- [ ] Aplikasi Flutter minimal: scan device, konek BLE
- [ ] UI slider EQ di app → kirim ke firmware via BLE
- [ ] Implementasi dynamic compressor di firmware
- [ ] Storage profil ke flash W25Q64JV
- [ ] Load profil saat boot

---

### Phase 3 — Hearing Test & Auto Kalibrasi (Minggu 15–20)
**Target: App bisa generate profil EQ otomatis dari hasil tes pendengaran**

- [ ] Implementasi Pure Tone Audiometry di Flutter
- [ ] Engine Rust: audiogram → EQ profile (NAL-NL2)
- [ ] Validasi hasil dengan audiolog volunteer
- [ ] Preset lingkungan (calm, indoor, outdoor, noisy)
- [ ] UI lengkap aplikasi (onboarding, test, profil, settings)
- [ ] OTA firmware update

---

### Phase 4 — Hardware Miniaturisasi (Minggu 21–28)
**Target: PCB custom ukuran earphone**

- [ ] Desain skema lengkap di KiCad
- [ ] Layout PCB 4-layer (22mm × 18mm)
- [ ] Pesan PCB dari JLCPCB (batch 10 untuk test)
- [ ] Solder komponen SMD (reflow atau hot air)
- [ ] Desain casing FreeCAD (ITE — In-The-Ear)
- [ ] Print 3D casing PETG
- [ ] Pengujian waterproof dasar (IPX4)

---

### Phase 5 — Uji Klinis & Validasi (Bulan 8–10)
**Target: Validasi medis awal, iterasi berdasarkan feedback**

- [ ] Uji dengan 20+ pengguna berbagai tipe gangguan pendengaran
- [ ] Dokumentasi hasil vs alat bantu dengar konvensional
- [ ] Revisi algoritma berdasarkan feedback audiolog
- [ ] Pengujian durabilitas: keringat, debu, jatuh
- [ ] Penulisan dokumentasi assembly lengkap

---

### Phase 6 — Open Source Launch (Bulan 11–12)
**Target: Dunia bisa rakit sendiri**

- [ ] Publish semua ke GitHub dengan lisensi CERN-OHL-S v2 + GPL-3.0
- [ ] Video tutorial assembly (YouTube, bahasa Indonesia + English)
- [ ] Dokumentasi GitBook lengkap
- [ ] Gerber files siap kirim ke JLCPCB
- [ ] STL files siap print di Thingiverse / Printables
- [ ] Komunitas Discord / Telegram untuk builder

---

## 🧪 TESTING & QUALITY ASSURANCE

### Standar Audio yang Ditarget
| Metrik | Target NFL v1 | Alat Bantu Dengar Kelas Menengah |
|---|---|---|
| Latency | < 15ms | 5–15ms |
| Frekuensi respon | 200Hz – 8kHz | 200Hz – 8kHz |
| THD (Total Harmonic Distortion) | < 2% | < 1% |
| Noise floor | < -55 dBFS | < -60 dBFS |
| Max gain | 40 dB | 40–60 dB |
| Battery life | 20+ jam | 16–20 jam |

### Prosedur Test Wajib Sebelum Rilis
1. **Latency test** — suntikkan sinyal di input, ukur delay di output (oscilloscope)
2. **Frequency response sweep** — putar sweep 100Hz–10kHz, rekam output, plot kurva
3. **THD measurement** — sinyal sinus 1kHz 0dBFS, ukur distorsi
4. **Reliability test** — jalankan 72 jam terus-menerus, pantau temperature MCU
5. **Drop test** — jatuhkan dari 1 meter ke lantai kayu 10x
6. **Sweat test** — oleskan larutan salin 1%, tunggu 24 jam

---

## 📦 MODEL DISTRIBUSI

### Tingkat 1 — GitHub (Gratis Sepenuhnya)
Semua file tersedia: firmware source, PCB Gerbers, STL casing, panduan assembly.
**Target:** Engineer, maker, komunitas kampus, NGO teknis

### Tingkat 2 — Kit NFL (Biaya Komponen + Ongkir)
Paket berisi semua komponen sudah disortir + PCB jadi (perlu solder sendiri).
**Target:** Individu yang ingin rakit tapi tidak mau cari komponen sendiri

### Tingkat 3 — Unit Rakitan (Subsidi untuk yang Tidak Mampu)
Unit jadi, dikalibrasi dasar. Subsidi silang: yang mampu bayar normal, surplus untuk donasi ke keluarga miskin.
**Target:** Lansia, anak-anak, keluarga pra-sejahtera

### Tingkat 4 — Kemitraan Komunitas
Pelatihan teknisi lokal (puskesmas, balai desa, SMK elektronika) untuk rakit dan servis.
**Target:** Ketahanan ekosistem jangka panjang

---

## 👥 TIM & KONTRIBUSI

### Tim Inti yang Dibutuhkan

| Peran | Skill Utama | Status |
|---|---|---|
| Embedded Engineer | Rust, nRF5340, audio DSP | **Dibutuhkan** |
| Mobile Developer | Flutter, Dart, BLE | **Dibutuhkan** |
| PCB Designer | KiCad, analog audio circuit | **Dibutuhkan** |
| Audiolog | Hearing science, NAL-NL2 | **Relawan dibutuhkan** |
| Industrial Designer | FreeCAD, ergonomi earpiece | **Dibutuhkan** |

### Cara Berkontribusi
- **Kode:** Fork → branch → PR dengan test
- **Hardware:** Kirim KiCad patch atau issue dengan foto
- **Testing:** Isi form laporan di GitHub Issues
- **Dokumentasi:** Edit di GitBook atau PR ke `/docs`
- **Dana:** GitHub Sponsors atau donasi Saweria untuk beli komponen uji

---

## ⚖️ LISENSI

| Layer | Lisensi | Artinya |
|---|---|---|
| **Firmware & Software** | GNU GPL v3.0 | Modifikasi harus tetap open source |
| **Hardware (PCB, Casing)** | CERN OHL-S v2 | Desain hardware modifikasi harus tetap terbuka |
| **Dokumentasi** | CC BY-SA 4.0 | Bebas digunakan dengan atribusi |

> **Tidak ada bagian dari proyek ini yang boleh dipatenkan atau ditutup.** Setiap fork komersial tetap wajib open source sesuai lisensi copyleft.

---

## 💬 PENUTUP

Alat bantu dengar bukan kemewahan.

Jutaan manusia hari ini tidak bisa mendengar tawa anaknya, suara ibunya, atau sirene bahaya — bukan karena tidak ada teknologinya, tapi karena ada yang memutuskan teknologi itu hanya untuk mereka yang bisa membayar.

NFL adalah penolakan terhadap premis itu.

Bukan untuk mengalahkan industri. Tapi untuk menjangkau semua orang yang selama ini tidak terjangkau. Satu perangkat, satu rakitan, satu orang yang kembali bisa mendengar dunianya — itu sudah cukup untuk memulai.

> *"Mendengar itu gratis dan hak semua orang."*

---

**Nafal Faturizki**
*Inisiator Proyek NFL*
GitHub: github.com/nfl-hearing
Versi Blueprint: 1.0.0
Tanggal: 2026

---
*Blueprint ini sendiri dilisensikan di bawah CC BY-SA 4.0 — boleh disebarkan, dimodifikasi, dan digunakan seluas-luasnya selama atribusi dijaga dan tetap terbuka.*
