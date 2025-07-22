# sciencecalc-rs

`sciencecalc-rs` adalah pustaka (crate) Rust yang menyediakan berbagai fungsi kalkulasi ilmiah (matematika, fisika, dan kimia) dengan antarmuka yang mudah digunakan.

## Instalasi

Tambahkan pustaka ini ke dalam proyek Cargo Anda dengan perintah berikut:

```sh
cargo add sciencecalc-rs
```

Atau tambahkan secara manual ke bagian `dependencies` pada file `Cargo.toml`:

```toml
sciencecalc-rs = "<Versi Crates>"
```

## Penggunaan

Untuk menggunakan pustaka ini, import terlebih dahulu `sciencecalc-rs` ke dalam kode Anda:

```rust
use sciencecalc_rs;

fn main() {
    // Implementasi kode di sini
}
```

Untuk mengakses metode yang tersedia, panggil pustaka diikuti dengan modul yang bersangkutan:

```rust
use sciencecalc_rs::math::algebra;      // Modul matematika
use sciencecalc_rs::physics::force;     // Modul fisika
use sciencecalc_rs::chemistry::stoichiometry; // Modul kimia
```

### Contoh Penggunaan

#### 1. Faktorial dan Persamaan Linear

```rust
use sciencecalc_rs::math::algebra;
use sciencecalc_rs::math::linear;

fn main() {
    let faktorial = algebra::factorial(5);
    let solusi_linear = linear::solve_linear_1x1(2.0, 3.0, 11.0);
    println!("Factorial 5: {}", faktorial);
    println!("Linear 1x1 (2x + 3 = 11): {:?}", solusi_linear);
}
```

**Output:**
```sh
Factorial 5: 120
Linear 1x1 (2x + 3 = 11): Some(4.0)
```

#### 2. Kalkulasi Fisika: Gaya

```rust
use sciencecalc_rs::physics::force;

fn main() {
    let m = 10.0;
    let a = 9.8;
    let gaya = force::calculate_force(m, a);
    println!("Force (m=10, a=9.8): {}", gaya);
}
```
**Output:**
```sh
Force (m=10, a=9.8): 98
```

#### 3. Kalkulasi Kimia: Mol

```rust
use sciencecalc_rs::chemistry::stoichiometry;

fn main() {
    let mass = 18.0;
    let molar_mass = 18.0;
    let mol = stoichiometry::calculate_moles(mass, molar_mass);
    println!("Moles (mass=18g, molar mass=18g/mol): {}", mol);
}
```
**Output:**
```sh
Moles (mass=18g, molar mass=18g/mol): 1
```

## Fitur

- **Matematika**
  - Operasi aritmetika dasar: tambah, kurang, kali, bagi, faktorial, pangkat, akar.
  - Operasi trigonometri: sinus, cosinus, tangen, dan invers.
  - Statistik dasar: rata-rata, median, modus, deviasi standar.
  - Persamaan dan sistem persamaan linear.
  - Operasi matriks: penjumlahan, perkalian, invers, determinan.

- **Fisika**
  - Kalkulasi gaya, energi, dan gerak.
  - Perhitungan hukum Newton.
  - Konversi satuan fisika.
  - Kalkulasi kelistrikan (arus, tegangan, hambatan).

- **Kimia**
  - Perhitungan mol dan massa molar.
  - Stoikiometri reaksi kimia.
  - Kalkulasi larutan: konsentrasi, molaritas.
  - Hukum gas ideal.

- **Konversi Sistem Bilangan**
  - Konversi desimal, biner, oktal, dan heksadesimal.

- **Contoh Penggunaan**
  - Tersedia contoh kode untuk tiap fitur utama.

## Dokumentasi Lengkap

Dokumentasi lengkap pustaka ini dapat diakses melalui tautan berikut:  
[Dokumentasi sciencecalc-rs](https://github.com/wirandhika-maulana/sciencecalc-rs/blob/master/DOKUMENTASI.md)

## Status Pengembangan

Pustaka ini masih dalam tahap pengembangan aktif, sehingga mungkin terdapat bug atau keterbatasan dalam fungsionalitasnya.  
Pengguna diharapkan untuk memberikan masukan dan kontribusi guna meningkatkan kualitas pustaka ini.

Selamat mencoba!

---

> [!WARNING]
>
> REPOSITORY INI SERING DAN SELALU MENDAPATKAN UPDATE.
>
> KESTABILAN DI DALAM *BRANCH* [`master`](https://github.com/wirandhika-maulana/sciencecalc-rs/tree/master) TIDAK DAPAT DIPASTIKAN!

---
