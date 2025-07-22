use sciencecalc_rs::chemistry::stoikiometri::jumlah_mol;
use sciencecalc_rs::chemistry::gas::tekanan_gas_ideal;
use sciencecalc_rs::chemistry::larutan::{molaritas, ph_asam_kuat};
use sciencecalc_rs::chemistry::reaksi::{massa_produk, persen_hasil};

fn main() {
    println!("=== Stoikiometri ===");
    println!("Jumlah mol (massa=18g, massa molar=18g/mol): {}", jumlah_mol(18.0, 18.0));

    println!("\n=== Hukum Gas Ideal ===");
    println!(
        "Tekanan gas ideal (n=2, R=0.082, T=300K, V=10L): {}",
        tekanan_gas_ideal(2.0, 0.082, 300.0, 10.0)
    );

    println!("\n=== Larutan ===");
    println!("Molaritas (jumlah mol=2, volume=1L): {}", molaritas(2.0, 1.0));
    println!("pH asam kuat ([H+]=0.01): {}", ph_asam_kuat(0.01));

    println!("\n=== Reaksi ===");
    println!("Massa produk (jumlah mol=2, massa molar=18g/mol): {}", massa_produk(2.0, 18.0));
    println!(
        "Persen hasil (aktual=15g, teoritis=18g): {:.2}%",
        persen_hasil(15.0, 18.0)
    );
}
