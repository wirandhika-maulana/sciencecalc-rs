/// Modul Gerak
/// Motion Module

/// Menghitung perpindahan (s) pada Gerak Lurus Berubah Beraturan (GLBB) berdasarkan kecepatan awal (v0), waktu (t), dan percepatan (a).
/// Calculates displacement (s) in Uniformly Accelerated Motion (GLBB) given initial velocity (v0), time (t), and acceleration (a).
/// Rumus: s = v0 × t + 0.5 × a × t²
/// Formula: s = v0 × t + 0.5 × a × t²
pub fn glbb_perpindahan(v0: f64, t: f64, a: f64) -> f64 {
    // v0 = kecepatan awal (m/s), t = waktu (s), a = percepatan (m/s^2)
    // v0 = initial velocity (m/s), t = time (s), a = acceleration (m/s^2)
    v0 * t + 0.5 * a * t.powi(2)
}

/// Menghitung kecepatan akhir pada GLBB berdasarkan kecepatan awal (v0), percepatan (a), dan waktu (t).
/// Calculates final velocity in GLBB given initial velocity (v0), acceleration (a), and time (t).
/// Rumus: v = v0 + a × t
/// Formula: v = v0 + a × t
pub fn glbb_kecepatan_akhir(v0: f64, a: f64, t: f64) -> f64 {
    // v0 = kecepatan awal (m/s), a = percepatan (m/s^2), t = waktu (s)
    // v0 = initial velocity (m/s), a = acceleration (m/s^2), t = time (s)
    v0 + a * t
}
