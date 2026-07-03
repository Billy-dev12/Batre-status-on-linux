use std::fs;
use std::time::Duration;
use std::io::Write;
// Tambahkan "QueueableCommand" dan perintah terminal/cursor di sini
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{self, Clear, ClearType},
    cursor::MoveTo,
    QueueableCommand,
};

fn baca_file_batre(nama_file: &str) -> String {
    let path = format!("/sys/class/power_supply/BAT0/{}", nama_file);
    fs::read_to_string(path)
        .unwrap_or_else(|_| "Tidak Diketahui".to_string())
        .trim()
        .to_string()
}
fn cetak_dashboard(stdout: &mut std::io::Stdout) {
    // Ambil data
    let status = baca_file_batre("status");
    let kapasitas = baca_file_batre("capacity");
    let level_kapasitas = baca_file_batre("capacity_level");
    let model = baca_file_batre("model_name");
    let pabrikan = baca_file_batre("manufacturer");
    let teknologi = baca_file_batre("technology");
    let siklus = baca_file_batre("cycle_count");
    
    let voltage_raw: f64 = baca_file_batre("voltage_now").parse().unwrap_or(0.0);
    let current_raw: f64 = baca_file_batre("current_now").parse().unwrap_or(0.0);
    let charge_full_raw: f64 = baca_file_batre("charge_full").parse().unwrap_or(0.0);
    let charge_now_raw: f64 = baca_file_batre("charge_now").parse().unwrap_or(0.0);

    let voltage = voltage_raw / 1_000_000.0;
    let arus = current_raw / 1_000_000.0;
    let kapasitas_penuh = charge_full_raw / 1_000_000.0;
    let kapasitas_sekarang = charge_now_raw / 1_000_000.0;

    let ikon_status = match status.as_str() {
        "Charging" => "Charging",
        "Discharging" => "Discharging",
        "Full" => "Full",
        _ => "Unknown",
    };

    // Bersihkan layar & cetak
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(MoveTo(0, 0)).unwrap();

    // Cetak raw data (tanpa emoji & garis)
    println!("Battery Status Report\r");
    println!("Model: {} ({})\r", model, pabrikan);
    println!("Technology: {}\r", teknologi);
    println!("Status: {}\r", ikon_status);
    println!("Capacity: {}% ({})\r", kapasitas, level_kapasitas);
    println!("Cycle Count: {}\r", siklus);
    println!("Voltage: {:.2} V\r", voltage);
    println!("Current: {:.2} A\r", arus);
    println!("Charge: {:.2} Ah / {:.2} Ah\r", kapasitas_sekarang, kapasitas_penuh);
    println!("Press ESC or Ctrl+C to exit.\r");

    stdout.flush().unwrap();
}

fn main() {
    let mut stdout = std::io::stdout();
    
    // 1. Aktifkan Raw Mode
    terminal::enable_raw_mode().unwrap();

    loop {
        cetak_dashboard(&mut stdout);

        if event::poll(Duration::from_secs(1)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.code == KeyCode::Esc {
                    terminal::disable_raw_mode().unwrap();
                    println!("\n\rKeluar dari program (ESC). Bye bro!\n\r");
                    break;
                }
                if key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    terminal::disable_raw_mode().unwrap();
                    println!("\n\rKeluar dari program (Ctrl+C). Bye bro!\n\r");
                    break;
                }
            }
        }
    }
}