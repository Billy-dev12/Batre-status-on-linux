# batre_status

Dashboard monitoring status baterai laptop secara real-time yang dibuat dengan Rust.

## Fitur

- Membaca status baterai langsung dari sistem Linux (`/sys/class/power_supply/BAT0/`)
- Menampilkan informasi secara real-time dengan auto-refresh setiap 1 detik
- Info yang ditampilkan:
  - Model & pabrikan baterai
  - Tipe teknologi baterai
  - Status充电 / discharge
  - Persentase kapasitas
  - Cycle count
  - Voltage & current
  - Kapasitas saat ini vs kapasitas penuh (Ah)

## Persyaratan

- **OS**: Linux (karena membaca dari `/sys/class/power_supply/BAT0/`)
- **Rust**: versi 2024 edition atau lebih baru
- **Cargo**: package manager Rust

## Instalasi & Menjalankan

```bash
# Clone repository
git clone <url-repo-kamu>
cd batre_status

# Build & jalankan
cargo run --release
```

## Penggunaan

Setelah program berjalan, dashboard akan tampil di terminal dan otomatis refresh setiap detik.

**Tombol:**
| Tombol | Fungsi |
|--------|--------|
| `ESC` | Keluar dari program |
| `Ctrl+C` | Keluar dari program |

## Contoh Output

```
Battery Status Report
Model: XXX-XXX (Manufacturer)
Technology: Li-ion
Status: Charging
Capacity: 85% (Normal)
Cycle Count: 120
Voltage: 12.45 V
Current: 1.23 A
Charge: 3.80 Ah / 4.50 Ah
Press ESC or Ctrl+C to exit.
```

## Struktur Proyek

```
batre_status/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs              # Entry point & dashboard loop
│   └── status_batre.rs      # (unused, kode lama)
└── README.md
```

## Dependencies

- [`crossterm`](https://crates.io/crates/crossterm) - Terminal manipulation library

## Catatan

- Program ini hanya berjalan di laptop/PC yang memiliki baterai (BAT0).
- Jika tidak ada baterai, program akan menampilkan "Tidak Diketahui" pada beberapa field.
- Untuk menentukan path baterai yang benar, bisa cek `/sys/class/power_supply/` dan sesuaikan jika baterai kamu bukan `BAT0`.
