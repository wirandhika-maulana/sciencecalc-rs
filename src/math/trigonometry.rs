/// Fungsi sinus (sin) - Sine function
pub fn sin(degree: f64) -> f64 {
    // Input dalam derajat - Input in degrees
    (degree.to_radians()).sin()
}

/// Fungsi kosinus (cos) - Cosine function
pub fn cos(degree: f64) -> f64 {
    // Input dalam derajat - Input in degrees
    (degree.to_radians()).cos()
}

/// Fungsi tangen (tan) - Tangent function
pub fn tan(degree: f64) -> f64 {
    // Input dalam derajat - Input in degrees
    (degree.to_radians()).tan()
}
