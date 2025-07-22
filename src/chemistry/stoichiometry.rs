/// Hitung jumlah mol dari massa dan massa molar: n = m / Mr
/// Calculate moles from mass and molar mass: n = m / Mr
pub fn moles(mass: f64, molar_mass: f64) -> f64 {
    // mass = massa (gram), molar_mass = massa molar (gram/mol)
    // mass = mass (gram), molar_mass = molar mass (gram/mol)
    mass / molar_mass
}
