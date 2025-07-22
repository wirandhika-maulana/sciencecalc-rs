/// Modul Trigonometri - Trigonometry Module
/// Fitur: Fungsi dasar trigonometri (sin, cos, tan, csc, sec, cot), konversi sudut, hukum trigonometri.
/// Features: Basic trigonometric functions, angle conversion, trigonometric laws.

pub struct Trigonometri;

impl Trigonometri {
    /// Konversi derajat ke radian / Convert degrees to radians
    pub fn deg_to_rad(degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }

    /// Konversi radian ke derajat / Convert radians to degrees
    pub fn rad_to_deg(radians: f64) -> f64 {
        radians * 180.0 / std::f64::consts::PI
    }

    /// Sinus (sin) fungsi / Sine function
    pub fn sin(degrees: f64) -> f64 {
        Self::deg_to_rad(degrees).sin()
    }

    /// Kosinus (cos) fungsi / Cosine function
    pub fn cos(degrees: f64) -> f64 {
        Self::deg_to_rad(degrees).cos()
    }

    /// Tangen (tan) fungsi / Tangent function
    pub fn tan(degrees: f64) -> f64 {
        Self::deg_to_rad(degrees).tan()
    }

    /// Kosekan (csc) fungsi / Cosecant function
    pub fn csc(degrees: f64) -> Option<f64> {
        let sin_val = Self::sin(degrees);
        if sin_val == 0.0 { None } else { Some(1.0 / sin_val) }
    }

    /// Sekan (sec) fungsi / Secant function
    pub fn sec(degrees: f64) -> Option<f64> {
        let cos_val = Self::cos(degrees);
        if cos_val == 0.0 { None } else { Some(1.0 / cos_val) }
    }

    /// Kotangen (cot) fungsi / Cotangent function
    pub fn cot(degrees: f64) -> Option<f64> {
        let tan_val = Self::tan(degrees);
        if tan_val == 0.0 { None } else { Some(1.0 / tan_val) }
    }

    /// Hukum sinus / Law of Sines
    /// a/sinA = b/sinB = c/sinC
    pub fn law_of_sines(a: f64, angle_a: f64, angle_b: f64) -> f64 {
        // Menghitung sisi b dari sisi a dan sudut A, B
        a * Self::sin(angle_b) / Self::sin(angle_a)
    }

    /// Hukum cosinus / Law of Cosines
    /// c^2 = a^2 + b^2 - 2ab cos(C)
    pub fn law_of_cosines(a: f64, b: f64, angle_c: f64) -> f64 {
        (a.powi(2) + b.powi(2) - 2.0 * a * b * Self::cos(angle_c)).sqrt()
    }
}
