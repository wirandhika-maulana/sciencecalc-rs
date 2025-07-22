/// Hukum Ohm: V = I * R
/// Ohm's Law: V = I * R
pub fn ohm_voltage(i: f64, r: f64) -> f64 {
    // V = tegangan (volt), I = arus (ampere), R = hambatan (ohm)
    // V = voltage (volt), I = current (ampere), R = resistance (ohm)
    i * r
}

/// Hukum Ohm: I = V / R
/// Ohm's Law: I = V / R
pub fn ohm_current(v: f64, r: f64) -> f64 {
    // I = arus (ampere), V = tegangan (volt), R = hambatan (ohm)
    // I = current (ampere), V = voltage (volt), R = resistance (ohm)
    v / r
}

/// Hukum Ohm: R = V / I
/// Ohm's Law: R = V / I
pub fn ohm_resistance(v: f64, i: f64) -> f64 {
    // R = hambatan (ohm), V = tegangan (volt), I = arus (ampere)
    // R = resistance (ohm), V = voltage (volt), I = current (ampere)
    v / i
}
