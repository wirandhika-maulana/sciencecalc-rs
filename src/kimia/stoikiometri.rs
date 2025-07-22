/// Modul Stoikiometri
/// Stoichiometry Module

/// Menghitung jumlah mol (n) dari massa (m) dan massa molar (Mr).
/// Calculates moles (n) from mass (m) and molar mass (Mr).
/// Rumus: n = m / Mr
/// Formula: n = m / Mr
pub fn jumlah_mol(massa: f64, massa_molar: f64) -> f64 {
    // massa = massa (gram), massa_molar = massa molar (gram/mol)
    // massa = mass (gram), massa_molar = molar mass (gram/mol)
    massa / massa_molar
}
