/// Modular exponentiation
/// https://en.wikipedia.org/wiki/Modular_exponentiation

use std::ops::{Shr};
use num::{Bounded, Num, One, Zero};

/// straightforward implementation

pub fn sf(base: u32, exponent: u32, modulus: u32) -> u32 {
    base.pow(exponent) % modulus
}

// memory efficient implementation

pub fn meff(base: u32, exponent: u32, modulus: u32) -> u32 {
    if modulus == 0 {
        return 0
    }

    let mut result: u32 = 1;

    for _ in 1..exponent+1 {
        result = (result * base) % modulus
    }

    result
}
