//! ScienceCalc - Toolkit perhitungan Matematika, Fisika, dan Kimia untuk Rust.
//! ScienceCalc - Math, Physics, and Chemistry calculations toolkit for Rust.

// Modul Matematika
pub mod math {
    pub mod aljabar;
    pub mod basis;
    pub mod geometri;
    pub mod aritmetika;
    pub mod trigonometri;
    pub mod statistika;
}

// Modul Fisika
pub mod physics {
    pub mod gaya;          // Force
    pub mod gerak;         // Motion
    pub mod listrik;       // Electricity
    pub mod energi;        // Energy
}

// Modul Kimia
pub mod chemistry {
    pub mod stoikiometri;  // Stoichiometry
    pub mod gas;           // Gas
    pub mod larutan;       // Solution
    pub mod reaksi;        // Reaction
}
