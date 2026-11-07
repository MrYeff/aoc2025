use std::ops::{Shl, Shr};

use derive_more::{BitAnd, BitOr, Not};

#[derive(BitAnd, BitOr, Not, Default, Clone, Copy)]
pub struct U256(u128, u128);

impl From<usize> for U256 {
    fn from(value: usize) -> Self {
        U256(0, value as u128)
    }
}

impl U256 {
    pub const ZERO: U256 = U256(0, 0);
    pub const ONE: U256 = U256(0, 1);

    pub fn count_ones(&self) -> usize {
        self.0.count_ones() as usize + self.1.count_ones() as usize
    }
}

impl Shr<usize> for U256 {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self {
        let Self(high, low) = self;

        match rhs {
            0 => self,
            1..=127 => Self(high >> rhs, (low >> rhs) | (high << (128 - rhs))),
            128..=255 => Self(0, high >> (rhs - 128)),
            _ => Self::ZERO,
        }
    }
}

impl Shl<usize> for U256 {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self {
        let Self(high, low) = self;

        match rhs {
            0 => self,
            1..=127 => Self((high << rhs) | (low >> (128 - rhs)), low << rhs),
            128..=255 => Self(low << (rhs - 128), 0),
            _ => Self::ZERO,
        }
    }
}
