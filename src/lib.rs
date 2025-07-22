//! ScienceCalc - Math, Physics and Chemistry calculations toolkit for Rust.

pub mod math {
    pub mod algebra;
    pub mod matrix;
    pub mod quadratic;
    pub mod linear;
    pub mod trigonometry;
    pub mod statistics;
}

pub mod physics {
    pub mod force;
    pub mod motion;
    pub mod electricity;
    pub mod energy;
}

pub mod chemistry {
    pub mod stoichiometry;
    pub mod gas;
    pub mod solution;
    pub mod reaction;
}
