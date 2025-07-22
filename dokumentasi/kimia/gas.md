# Gas

Modul [`gas.rs`](../src/kimia/gas.rs) pada pustaka **sciencecalc-rs** menyediakan fungsi untuk menghitung tekanan gas ideal menggunakan **Hukum Gas Ideal**.

---

## 📍 Fungsi Utama

| Fungsi                         | Parameter                                       | Return | Deskripsi                                                         |
|--------------------------------|-------------------------------------------------|--------|-------------------------------------------------------------------|
| `tekanan_gas_ideal(n, r, t, v)`| `n: jumlah mol`, `r: konstanta gas`, `t: suhu (K)`, `v: volume (L)` | `f64`  | Menghitung **tekanan gas ideal (P)**: \(P = \frac{nRT}{V}\)       |

---

## 📍 Penjelasan & Contoh Penggunaan

```rust
use sciencecalc_rs::kimia::gas::tekanan_gas_ideal;

fn main() {
    let n = 2.0;       // mol
    let r = 0.082;     // L·atm/(mol·K) (konstanta gas ideal)
    let t = 300.0;     // Kelvin
    let v = 10.0;      // Liter
    let p = tekanan_gas_ideal(n, r, t, v);
    println!("Tekanan Gas Ideal: {:.2} atm", p); // Output: Tekanan Gas Ideal: 4.92 atm
}
```

---

## Catatan

- Satuan hasil perhitungan **tekanan** biasanya dalam **atm** jika menggunakan \(R = 0.082\, \text{L·atm/(mol·K)}\).
- Pastikan satuan parameter sudah sesuai rumus:  
  - n: jumlah mol  
  - r: konstanta gas ideal  
  - t: suhu (Kelvin)  
  - v: volume (Liter)

---

Dokumentasi ini mencakup seluruh fungsi gas pada pustaka **sciencecalc-rs**.  
Semoga bermanfaat!

---
