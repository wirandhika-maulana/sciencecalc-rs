/// Modul Energi
/// Energy Module

/// Menghitung energi kinetik (Ek) dari massa (m) dan kecepatan (v).
/// Calculates kinetic energy (Ek) from mass (m) and velocity (v).
/// Rumus: Ek = 0.5 × m × v²
/// Formula: Ek = 0.5 × m × v²
pub fn energi_kinetik(m: f64, v: f64) -> f64 {
    // m = massa (kg), v = kecepatan (m/s)
    // m = mass (kg), v = velocity (m/s)
    0.5 * m * v.powi(2)
}

/// Menghitung energi potensial (Ep) dari massa (m), gravitasi (g), dan ketinggian (h).
/// Calculates potential energy (Ep) from mass (m), gravity (g), and height (h).
/// Rumus: Ep = m × g × h
/// Formula: Ep = m × g × h
pub fn energi_potensial(m: f64, g: f64, h: f64) -> f64 {
    // m = massa (kg), g = gravitasi (m/s^2), h = ketinggian (m)
    // m = mass (kg), g = gravity (m/s^2), h = height (m)
    m * g * h
}
