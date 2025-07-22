# Listrik

Modul [`listrik.rs`](../src/fisika/listrik.rs) pada pustaka **sciencecalc-rs** menyediakan fungsi-fungsi perhitungan dasar kelistrikan menggunakan **Hukum Ohm**.

---

## 📍 Fungsi Utama

| Fungsi                       | Parameter                            | Return | Deskripsi                                                      |
|------------------------------|--------------------------------------|--------|----------------------------------------------------------------|
| `ohm_tegangannya(i, r)`      | `i: arus (A)`, `r: hambatan (Ω)`     | `f64`  | Menghitung **tegangan listrik (V)**: \(V = I \times R\)        |
| `ohm_arusnya(v, r)`          | `v: tegangan (V)`, `r: hambatan (Ω)` | `f64`  | Menghitung **arus listrik (I)**: \(I = V / R\)                 |
| `ohm_hambatannya(v, i)`      | `v: tegangan (V)`, `i: arus (A)`     | `f64`  | Menghitung **hambatan listrik (R)**: \(R = V / I\)             |

---

## 📍 Penjelasan & Contoh Penggunaan

### 1. Menghitung Tegangan (V)

```rust
use sciencecalc_rs::fisika::listrik::ohm_tegangannya;

fn main() {
    let arus = 2.0;      // ampere
    let hambatan = 5.0;  // ohm
    let v = ohm_tegangannya(arus, hambatan);
    println!("Tegangan: {:.2} Volt", v); // Output: Tegangan: 10.00 Volt
}
```

---

### 2. Menghitung Arus (I)

```rust
use sciencecalc_rs::fisika::listrik::ohm_arusnya;

fn main() {
    let tegangan = 10.0; // volt
    let hambatan = 5.0;  // ohm
    let i = ohm_arusnya(tegangan, hambatan);
    println!("Arus: {:.2} Ampere", i); // Output: Arus: 2.00 Ampere
}
```

---

### 3. Menghitung Hambatan (R)

```rust
use sciencecalc_rs::fisika::listrik::ohm_hambatannya;

fn main() {
    let tegangan = 10.0; // volt
    let arus = 2.0;      // ampere
    let r = ohm_hambatannya(tegangan, arus);
    println!("Hambatan: {:.2} Ohm", r); // Output: Hambatan: 5.00 Ohm
}
```

---

## Catatan

- Satuan hasil perhitungan: **Volt (V)** untuk tegangan, **Ampere (A)** untuk arus, **Ohm (Ω)** untuk hambatan.
- Pastikan satuan parameter sudah sesuai.

---

Dokumentasi ini mencakup seluruh fungsi kelistrikan pada pustaka **sciencecalc-rs**.  
Semoga bermanfaat!

---
