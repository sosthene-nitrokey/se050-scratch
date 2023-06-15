#![no_std]

extern crate delog;
delog::generate_macros!();

pub mod se050;
pub mod t1;
pub mod types;

pub use crate::se050::{Se050, Se050Device};
pub use t1::T1overI2C;
pub use types::{
    ApduClass, ApduStandardInstruction, CApduByteIterator, DelayWrapper, ObjectId, RawCApdu,
};

pub(crate) mod macros;

#[cfg(test)]
mod tests;