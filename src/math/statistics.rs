/// Rata-rata (mean) - Mean
pub fn mean(data: &[f64]) -> f64 {
    // Menghitung rata-rata dari data
    // Calculates mean of data
    data.iter().sum::<f64>() / (data.len() as f64)
}

/// Median - Median
pub fn median(data: &mut [f64]) -> f64 {
    // Menghitung median dari data
    // Calculates median of data
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = data.len();
    if n % 2 == 0 {
        (data[n/2 - 1] + data[n/2]) / 2.0
    } else {
        data[n/2]
    }
}

/// Modus - Mode
pub fn mode(data: &[f64]) -> Option<f64> {
    use std::collections::HashMap;
    // Bulatkan ke 4 angka di belakang koma untuk menghindari error floating point
    let mut freqs = HashMap::new();
    for &v in data {
        let rounded = (v * 10000.0).round() as i64; // 4 digit
        *freqs.entry(rounded).or_insert(0) += 1;
    }
    // Cari nilai dengan frekuensi terbanyak
    freqs.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val as f64 / 10000.0)
}
