//! A big integer library with good performance.
//!
//! The library implements efficient large integer arithmetic in pure Rust.
//!
//! The two integer types are [UBig](struct.UBig.html) (for unsigned integers)
//! and [IBig](struct.IBig.html) (for signed integers).
//!
//! ```
//! # use ibig::ParseError;
//! use ibig::prelude::*;
//!
//! let a = ubig!(12345678);
//! let b = ubig!(0x10ff);
//! let c = ibig!(-azz base 36);
//! let d: UBig = "15033211231241234523452345345787".parse()?;
//!
//! assert_eq!(c.to_string(), "-14255");
//! assert_eq!(
//!     (a * b.pow(10)).in_radix(16).to_string(),
//!     "1589bda8effbfc495d8d73c83d8b27f94954e"
//! );
//! assert_eq!(
//!     format!("hello {:#x}", d % ubig!(0xabcd1234134132451345)),
//!     "hello 0x1a7e7c487267d2658a93"
//! );
//! # Ok::<(), ParseError>(())
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use crate::{
    bits::{AndNot, NextPowerOfTwo},
    div_ops::{DivEuclid, DivRem, DivRemEuclid, RemEuclid},
    fmt::InRadix,
    ibig::IBig,
    parse::ParseError,
    primitive::OutOfBoundsError,
    sign::{Abs, UnsignedAbs},
    ubig::UBig,
};

mod add;
mod add_ops;
mod arch;
mod bits;
mod buffer;
mod cmp;
mod convert;
mod div;
mod div_ops;
mod fast_divide;
mod fmt;
mod ibig;
mod math;
mod mul;
mod mul_ops;
mod parse;
mod pow;
pub mod prelude;
mod primitive;
mod radix;
mod shift;
mod shift_ops;
mod sign;
mod ubig;

#[macro_use]
mod macros;

#[cfg(feature = "rand")]
mod random;
#[cfg(feature = "rand")]
pub use crate::random::{UniformIBig, UniformUBig};

#[cfg(feature = "num-traits")]
mod num_traits;
