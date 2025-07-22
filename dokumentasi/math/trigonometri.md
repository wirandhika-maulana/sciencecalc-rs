# Trigonometri

Modul [`trigonometri.rs`](../src/math/trigonometri.rs) pada pustaka **sciencecalc-rs** menyediakan fungsi-fungsi trigonometri dasar, konversi sudut, dan hukum trigonometri.

---

## Struktur

Seluruh fungsi diakses melalui struktur `Trigonometri`.

---

## 📍 Fungsi Utama

| Fungsi                            | Parameter         | Return         | Deskripsi                                   |
|------------------------------------|------------------|----------------|---------------------------------------------|
| `deg_to_rad(degrees: f64)`         | `degrees: f64`   | `f64`          | Konversi derajat ke radian                  |
| `rad_to_deg(radians: f64)`         | `radians: f64`   | `f64`          | Konversi radian ke derajat                  |
| `sin(degrees: f64)`                | `degrees: f64`   | `f64`          | Nilai sinus dari sudut (derajat)            |
| `cos(degrees: f64)`                | `degrees: f64`   | `f64`          | Nilai cosinus dari sudut (derajat)          |
| `tan(degrees: f64)`                | `degrees: f64`   | `f64`          | Nilai tangen dari sudut (derajat)           |
| `csc(degrees: f64)`                | `degrees: f64`   | `Option<f64>`  | Nilai kosekan (csc), hasil `None` jika sin=0|
| `sec(degrees: f64)`                | `degrees: f64`   | `Option<f64>`  | Nilai sekan (sec), hasil `None` jika cos=0  |
| `cot(degrees: f64)`                | `degrees: f64`   | `Option<f64>`  | Nilai kotangen (cot), hasil `None` jika tan=0|
| `law_of_sines(a, angle_a, angle_b)`| `a: f64, angle_a: f64, angle_b: f64` | `f64` | Hukum sinus: cari sisi b dari a, sudut A & B|
| `law_of_cosines(a, b, angle_c)`    | `a: f64, b: f64, angle_c: f64` | `f64` | Hukum cosinus: cari sisi c dari a, b, sudut C|

---

## 📍 Penjelasan & Contoh Penggunaan

### 1. Konversi Sudut
```rust
use sciencecalc_rs::math::trigonometri::Trigonometri;

fn main() {
    let rad = Trigonometri::deg_to_rad(180.0);
    let deg = Trigonometri::rad_to_deg(std::f64::consts::PI);
    println!("180 derajat = {:.2} radian", rad); // Output: 180 derajat = 3.14 radian
    println!("π radian = {:.2} derajat", deg);   // Output: π radian = 180.00 derajat
}
```

---

### 2. Fungsi Dasar Trigonometri
```rust
use sciencecalc_rs::math::trigonometri::Trigonometri;

fn main() {
    println!("sin(30) = {:.2}", Trigonometri::sin(30.0)); // Output: sin(30) = 0.50
    println!("cos(60) = {:.2}", Trigonometri::cos(60.0)); // Output: cos(60) = 0.50
    println!("tan(45) = {:.2}", Trigonometri::tan(45.0)); // Output: tan(45) = 1.00
}
```

---

### 3. Fungsi Turunan (csc, sec, cot)
```rust
use sciencecalc_rs::math::trigonometri::Trigonometri;

fn main() {
    println!("csc(30) = {:?}", Trigonometri::csc(30.0)); // Output: csc(30) = Some(2.0)
    println!("sec(60) = {:?}", Trigonometri::sec(60.0)); // Output: sec(60) = Some(2.0)
    println!("cot(45) = {:?}", Trigonometri::cot(45.0)); // Output: cot(45) = Some(1.0)
}
```

---

### 4. Hukum Sinus
```rust
use sciencecalc_rs::math::trigonometri::Trigonometri;

fn main() {
    let b = Trigonometri::law_of_sines(5.0, 30.0, 45.0);
    println!("Sisi b (hukum sinus): {:.2}", b);
    // Output: Sisi b (hukum sinus): 7.07
}
```

---

### 5. Hukum Cosinus
```rust
use sciencecalc_rs::math::trigonometri::Trigonometri;

fn main() {
    let c = Trigonometri::law_of_cosines(5.0, 7.0, 60.0);
    println!("Sisi c (hukum cosinus): {:.2}", c);
    // Output: Sisi c (hukum cosinus): 7.00
}
```

---

## Catatan

- Fungsi `csc`, `sec`, dan `cot` akan mengembalikan `None` jika nilai penyebutnya adalah 0.
- Semua sudut untuk fungsi trigonometri diinput dalam **derajat**.
- Untuk hukum sinus dan cosinus, pastikan **sudut** dalam derajat.

---

Dokumentasi ini mencakup seluruh fungsi pada modul Trigonometri di pustaka **sciencecalc-rs**.  
Semoga bermanfaat!

---
