/// Modul Listrik
/// Electricity Module

/// Menghitung tegangan listrik (V) berdasarkan arus listrik (I) dan hambatan (R) menggunakan Hukum Ohm: V = I × R.
/// Calculates electric voltage (V) given current (I) and resistance (R) using Ohm's Law: V = I × R.
pub fn ohm_tegangannya(i: f64, r: f64) -> f64 {
    // V = tegangan (volt), I = arus (ampere), R = hambatan (ohm)
    // V = voltage (volt), I = current (ampere), R = resistance (ohm)
    i * r
}

/// Menghitung arus listrik (I) berdasarkan tegangan (V) dan hambatan (R) menggunakan Hukum Ohm: I = V / R.
/// Calculates electric current (I) given voltage (V) and resistance (R) using Ohm's Law: I = V / R.
pub fn ohm_arusnya(v: f64, r: f64) -> f64 {
    // I = arus (ampere), V = tegangan (volt), R = hambatan (ohm)
    // I = current (ampere), V = voltage (volt), R = resistance (ohm)
    v / r
}

/// Menghitung hambatan listrik (R) berdasarkan tegangan (V) dan arus (I) menggunakan Hukum Ohm: R = V / I.
/// Calculates electric resistance (R) given voltage (V) and current (I) using Ohm's Law: R = V / I.
pub fn ohm_hambatannya(v: f64, i: f64) -> f64 {
    // R = hambatan (ohm), V = tegangan (volt), I = arus (ampere)
    // R = resistance (ohm), V = voltage (volt), I = current (ampere)
    v / i
}
