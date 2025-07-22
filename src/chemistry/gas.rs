/// Modul Gas
/// Gas Module

/// Menghitung tekanan gas ideal (P) berdasarkan jumlah mol (n), konstanta gas (R), suhu (T), dan volume (V) menggunakan Hukum Gas Ideal: PV = nRT.
/// Calculates ideal gas pressure (P) from number of moles (n), gas constant (R), temperature (T), and volume (V) using the Ideal Gas Law: PV = nRT.
/// Rumus: P = (n × R × T) / V
/// Formula: P = (n × R × T) / V
pub fn tekanan_gas_ideal(n: f64, r: f64, t: f64, v: f64) -> f64 {
    // n = jumlah mol, r = konstanta gas ideal, t = suhu (Kelvin), v = volume (L)
    // n = number of moles, r = ideal gas constant, t = temperature (Kelvin), v = volume (L)
    (n * r * t) / v
}
