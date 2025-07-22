# Konversi Basis

Modul [**Konversi Basis**](../src/math/basis.rs) pada pustaka **sciencecalc-rs** menyediakan fungsi untuk mengubah angka dari satu basis ke basis lain. Modul ini mendukung basis antara 2 hingga 36.

---

## Fungsi Utama

### `basis`

#### Deskripsi
Fungsi untuk mengonversi angka desimal ke basis lain (2–36).

#### Sintaks
```rust
pub fn basis(num: u64, base: u32) -> String;
```

#### Parameter
- `num`: Angka desimal yang akan dikonversi.
- `base`: Basis tujuan (2 sampai 36).

#### Contoh Penggunaan
```rust
use sciencecalc_rs::math::basis::basis;

fn main() {
    let hasil = basis(255, 16);
    println!("{}", hasil); // Output: "FF"
}
```

---

### `parse_number`

#### Deskripsi
Fungsi untuk mengubah angka dari basis tertentu ke desimal (basis 10).

#### Sintaks
```rust
pub fn parse_number(num_str: &str, from_base: u32) -> u64;
```

#### Parameter
- `num_str`: Angka pada basis asal, dalam bentuk string.
- `from_base`: Basis asal (2 sampai 36).

#### Contoh Penggunaan
```rust
use sciencecalc_rs::math::basis::parse_number;

fn main() {
    let hasil = parse_number("FF", 16);
    println!("{}", hasil); // Output: 255
}
```

---

## Fungsi Konversi Antar Basis Populer

Modul ini juga menyediakan fungsi khusus untuk konversi antar basis populer: biner, oktal, desimal, dan heksadesimal.

### Dari Desimal

```rust
pub fn desimal_ke_biner(num: u64) -> String;
pub fn desimal_ke_oktal(num: u64) -> String;
pub fn desimal_ke_hexadesimal(num: u64) -> String;
```

### Dari Biner

```rust
pub fn biner_ke_desimal(num_str: &str) -> u64;
pub fn biner_ke_oktal(num_str: &str) -> String;
pub fn biner_ke_hexadesimal(num_str: &str) -> String;
```

### Dari Heksadesimal

```rust
pub fn hexadesimal_ke_desimal(num_str: &str) -> u64;
pub fn hexadesimal_ke_biner(num_str: &str) -> String;
pub fn hexadesimal_ke_oktal(num_str: &str) -> String;
```

### Dari Oktal

```rust
pub fn oktal_ke_desimal(num_str: &str) -> u64;
pub fn oktal_ke_biner(num_str: &str) -> String;
pub fn oktal_ke_hexadesimal(num_str: &str) -> String;
```

---

## Contoh Penggunaan

### Desimal ke Biner
```rust
use sciencecalc_rs::math::basis::desimal_ke_biner;

fn main() {
    let hasil = desimal_ke_biner(10);
    println!("{}", hasil); // Output: "1010"
}
```

### Biner ke Desimal
```rust
use sciencecalc_rs::math::basis::biner_ke_desimal;

fn main() {
    let hasil = biner_ke_desimal("1010");
    println!("{}", hasil); // Output: 10
}
```

### Heksadesimal ke Biner
```rust
use sciencecalc_rs::math::basis::hexadesimal_ke_biner;

fn main() {
    let hasil = hexadesimal_ke_biner("FF");
    println!("{}", hasil); // Output: "11111111"
}
```

### Oktal ke Desimal
```rust
use sciencecalc_rs::math::basis::oktal_ke_desimal;

fn main() {
    let hasil = oktal_ke_desimal("17");
    println!("{}", hasil); // Output: 15
}
```

---

Dokumentasi ini mencakup seluruh fungsi yang tersedia dalam modul Konversi Basis di pustaka **sciencecalc-rs**.  
Semoga bermanfaat!

---
