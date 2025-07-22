/// Hitung molaritas larutan: M = n / V
/// Calculate solution molarity: M = n / V
pub fn molarity(moles: f64, volume_liter: f64) -> f64 {
    // moles = jumlah mol, volume_liter = volume larutan (liter)
    // moles = number of moles, volume_liter = solution volume (liter)
    moles / volume_liter
}

/// Hitung pH larutan asam kuat: pH = -log[H+]
/// Calculate pH of strong acid solution: pH = -log[H+]
pub fn ph_strong_acid(h_concentration: f64) -> f64 {
    // h_concentration = konsentrasi [H+] (mol/L)
    // h_concentration = [H+] concentration (mol/L)
    -h_concentration.log10()
}
