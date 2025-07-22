/// Modul Statistik - Statistics Module
/// Fitur: Mean, Median, Mode, Variance, Standard Deviation
/// Features: Mean, Median, Mode, Variance, Standard Deviation

pub struct Statistik;

impl Statistik {
    /// Rata-rata (Mean)
    pub fn mean(data: &[f64]) -> Option<f64> {
        if data.is_empty() { return None; }
        Some(data.iter().sum::<f64>() / data.len() as f64)
    }

    /// Median
    pub fn median(data: &mut [f64]) -> Option<f64> {
        if data.is_empty() { return None; }
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = data.len() / 2;
        if data.len() % 2 == 0 {
            Some((data[mid - 1] + data[mid]) / 2.0)
        } else {
            Some(data[mid])
        }
    }

    /// Modus (Mode) - Mengembalikan modus pertama jika lebih dari satu
    /// Returns the first mode if there are multiple
    pub fn mode(data: &[f64]) -> Option<f64> {
        use std::collections::HashMap;
        if data.is_empty() { return None; }
        let mut counts = HashMap::new();
        for &value in data {
            *counts.entry(value).or_insert(0) += 1;
        }
        counts.into_iter().max_by_key(|&(_, count)| count).map(|(value, _)| value)
    }

    /// Varians (Variance)
    pub fn variance(data: &[f64]) -> Option<f64> {
        let mean = Self::mean(data)?;
        Some(data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64)
    }

    /// Simpangan baku (Standard deviation)
    pub fn stddev(data: &[f64]) -> Option<f64> {
        Self::variance(data).map(|var| var.sqrt())
    }
}
