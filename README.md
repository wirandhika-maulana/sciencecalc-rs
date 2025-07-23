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
use sciencecalc_rs::matematika::aritmetika;
use sciencecalc_rs::fisika::gaya;
use sciencecalc_rs::kimia::stoikiometri;
```

### Contoh Penggunaan

#### 1. Operasi Aritmetika Dasar

```rust
use sciencecalc_rs::matematika::aritmetika;

fn main() {
    let x = 100;
    let y = 5;
    let z = [5, 6, 8, 125];
    let hasil = aritmetika::tambah(
        aritmetika::tambah(
            aritmetika::tambah(x, y),
            aritmetika::kali(x, y)
        ),
        aritmetika::super_kurang(&z)
    );
    println!("{}", hasil);
}
```

**Output:**
```sh
471
```

#### 2. Matematika: Sistem Persamaan Linear

```rust
use sciencecalc_rs::matematika::aljabar::*;

fn main() {
    let a = 6.0; 
    let b = 12.0;
    let x = Aljabar::splsv(a, b);

    let a1 = 4.0; let b1 = -3.0; let c1 = 18.0;
    let a2 = 3.0; let b2 = 1.0; let c2 = 7.0;
    let hasil = Aljabar::spldv(a1, b1, c1, a2, b2, c2);

    println!("{}\n{:?}", x, hasil.unwrap());
}
```

**Output:**
```sh
(-2.0)
(3.0, -2.0)
```
#### 3. Konversi Basis Bilangan dan Operasi Aritmetika

```rust
use sciencecalc_rs::matematika::aritmetika;
use sciencecalc_rs::matematika::basis;

fn main() {
    let x: u64 = 680;
    let y: u64 = 87;
    let a = basis::konversi_basis(aritmetika::tambah(x, y), 2);
    let b = basis::desimal_ke_biner(aritmetika::kali(x, y));
    let c = basis::biner_ke_hexadesimal(
        &basis::desimal_ke_biner(aritmetika::kurang(x, y))
    );
    let d = basis::hexadesimal_ke_oktal(
        &basis::desimal_ke_hexadesimal(aritmetika::bagi(x, y))
    );
    println!("{}\n{}\n{}\n{}", a, b, c, d);
}
```

**Output:**
```sh
1011111111
1110011100011000
251
7
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
  <li><strong>Aritmetika dasar:</strong> tambah, kurang, kali, bagi, pembulatan, modulo, nilai absolut.</li>
  <li><strong>Aljabar:</strong> faktorial, kombinasi, permutasi, sistem persamaan linear (SPL satu, dua, tiga variabel), persamaan kuadrat, operasi matriks 2x2 dan 3x3 (perkalian, invers, determinan, transpose).</li>
  <li><strong>Trigonometri:</strong> sinus, cosinus, tangen, invers, konversi derajat/radian.</li>
  <li><strong>Geometri:</strong> rumus luas dan keliling berbagai bangun datar dan volume bangun ruang.</li>
  <li><strong>Statistika:</strong> rata-rata, median, modus, deviasi standar, operasi data statistik.</li>
  <li><strong>Kombinatorika & Peluang:</strong> perhitungan kombinatorik (faktorial, permutasi, kombinasi), peluang sederhana.</li>
  <li><strong>Konversi basis bilangan:</strong> desimal, biner, oktal, heksadesimal.</li>
  <li><strong>Deret Fibonacci:</strong> rekursif, iteratif, rumus Binet, cek genap dan prima.</li>
  <li><strong>Operasi pada array:</strong> penjumlahan, pengurangan, perkalian, pembagian banyak bilangan sekaligus.</li>
</ul>
</details>

<details>
<summary><strong>Fisika</strong></summary>

<ul>
  <li><strong>Gaya:</strong> perhitungan gaya (F = m × a).</li>
  <li><strong>Energi:</strong> energi kinetik, energi potensial, hukum kekekalan energi.</li>
  <li><strong>Listrik:</strong> hukum Ohm (arus, tegangan, hambatan), daya listrik.</li>
  <li><strong>Gerak:</strong> GLB, GLBB, jarak, kecepatan, percepatan.</li>
  <li><strong>Konversi satuan fisika:</strong> massa, panjang, waktu, energi, dan lain-lain.</li>
  <li><strong>Hukum Newton:</strong> gaya, percepatan, massa, dan penerapannya.</li>
</ul>
</details>

<details>
<summary><strong>Kimia</strong></summary>

<ul>
  <li><strong>Gas:</strong> hukum gas ideal, tekanan, volume, suhu, jumlah mol.</li>
  <li><strong>Reaksi:</strong> massa produk reaksi, persentase hasil reaksi, stoikiometri reaksi.</li>
  <li><strong>Larutan:</strong> konsentrasi, molaritas, pH, volume, massa zat terlarut.</li>
  <li><strong>Stoikiometri:</strong> perhitungan jumlah mol, konversi massa ke mol, dan sebaliknya.</li>
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
