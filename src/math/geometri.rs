/// Modul Geometri - Geometry Module
/// Berisi rumus dan operasi Bangun Datar & Bangun Ruang
/// Contains formulas and operations for 2D & 3D shapes

// ================= BANGUN DATAR ==================

#[derive(Debug)]
pub struct Persegi { pub sisi: f64 }
impl Persegi {
    pub fn new(sisi: f64) -> Self { Self { sisi } }
    pub fn luas(&self) -> f64 { self.sisi.powi(2) }
    pub fn keliling(&self) -> f64 { self.sisi * 4.0 }
}

#[derive(Debug)]
pub struct PersegiPanjang { pub panjang: f64, pub lebar: f64 }
impl PersegiPanjang {
    pub fn new(panjang: f64, lebar: f64) -> Self { Self { panjang, lebar } }
    pub fn luas(&self) -> f64 { self.panjang * self.lebar }
    pub fn keliling(&self) -> f64 { 2.0 * (self.panjang + self.lebar) }
}

#[derive(Debug)]
pub struct Segitiga { pub alas: f64, pub tinggi: f64, pub sisi: [f64; 3] }
impl Segitiga {
    pub fn new(alas: f64, tinggi: f64, sisi: [f64; 3]) -> Self {
        Self { alas, tinggi, sisi }
    }
    pub fn luas(&self) -> f64 { 0.5 * self.alas * self.tinggi }
    pub fn keliling(&self) -> f64 { self.sisi.iter().sum() }
}

// Tambah Lingkaran, Jajargenjang, Trapesium seperti sebelumnya

// ================= BANGUN RUANG ==================

#[derive(Debug)]
pub struct Kubus { pub sisi: f64 }
impl Kubus {
    pub fn new(sisi: f64) -> Self { Self { sisi } }
    pub fn volume(&self) -> f64 { self.sisi.powi(3) }
    pub fn luas_permukaan(&self) -> f64 { 6.0 * self.sisi.powi(2) }
    // dst...
}

// Tambah Balok, Bola, Tabung, Kerucut, Limas, dst seperti sebelumnya
