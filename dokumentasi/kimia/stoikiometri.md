# Stoikiometri

Modul [`stoikiometri.rs`](../src/kimia/stoikiometri.rs) pada pustaka **sciencecalc-rs** menyediakan fungsi untuk menghitung **jumlah mol** berdasarkan massa dan massa molar.

---

## 📍 Fungsi Utama

| Fungsi                             | Parameter                               | Return | Deskripsi                                                         |
|-------------------------------------|-----------------------------------------|--------|-------------------------------------------------------------------|
| `jumlah_mol(massa, massa_molar)`   | `massa: massa (g)`, `massa_molar: massa molar (g/mol)` | `f64`  | Menghitung **jumlah mol**: \(n = \frac{m}{M_r}\)                  |

---

## 📍 Penjelasan & Contoh Penggunaan

```rust
use sciencecalc_rs::math::stoikiometri::jumlah_mol;

fn main() {
    let massa = 18.0;         // gram (contoh: air)
    let massa_molar = 18.0;   // gram/mol
    let n = jumlah_mol(massa, massa_molar);
    println!("Jumlah mol: {:.2} mol", n); // Output: Jumlah mol: 1.00 mol
}
```

---

## Catatan

- Pastikan satuan **massa** dalam gram (g) dan **massa molar** dalam gram/mol (g/mol).
- Fungsi ini digunakan untuk konversi massa ke mol dalam perhitungan stoikiometri reaksi kimia.

---

Dokumentasi ini mencakup seluruh fungsi stoikiometri pada pustaka **sciencecalc-rs**.  
Semoga bermanfaat!

---
