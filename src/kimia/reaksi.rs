/// Modul Reaksi
/// Reaction Module

/// Menghitung massa hasil reaksi (m) dari jumlah mol (n) dan massa molar (Mr).
/// Calculates product mass (m) from the number of moles (n) and molar mass (Mr).
/// Rumus: m = n × Mr
/// Formula: m = n × Mr
pub fn massa_produk(n: f64, mr: f64) -> f64 {
    // n = jumlah mol, mr = massa molar (gram/mol)
    // n = number of moles, mr = molar mass (gram/mol)
    n * mr
}

/// Menghitung persentase hasil reaksi (%yield) dari massa aktual dan massa teoritis.
/// Calculates percent yield (%yield) from actual product mass and theoretical product mass.
/// Rumus: %yield = (massa aktual / massa teoritis) × 100
/// Formula: %yield = (actual mass / theoretical mass) × 100
pub fn persen_hasil(massa_aktual: f64, massa_teoritis: f64) -> f64 {
    // massa_aktual = massa hasil aktual (gram), massa_teoritis = massa hasil teoritis (gram)
    // actual_mass = actual product mass (gram), theoretical_mass = theoretical product mass (gram)
    (massa_aktual / massa_teoritis) * 100.0
}
