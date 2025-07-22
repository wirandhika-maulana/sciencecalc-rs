# Gaya

Modul [`gaya.rs`](../src/fisika/gaya.rs) pada pustaka **sciencecalc-rs** menyediakan fungsi untuk menghitung **gaya** berdasarkan massa dan percepatan.

---

## 📍 Fungsi Utama

| Fungsi                | Parameter                  | Return  | Deskripsi                                  |
|-----------------------|---------------------------|---------|---------------------------------------------|
| `gaya(m: f64, a: f64)` | `m: massa (kg)`, `a: percepatan (m/s²)` | `f64`   | Menghitung **gaya**: \(F = m \times a\)    |

---

## 📍 Penjelasan & Contoh Penggunaan

```rust
use sciencecalc_rs::math::gaya::gaya;

fn main() {
    let massa = 10.0;       // kg
    let percepatan = 2.0;   // m/s^2
    let F = gaya(massa, percepatan);
    println!("Gaya: {:.2} Newton", F); // Output: Gaya: 20.00 Newton
}
```

---

## Catatan

- Satuan hasil perhitungan adalah **Newton (N)**.
- Pastikan satuan massa dalam **kilogram (kg)** dan percepatan dalam **meter per detik kuadrat (m/s²)**.

---

Dokumentasi ini mencakup seluruh fungsi gaya pada pustaka **sciencecalc-rs**.  
Semoga bermanfaat!

---
