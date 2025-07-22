/// Hukum Gas Ideal: PV = nRT
/// Ideal Gas Law: PV = nRT
pub fn ideal_gas_pressure(n: f64, r: f64, t: f64, v: f64) -> f64 {
    // n = jumlah mol, r = konstanta gas ideal, t = suhu (Kelvin), v = volume (L)
    // n = number of moles, r = ideal gas constant, t = temperature (Kelvin), v = volume (L)
    (n * r * t) / v
}
