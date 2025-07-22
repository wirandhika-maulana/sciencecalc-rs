use sciencecalc_rs::chemistry::stoichiometry::moles;
use sciencecalc_rs::chemistry::gas::ideal_gas_pressure;
use sciencecalc_rs::chemistry::solution::{molarity, ph_strong_acid};
use sciencecalc_rs::chemistry::reaction::{product_mass, percent_yield};

fn main() {
    println!("=== Stoichiometry ===");
    println!("Moles (mass=18g, molar mass=18g/mol): {}", moles(18.0, 18.0));

    println!("\n=== Gas Law ===");
    println!("Ideal Gas Pressure (n=2, R=0.082, T=300K, V=10L): {}", ideal_gas_pressure(2.0, 0.082, 300.0, 10.0));

    println!("\n=== Solution ===");
    println!("Molarity (moles=2, volume=1L): {}", molarity(2.0, 1.0));
    println!("pH of strong acid ([H+]=0.01): {}", ph_strong_acid(0.01));

    println!("\n=== Reaction ===");
    println!("Product mass (moles=2, molar mass=18g/mol): {}", product_mass(2.0, 18.0));
    println!("Percent yield (actual=15g, theoretical=18g): {:.2}%", percent_yield(15.0, 18.0));
}
