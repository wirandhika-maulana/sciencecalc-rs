/// Modul Larutan
/// Solution Module

/// Menghitung molaritas larutan (M) dari jumlah mol (n) dan volume larutan (V).
/// Calculates solution molarity (M) from the number of moles (n) and solution volume (V).
/// Rumus: M = n / V
/// Formula: M = n / V
pub fn molaritas(jumlah_mol: f64, volume_liter: f64) -> f64 {
    // jumlah_mol = jumlah mol, volume_liter = volume larutan (liter)
    // jumlah_mol = number of moles, volume_liter = solution volume (liter)
    jumlah_mol / volume_liter
}

/// Menghitung pH larutan asam kuat dari konsentrasi ion hidrogen [H+].
/// Calculates pH of strong acid solution from hydrogen ion concentration [H+].
/// Rumus: pH = -log[H+]
/// Formula: pH = -log[H+]
pub fn ph_asam_kuat(konsentrasi_h: f64) -> f64 {
    // konsentrasi_h = konsentrasi [H+] (mol/L)
    // konsentrasi_h = [H+] concentration (mol/L)
    -konsentrasi_h.log10()
}
