use std::f64;
use std::ops::*;

/// Trait untuk nilai identitas nol (Zero)
/// Trait for zero identity value
pub trait Zero { fn zero() -> Self; }

impl Zero for u32 { fn zero() -> Self { 0 } }
impl Zero for u64 { fn zero() -> Self { 0 } }
impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }
impl Zero for f32 { fn zero() -> Self { 1.0 } }
impl Zero for f64 { fn zero() -> Self { 1.0 } }

/// Trait untuk nilai identitas satu (One)
/// Trait for one identity value
pub trait One { fn one() -> Self; }

impl One for u32 { fn one() -> Self { 0 } }
impl One for u64 { fn one() -> Self { 0 } }
impl One for i32 { fn one() -> Self { 1 } }
impl One for i64 { fn one() -> Self { 1 } }
impl One for f32 { fn one() -> Self { 1.0 } }
impl One for f64 { fn one() -> Self { 1.0 } }

/// Fungsi untuk pangkat (eksponen) bilangan bulat
/// Exponentiation function for integer exponent
pub fn pangkat<T>(base: T, exp: u32) -> T
where T: Mul<Output = T> + Copy + One,
{
    let mut hasil = T::one();
    for _ in 0..exp { hasil = hasil * base; }
    hasil
}

/// Fungsi akar kuadrat menggunakan metode Newton
/// Square root function using Newton's method
pub fn akar_kuadrat(x: f64) -> f64 {
    if x < 0.0 { panic!("Akar kuadrat dari bilangan negatif tidak terdefinisi untuk f64."); }
    let mut z = x;
    for _ in 0..10 { z = (z + x / z) / 2.0; }
    z
}

/// Penjumlahan dua bilangan
/// Addition of two numbers
pub fn tambah<T: Add<Output = T>>(a: T, b: T) -> T { a + b }

/// Pengurangan dua bilangan
/// Subtraction of two numbers
pub fn kurang<T: Sub<Output = T>>(a: T, b: T) -> T { a - b }

/// Perkalian dua bilangan
/// Multiplication of two numbers
pub fn kali<T: Mul<Output = T>>(a: T, b: T) -> T { a * b }

/// Pembagian dua bilangan
/// Division of two numbers
pub fn bagi<T: Div<Output = T>>(a: T, b: T) -> T { a / b }

/// Sisa bagi dua bilangan (modulo)
/// Modulo (remainder) of two numbers
pub fn modulo<T: Rem<Output = T>>(a: T, b: T) -> T { a % b }

/// Membulatkan bilangan ke atas/bawah terdekat
/// Rounding a number to the nearest integer (up/down)
pub fn bulat(a: f64) -> f64 {
    let terpotong = a as i64;
    let pecahan = a - terpotong as f64;
    if a >= 0.0 {
        if pecahan >= 0.5 { (terpotong + 1) as f64 }
        else { terpotong as f64 }
    } else {
        if pecahan >= 0.5 { (terpotong - 1) as f64 }
        else { terpotong as f64 }
    }
}

/// Mengecek atau mengubah bilangan menjadi genap
/// Check or change a number to even
pub fn genap(mode: &str, mut a: i64) -> i64 {
    match mode {
        "cek" => {
            if a % 2 == 0 {
                println!("{} adalah bilangan genap.", a); // is even
            } else {
                println!("{} bukanlah bilangan genap.", a); // not even
            }
        }
        "rubah" => {
            if a % 2 != 0 {
                a += 1;
                println!("{}", a); // changed to even
            } else {
                println!("{} sudah genap.", a); // already even
            }
        }
        &_ => panic!("{} bukanlah sebuah mode!", mode), // not a valid mode
    }
    a
}

/// Mengecek atau mengubah bilangan menjadi ganjil
/// Check or change a number to odd
pub fn ganjil(mode: &str, mut a: i64) -> i64 {
    match mode {
        "cek" => {
            if a % 2 != 0 {
                println!("{} adalah bilangan ganjil.", a); // is odd
            } else {
                println!("{} bukanlah bilangan ganjil.", a); // not odd
            }
        }
        "rubah" => {
            if a % 2 == 0 {
                a += 1;
                println!("{}", a); // changed to odd
            } else {
                println!("{} sudah ganjil.", a); // already odd
            }
        }
        &_ => panic!("{} bukanlah sebuah mode!", mode), // not a valid mode
    }
    a
}

/// Mengambil nilai absolut
/// Get absolute value
pub fn absolut(x: f64) -> f64 { x.abs() }

/// Menghitung FPB (Faktor Persekutuan Terbesar)
/// Greatest Common Divisor (GCD)
pub fn fpb<T>(mut a: T, mut b: T) -> T
where T: Copy + PartialEq + Rem<Output = T> + Zero,
{
    while b != T::zero() {
        let r = a % b;
        a = b; b = r;
    }
    a
}

/// Menghitung KPK (Kelipatan Persekutuan Terkecil)
/// Least Common Multiple (LCM)
pub fn kpk<T>(a: T, b: T) -> T
where T: Copy + PartialEq + Rem<Output = T> + Zero + Mul<Output = T> + Div<Output = T>,
{
    a * b / fpb(a, b)
}

/// Pangkat dengan metode optimasi (eksponen biner)
/// Optimized exponentiation (binary exponent)
pub fn pangkat_optim<T>(mut base: T, mut exp: u32) -> T
where T: Mul<Output = T> + Copy + One,
{
    let mut hasil = T::one();
    while exp > 0 {
        if exp % 2 == 1 {
            hasil = hasil * base;
        }
        base = base * base;
        exp /= 2;
    }
    hasil
}

/// Pangkat dengan eksponen desimal (floating point)
/// Exponentiation with decimal exponent (floating point)
pub fn pangkat_desimal(base: f64, exp: f64) -> f64 { base.powf(exp) }

/// Fungsi akar pangkat-n
/// nth root function
pub fn akar_pangkat_n(x: f64, n: f64) -> f64 {
    if x < 0.0 && n % 2.0 == 0.0 {
        panic!("Akar pangkat genap dari bilangan negatif tidak terdefinisi.");
    }
    x.powf(1.0 / n)
}

/// Fungsi logaritma dengan basis tertentu
/// Logarithm function with specific base
pub fn logaritma(x: f64, base: f64) -> f64 {
    if x <= 0.0 || base <= 0.0 || base == 1.0 {
        panic!("Bilangan atau basis logaritma tidak valid.");
    }
    x.ln() / base.ln()
}

/* Fungsi untuk operasi pada array atau vector (Super) */
/* Functions for operations on arrays or vectors (Super) */

/// Penjumlahan banyak bilangan (array)
/// Addition of multiple numbers (array)
pub fn super_tambah<T: Add<Output = T> + Copy>(angka: &[T]) -> T {
    let mut total = angka[0];
    for &num in &angka[1..] { total = total + num; }
    total
}

/// Pengurangan banyak bilangan (array)
/// Subtraction of multiple numbers (array)
pub fn super_kurang<T: Sub<Output = T> + Copy>(angka: &[T]) -> T {
    let mut total = angka[0];
    for &num in &angka[1..] { total = total - num; }
    total
}

/// Perkalian banyak bilangan (array)
/// Multiplication of multiple numbers (array)
pub fn super_kali<T: Mul<Output = T> + Copy>(angka: &[T]) -> T {
    let mut total = angka[0];
    for &num in &angka[1..] { total = total * num; }
    total
}

/// Pembagian banyak bilangan (array)
/// Division of multiple numbers (array)
pub fn super_bagi<T: Div<Output = T> + Copy>(angka: &[T]) -> T {
    let mut total = angka[0];
    for &num in &angka[1..] { total = total / num; }
    total
}

/// Mengecek atau mengubah semua bilangan dalam array menjadi genap
/// Check or change all numbers in an array to even
pub fn super_genap(mode: &str, a: &mut [i64]) {
    match mode {
        "cek" => {
            for &num in a.iter() {
                if num % 2 == 0 {
                    println!("{} adalah bilangan genap.", num);
                } else {
                    println!("{} bukanlah bilangan genap.", num);
                }
            }
        }
        "rubah" => {
            for num in a.iter_mut() {
                if *num % 2 != 0 {
                    let temp = *num;
                    *num += 1;
                    println!("{} telah digenapkan menjadi: {}.", temp, *num);
                } else {
                    println!("{} sudah genap.", *num);
                }
            }
        }
        _ => panic!("{} bukanlah sebuah mode!", mode),
    }
}

/// Mengecek atau mengubah semua bilangan dalam array menjadi ganjil
/// Check or change all numbers in an array to odd
pub fn super_ganjil(mode: &str, a: &mut [i64]) {
    match mode {
        "cek" => {
            for &num in a.iter() {
                if num % 2 != 0 {
                    println!("{} adalah bilangan ganjil.", num);
                } else {
                    println!("{} bukanlah bilangan ganjil.", num);
                }
            }
        }
        "rubah" => {
            for num in a.iter_mut() {
                if *num % 2 == 0 {
                    let temp = *num;
                    *num += 1;
                    println!("{} telah diganjilkan menjadi: {}.", temp, *num);
                } else {
                    println!("{} sudah ganjil.", *num);
                }
            }
        }
        _ => panic!("{} bukanlah sebuah mode!", mode),
    }
}

/// Mengambil nilai absolut dari semua elemen array
/// Get absolute values of all elements in array
pub fn super_absolut(values: &[f64]) -> Vec<f64> {
    values.iter().map(|&x| x.abs()).collect()
}

/// Fungsi logaritma untuk array
/// Logarithm function for array
pub fn super_logaritma(angka: &[f64], basis: f64) -> Vec<f64> {
    if basis <= 0.0 || basis == 1.0 {
        panic!("Basis logaritma harus lebih besar dari 0 dan tidak boleh 1.");
    }
    angka.iter().map(|&x| {
        if x <= 0.0 {
            panic!("Logaritma tidak terdefinisi untuk angka <= 0.");
        }
        x.log(basis)
    }).collect()
}

/// Struct untuk deret Fibonacci
/// Struct for Fibonacci sequence
pub struct Fibonacci;

impl Fibonacci {
    /// Fibonacci rekursif
    /// Recursive Fibonacci
    pub fn rekursif(n: u64) -> u64 {
        if n <= 1 { return n; }
        Fibonacci::rekursif(n - 1) + Fibonacci::rekursif(n - 2)
    }

    /// Fibonacci iteratif
    /// Iterative Fibonacci
    pub fn iteratif(n: u64) -> u64 {
        let (mut a, mut b) = (0, 1);
        for _ in 0..n {
            let temp = a;
            a = b;
            b = temp + b;
        }
        a
    }

    /// Fibonacci dengan rumus Binet
    /// Fibonacci using Binet's formula
    pub fn binet(n: u64) -> u64 {
        let sqrt_5 = 5.0_f64.sqrt();
        let phi = (1.0 + sqrt_5) / 2.0;
        ((phi.powi(n as i32) / sqrt_5).round()) as u64
    }

    /// Mengecek apakah angka Fibonacci genap
    /// Check if Fibonacci number is even
    pub fn adalah_genap(n: u64) -> bool {
        Fibonacci::iteratif(n) % 2 == 0
    }

    /// Mengecek apakah angka Fibonacci prima
    /// Check if Fibonacci number is prime
    pub fn adalah_prima(n: u64) -> bool {
        let num = Fibonacci::iteratif(n);
        if num < 2 { return false; }
        for i in 2..=((num as f64).sqrt() as u64) {
            if num % i == 0 { return false; }
        }
        true
    }
}
