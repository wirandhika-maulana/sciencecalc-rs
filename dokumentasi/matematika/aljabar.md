# Aljabar

Dokumentasi modul [`aljabar`](../src/matematika/aljabar.rs).

```rust
use sciencecalc_rs::matematika::aljabar::*; (mengambil semua fungsi yang ada di dalam struktur file aljabar.rs)
```
Note : Intinya memanggil fungsi ke semua yang ada di dalam file aljabar atau langsung ke fungsi tertentu bisa!

## Sistem Persamaan Linear

### Sistem Persamaan Linear Satu Variabel (SPLSV)

#### Sintaks

```rust
struct Aljabar;

Aljabar::splsv(a: f64, b: f64) -> Option<f64>
        Some(-b / a)
```

Misalkan terdapat persamaan berikut:

*ax + b = 0*

Penyelesaiannya menggunakan rumus:

*x = −b / a*

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar::*;

fn main() {
    let x = Aljabar::splsv(6.0, 12.0);  // x = -b / a
    println!("{}", x.unwrap());
    // x = -12 / 6 = -2
}
```

#### Proses Penyelesaian SPLSV dalam Kode

1. Fungsi `splsv(a, b)` menerima dua parameter, yaitu koefisien *a* dan konstanta *b*.
    ```rust
    pub fn splsv(a: f64, b: f64) -> Option<f64>
    ```
2. Jika *a* bernilai nol, maka persamaan tidak memiliki solusi dan fungsi mengembalikan `None`.
    ```rust
    if a == 0.0 { return None; }
    ```
3. Jika *a* tidak nol, maka solusi dihitung menggunakan rumus *x = -b / a*.
    ```rust
    Some(-b / a)
    ```
4. Hasil dikembalikan dalam bentuk `Some(x)`.

---

### Sistem Persamaan Linear Dua Variabel (SPLDV)

#### Sintaks

```rust
struct Aljabar;

Aljabar::spldv(
    a1: f64, b1: f64, c1: f64,
    a2: f64, b2: f64, c2: f64) -> Option<(f64, f64)>
        Some(x, y)
```

Misalkan terdapat sistem persamaan berikut:

*a₁x + b₁y = c₁*  
*a₂x + b₂y = c₂*

Penyelesaiannya menggunakan metode eliminasi:
*{ a₁x - b₁y = c₁ } . a₂*

*{ a₂x + b₂y = c₂ } . a₁*

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar;

fn main() {
    let hasil = Aljabar::spldv(
        4.0, -3.0, 18.0,  //  a₁x - b₁y = c₁ // Persamaan 1: 4x - 3y = 18
        3.0, 1.0, 7.0     //  a₂x + b₂y = c₂ // Persamaan 2: 3x + 1y = 7
    );
    println!("{:?}", hasil.unwrap());
}
```

Output:
```
(3.0, -2.0)
```

#### Proses Penyelesaian SPLDV dalam Kode

1. Fungsi `spldv(a1, b1, c1, a2, b2, c2)` menerima enam parameter, yaitu koefisien dan konstanta dari dua persamaan.

   ```rust
   pub fn spldv(
       a1: f64, b1: f64, c1: f64,
       a2: f64, b2: f64, c2: f64
   ) -> Option<(f64, f64)> 
   ```
   
3. Jika koefisien *a1* dan *a2* sama, maka persamaan tidak diubah.

   ```rust
   if a1 == a2 {
       new_b1 = b1; new_b2 = b2;
       new_c1 = c1; new_c2 = c2;
   ```
   
4. Jika berbeda, maka kedua persamaan dikalikan dengan koefisien masing-masing agar nilai *x* sama, sehingga dapat dieliminasi.

   ```rust
   new_b1 = b1 * a2; new_b2 = b2 * a1;
   new_c1 = c1 * a2; new_c2 = c2 * a1;
   ```
   
5. Setelah itu, dilakukan operasi penjumlahan atau pengurangan tergantung pada tanda koefisien *a1* dan *a2* untuk mengeliminasi *x*.

   ```rust
   if a1 < 0.00 || a2 < 0.00 {
       b3 = new_b1 + new_b2;
       c3 = new_c1 + new_c2;
   } else {
       b3 = new_b1 - new_b2;
       c3 = new_c1 - new_c2;
   }
   ```
   
6. Nilai *y* dihitung dengan *c3 / b3*.

   ```rust
   let y = c3 / b3;
   ```
   
7. Nilai *x* dihitung dengan mensubstitusi *y* ke dalam salah satu persamaan awal.

   ```rust
   let x = (c2 - (b2 * y)) / a2;
   ```
   
8. Hasil dikembalikan dalam bentuk `Some((x, y))`.

   ```rust
   Some((x, y))
   ```

### Sistem Persamaan Linear Tiga Variabel (SPLTV)

#### Sintaks

```rust
struct Aljabar;

Aljabar::spltv(
    a1: f64, b1: f64, c1: f64, d1: f64,
    a2: f64, b2: f64, c2: f64, d2: f64,
    a3: f64, b3: f64, c3: f64, d3: f64
) -> Option<(f64, f64, f64)>
```

Misalkan terdapat sistem persamaan berikut:

* *2x - y + 3z = 9*
* *x + y - 2z = -2*
* *3x - 2y + 4z = 15*

Penyelesaiannya menggunakan metode determinan:

1. Hitung determinan utama *(det)* menggunakan rumus:

```
det = a₁(b₂c₃ - b₃c₂) - b₁(a₂c₃ - a₃c₂) + c₁(a₂b₃ - a₃b₂)
```

2. Jika determinan *(det)* bernilai nol, maka sistem tidak memiliki solusi atau memiliki solusi tak hingga.

3. Jika determinan tidak nol, lanjutkan dengan menghitung determinan x, y, dan z:

```
det_x = d₁(b₂c₃ - b₃c₂) - b₁(d₂c₃ - d₃c₂) + c₁(d₂b₃ - d₃b₂)
det_y = a₁(d₂c₃ - d₃c₂) - d₁(a₂c₃ - a₃c₂) + c₁(a₂d₃ - a₃d₂)
det_z = a₁(b₂d₃ - b₃d₂) - b₁(a₂d₃ - a₃d₂) + d₁(a₂b₃ - a₃b₂)
```

4. Hitung nilai x, y, dan z:

```
x = det_x / det
y = det_y / det
z = det_z / det
```

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar::*;

fn main() {
    let hasil = Aljabar::spltv(
        2.0, -1.0, 3.0, 9.0,  // Persamaan 1
        1.0, 1.0, -2.0, -2.0, // Persamaan 2
        3.0, -2.0, 4.0, 15.0  // Persamaan 3
    );
    println!("{:?}", hasil.unwrap());
}
```

Output:

```sh
(1.0, 2.0, 1.0)
```

#### Proses Penyelesaian SPLTV dalam Kode
1. Fungsi `spltv` menerima 12 parameter yang merepresentasikan koefisien dan konstanta dari tiga persamaan linear.

```rust
pub fn spltv(
    a1: f64, b1: f64, c1: f64, d1: f64,
    a2: f64, b2: f64, c2: f64, d2: f64,
    a3: f64, b3: f64, c3: f64, d3: f64
) -> Option<(f64, f64, f64)>
```

2. Hitung determinan utama *(det)* menggunakan rumus determinan matriks 3x3.

```rust
let det = a1 * (b2 * c3 - b3 * c2)
        - b1 * (a2 * c3 - a3 * c2)
        + c1 * (a2 * b3 - a3 * b2);

if det == 0.0 { return None; }
```

3. Hitung determinan untuk variabel *x*, *y*, dan *z*.

```rust
let det_x = d1 * (b2 * c3 - b3 * c2)
          - b1 * (d2 * c3 - d3 * c2)
          + c1 * (d2 * b3 - d3 * b2);

let det_y = a1 * (d2 * c3 - d3 * c2)
          - d1 * (a2 * c3 - a3 * c2)
          + c1 * (a2 * d3 - a3 * d2);

let det_z = a1 * (b2 * d3 - b3 * d2)
          - b1 * (a2 * d3 - a3 * d2)
          + d1 * (a2 * b3 - a3 * b2);
```

4. Hitung hasil akhir untuk *x*, *y*, dan *z*.

```rust
let x = det_x / det;
let y = det_y / det;
let z = det_z / det;
```

5. Kembalikan hasil sebagai tuple `(x, y, z)`.

```rust
Some((x, y, z))
```

---

# Kuadrat dan Matriks

Dokumentasi modul [`aljabar`](../src/matematika/aljabar.rs).

## Persamaan Kuadrat

### Sintaks

```rust
struct Aljabar;

Aljabar::kuadrat(a: f64, b: f64, c: f64) -> Option<(f64, f64)>
```

Misalkan terdapat persamaan kuadrat:

*a*x² + b*x + c = 0*

Penyelesaiannya menggunakan rumus diskriminan:

*x₁ = (-b + √(b² - 4ac)) / 2a*  
*x₂ = (-b - √(b² - 4ac)) / 2a*

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar::*;

fn main() {
    let akar = Aljabar::kuadrat(1.0, -3.0, 2.0);
    println!("{:?}", akar.unwrap());
    // Output: (2.0, 1.0)
}
```

#### Proses Penyelesaian Persamaan Kuadrat

1. Fungsi `kuadrat(a, b, c)` menerima tiga parameter, yaitu koefisien a, b, dan c.
2. Hitung diskriminan:  
   ```rust
   let discriminant = b * b - 4.0 * a * c;
   ```
3. Jika a == 0 atau diskriminan < 0, tidak ada solusi real.
4. Hitung akar-akarnya:  
   ```rust
   let sqrt_disc = discriminant.sqrt();
   let x1 = (-b + sqrt_disc) / (2.0 * a);
   let x2 = (-b - sqrt_disc) / (2.0 * a);
   ```
5. Kembalikan `Some((x1, x2))`.

---

## Operasi Matriks

### Determinan Matriks 2x2

#### Sintaks

```rust
stuct Aljabar;

Aljabar::determinant_2x2(a: f64, b: f64, c: f64, d: f64) -> f64
```

Persamaan:  
| a  b |  
| c  d |  
Determinannya: `ad - bc`

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar::*;

fn main() {
    let det = Aljabar::determinant_2x2(2.0, 3.0, 1.0, 4.0);
    println!("{}", det); // Output: 5.0
}
```

---

### Perkalian Matriks 2x2

#### Sintaks

```rust
struct Aljabar;

Aljabar::matriks_2x2(m1: [[f64; 2]; 2], m2: [[f64; 2]; 2]) -> [[f64; 2]; 2]
```

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar::*;

fn main() {
    let a = [[1.0, 2.0], [3.0, 4.0]];
    let b = [[2.0, 0.0], [1.0, 2.0]];
    let hasil = Aljabar::matriks_2x2(a, b);
    println!("{:?}", hasil); // Output: [[4.0, 4.0], [10.0, 8.0]]
}
```

---

### Invers Matriks 2x2

#### Sintaks

```rust
struct Aljabar;

Aljabar::inverse_2x2(a: f64, b: f64, c: f64, d: f64) -> Option<[[f64; 2]; 2]>
```

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar::*;

fn main() {
    let invers = Aljabar::inverse_2x2(2.0, 1.0, 1.0, 2.0).unwrap();
    println!("{:?}", invers);
    // Output: [[0.666..., -0.333...], [-0.333..., 0.666...]]
}
```

---

### Determinan Matriks 3x3

#### Sintaks

```rust
struct Aljabar;

Aljabar::determinant_3x3(m: [[f64; 3]; 3]) -> f64
```

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar::*;

fn main() {
    let m = [
        [1.0, 2.0, 3.0],
        [0.0, 4.0, 5.0],
        [1.0, 0.0, 6.0]
    ];
    let det = Aljabar::determinant_3x3(m);
    println!("{}", det); // Output: 22.0
}
```

---

### Perkalian Matriks 3x3

#### Sintaks

```rust
struct Aljabar;

Aljabar::matriks_3x3(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3]
```

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar::*;

fn main() {
    let a = [
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        [7.0, 8.0, 9.0]
    ];
    let b = [
        [9.0, 8.0, 7.0],
        [6.0, 5.0, 4.0],
        [3.0, 2.0, 1.0]
    ];
    let hasil = Aljabar::matriks_3x3(a, b);
    println!("{:?}", hasil);
}
```

---

### Invers Matriks 3x3

#### Sintaks

```rust
struct Aljabar;

Aljabar::inverse_3x3(m: [[f64; 3]; 3]) -> Option<[[f64; 3]; 3]>
```

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar::*;

fn main() {
    let m = [
        [2.0, 1.0, 1.0],
        [1.0, 3.0, 2.0],
        [1.0, 0.0, 0.0]
    ];
    let invers = Aljabar::inverse_3x3(m).unwrap();
    println!("{:?}", invers);
}
```

---

## Referensi

- [Source Code](../src/matematika/aljabar.rs)
- [Rust Documentation](https://doc.rust-lang.org/)

---
