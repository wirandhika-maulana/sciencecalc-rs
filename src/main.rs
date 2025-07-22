fn main() {
    // MATEMATIKA
    println!(
        "Aritmetika (Penjumlahan 5 + 3): {}",
        sciencecalc_rs::matematika::aritmetika::tambah(5.0, 3.0)
    );
    println!(
        "Aljabar (Linear 1x1, 2x + 3 = 11): {:?}",
        sciencecalc_rs::matematika::aljabar::persamaan_linear_1x1(2.0, 3.0, 11.0)
    );
    println!(
        "Statistika (Rata-rata [1,2,3]): {}",
        sciencecalc_rs::matematika::statistika::rata_rata(&[1.0, 2.0, 3.0])
    );

    // FISIKA
    println!(
        "Gaya (m=10, a=9.8): {}",
        sciencecalc_rs::fisika::gaya::gaya(10.0, 9.8)
    );
    println!(
        "Energi Kinetik (m=2, v=3): {}",
        sciencecalc_rs::fisika::energi::energi_kinetik(2.0, 3.0)
    );
    println!(
        "Tegangan listrik (I=2, R=5): {}",
        sciencecalc_rs::fisika::listrik::ohm_tegangannya(2.0, 5.0)
    );

    // KIMIA
    println!(
        "Jumlah mol (massa=18g, massa molar=18g/mol): {}",
        sciencecalc_rs::kimia::stoikiometri::jumlah_mol(18.0, 18.0)
    );
    println!(
        "Molaritas (0.5 mol, 2 L): {}",
        sciencecalc_rs::kimia::larutan::molaritas(0.5, 2.0)
    );
    println!(
        "Tekanan gas ideal (n=2, R=0.082, T=300, V=10): {}",
        sciencecalc_rs::kimia::gas::tekanan_gas_ideal(2.0, 0.082, 300.0, 10.0)
    );
}
