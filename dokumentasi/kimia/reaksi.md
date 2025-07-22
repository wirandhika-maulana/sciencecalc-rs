# Reaksi

Modul [`reaksi.rs`](../src/kimia/reaksi.rs) pada pustaka **sciencecalc-rs** menyediakan fungsi untuk menghitung **massa produk reaksi** dan **persentase hasil reaksi kimia**.

---

## 📍 Fungsi Utama

| Fungsi                              | Parameter                                    | Return | Deskripsi                                                         |
|------------------------------------- |----------------------------------------------|--------|-------------------------------------------------------------------|
| `massa_produk(n, mr)`                | `n: jumlah mol`, `mr: massa molar (g/mol)`   | `f64`  | Menghitung **massa produk reaksi**: \(m = n \times Mr\)           |
| `persen_hasil(massa_aktual, massa_teoritis)` | `massa_aktual: massa aktual (g)`, `massa_teoritis: massa teoritis (g)` | `f64`  | Menghitung **persentase hasil reaksi**: \(%yield = \frac{massa\ aktual}{massa\ teoritis} \times 100\) |

---

## 📍 Penjelasan & Contoh Penggunaan

### 1. Menghitung Massa Produk Reaksi

```rust
use sciencecalc_rs::kimia::reaksi::massa_produk;

fn main() {
    let jumlah_mol = 2.0;   // mol
    let massa_molar = 18.0; // g/mol (contoh: air)
    let massa = massa_produk(jumlah_mol, massa_molar);
    println!("Massa produk reaksi: {:.2} gram", massa); // Output: Massa produk reaksi: 36.00 gram
}
```

---

### 2. Menghitung Persentase Hasil Reaksi

```rust
use sciencecalc_rs::kimia::reaksi::persen_hasil;

fn main() {
    let massa_aktual = 30.0;      // gram
    let massa_teoritis = 36.0;    // gram
    let persen = persen_hasil(massa_aktual, massa_teoritis);
    println!("Persentase hasil reaksi: {:.2}%", persen); // Output: Persentase hasil reaksi: 83.33%
}
```

---

## Catatan

- Pastikan satuan massa dalam **gram (g)** dan massa molar dalam **gram/mol (g/mol)**.
- Persentase hasil reaksi menunjukkan efisiensi reaksi terhadap hasil teoritis.

---

Dokumentasi ini mencakup seluruh fungsi reaksi kimia pada pustaka **sciencecalc-rs**.  
Semoga bermanfaat!

---
