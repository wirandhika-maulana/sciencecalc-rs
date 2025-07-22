# Aljabar

Dokumentasi modul [`aljabar`](../src/matematika/aljabar.rs).

## Sistem Persamaan Linear

### Sistem Persamaan Linear Satu Variabel (SPLSV)

#### Sintaks

```rust
splsv(a: f64, b: f64) -> Option<f64>
```

Misalkan terdapat persamaan berikut:

*ax + b = 0*

Penyelesaiannya menggunakan rumus:

*x = −b / a*

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar;

fn main() {
    let x = aljabar::splsv(6.0, 12.0);  // x = -b / a
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
spldv(
    a1: f64, b1: f64, c1: f64,
    a2: f64, b2: f64, c2: f64
) -> Option<(f64, f64)>
```

Misalkan terdapat sistem persamaan berikut:

*a₁x + b₁y = c₁*  
*a₂x + b₂y = c₂*

Penyelesaiannya menggunakan metode determinan:

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar;

fn main() {
    let hasil = aljabar::spldv(
        4.0, -3.0, 18.0,  // Persamaan 1: 4x - 3y = 18
        3.0, 1.0, 7.0     // Persamaan 2: 3x + 1y = 7
    );
    println!("{:?}", hasil.unwrap());
}
```

Output:
```
(3.0, -2.0)
```

#### Proses Penyelesaian SPLDV dalam Kode

1. Fungsi `spldv(a1, b1, c1, a2, b2, c2)` menerima enam parameter koefisien dan konstanta dari dua persamaan.
    ```rust
    pub fn spldv(
        a1: f64, b1: f64, c1: f64,
        a2: f64, b2: f64, c2: f64
    ) -> Option<(f64, f64)>
    ```
2. Hitung determinan utama:
    ```rust
    let det = a1 * b2 - a2 * b1;
    if det == 0.0 { return None; }
    ```
3. Hitung x dan y:
    ```rust
    let x = (c1 * b2 - c2 * b1) / det;
    let y = (a1 * c2 - a2 * c1) / det;
    ```
4. Hasil dikembalikan sebagai tuple `Some((x, y))`.

---

### Sistem Persamaan Linear Tiga Variabel (SPLTV)

#### Sintaks

```rust
spltv(
    a1: f64, b1: f64, c1: f64, d1: f64,
    a2: f64, b2: f64, c2: f64, d2: f64,
    a3: f64, b3: f64, c3: f64, d3: f64
) -> Option<(f64, f64, f64)>
```

Misalkan terdapat sistem persamaan berikut:

*2x - y + 3z = 9*  
*x + y - 2z = -2*  
*3x - 2y + 4z = 15*

Penyelesaiannya menggunakan metode determinan:

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar;

fn main() {
    let hasil = aljabar::spltv(
        2.0, -1.0, 3.0, 9.0,    // Persamaan 1
        1.0, 1.0, -2.0, -2.0,   // Persamaan 2
        3.0, -2.0, 4.0, 15.0    // Persamaan 3
    );
    println!("{:?}", hasil.unwrap());
}
```

Output:
```
(1.0, 2.0, 1.0)
```

#### Proses Penyelesaian SPLTV dalam Kode

1. Fungsi `spltv` menerima 12 parameter koefisien dan konstanta dari tiga persamaan linear.
    ```rust
    pub fn spltv(
        a1: f64, b1: f64, c1: f64, d1: f64,
        a2: f64, b2: f64, c2: f64, d2: f64,
        a3: f64, b3: f64, c3: f64, d3: f64
    ) -> Option<(f64, f64, f64)>
    ```
2. Hitung determinan utama (det):
    ```rust
    let det = a1 * (b2 * c3 - b3 * c2)
            - b1 * (a2 * c3 - a3 * c2)
            + c1 * (a2 * b3 - a3 * b2);

    if det == 0.0 { return None; }
    ```
3. Hitung determinan untuk variabel x, y, dan z:
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
4. Hitung hasil akhir untuk x, y, dan z:
    ```rust
    let x = det_x / det;
    let y = det_y / det;
    let z = det_z / det;
    ```
5. Kembalikan hasil sebagai tuple `(x, y, z)`:
    ```rust
    Some((x, y, z))
    ```

---

## Fungsi Aljabar Lainnya

### Faktorial

```rust
let hasil = aljabar::factorial(5); // Output: 120
```

### Kombinasi

```rust
let hasil = aljabar::combination(5, 2); // Output: 10
```

### Permutasi

```rust
let hasil = aljabar::permutation(5, 2); // Output: 20
```

---

# Kuadrat dan Matriks

Dokumentasi modul [`aljabar`](../src/math/aljabar.rs).

## Persamaan Kuadrat

### Sintaks

```rust
quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)>
```

Misalkan terdapat persamaan kuadrat:

*a*x² + b*x + c = 0*

Penyelesaiannya menggunakan rumus diskriminan:

*x₁ = (-b + √(b² - 4ac)) / 2a*  
*x₂ = (-b - √(b² - 4ac)) / 2a*

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar;

fn main() {
    let akar = aljabar::quadratic(1.0, -3.0, 2.0);
    println!("{:?}", akar.unwrap());
    // Output: (2.0, 1.0)
}
```

#### Proses Penyelesaian Persamaan Kuadrat

1. Fungsi `quadratic(a, b, c)` menerima tiga parameter, yaitu koefisien a, b, dan c.
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
determinant_2x2(a: f64, b: f64, c: f64, d: f64) -> f64
```

Persamaan:  
| a  b |  
| c  d |  
Determinannya: `ad - bc`

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar;

fn main() {
    let det = aljabar::determinant_2x2(2.0, 3.0, 1.0, 4.0);
    println!("{}", det); // Output: 5.0
}
```

---

### Perkalian Matriks 2x2

#### Sintaks

```rust
multiply_2x2(m1: [[f64; 2]; 2], m2: [[f64; 2]; 2]) -> [[f64; 2]; 2]
```

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar;

fn main() {
    let a = [[1.0, 2.0], [3.0, 4.0]];
    let b = [[2.0, 0.0], [1.0, 2.0]];
    let hasil = aljabar::multiply_2x2(a, b);
    println!("{:?}", hasil); // Output: [[4.0, 4.0], [10.0, 8.0]]
}
```

---

### Invers Matriks 2x2

#### Sintaks

```rust
inverse_2x2(a: f64, b: f64, c: f64, d: f64) -> Option<[[f64; 2]; 2]>
```

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar;

fn main() {
    let invers = aljabar::inverse_2x2(2.0, 1.0, 1.0, 2.0).unwrap();
    println!("{:?}", invers);
    // Output: [[0.666..., -0.333...], [-0.333..., 0.666...]]
}
```

---

### Determinan Matriks 3x3

#### Sintaks

```rust
determinant_3x3(m: [[f64; 3]; 3]) -> f64
```

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar;

fn main() {
    let m = [
        [1.0, 2.0, 3.0],
        [0.0, 4.0, 5.0],
        [1.0, 0.0, 6.0]
    ];
    let det = aljabar::determinant_3x3(m);
    println!("{}", det); // Output: 22.0
}
```

---

### Perkalian Matriks 3x3

#### Sintaks

```rust
multiply_3x3(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3]
```

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar;

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
    let hasil = aljabar::multiply_3x3(a, b);
    println!("{:?}", hasil);
}
```

---

### Invers Matriks 3x3

#### Sintaks

```rust
inverse_3x3(m: [[f64; 3]; 3]) -> Option<[[f64; 3]; 3]>
```

#### Implementasi dalam Kode

```rust
use sciencecalc_rs::matematika::aljabar;

fn main() {
    let m = [
        [2.0, 1.0, 1.0],
        [1.0, 3.0, 2.0],
        [1.0, 0.0, 0.0]
    ];
    let invers = aljabar::inverse_3x3(m).unwrap();
    println!("{:?}", invers);
}
```

---

## Referensi

- [Source Code](../src/matematika/aljabar.rs)
- [Rust Documentation](https://doc.rust-lang.org/)

---
