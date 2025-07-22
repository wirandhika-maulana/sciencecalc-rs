# sciencecalc-rs

`sciencecalc-rs` adalah pustaka (crate) Rust yang menyediakan berbagai fungsi kalkulasi ilmiah — matematika, fisika, dan kimia — dengan antarmuka yang mudah digunakan.

## Instalasi

Tambahkan pustaka ini ke proyek Cargo Anda dengan perintah:

```sh
cargo add sciencecalc-rs
```

Atau tambahkan secara manual ke bagian `dependencies` pada file `Cargo.toml`:

```toml
sciencecalc-rs = "<Versi Crates>"
```

## Penggunaan

Import pustaka dan modul sesuai kebutuhan:

```rust
use sciencecalc_rs::matematika;
use sciencecalc_rs::fisika;
use sciencecalc_rs::kimia;

fn main() {
    // Implementasi kode di sini
}
```

Untuk mengakses fungsi-fungsi kalkulasi, panggil dengan namespace modul yang bersangkutan:

```rust
use sciencecalc_rs::matematika::aljabar;
use sciencecalc_rs::fisika::gaya;
use sciencecalc_rs::kimia::stoikiometri;
```

### Contoh Penggunaan

#### 1. Matematika: Sistem Persamaan Linear Satu Variabel (SPLSV)

```rust
use sciencecalc_rs::matematika::aljabar;

fn main() {
    // SPLSV: ax + b = 0
    let solusi = aljabar::splsv(2.0, 3.0);
    println!("Solusi SPLSV (2x + 3 = 0): {:?}", solusi);
}
```

**Output:**
```sh
Solusi SPLSV (2x + 3 = 0): Some(-1.5)
```

#### 2. Fisika: Gaya

```rust
use sciencecalc_rs::fisika::gaya;

fn main() {
    let m = 10.0;
    let a = 9.8;
    let gaya = gaya::gaya(m, a);
    println!("Gaya (m=10, a=9.8): {}", gaya);
}
```
**Output:**
```sh
Gaya (m=10, a=9.8): 98
```

#### 3. Kimia: Jumlah Mol

```rust
use sciencecalc_rs::kimia::stoikiometri;

fn main() {
    let massa = 18.0;
    let massa_molar = 18.0;
    let mol = stoikiometri::jumlah_mol(massa, massa_molar);
    println!("Jumlah mol (massa=18g, massa molar=18g/mol): {}", mol);
}
```
**Output:**
```sh
Jumlah mol (massa=18g, massa molar=18g/mol): 1
```

## Fitur

<details>
<summary><strong>Matematika</strong></summary>

<ul>
  <li>Operasi aritmetika dasar: tambah, kurang, kali, bagi, faktorial, pangkat, akar.</li>
  <li>Operasi trigonometri: sinus, cosinus, tangen, dan invers.</li>
  <li>Statistika dasar: rata-rata, median, modus, deviasi standar.</li>
  <li>Persamaan dan sistem persamaan linear.</li>
  <li>Operasi matriks: penjumlahan, perkalian, invers, determinan.</li>
</ul>
</details>

<details>
<summary><strong>Fisika</strong></summary>

<ul>
  <li>Kalkulasi gaya, energi, gerak, dan listrik.</li>
  <li>Perhitungan hukum Newton.</li>
  <li>Konversi satuan fisika.</li>
  <li>Kalkulasi kelistrikan (arus, tegangan, hambatan).</li>
</ul>
</details>

<details>
<summary><strong>Kimia</strong></summary>

<ul>
  <li>Perhitungan mol dan massa molar.</li>
  <li>Stoikiometri reaksi kimia.</li>
  <li>Kalkulasi larutan: konsentrasi, molaritas, pH.</li>
  <li>Hukum gas ideal.</li>
</ul>
</details>

## Dokumentasi Lengkap

Dokumentasi lengkap pustaka ini dapat diakses di:  
[Dokumentasi sciencecalc-rs](https://github.com/wirandhika-maulana/sciencecalc-rs/blob/master/DOKUMENTASI.md)

## Status Pengembangan

Pustaka ini masih dalam tahap pengembangan aktif, sehingga mungkin terdapat bug atau keterbatasan.  
Kontribusi dan masukan sangat diharapkan untuk meningkatkan kualitas pustaka ini.

Selamat mencoba!

---

> [!WARNING]
>
> REPOSITORI INI SERING DAN SELALU MENDAPATKAN UPDATE.
>
> KESTABILAN DI DALAM *BRANCH* [`master`](https://github.com/wirandhika-maulana/sciencecalc-rs/tree/master) TIDAK DAPAT DIPASTIKAN!

---
