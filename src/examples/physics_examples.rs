use sciencecalc_rs::physics::force::force;
use sciencecalc_rs::physics::motion::{glbb_displacement, glbb_final_velocity};
use sciencecalc_rs::physics::electricity::{ohm_voltage, ohm_current, ohm_resistance};
use sciencecalc_rs::physics::energy::{kinetic_energy, potential_energy};

fn main() {
    println!("=== Force ===");
    println!("Force (m=10kg, a=9.8m/s^2): {}", force(10.0, 9.8));

    println!("\n=== Motion (GLBB) ===");
    println!("Displacement (v0=2, t=3, a=1): {}", glbb_displacement(2.0, 3.0, 1.0));
    println!("Final velocity (v0=2, a=1, t=3): {}", glbb_final_velocity(2.0, 1.0, 3.0));

    println!("\n=== Electricity ===");
    println!("Voltage (I=2A, R=5Ω): {}", ohm_voltage(2.0, 5.0));
    println!("Current (V=10V, R=5Ω): {}", ohm_current(10.0, 5.0));
    println!("Resistance (V=10V, I=2A): {}", ohm_resistance(10.0, 2.0));

    println!("\n=== Energy ===");
    println!("Kinetic energy (m=5kg, v=3m/s): {}", kinetic_energy(5.0, 3.0));
    println!("Potential energy (m=5kg, g=9.8m/s^2, h=2m): {}", potential_energy(5.0, 9.8, 2.0));
}
