fn main() {
    // Import modul sesuai dengan struktur library kamu
    // Pastikan library sudah di-include di Cargo.toml dengan [lib] path yang benar
    // atau gunakan relative path jika masih dalam pengembangan

    // Matematika
    println!("Factorial 5: {}", sciencecalc_rs::math::algebra::factorial(5));
    println!("Linear 1x1 (2x + 3 = 11): {:?}", sciencecalc_rs::math::linear::solve_linear_1x1(2.0, 3.0, 11.0));
    
    // Fisika
    println!("Force (m=10, a=9.8): {}", sciencecalc_rs::physics::force::force(10.0, 9.8));
    
    // Kimia
    println!("Moles (mass=18g, molar mass=18g/mol): {}", sciencecalc_rs::chemistry::stoichiometry::moles(18.0, 18.0));
}
