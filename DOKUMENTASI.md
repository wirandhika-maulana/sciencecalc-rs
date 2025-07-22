# DOKUMENTASI sciencecalc-rs

`sciencecalc-rs` adalah pustaka Rust untuk kalkulasi matematika, fisika, dan kimia yang mudah digunakan dan extensible.

## Daftar Isi

- [Modul Matematika](#modul-matematika)
- [Modul Fisika](#modul-fisika)
- [Modul Kimia](#modul-kimia)
- [Konversi Sistem Bilangan](#konversi-sistem-bilangan)
- [Contoh Penggunaan](#contoh-penggunaan)
- [Kontribusi](#kontribusi)
- [Lisensi](#lisensi)

---

## Modul Matematika

### Struktur Modul
- `math/algebra.rs` — Fungsi aljabar dasar: faktorial, pangkat, akar.
- `math/linear.rs` — Sistem persamaan linear.
- `math/matrix.rs` — Operasi matriks: penjumlahan, perkalian, invers, determinan.
- `math/quadratic.rs` — Penyelesaian persamaan kuadrat.
- `math/statistics.rs` — Statistik: mean, median, modus, deviasi standar.
- `math/trigonometry.rs` — Fungsi trigonometri: sin, cos, tan, dan invers.

### Contoh Fungsi
```rust
use sciencecalc_rs::math::algebra;
let hasil = algebra::factorial(5); // Output: 120
```
```rust
use sciencecalc_rs::math::linear;
let solusi = linear::solve_linear_1x1(2.0, 3.0, 11.0); // Output: Some(4.0)
```

---

## Modul Fisika

### Struktur Modul
- `physics/force.rs` — Kalkulasi gaya (F=ma).
- `physics/energy.rs` — Energi kinetik, potensial, dsb.
- `physics/motion.rs` — Gerak lurus, GLBB, kecepatan, percepatan.
- `physics/electricity.rs` — Arus, tegangan, hambatan.

### Contoh Fungsi
```rust
use sciencecalc_rs::physics::force;
let gaya = force::calculate_force(10.0, 9.8); // Output: 98.0
```

---

## Modul Kimia

### Struktur Modul
- `chemistry/gas.rs` — Hukum gas ideal.
- `chemistry/reaction.rs` — Persamaan reaksi kimia.
- `chemistry/stoichiometry.rs` — Perhitungan mol, massa molar, konsentrasi.
- `chemistry/solution.rs` — Kalkulasi larutan.

### Contoh Fungsi
```rust
use sciencecalc_rs::chemistry::stoichiometry;
let mol = stoichiometry::calculate_moles(18.0, 18.0); // Output: 1.0
```

---

## Konversi Sistem Bilangan

### Modul
- `math/basis.rs` — Konversi desimal, biner, oktal, heksadesimal.

### Contoh Fungsi
```rust
use sciencecalc_rs::math::basis;
let biner = basis::desimal_ke_biner(25); // Output: "11001"
```

---

## Contoh Penggunaan Lengkap

```rust
use sciencecalc_rs::math::algebra;
use sciencecalc_rs::physics::force;
use sciencecalc_rs::chemistry::stoichiometry;

fn main() {
    // Matematika
    let faktorial = algebra::factorial(5);
    println!("Faktorial 5: {}", faktorial);

    // Fisika
    let gaya = force::calculate_force(10.0, 9.8);
    println!("Gaya: {}", gaya);

    // Kimia
    let mol = stoichiometry::calculate_moles(18.0, 18.0);
    println!("Mol: {}", mol);
}
```

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
