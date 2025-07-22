/// Sistem persamaan linear satu variabel: ax + b = c
/// Linear equation in one variable: ax + b = c
pub fn solve_linear_1x1(a: f64, b: f64, c: f64) -> Option<f64> {
    // Menyelesaikan persamaan linear satu variabel
    // Solves one-variable linear equation
    if a == 0.0 {
        None // Tidak ada solusi unik / No unique solution
    } else {
        Some((c - b) / a)
    }
}

/// Sistem persamaan linear dua variabel
/// 2-variable linear equation system
/// ax + by = c
/// dx + ey = f
pub fn solve_linear_2x2(a: f64, b: f64, c: f64, d: f64, e: f64, f_: f64) -> Option<(f64, f64)> {
    // Menyelesaikan sistem persamaan dengan metode eliminasi/determinant
    // Solves system by elimination/determinant
    let det = a*e - b*d;
    if det == 0.0 { return None; }
    let x = (c*e - b*f_) / det;
    let y = (a*f_ - c*d) / det;
    Some((x, y))
}
