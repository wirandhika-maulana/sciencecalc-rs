/// Modul Statistik - Statistics Module
/// Fitur: Mean, Median, Mode, Variance, Standard Deviation
/// Features: Mean, Median, Mode, Variance, Standard Deviation

use std::collections::HashMap;

pub struct Statistika;

impl Statistika {
    /// Menghitung rata-rata (mean)
    pub fn mean(data: &[f64]) -> f64 {
        if data.is_empty() {
            panic!("Tidak bisa menghitung rata-rata dari array kosong.");
        }
        let sum: f64 = data.iter().sum();
        sum / (data.len() as f64)
    }

    /// Menghitung median
    pub fn median(data: &mut [f64]) -> f64 {
        if data.is_empty() {
            panic!("Tidak dapat menghitung median dari data kosong.");
        }
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = data.len();
        if len % 2 == 0 {
            (data[len / 2 - 1] + data[len / 2]) / 2.0
        } else {
            data[len / 2]
        }
    }

    /// Menghitung modus (support data lebih dari satu modus)
    pub fn modus(data: &[i64]) -> Vec<i64> {
        if data.is_empty() {
            panic!("Tidak dapat menghitung modus dari data kosong.");
        }
        let mut frekuensi = HashMap::new();
        for &num in data {
            *frekuensi.entry(num).or_insert(0) += 1;
        }
        let max_freq = frekuensi.values().copied().max().unwrap();
        frekuensi
            .into_iter()
            .filter(|&(_, count)| count == max_freq)
            .map(|(num, _)| num)
            .collect()
    }

    /// Menghitung varian
    pub fn varian(data: &[f64]) -> f64 {
        if data.is_empty() {
            panic!("Tidak dapat menghitung varian dari data kosong.");
        }
        let mean = Self::mean(data);
        let sq_diff: f64 = data.iter().map(|value| (value - mean).powi(2)).sum();
        sq_diff / (data.len() as f64)
    }

    /// Menghitung standar deviasi
    pub fn standar_deviasi(data: &[f64]) -> f64 {
        Self::varian(data).sqrt()
    }
}
