# sciencecalc-rs - Modul Aritmetika

Selamat datang di dokumentasi **sciencecalc-rs** khusus modul **aritmetika**.

---

## Instalasi

Tambahkan ke proyek Cargo Anda:

```sh
cargo add sciencecalc-rs
```

Atau, tambahkan manual pada bagian `[dependencies]` di `Cargo.toml`:

```toml
sciencecalc-rs = "<VersiCrate>"
```

---

## Penggunaan

Impor modul aritmetika:

```rust
use sciencecalc_rs::matematika::aritmetika::*; (mengambil semua fungsi yang ada di dalam struktur file aritmetika.rs)
```
Note : Intinya memanggil fungsi ke semua yang ada di dalam file aljabar atau langsung ke fungsi tertentu bisa!

---

## Fungsi Dasar

| Fungsi           | Parameter           | Return      | Penjelasan                            |
|------------------|--------------------|-------------|---------------------------------------|
| `tambah`         | `a, b`             | Nilai       | Penjumlahan dua bilangan              |
| `kurang`         | `a, b`             | Nilai       | Pengurangan dua bilangan              |
| `kali`           | `a, b`             | Nilai       | Perkalian dua bilangan                |
| `bagi`           | `a, b`             | Nilai       | Pembagian dua bilangan                |
| `modulo`         | `a, b`             | Nilai       | Sisa bagi dua bilangan                |
| `bulat`          | `a: f64`           | `f64`       | Membulatkan ke atas/bawah terdekat    |
| `absolut`        | `x: f64`           | `f64`       | Nilai mutlak dari x                   |

**Contoh:**
```rust
fn main() {
    println!("{}", tambah(10, 5));    // 15
    println!("{}", kurang(10, 5));    // 5
    println!("{}", kali(10, 5));      // 50
    println!("{}", bagi(10.0, 2.0));  // 5.0
    println!("{}", modulo(10, 3));    // 1
    println!("{}", bulat(4.7));       // 5.0
    println!("{}", absolut(-8.5));    // 8.5
}
```

---

## Fungsi Pangkat dan Akar

| Fungsi                 | Parameter                | Return    | Penjelasan               |
|------------------------|-------------------------|-----------|--------------------------|
| `pangkat`              | `base, exp: u32`        | Nilai     | Pangkat bilangan bulat   |
| `pangkat_optim`        | `base, exp: u32`        | Nilai     | Pangkat metode optimasi  |
| `pangkat_desimal`      | `base: f64, exp: f64`   | `f64`     | Pangkat desimal          |
| `akar_kuadrat`         | `x: f64`                | `f64`     | Akar kuadrat (Newton)    |
| `akar_pangkat_n`       | `x: f64, n: f64`        | `f64`     | Akar pangkat-n           |
| `logaritma`            | `x: f64, base: f64`     | `f64`     | Logaritma basis tertentu |

**Contoh:**
```rust
fn main() {
    println!("{}", pangkat(2, 5));         // 32
    println!("{}", pangkat_optim(2, 10));  // 1024
    println!("{}", pangkat_desimal(2.0, 2.5)); // ~5.656
    println!("{}", akar_kuadrat(9.0));     // 3.0
    println!("{}", akar_pangkat_n(27.0, 3.0)); // 3.0
    println!("{}", logaritma(8.0, 2.0));   // 3.0
}
```

---

## Fungsi Genap & Ganjil

| Fungsi         | Parameter                | Return  | Penjelasan                            |
|----------------|-------------------------|---------|---------------------------------------|
| `genap`        | `"cek"/"rubah", a: i64` | `i64`   | Cek atau ubah bilangan jadi genap     |
| `ganjil`       | `"cek"/"rubah", a: i64` | `i64`   | Cek atau ubah bilangan jadi ganjil    |

**Contoh:**
```rust
fn main() {
    genap("cek", 7);       // Output: 7 bukanlah bilangan genap.
    let x = genap("rubah", 7); // Output: 8
    println!("{}", x);     // 8

    ganjil("cek", 8);      // Output: 8 bukanlah bilangan ganjil.
    let y = ganjil("rubah", 8); // Output: 9
    println!("{}", y);     // 9
}
```

---

## FPB & KPK

| Fungsi   | Parameter      | Return | Penjelasan                       |
|----------|---------------|--------|----------------------------------|
| `fpb`    | `a, b`        | Nilai  | Faktor persekutuan terbesar      |
| `kpk`    | `a, b`        | Nilai  | Kelipatan persekutuan terkecil   |

**Contoh:**
```rust
fn main() {
    println!("{}", fpb(12, 18)); // 6
    println!("{}", kpk(4, 6));   // 12
}
```

---

## Fungsi Array "Super"

| Fungsi                | Parameter                 | Return    | Penjelasan                                 |
|-----------------------|--------------------------|-----------|--------------------------------------------|
| `super_tambah`        | `&[T]`                   | Nilai     | Penjumlahan seluruh elemen                 |
| `super_kurang`        | `&[T]`                   | Nilai     | Pengurangan berurutan seluruh elemen       |
| `super_kali`          | `&[T]`                   | Nilai     | Perkalian seluruh elemen                   |
| `super_bagi`          | `&[T]`                   | Nilai     | Pembagian berurutan seluruh elemen         |
| `super_genap`         | `"cek"/"rubah", &mut [i64]` | -      | Cek/ubah semua elemen array jadi genap     |
| `super_ganjil`        | `"cek"/"rubah", &mut [i64]` | -      | Cek/ubah semua elemen array jadi ganjil    |
| `super_absolut`       | `&[f64]`                 | `Vec<f64>`| Array nilai mutlak                         |
| `super_logaritma`     | `&[f64], base: f64`      | `Vec<f64>`| Logaritma tiap elemen                      |

**Contoh:**
```rust
fn main() {
    let arr = [2, 3, 4];
    println!("{}", super_tambah(&arr)); // 9

    let mut arr = [3, 6, 9];
    super_genap("rubah", &mut arr);
    println!("{:?}", arr); // [4, 6, 10]

    let v = [2.0, -3.0, 4.0];
    println!("{:?}", super_absolut(&v)); // [2.0, 3.0, 4.0]

    let vf = [16.0, 32.0];
    println!("{:?}", super_logaritma(&vf, 2.0)); // [4.0, 5.0]
}
```

---

## Deret Fibonacci

Struktur & metode untuk menghitung dan memeriksa deret Fibonacci.

| Fungsi              | Parameter    | Return      | Penjelasan                      |
|---------------------|-------------|-------------|---------------------------------|
| `rekursif`          | `n: u64`    | `u64`       | Fibonacci ke-n (rekursif)       |
| `iteratif`          | `n: u64`    | `u64`       | Fibonacci ke-n (iteratif)       |
| `binet`             | `n: u64`    | `u64`       | Fibonacci ke-n (rumus Binet)    |
| `adalah_genap`      | `n: u64`    | `bool`      | Apakah Fibonacci ke-n genap     |
| `adalah_prima`      | `n: u64`    | `bool`      | Apakah Fibonacci ke-n prima     |

**Contoh:**
```rust
use sciencecalc_rs::matematika::aritmetika::*;

fn main() {
    println!("{}", Fibonacci::rekursif(6));           // 8
    println!("{}", Fibonacci::iteratif(10));          // 55
    println!("{}", Fibonacci::binet(10));             // 55
    println!("{}", Fibonacci::adalah_genap(10));      // true
    println!("{}", Fibonacci::adalah_prima(10));      // false
}
```

---

## Penutup

semoga ngertii gaess

---

