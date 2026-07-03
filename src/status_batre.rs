// use std::fs;
// fn jawa() {
//     // membaca kapasitas batrai di sistem
//     let batre_kapasitas = "/sys/class/power_supply/BAT0/capacity";
//     let batre_status = "/sys/class/power_supply/BAT0/status";

//     let kapasitas = fs::read_to_string(batre_kapasitas)
//         .expect("Gagal membaca kapasitas batrai")
//         .trim()
//         .to_string();
//     let status = fs::read_to_string(batre_status)
//         .expect("Gagal membaca status batrai")
//         .trim()
//         .to_string();

//     let status_cas = if status == "Charging" {
//         "Sedang mengisi daya"
//     } else if status == "Discharging" {
//         "Sedang digunakan"
//     } else {
//         "Status tidak diketahui"
//     };

//     println!("Kapasitas batrai: {}%, Status: {}", kapasitas, status_cas);
// }