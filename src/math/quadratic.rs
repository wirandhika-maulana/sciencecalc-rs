/// Persamaan kuadrat: ax^2 + bx + c = 0
/// Quadratic equation: ax^2 + bx + c = 0
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    // Mencari akar-akar persamaan kuadrat
    // Finds roots of quadratic equation
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        // Tidak ada akar real - No real roots
        None
    } else {
        let sqrt_d = discriminant.sqrt();
        let x1 = (-b + sqrt_d) / (2.0 * a);
        let x2 = (-b - sqrt_d) / (2.0 * a);
        Some((x1, x2))
    }
}
