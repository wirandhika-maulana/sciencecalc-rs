/// Modul Aljabar - Algebra Module
/// Fitur: Faktorial, Kombinasi, Permutasi, Sistem Persamaan Linear, Kuadrat, Matriks
/// Features: Factorial, Combination, Permutation, Linear & Quadratic System, Matrix Operations

pub struct Aljabar;

impl Aljabar {
    // ==================
    // Helper: Float to Fraction - Mengubah Float ke Pecahan String
    // ==================
    pub fn float_to_fraction(value: f64) -> String {
        if value.is_nan() || value.is_infinite() {
            return "Cannot convert / Tidak dapat dikonversi".to_string();
        }
        
        // Menangani nilai bulat
        if value == (value as i64) as f64 {
            return format!("{}/1", value as i64);
        }
        // Konstanta untuk batas iterasi dan presisi
        const MAX_ITER: usize = 30;
        const EPS: f64 = 1e-10;

        let sign = if value < 0.0 { -1 } else { 1 };
        let value = value.abs();

        let mut h1: i64 = 1;
        let mut h2: i64 = 0;
        let mut k1: i64 = 0;
        let mut k2: i64 = 1;

        let mut b = value;
        for _ in 0..MAX_ITER {
            let a = b.floor();
            let aux = h1;
            h1 = (a as i64) * h1 + h2;
            h2 = aux;
            let aux = k1;
            k1 = (a as i64) * k1 + k2;
            k2 = aux;
            b = 1.0 / (b - a);
            if (value - (h1 as f64) / (k1 as f64)).abs() < EPS * (value.abs()) {
                break;
            }
        }
        if sign < 0 {
            format!("-{}/{}", h1, k1)
        } else {
            format!("{}/{}", h1, k1)
        }
    }

    // ==================
    // Sistem Persamaan Linear Satu Variabel / Single Variable (SPLSV)
    // ==================
    // x = -b / a
    // Versi original (hasil float)
    //
    pub fn splsv(a: f64, b: f64) -> Option<f64> {
        if a == 0.0 {
            return None;
        }
        Some(-b / a)
    }
    //
    // Sistem Persamaan Linear Satu Variable
    // x = -b / a
    // Versi hasil pecahan (String)
    //
    pub fn splsv_frac(a: f64, b: f64) -> Option<String> {
        if a == 0.0 {
            return None;
        }
        Some(Self::float_to_fraction(-b / a))
    }
    
    pub fn splsv_steps(a: f64, b: f64) -> (Option<f64>, String) {
        let mut steps = format!("Persamaan: {:.2}x + {:.2} = 0\nEquation: {:.2}x + {:.2} = 0\n", a, b, a, b);
        if a == 0.0 {
            steps.push_str("Koefisien x = 0, tidak ada solusi.\nCoefficient x = 0, no solution.\n");
            return (None, steps);
        }
        let x = -b / a;
        steps.push_str(&format!("x = -b/a = {:.2}\nx = {}\n", x, Self::float_to_fraction(x)));
        (Some(x), steps)
    }

    // ==================
    // Sistem Persamaan Linear Dua Variabel / Two Variables (SPLDV)
    // ==================
    // a1x + b1y = c1
    // a2x + b2y = c2
    // Versi original (hasil float)
    //
    pub fn spldv(a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> Option<(f64, f64)> {
        let new_b1;
        let new_b2;
        let new_c1;
        let new_c2;

        if a1 == a2 {
            // Kalau koefisien x pertama
            new_b1 = b1;
            new_b2 = b2; // sama dengan koefisien x kedua,
            new_c1 = c1;
            new_c2 = c2; // maka kedua persamaan tidak ada yang berubah.
        } else {
            new_b1 = b1 * a2;
            new_b2 = b2 * a1; // Kalau tidak, maka persamaan pertama
            new_c1 = c1 * a2;
            new_c2 = c2 * a1; // dikalikan dengan a1 dan persamaan kedua
                              // dikalikan denga a2 (koefisien 1 dan 2 x).
        }

        let b3;
        let c3;

        if a1 < 0.00 || a2 < 0.00 {
            // Jika a1 atau a2 bernilai negatif,
            b3 = new_b1 + new_b2; // Maka kedua persamaan akan dijumlah.
            c3 = new_c1 + new_c2;
        } else {
            // Sebaliknya, jika salah satu koefisien
            b3 = new_b1 - new_b2; // bernilai positif, maka kedua persamaan
            c3 = new_c1 - new_c2; // akan dikurangi.
        }

        let y = c3 / b3;
        let x = (c2 - (b2 * y)) / a2;

        Some((x, y))
    }

    //
    // Sistem Persamaan Linear Dua Variable
    // a1x + b1y = c1
    // a2x + b2y = c2
    // Versi hasil pecahan (String)
    //
    pub fn spldv_frac(
        a1: f64,
        b1: f64,
        c1: f64,
        a2: f64,
        b2: f64,
        c2: f64,
    ) -> Option<(String, String)> {
        if let Some((x, y)) = Self::spldv(a1, b1, c1, a2, b2, c2) {
            Some((Self::float_to_fraction(x), Self::float_to_fraction(y)))
        } else {
            None
        }
    }


    pub fn spldv_steps(a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> (Option<(f64, f64)>, String) {
        let mut steps = format!(
            "Persamaan:\n{:.2}x + {:.2}y = {:.2}\n{:.2}x + {:.2}y = {:.2}\nEquation:\n{:.2}x + {:.2}y = {:.2}\n{:.2}x + {:.2}y = {:.2}\n",
            a1, b1, c1, a2, b2, c2, a1, b1, c1, a2, b2, c2
        );
        let det = a1 * b2 - a2 * b1;
        if det == 0.0 {
            steps.push_str("Determinan = 0, tidak ada solusi unik.\nDeterminant = 0, no unique solution.\n");
            return (None, steps);
        }
        let x = (c1 * b2 - c2 * b1) / det;
        let y = (a1 * c2 - a2 * c1) / det;
        steps.push_str(&format!(
            "x = (c1*b2 - c2*b1)/det = {:.2}\ny = (a1*c2 - a2*c1)/det = {:.2}\nx = {}\ny = {}\n",
            x, y, Self::float_to_fraction(x), Self::float_to_fraction(y)
        ));
        (Some((x, y)), steps)
    }

    // ==================
    // Sistem Persamaan Linear Tiga Variabel / Three Variables (SPLTV)
    // ==================
    //
    // Sistem Persamaan Linear Tiga Variable
    // a1x - b1y + c1z = d1
    // a2x + b2y - c2z = d2
    // a3x - b3y + c3z = d3
    // Versi original (hasil float)
    //
    pub fn spltv(
        a1: f64,
        b1: f64,
        c1: f64,
        d1: f64,
        a2: f64,
        b2: f64,
        c2: f64,
        d2: f64,
        a3: f64,
        b3: f64,
        c3: f64,
        d3: f64,
    ) -> Option<(f64, f64, f64)> {
        // Determinan utama
        let det = a1 * (b2 * c3 - b3 * c2) - b1 * (a2 * c3 - a3 * c2) + c1 * (a2 * b3 - a3 * b2);

        if det == 0.0 {
            return None;
        }

        // Determinan x
        let det_x = d1 * (b2 * c3 - b3 * c2) - b1 * (d2 * c3 - d3 * c2) + c1 * (d2 * b3 - d3 * b2);
        // Determinan y
        let det_y = a1 * (d2 * c3 - d3 * c2) - d1 * (a2 * c3 - a3 * c2) + c1 * (a2 * d3 - a3 * d2);
        // Determinan z
        let det_z = a1 * (b2 * d3 - b3 * d2) - b1 * (a2 * d3 - a3 * d2) + d1 * (a2 * b3 - a3 * b2);
        // Menghitung x, y, dan z
        let x = det_x / det;
        let y = det_y / det;
        let z = det_z / det;

        Some((x, y, z))
    }

    //
    // Sistem Persamaan Linear Tiga Variable
    // a1x - b1y + c1z = d1
    // a2x + b2y - c2z = d2
    // a3x - b3y + c3z = d3
    // Versi hasil pecahan (String)
    //
    pub fn spltv_frac(
        a1: f64,
        b1: f64,
        c1: f64,
        d1: f64,
        a2: f64,
        b2: f64,
        c2: f64,
        d2: f64,
        a3: f64,
        b3: f64,
        c3: f64,
        d3: f64,
    ) -> Option<(String, String, String)> {
        if let Some((x, y, z)) = Self::spltv(a1, b1, c1, d1, a2, b2, c2, d2, a3, b3, c3, d3) {
            Some((
                Self::float_to_fraction(x),
                Self::float_to_fraction(y),
                Self::float_to_fraction(z),
            ))
        } else {
            None
        }
    }

    pub fn splsv_proses(a: f64, b: f64) -> (Option<f64>, String) {
        let mut steps = format!("Persamaan: {:.2}x + {:.2} = 0\n\n", a, b);

        if a == 0.0 {
            steps.push_str("Koefisien x (a) = 0 → Tidak ada solusi.\n\n");
            return (None, steps);
        }

        let x = -b / a;
        steps.push_str(&format!(
            "Pindahkan b ke kanan:\n  {:.2}x = {:.2}\nBagi dengan a:\n  x = {:.2}\n\n",
            a, -b, x
        ));

        (Some(x), steps)
    }

    // Versi proses dengan hasil pecahan
    pub fn splsv_proses_frac(a: f64, b: f64) -> (Option<f64>, String) {
        let (result_opt, mut steps) = Self::splsv_proses(a, b);

        if let Some(x) = result_opt {
            let x_frac = Self::float_to_fraction(x);
            steps.push_str(&format!("Hasil dalam bentuk pecahan: x = {}\n\n", x_frac));
            (Some(x), steps)
        } else {
            (None, steps)
        }
    }

    pub fn spldv_proses(
        a1: f64,
        b1: f64,
        c1: f64,
        a2: f64,
        b2: f64,
        c2: f64,
    ) -> (Option<(f64, f64)>, String) {
        let mut steps = String::new();

        steps.push_str(&format!(
            "Persamaan:\n  a1x + b1y = c1  →  {:.2}x + {:.2}y = {:.2}\n  a2x + b2y = c2  →  {:.2}x + {:.2}y = {:.2}\n\n",
            a1, b1, c1, a2, b2, c2
        ));

        let (new_b1, new_b2, new_c1, new_c2) = if a1 == a2 {
            steps.push_str("Karena a1 == a2, persamaan tidak diubah.\n\n");
            (b1, b2, c1, c2)
        } else {
            steps.push_str(&format!(
                "Mengalikan P1 dengan {:.2}, dan P2 dengan {:.2}:\n",
                a2, a1
            ));
            (b1 * a2, b2 * a1, c1 * a2, c2 * a1)
        };

        let (b3, c3) = if a1 < 0.0 || a2 < 0.0 {
            steps.push_str("a1 atau a2 negatif → Jumlahkan kedua persamaan:\n");
            (new_b1 + new_b2, new_c1 + new_c2)
        } else {
            steps.push_str("a1 dan a2 positif → Kurangkan kedua persamaan:\n");
            (new_b1 - new_b2, new_c1 - new_c2)
        };

        steps.push_str(&format!("Hasil eliminasi x:\n  {:.2}y = {:.2}\n", b3, c3));

        if b3 == 0.0 {
            return (
                None,
                steps + "\nTidak dapat menyelesaikan karena pembagi 0.",
            );
        }

        let y = c3 / b3;
        steps.push_str(&format!("  y = {:.2}\n", y));

        let x = (c2 - (b2 * y)) / a2;
        steps.push_str(&format!(
            "\nSubstitusi y ke P2:\n  x = ({:.2} - {:.2} × {:.2}) / {:.2} = {:.2}\n",
            c2, b2, y, a2, x
        ));

        (Some((x, y)), steps)
    }

    // Versi proses dengan hasil pecahan
    pub fn spldv_proses_frac(
        a1: f64,
        b1: f64,
        c1: f64,
        a2: f64,
        b2: f64,
        c2: f64,
    ) -> (Option<(f64, f64)>, String) {
        let (result_opt, mut steps) = Self::spldv_proses(a1, b1, c1, a2, b2, c2);

        if let Some((x, y)) = result_opt {
            let x_frac = Self::float_to_fraction(x);
            let y_frac = Self::float_to_fraction(y);

            steps.push_str(&format!(
                "\nHasil dalam bentuk pecahan:\n  x = {}\n  y = {}\n",
                x_frac, y_frac
            ));

            (Some((x, y)), steps)
        } else {
            (None, steps)
        }
    }

    // ==================
    // Persamaan Kuadrat / Quadratic Equation
    // ==================
    pub fn kuadrat(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
        let discriminant = b * b - 4.0 * a * c;
        if a == 0.0 || discriminant < 0.0 {
            return None;
        }
        let sqrt_disc = discriminant.sqrt();
        let x1 = (-b + sqrt_disc) / (2.0 * a);
        let x2 = (-b - sqrt_disc) / (2.0 * a);
        Some((x1, x2))
    }

    pub fn kuadrat_frac(a: f64, b: f64, c: f64) -> Option<(String, String)> {
        Self::kuadrat(a, b, c)
            .map(|(x1, x2)| (Self::float_to_fraction(x1), Self::float_to_fraction(x2)))
    }

    // ==================
    // Operasi Matriks Sederhana / Simple Matrix Operations
    // ==================
    /// Determinan matriks 2x2 / 2x2 matrix determinant
    pub fn determinant_2x2(a: f64, b: f64, c: f64, d: f64) -> f64 {
        a * d - b * c
    }

    /// Perkalian matriks 2x2 / 2x2 matrix multiplication
    pub fn matriks_2x2(m1: [[f64; 2]; 2], m2: [[f64; 2]; 2]) -> [[f64; 2]; 2] {
        [
            [
                m1[0][0]*m2[0][0] + m1[0][1]*m2[1][0],
                m1[0][0]*m2[0][1] + m1[0][1]*m2[1][1],
            ],
            [
                m1[1][0]*m2[0][0] + m1[1][1]*m2[1][0],
                m1[1][0]*m2[0][1] + m1[1][1]*m2[1][1],
            ],
        ]
    }

    /// Invers matriks 2x2 / 2x2 matrix inverse
    pub fn inverse_2x2(a: f64, b: f64, c: f64, d: f64) -> Option<[[f64; 2]; 2]> {
        let det = Self::determinant_2x2(a, b, c, d);
        if det == 0.0 {
            return None;
        }
        Some([
            [ d/det, -b/det ],
            [ -c/det, a/det ],
        ])
    }

    /// Transpose matriks 2x2 / 2x2 matrix transpose
pub fn transpose_2x2(m: [[f64; 2]; 2]) -> [[f64; 2]; 2] {
    [
        [m[0][0], m[1][0]],
        [m[0][1], m[1][1]],
    ]
}
    
// ==================
// Matriks 3x3 dan Transpose / 3x3 Matrix and Transpose
// ==================

/// Determinan matriks 3x3 / 3x3 matrix determinant
pub fn determinant_3x3(m: [[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1]*m[2][2] - m[1][2]*m[2][1]) -
    m[0][1] * (m[1][0]*m[2][2] - m[1][2]*m[2][0]) +
    m[0][2] * (m[1][0]*m[2][1] - m[1][1]*m[2][0])
}

/// Transpose matriks 3x3 / 3x3 matrix transpose
pub fn transpose_3x3(m: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    [
        [m[0][0], m[1][0], m[2][0]],
        [m[0][1], m[1][1], m[2][1]],
        [m[0][2], m[1][2], m[2][2]],
    ]
}

/// Perkalian matriks 3x3 / 3x3 matrix multiplication
pub fn matriks_3x3(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut result = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

/// Invers matriks 3x3 / 3x3 matrix inverse (mengembalikan Option, None jika tak invertible)
pub fn inverse_3x3(m: [[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    let det = Self::determinant_3x3(m);
    if det == 0.0 { return None; }
    // Hitung kofaktor
    let mut cof = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let submat = |_x: usize, _y: usize| -> f64 {
                let mut vals = vec![];
                for r in 0..3 {
                    for c in 0..3 {
                        if r != i && c != j {
                            vals.push(m[r][c]);
                        }
                    }
                }
                vals[0]*vals[3] - vals[1]*vals[2]
            };
            cof[i][j] = ((i+j)%2 == 0).then_some(1.0).unwrap_or(-1.0) * submat(i,j);
        }
    }
    // Transpose kofaktor dan bagi determinan
    let cof_t = Self::transpose_3x3(cof);
    let mut inv = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            inv[i][j] = cof_t[i][j] / det;
        }
    }
    Some(inv)
}
    
}
 
