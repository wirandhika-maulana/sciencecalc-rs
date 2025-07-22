/// Energi kinetik: Ek = 0.5 * m * v^2
/// Kinetic energy: Ek = 0.5 * m * v^2
pub fn kinetic_energy(m: f64, v: f64) -> f64 {
    // m = massa (kg), v = kecepatan (m/s)
    // m = mass (kg), v = velocity (m/s)
    0.5 * m * v.powi(2)
}

/// Energi potensial: Ep = m * g * h
/// Potential energy: Ep = m * g * h
pub fn potential_energy(m: f64, g: f64, h: f64) -> f64 {
    // m = massa (kg), g = gravitasi (m/s^2), h = ketinggian (m)
    // m = mass (kg), g = gravity (m/s^2), h = height (m)
    m * g * h
}
