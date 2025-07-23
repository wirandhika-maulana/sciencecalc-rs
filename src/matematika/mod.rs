/// Modul utama matematika
/// Main mathematics module

pub mod aritmetika;      // Aritmetika dasar / Basic arithmetic
pub mod aljabar;         // Aljabar / Algebra
pub mod geometri;        // Geometri / Geometry
pub mod statistika;      // Statistika / Statistics
pub mod kombinatorika;    // Kombinatorik & Peluang / Combinatorics & Probability
pub mod basis;  // Konversi basis bilangan / Number base conversion

// Re-export semua modul utama agar bisa diakses langsung dari math::...
// Re-export all main modules for direct access via math::...
pub use aritmetika::*;
pub use aljabar::*;
pub use geometri::*;
pub use statistika::*;
pub use kombinatorika::*;
pub use basis::*;
