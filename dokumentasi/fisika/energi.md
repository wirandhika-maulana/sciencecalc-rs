# Energi

Modul [`energi.rs`](../src/math/energi.rs) pada pustaka **sciencecalc-rs** menyediakan fungsi-fungsi untuk menghitung energi **kinetik** dan **potensial**.

---

## 📍 Fungsi Utama

| Fungsi                              | Parameter                      | Return  | Deskripsi                                      |
|--------------------------------------|-------------------------------|---------|------------------------------------------------|
| `energi_kinetik(m: f64, v: f64)`    | `m: massa (kg)`, `v: kecepatan (m/s)` | `f64`   | Menghitung **energi kinetik**: \(E_k = 0.5 \times m \times v^2\) |
| `energi_potensial(m: f64, g: f64, h: f64)` | `m: massa (kg)`, `g: gravitasi (m/s²)`, `h: ketinggian (m)` | `f64` | Menghitung **energi potensial**: \(E_p = m \times g \times h\) |

---

## 📍 Penjelasan & Contoh Penggunaan

### Energi Kinetik

```rust
use sciencecalc_rs::math::energi::energi_kinetik;

fn main() {
    let massa = 2.0;    // kg
    let kecepatan = 3.0; // m/s
    let ek = energi_kinetik(massa, kecepatan);
    println!("Energi Kinetik: {:.2} Joule", ek); // Output: Energi Kinetik: 9.00 Joule
}
```

---

### Energi Potensial

```rust
use sciencecalc_rs::math::energi::energi_potensial;

fn main() {
    let massa = 2.0;     // kg
    let gravitasi = 9.8; // m/s^2
    let tinggi = 5.0;    // m
    let ep = energi_potensial(massa, gravitasi, tinggi);
    println!("Energi Potensial: {:.2} Joule", ep); // Output: Energi Potensial: 98.00 Joule
}
```

---

## Catatan

- Satuan hasil perhitungan adalah **Joule (J)**.
- Untuk gravitasi di permukaan bumi, gunakan nilai **9.8 m/s²**.

---

Dokumentasi ini mencakup seluruh fungsi energi pada pustaka **sciencecalc-rs**.  
Semoga bermanfaat!

---
