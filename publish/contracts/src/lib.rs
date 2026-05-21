#![no_std]

// AUTO-GENERATED module composition
mod modules {
    pub mod payments;
}

pub use modules::payments::*;

// shared platform constants
pub const FEE_BPS: i128 = 100;
pub const MIN_DEPOSIT: i128 = 1000000;
pub const GRACE_SECS: i128 = 604800;
