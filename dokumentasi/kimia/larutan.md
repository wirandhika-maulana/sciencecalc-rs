# Larutan

Modul [`larutan.rs`](../src/kimia/larutan.rs) pada pustaka **sciencecalc-rs** menyediakan fungsi untuk menghitung **molaritas** dan **pH larutan asam kuat**.

---

## 📍 Fungsi Utama

| Fungsi                        | Parameter                                   | Return | Deskripsi                                                         |
|-------------------------------|---------------------------------------------|--------|-------------------------------------------------------------------|
| `molaritas(jumlah_mol, volume_liter)` | `jumlah_mol: jumlah mol`, `volume_liter: volume larutan (L)` | `f64`  | Menghitung **molaritas (M)**: \(M = \frac{n}{V}\)                 |
| `ph_asam_kuat(konsentrasi_h)` | `konsentrasi_h: konsentrasi [H+] (mol/L)`   | `f64`  | Menghitung **pH larutan asam kuat**: \(pH = -\log[H^+]\)          |

---

## 📍 Penjelasan & Contoh Penggunaan

### 1. Menghitung Molaritas

```rust
use sciencecalc_rs::kimia::larutan::molaritas;

fn main() {
    let jumlah_mol = 0.5;      // mol
    let volume_liter = 2.0;    // liter
    let M = molaritas(jumlah_mol, volume_liter);
    println!("Molaritas: {:.2} M", M); // Output: Molaritas: 0.25 M
}
```

---

### 2. Menghitung pH Larutan Asam Kuat

```rust
use sciencecalc_rs::kimia::larutan::ph_asam_kuat;

fn main() {
    let konsentrasi_h = 0.01;  // mol/L
    let pH = ph_asam_kuat(konsentrasi_h);
    println!("pH larutan: {:.2}", pH); // Output: pH larutan: 2.00
}
```

---

## Catatan

- Satuan molaritas adalah **mol/L (M)**.
- Untuk pH, fungsi ini hanya berlaku untuk **asam kuat** (diasumsikan ionisasi sempurna).

---

Dokumentasi ini mencakup seluruh fungsi larutan pada pustaka **sciencecalc-rs**.  
Semoga bermanfaat!

---
