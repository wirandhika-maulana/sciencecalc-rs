# Gerak

Modul [`gerak.rs`](../src/fisika/gerak.rs) pada pustaka **sciencecalc-rs** menyediakan fungsi perhitungan untuk **gerak lurus berubah beraturan (GLBB)**, yaitu perpindahan dan kecepatan akhir.

---

## 📍 Fungsi Utama

| Fungsi                             | Parameter                                  | Return  | Deskripsi                                                    |
|------------------------------------ |-------------------------------------------|---------|--------------------------------------------------------------|
| `glbb_perpindahan(v0, t, a)`       | `v0: kecepatan awal (m/s)`, `t: waktu (s)`, `a: percepatan (m/s²)` | `f64`   | Menghitung **perpindahan** GLBB: \(s = v_0 \times t + 0.5 \times a \times t^2\) |
| `glbb_kecepatan_akhir(v0, a, t)`   | `v0: kecepatan awal (m/s)`, `a: percepatan (m/s²)`, `t: waktu (s)` | `f64`   | Menghitung **kecepatan akhir** GLBB: \(v = v_0 + a \times t\)                 |

---

## 📍 Penjelasan & Contoh Penggunaan

### 1. Perpindahan GLBB

```rust
use sciencecalc_rs::math::gerak::glbb_perpindahan;

fn main() {
    let v0 = 2.0;   // m/s
    let t = 3.0;    // s
    let a = 4.0;    // m/s^2
    let s = glbb_perpindahan(v0, t, a);
    println!("Perpindahan: {:.2} meter", s); // Output: Perpindahan: 24.00 meter
}
```

---

### 2. Kecepatan Akhir GLBB

```rust
use sciencecalc_rs::math::gerak::glbb_kecepatan_akhir;

fn main() {
    let v0 = 2.0;   // m/s
    let a = 4.0;    // m/s^2
    let t = 3.0;    // s
    let v = glbb_kecepatan_akhir(v0, a, t);
    println!("Kecepatan Akhir: {:.2} m/s", v); // Output: Kecepatan Akhir: 14.00 m/s
}
```

---

## Catatan

- Satuan hasil perhitungan: **meter (m)** untuk perpindahan, **meter per detik (m/s)** untuk kecepatan.
- Pastikan parameter satuan sudah sesuai.

---

Dokumentasi ini mencakup seluruh fungsi gerak pada pustaka **sciencecalc-rs**.  
Semoga bermanfaat!

---
