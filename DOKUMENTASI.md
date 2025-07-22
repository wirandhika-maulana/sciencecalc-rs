# DOKUMENTASI sciencecalc-rs

`sciencecalc-rs` adalah pustaka Rust untuk kalkulasi **matematika**, **fisika**, dan **kimia** yang mudah digunakan dan dapat dikembangkan sesuai kebutuhan.

## Daftar Isi

- [Modul Matematika](#modul-matematika)
- [Modul Fisika](#modul-fisika)
- [Modul Kimia](#modul-kimia)
- [Contoh Penggunaan](#contoh-penggunaan)
- [Dokumentasi Lengkap](#dokumentasi-lengkap)
- [Kontribusi](#kontribusi)
- [Lisensi](#lisensi)

---

## Modul Matematika

### Struktur Modul
- `matematika/aritmetika.rs` — Operasi aritmetika dasar: tambah, kurang, kali, bagi, pangkat, akar, absolut, modulo, pembulatan.
- `matematika/aljabar.rs` — Fungsi aljabar: faktorial, kombinasi, permutasi, persamaan linear (SPLSV, SPLDV, SPLTV), persamaan kuadrat, operasi matriks 2x2 & 3x3.
- `matematika/trigonometri.rs` — Fungsi trigonometri: sinus, cosinus, tangen, invers, konversi derajat/radian.
- `matematika/geometri.rs` — Rumus luas, keliling, volume bangun datar dan ruang.
- `matematika/statistika.rs` — Statistik: rata-rata, median, modus, deviasi standar.
- `matematika/kombinatorika.rs` — Kombinatorik & peluang: faktorial, permutasi, kombinasi, peluang sederhana.
- `matematika/basis.rs` — Konversi basis bilangan: desimal, biner, oktal, heksadesimal.

### Contoh Fungsi
```rust
use sciencecalc_rs::matematika::aljabar;
let hasil = aljabar::factorial(5); // Output: 120
```
```rust
use sciencecalc_rs::matematika::aljabar;
let solusi = aljabar::splsv(2.0, 3.0); // Output: Some(-1.5)
```

---

## Modul Fisika

### Struktur Modul
- `fisika/gaya.rs` — Perhitungan gaya (F = m × a).
- `fisika/energi.rs` — Energi kinetik, potensial, kekekalan energi.
- `fisika/gerak.rs` — Gerak lurus, GLBB, kecepatan, percepatan.
- `fisika/listrik.rs` — Arus, tegangan, hambatan, hukum Ohm.

### Contoh Fungsi
```rust
use sciencecalc_rs::fisika::gaya;
let gaya = gaya::gaya(10.0, 9.8); // Output: 98.0
```

---

## Modul Kimia

### Struktur Modul
- `kimia/gas.rs` — Hukum gas ideal.
- `kimia/reaksi.rs` — Persamaan reaksi kimia, stoikiometri reaksi.
- `kimia/stoikiometri.rs` — Perhitungan mol, massa molar, konversi massa ↔ mol.
- `kimia/larutan.rs` — Kalkulasi larutan: konsentrasi, molaritas, pH, massa zat terlarut.

### Contoh Fungsi
```rust
use sciencecalc_rs::kimia::stoikiometri;
let mol = stoikiometri::jumlah_mol(18.0, 18.0); // Output: 1.0
```

---

## Contoh Penggunaan Lengkap

```rust
use sciencecalc_rs::matematika::aljabar;
use sciencecalc_rs::fisika::gaya;
use sciencecalc_rs::kimia::stoikiometri;

fn main() {
    // Matematika
    let faktorial = aljabar::factorial(5);
    println!("Faktorial 5: {}", faktorial);

    // Fisika
    let gaya = gaya::gaya(10.0, 9.8);
    println!("Gaya: {}", gaya);

    // Kimia
    let mol = stoikiometri::jumlah_mol(18.0, 18.0);
    println!("Mol: {}", mol);
}
```

---

## Dokumentasi Lengkap

Dokumentasi detail setiap modul, fungsi, dan contoh kasus dapat diakses di folder:  
[`/dokumentasi`](./dokumentasi)

---

## Kontribusi

Kontribusi sangat terbuka!  
- Fork repo ini
- Buat branch baru
- Pull request dengan deskripsi perubahan

---

## Lisensi

Proyek ini menggunakan lisensi MIT.  
Silakan digunakan dan dikembangkan sesuai kebutuhan.

---
