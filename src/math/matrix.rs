/// Penjumlahan matriks - Matrix addition
pub fn add_matrix(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // Menjumlahkan dua matriks dengan ukuran sama
    // Add two matrices of the same size
    a.iter().zip(b.iter())
     .map(|(row_a, row_b)| row_a.iter().zip(row_b.iter()).map(|(x, y)| x + y).collect())
     .collect()
}

/// Perkalian matriks - Matrix multiplication
pub fn multiply_matrix(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // Mengalikan dua matriks (a: m x n, b: n x p)
    // Multiply two matrices (a: m x n, b: n x p)
    let n = a.len();
    let m = b[0].len();
    let mut result = vec![vec![0.0; m]; n];
    for i in 0..n {
        for j in 0..m {
            for k in 0..b.len() {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

/// Determinan matriks 2x2 - 2x2 Matrix Determinant
pub fn determinant_2x2(a: &[[f64; 2]; 2]) -> f64 {
    // Determinan 2x2: ad - bc
    // Determinant 2x2: ad - bc
    a[0][0]*a[1][1] - a[0][1]*a[1][0]
}
