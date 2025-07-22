/// Hitung massa hasil reaksi dari jumlah mol dan massa molar: m = n * Mr
/// Calculate mass of reaction product from moles and molar mass: m = n * Mr
pub fn product_mass(moles: f64, molar_mass: f64) -> f64 {
    // moles = jumlah mol, molar_mass = massa molar (gram/mol)
    // moles = number of moles, molar_mass = molar mass (gram/mol)
    moles * molar_mass
}

/// Hitung persentase hasil reaksi: %yield = (actual / theoretical) * 100
/// Calculate percent yield of reaction: %yield = (actual / theoretical) * 100
pub fn percent_yield(actual_mass: f64, theoretical_mass: f64) -> f64 {
    // actual_mass = massa hasil aktual (gram), theoretical_mass = massa hasil teoritis (gram)
    // actual_mass = actual product mass (gram), theoretical_mass = theoretical product mass (gram)
    (actual_mass / theoretical_mass) * 100.0
}
