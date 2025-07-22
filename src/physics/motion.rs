/// Gerak Lurus Berubah Beraturan (GLBB): s = v0 * t + 0.5 * a * t^2
/// Uniformly Accelerated Motion (GLBB): s = v0 * t + 0.5 * a * t^2
pub fn glbb_displacement(v0: f64, t: f64, a: f64) -> f64 {
    // v0 = kecepatan awal, t = waktu, a = percepatan
    // v0 = initial velocity, t = time, a = acceleration
    v0 * t + 0.5 * a * t.powi(2)
}

/// Kecepatan akhir GLBB: v = v0 + a * t
/// Final velocity of GLBB: v = v0 + a * t
pub fn glbb_final_velocity(v0: f64, a: f64, t: f64) -> f64 {
    // v0 = kecepatan awal, a = percepatan, t = waktu
    // v0 = initial velocity, a = acceleration, t = time
    v0 + a * t
}
