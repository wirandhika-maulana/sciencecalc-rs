/// Modul utama fisika
/// Main physics module

pub mod listrik;    // Listrik / Electricity
pub mod energi;     // Energi / Energy
pub mod gaya;       // Gaya / Force
pub mod gerak;      // Gerak / Motion

// Re-export semua modul utama agar bisa diakses langsung dari physics::...
// Re-export all main modules for direct access via physics::...
pub use listrik::*;
pub use energi::*;
pub use gaya::*;
pub use gerak::*;
