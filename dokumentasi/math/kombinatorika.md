# Kombinatorika

Dokumentasi modul [`kombinatorika.rs`](../src/math/kombinatorika.rs) pada pustaka **sciencecalc-rs**.

---

## 📍 **Fungsi Dasar**

Fungsi dasar untuk menghitung **faktorial**, **kombinasi**, dan **permutasi**.

| Fungsi                                      | Deskripsi                                                                            |
|----------------------------------------------|--------------------------------------------------------------------------------------|
| `faktorial(n: u64) -> u64`                  | Menghitung faktorial dari `n` (\(n!\))                                               |
| `kombinasi(n: u64, k: u64) -> u64`          | Kombinasi tanpa pengulangan \(C(n, k) = \frac{n!}{k!(n-k)!}\)                        |
| `permutasi(n: u64, r: u64) -> u64`          | Permutasi tanpa pengulangan \(P(n, r) = \frac{n!}{(n-r)!}\)                          |
| `kombinasi_perulangan(n: u64, r: u64) -> u64` | Kombinasi dengan pengulangan \(C'(n, r) = \frac{(n+r-1)!}{r!(n-1)!}\)              |
| `permutasi_perulangan(n: u64, &[u64]) -> u64` | Permutasi dengan pengulangan \(P(n; n_1, n_2, ...) = \frac{n!}{n_1! n_2! ...}\)     |

**Contoh:**
```rust
use sciencecalc_rs::math::kombinatorika::{faktorial, kombinasi, permutasi};

fn main() {
    let n = 5;
    let k = 2;

    println!("Faktorial dari {} adalah: {}", n, faktorial(n));
    println!("Kombinasi C({}, {}) adalah: {}", n, k, kombinasi(n, k));
    println!("Permutasi P({}, {}) adalah: {}", n, k, permutasi(n, k));
}
```

Output:
```
Faktorial dari 5 adalah: 120
Kombinasi C(5, 2) adalah: 10
Permutasi P(5, 2) adalah: 20
```

---

## 📍 **Peluang**

Sub-modul `Peluang` berisi implementasi peluang menggunakan **dadu, koin, dan kantong kelereng**.

### 🎲 Dadu

Mewakili dadu standar (angka 1-6) dan peluangnya.

| Struktur  | Metode                 | Parameter         | Return  | Deskripsi                                           |
|-----------|------------------------|-------------------|---------|-----------------------------------------------------|
| `Dadu`    | `new()`                | -                 | Dadu    | Membuat objek dadu baru dengan angka 1-6            |
|           | `muncul_angka(target)` | `target: i64`     | f64     | Peluang muncul angka tertentu                       |
|           | `muncul_genap()`       | -                 | f64     | Peluang muncul angka genap                          |
|           | `muncul_lebih_dari()`  | `batas: i64`      | f64     | Peluang muncul angka lebih dari batas               |
|           | `muncul_kurang_dari()` | `batas: i64`      | f64     | Peluang muncul angka kurang dari batas              |

**Contoh:**
```rust
use sciencecalc_rs::math::kombinatorika::Peluang::Dadu;

fn main() {
    let dadu = Dadu::new();
    println!("Peluang muncul angka 3: {}", dadu.muncul_angka(3));
    println!("Peluang muncul angka genap: {}", dadu.muncul_genap());
    println!("Peluang angka > 4: {}", dadu.muncul_lebih_dari(4));
    println!("Peluang angka < 3: {}", dadu.muncul_kurang_dari(3));
}
```

---

### 🪙 Koin

Mewakili koin dua sisi (Angka = 'A', Gambar = 'G').

| Struktur  | Metode                       | Parameter                      | Return | Deskripsi                                      |
|-----------|------------------------------|--------------------------------|--------|------------------------------------------------|
| `Koin`    | `new()`                      | -                              | Koin   | Membuat objek koin baru                        |
|           | `muncul(target)`             | `target: char`                 | f64    | Peluang muncul sisi tertentu                   |
|           | `muncul_beruntun(target, j)` | `target: char, jumlah: u32`    | f64    | Peluang muncul sisi tertentu beruntun          |
|           | `muncul_setidaknya_satu()`   | `target: char, jumlah: u32`    | f64    | Peluang muncul setidaknya satu sisi tertentu   |

**Contoh:**
```rust
use sciencecalc_rs::math::kombinatorika::Peluang::Koin;

fn main() {
    let koin = Koin::new();
    println!("Peluang muncul Angka: {}", koin.muncul('A'));
    println!("Peluang Angka 3x beruntun: {}", koin.muncul_beruntun('A', 3));
    println!("Peluang Angka setidaknya sekali dalam 3 lemparan: {}", koin.muncul_setidaknya_satu('A', 3));
}
```

---

### 🔴⚪ Kantong Kelereng

Mewakili kantong kelereng dengan dua warna (Merah = 'M', Putih = 'P').

| Struktur           | Metode                         | Parameter                             | Return | Deskripsi                                         |
|--------------------|--------------------------------|---------------------------------------|--------|---------------------------------------------------|
| `KantongKelereng`  | `new(merah, putih)`            | merah: u32, putih: u32                | KantongKelereng | Membuat objek kantong kelereng baru          |
|                    | `muncul_satu(warna)`           | warna: char                           | f64    | Peluang mengambil satu kelereng warna tertentu    |
|                    | `muncul_dua_berurutan(w1, w2)` | warna_pertama: char, warna_kedua: char| f64    | Peluang dua kelereng warna tertentu berurutan     |

**Contoh:**
```rust
use sciencecalc_rs::math::kombinatorika::Peluang::KantongKelereng;

fn main() {
    let kantong = KantongKelereng::new(5, 7);
    println!("Peluang kelereng merah: {}", kantong.muncul_satu('M'));
    println!("Peluang merah-ikuti-putih: {}", kantong.muncul_dua_berurutan('M', 'P'));
}
```

---

Dokumentasi ini mencakup seluruh fungsi dan struktur kombinatorika serta peluang pada pustaka **sciencecalc-rs**.  
Semoga bermanfaat!

---
