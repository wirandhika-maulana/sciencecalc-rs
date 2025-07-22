/// Modul utama matematika (math)
/// Main mathematics module

pub mod aritmetika;      // Aritmetika dasar / Basic arithmetic
pub mod aljabar;         // Aljabar / Algebra
pub mod trigonometri;    // Trigonometri / Trigonometry
pub mod geometri;        // Geometri / Geometry
pub mod statistika;      // Statistika / Statistics
pub mod kombinatorika;    // Kombinatorik & Peluang / Combinatorics & Probability

pub use aritmetika::*;
pub use aljabar::*;
pub use trigonometri::*;
pub use geometri::*;
pub use statistika::*;
pub use kombinatorika::*;
