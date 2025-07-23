# Statistika

Modul [`statistika.rs`](../src/matematika/statistika.rs) pada pustaka **sciencecalc-rs** menyediakan fungsi-fungsi statistik dasar seperti **mean, median, modus, varian, dan standar deviasi**.

---

## Struktur

Seluruh fungsi statistik diakses melalui struktur `statistika`.

---

## 📍 Fungsi Utama

| Fungsi                               | Parameter          | Return      | Deskripsi                                              |
|-------------------------------------- |-------------------|-------------|--------------------------------------------------------|
| `mean(data: &[f64])`                 | slice f64         | `f64`       | Rata-rata (mean) data                                  |
| `median(data: &mut [f64])`           | slice f64 (mut)   | `f64`       | Nilai tengah (median); data diurutkan dahulu           |
| `modus(data: &[i64])`                | slice i64         | `Vec<i64>`  | Modus (nilai paling sering muncul, bisa lebih dari satu)|
| `varian(data: &[f64])`               | slice f64         | `f64`       | Varian data                                            |
| `standar_deviasi(data: &[f64])`      | slice f64         | `f64`       | Standar deviasi data                                   |

---

## 📍 Penjelasan & Contoh Penggunaan

### 1. Mean (Rata-rata)
```rust
use sciencecalc_rs::matematika::statistika::*;

fn main() {
    let data = [3.0, 5.0, 7.0, 9.0];
    let rataan = statistika::mean(&data);
    println!("Mean: {:.2}", rataan); // Output: Mean: 6.00
}
```

---

### 2. Median
```rust
use sciencecalc_rs::matematika::statistika::*;

fn main() {
    let mut data = [4.0, 1.0, 7.0, 3.0, 9.0];
    let med = statistika::median(&mut data);
    println!("Median: {:.2}", med); // Output: Median: 4.00
}
```

---

### 3. Modus
```rust
use sciencecalc_rs::matematika::statistika::*;

fn main() {
    let data = [2, 4, 4, 5, 5, 5, 2, 2];
    let modus = statistika::modus(&data);
    println!("Modus: {:?}", modus); // Output: Modus: [2, 5]
}
```

---

### 4. Varian
```rust
use sciencecalc_rs::matematika::statistika::*;

fn main() {
    let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let varian = statistika::varian(&data);
    println!("Varian: {:.2}", varian); // Output: Varian: 4.00
}
```

---

### 5. Standar Deviasi
```rust
use sciencecalc_rs::matematika::statistika::*;

fn main() {
    let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let stddev = statistika::standar_deviasi(&data);
    println!("Standar Deviasi: {:.2}", stddev); // Output: Standar Deviasi: 2.00
}
```

---

## Catatan

- Semua fungsi akan `panic!` jika array kosong diberikan sebagai input.
- Untuk `median`, data akan **diurutkan** di tempat (in-place sort).
- Untuk `modus`, hasil berupa `Vec<i64>` berisi semua nilai dengan frekuensi maksimum.

---

Dokumentasi ini mencakup semua fungsi statistik pada pustaka **sciencecalc-rs**.  
Semoga bermanfaat!

---
