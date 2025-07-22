/// Faktorial (n!) - Factorial calculation
pub fn factorial(n: u64) -> u64 {
    // Faktorial: hasil perkalian semua bilangan bulat positif <= n
    // Factorial: product of all positive integers <= n
    (1..=n).product()
}

/// Kombinasi (nCr) - Combination calculation
pub fn combination(n: u64, r: u64) -> u64 {
    // Kombinasi: memilih r objek dari n tanpa memperhatikan urutan
    // Combination: choosing r objects from n without regard to order
    factorial(n) / (factorial(r) * factorial(n - r))
}

/// Permutasi (nPr) - Permutation calculation
pub fn permutation(n: u64, r: u64) -> u64 {
    // Permutasi: memilih r objek dari n dengan memperhatikan urutan
    // Permutation: choosing r objects from n with regard to order
    factorial(n) / factorial(n - r)
}
