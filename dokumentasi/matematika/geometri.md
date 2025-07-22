# Modul Geometri dalam `sciencecalc-rs`

Modul ini menyediakan berbagai struktur dan metode untuk menghitung luas, keliling, volume, dan properti lain dari bangun datar serta bangun ruang dalam bahasa Rust.

---

## Bangun Datar

### Persegi
```rust
pub struct Persegi { pub sisi: f64 }
impl Persegi {
    pub fn new(sisi: f64) -> Self
    pub fn luas(&self) -> f64
    pub fn keliling(&self) -> f64
}
```
- **Luas:** sisi × sisi  
- **Keliling:** 4 × sisi

---

### Persegi Panjang
```rust
pub struct PersegiPanjang { pub panjang: f64, pub lebar: f64 }
impl PersegiPanjang {
    pub fn new(panjang: f64, lebar: f64) -> Self
    pub fn luas(&self) -> f64
    pub fn keliling(&self) -> f64
}
```
- **Luas:** panjang × lebar  
- **Keliling:** 2 × (panjang + lebar)

---

### Segitiga
```rust
pub struct Segitiga { pub alas: f64, pub tinggi: f64, pub sisi: [f64; 3] }
impl Segitiga {
    pub fn new(alas: f64, tinggi: f64, sisi: [f64; 3]) -> Self
    pub fn luas(&self) -> f64
    pub fn keliling(&self) -> f64
}
```
- **Luas:** ½ × alas × tinggi  
- **Keliling:** jumlah semua sisi

---

### Lingkaran
```rust
pub struct Lingkaran { pub r: f64 }
impl Lingkaran {
    pub fn new(r: f64) -> Self
    pub fn luas(&self) -> f64
    pub fn keliling(&self) -> f64
}
```
- **Luas:** π × r²  
- **Keliling:** 2 × π × r

---

### Jajargenjang
```rust
pub struct Jajargenjang { pub alas: f64, pub tinggi: f64, pub sisi_miring: f64 }
impl Jajargenjang {
    pub fn new(alas: f64, tinggi: f64, sisi_miring: f64) -> Self
    pub fn luas(&self) -> f64
    pub fn keliling(&self) -> f64
}
```
- **Luas:** alas × tinggi  
- **Keliling:** 2 × (alas + sisi miring)

---

### Trapesium
```rust
pub struct Trapesium { pub sisi: [f64; 4], pub tinggi: f64 }
impl Trapesium {
    pub fn new(sisi: [f64; 4], tinggi: f64) -> Self
    pub fn luas(&self) -> f64
    pub fn keliling(&self) -> f64
}
```
- **Luas:** ½ × (sisi atas + sisi bawah) × tinggi (`sisi[0]` dan `sisi[1]`)  
- **Keliling:** jumlah semua sisi

---

## Contoh Penggunaan Bangun Datar
```rust
use sciencecalc_rs::matematika::geometri::*;

fn main() {
    let trapesium = Trapesium::new([10.0, 6.0, 5.0, 7.0], 4.0);
    println!("Luas Trapesium: {:.2}", trapesium.luas());
    println!("Keliling Trapesium: {:.2}", trapesium.keliling());
}
```
Output:
```
Luas Trapesium: 32.00
Keliling Trapesium: 28.00
```

---

## Bangun Ruang

### Kubus
```rust
pub struct Kubus { pub sisi: f64 }
impl Kubus {
    pub fn new(sisi: f64) -> Self
    pub fn volume(&self) -> f64
    pub fn luas_permukaan(&self) -> f64
    pub fn diagonal_bidang(&self) -> f64
    pub fn diagonal_ruang(&self) -> f64
}
```
- **Volume:** sisi³  
- **Luas permukaan:** 6 × sisi²

---

### Balok
```rust
pub struct Balok { pub panjang: f64, pub lebar: f64, pub tinggi: f64 }
impl Balok {
    pub fn new(panjang: f64, lebar: f64, tinggi: f64) -> Self
    pub fn volume(&self) -> f64
    pub fn luas_permukaan(&self) -> f64
    pub fn keliling(&self) -> f64
    pub fn luas_sisi(&self) -> [f64; 3]
    pub fn diagonal_bidang(&self) -> f64
    pub fn diagonal_ruang(&self) -> f64
}
```
- **Volume:** panjang × lebar × tinggi

---

### Bola
```rust
pub struct Bola { pub r: f64 }
impl Bola {
    pub fn new(r: f64) -> Self
    pub fn volume(&self) -> f64
    pub fn luas_permukaan(&self) -> f64
    pub fn keliling(&self) -> f64
    pub fn setengah_volume(&self) -> f64
}
```
- **Volume:** (4/3) × π × r³  
- **Luas permukaan:** 4 × π × r²

---

### Tabung
```rust
pub struct Tabung { pub r: f64, pub tinggi: f64 }
impl Tabung {
    pub fn new(r: f64, tinggi: f64) -> Self
    pub fn volume(&self) -> f64
    pub fn luas_permukaan(&self) -> f64
    pub fn luas_alas(&self) -> f64
    pub fn keliling_alas(&self) -> f64
}
```
- **Volume:** π × r² × tinggi  
- **Luas permukaan:** 2 × π × r × (r + t)

---

### Kerucut
```rust
pub struct Kerucut { pub r: f64, pub tinggi: f64 }
impl Kerucut {
    pub fn new(r: f64, tinggi: f64) -> Self
    pub fn volume(&self) -> f64
    pub fn luas_alas(&self) -> f64
    pub fn garis_pelukis(&self) -> f64
    pub fn luas_permukaan(&self) -> f64
    pub fn luas_selimut(&self) -> f64
}
```
- **Volume:** (1/3) × π × r² × t  
- **Luas permukaan:** π × r × (r + s), s = garis pelukis

---

### Limas Segitiga
```rust
pub struct LimasSegitiga {
    pub tinggi: f64,
    pub alas_segitiga: f64,
    pub tinggi_segitiga: f64,
    pub tinggi_alas: f64,
    pub sisi_tegak: [(f64, f64); 3],
}
impl LimasSegitiga {
    pub fn new(...)
    pub fn volume(&self) -> f64
    pub fn luas_alas(&self) -> f64
    pub fn luas_permukaan(&self) -> f64
}
```

### Limas Persegi
```rust
pub struct LimasPersegi {
    pub panjang_alas: f64,
    pub lebar_alas: f64,
    pub tinggi: f64,
    pub tinggi_tegak: f64,
}
impl LimasPersegi {
    pub fn new(...)
    pub fn volume(&self) -> f64
    pub fn luas_alas(&self) -> f64
    pub fn luas_permukaan(&self) -> f64
    pub fn keliling_alas(&self) -> f64
    pub fn keliling_total(&self) -> f64
}
```

---

## Contoh Penggunaan Bangun Ruang
```rust
use sciencecalc_rs::matematika::geometri::*;

fn main() {
    let kubus = Kubus::new(5.0);

    println!("Volume Kubus: {:.2}", kubus.volume());
    println!("Luas Permukaan Kubus: {:.2}", kubus.luas_permukaan());
    println!("Diagonal Bidang Kubus: {:.2}", kubus.diagonal_bidang());
    println!("Diagonal Ruang Kubus: {:.2}", kubus.diagonal_ruang());
}
```
Output:
```
Volume Kubus: 125.00
Luas Permukaan Kubus: 150.00
Diagonal Bidang Kubus: 7.07
Diagonal Ruang Kubus: 8.66
```

---

Dokumentasi ini mencakup seluruh struktur dan fungsi geometri pada pustaka **sciencecalc-rs**.  
Untuk detail lebih lanjut, lihat source code di `src/matematika/geometri.rs`.  
Semoga bermanfaat!

---
