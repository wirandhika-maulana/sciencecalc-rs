//! ScienceCalc - Toolkit perhitungan Matematika, Fisika, dan Kimia untuk Rust.
//! ScienceCalc - Math, Physics, and Chemistry calculations toolkit for Rust.

// Modul Matematika
pub mod matematika {
    pub mod aritmetika;      // Aritmetika dasar / Basic arithmetic
    pub mod aljabar;         // Aljabar / Algebra
    pub mod trigonometri;    // Trigonometri / Trigonometry
    pub mod geometri;        // Geometri / Geometry
    pub mod statistika;      // Statistika / Statistics
    pub mod kombinatorika;    // Kombinatorik & Peluang / Combinatorics & Probability
    pub mod basis;  // Konversi basis bilangan / Number base conversion
}

// Modul Fisika
pub mod fisika {
    pub mod gaya;          // Force
    pub mod gerak;         // Motion
    pub mod listrik;       // Electricity
    pub mod energi;        // Energy
}

// Modul Kimia
pub mod kimia {
    pub mod stoikiometri;  // Stoichiometry
    pub mod gas;           // Gas
    pub mod larutan;       // Solution
    pub mod reaksi;        // Reaction
}
