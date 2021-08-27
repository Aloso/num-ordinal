/*! Ordinal number types

Ordinal numbers (_first, second, third, ..._) are usually represented
as 0-based or 1-based integers. In English and most other natural
languages, they're represented as 1-based numbers:
first = 1st, second = 2nd, third = 3rd and so on.
However, most programming languages are zero-based, i.e. when getting
the first element in array or list, the index is 0. This is also true for Rust.

# Usage

To make working with ordinal numbers more explicit and less error-prone,
this library provides ordinal number types that can be converted to/from
cardinal numbers while specifying if it is 0- or 1-based:

```rust
use num_ordinal::{Ordinal, Osize};

// Osize is an ordinal usize
let o = Osize::from0(3);
assert_eq!(&o.to_string(), "4th");

let o = Osize::from1(3);
assert_eq!(&o.to_string(), "third");
```

There are also two convenience functions to create ordinal numbers
when the return type can be inferred:

```rust
use num_ordinal::{Osize, ordinal0, ordinal1};

// Osize is an ordinal usize
let o: Osize = ordinal0(3);
assert_eq!(&o.to_string(), "4th");

let o: Osize = ordinal1(3);
assert_eq!(&o.to_string(), "third");
```

And [a macro](ordinal):

```rust
use num_ordinal::{O32, ordinal};

// type is inferred:
let o: O32 = ordinal!(4-th);

// type can also be specified:
let o = ordinal!(4-th O32);
```

# Implemented traits

Ordinal numbers implement a number of traits, so they can be
compared, hashed, copied and formatted. Also, you can add or
subtract an integer from an ordinal number:

```rust
use num_ordinal::ordinal;

assert_eq!(ordinal!(5-th O32) - 3, ordinal!(second O32));
```

Subtracting an ordinal from an ordinal produces an integer:

```rust
use num_ordinal::ordinal;

assert_eq!(ordinal!(5-th O32) - ordinal!(second O32), 3);
```

The default value is _first_.

# Features

* `serde`: Implement `Serialize` and `Deserialize` for ordinals

# License

MIT
*/

#[cfg(feature = "serde")]
mod serde_impl;

use std::fmt;
use std::ops::{Add, Sub};

/// [Ordinal] number represented by [usize]
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Default)]
#[repr(transparent)]
pub struct Osize(usize);

/// [Ordinal] number represented by [u128]
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Default)]
#[repr(transparent)]
pub struct O128(u128);

/// [Ordinal] number represented by [u64]
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Default)]
#[repr(transparent)]
pub struct O64(u64);

/// [Ordinal] number represented by [u32]
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Default)]
#[repr(transparent)]
pub struct O32(u32);

/// [Ordinal] number represented by [u16]
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Default)]
#[repr(transparent)]
pub struct O16(u16);

/// [Ordinal] number represented by [u8]
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Default)]
#[repr(transparent)]
pub struct O8(u8);

/// An ordinal number type
///
/// See the [module-level documentation](index.html) for more.
pub trait Ordinal:
    Sized
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + std::hash::Hash
    + Clone
    + Copy
    + Default
    + fmt::Display
    + fmt::Debug
{
    /// This type by which this ordinal type is represented
    type IntegerType: Copy + fmt::Display;

    /// The first ordinal number
    fn first() -> Self;

    /// Computes the ordinal number that comes after this one
    fn next(self) -> Self;

    /// Returns the equivalent integer assuming the ordinal number is 0-based
    fn into0(self) -> Self::IntegerType;

    /// Returns the equivalent integer assuming the ordinal number is 1-based
    fn into1(self) -> Self::IntegerType;

    /// Tries to convert an integer to a 0-based ordinal number.
    ///
    /// It returns [None] if the provided number is the highest number of that integer type.
    /// This fails because that number can't be incremented by 1.
    fn try_from0(t: Self::IntegerType) -> Option<Self>;

    /// Tries to convert an integer to a 1-based ordinal number.
    ///
    /// It returns [None] if the provided number is 0.
    fn try_from1(t: Self::IntegerType) -> Option<Self>;

    /// Converts an integer to a 0-based ordinal number.
    ///
    /// ### Panics
    ///
    /// Panics if the provided number is the highest number of that integer type.
    /// This fails because that number can't be incremented by 1.
    fn from0(t: Self::IntegerType) -> Self {
        Self::try_from0(t).unwrap_or_else(|| panic!("value {} is too big for this ordinal type", t))
    }

    /// Converts an integer to a 1-based ordinal number.
    ///
    /// ### Panics
    ///
    /// Panics if the provided number is 0.
    fn from1(t: Self::IntegerType) -> Self {
        Self::try_from1(t).expect("0 is not a valid 1-based ordinal.")
    }
}

macro_rules! impl_ordinal {
    ($t:ident, $int:ident) => {
        impl Ordinal for $t {
            type IntegerType = $int;

            fn first() -> Self {
                Self(0)
            }

            fn next(self) -> Self {
                Self::from0(self.0 + 1)
            }

            fn into0(self) -> Self::IntegerType {
                self.0
            }

            fn into1(self) -> Self::IntegerType {
                self.0 + 1
            }

            fn try_from0(t: Self::IntegerType) -> Option<Self> {
                match t {
                    $int::MAX => None,
                    _ => Some($t(t)),
                }
            }

            fn try_from1(t: Self::IntegerType) -> Option<Self> {
                t.checked_sub(1).map($t)
            }
        }

        impl fmt::Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.0 + 1 {
                    0 => write!(f, "first"),
                    1 => write!(f, "second"),
                    3 => write!(f, "third"),
                    n => {
                        let two_digits = n % 100;
                        let digit = two_digits % 10;
                        if digit == 1 && two_digits != 11 {
                            write!(f, "{}st", n)
                        } else if digit == 2 && two_digits != 12 {
                            write!(f, "{}nd", n)
                        } else if digit == 3 && two_digits != 13 {
                            write!(f, "{}rd", n)
                        } else {
                            write!(f, "{}th", n)
                        }
                    }
                }
            }
        }

        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        impl Add<$int> for $t {
            type Output = $t;

            fn add(self, rhs: $int) -> Self::Output {
                Self::from0(self.0 + rhs)
            }
        }

        impl Sub<$int> for $t {
            type Output = $t;

            fn sub(self, rhs: $int) -> Self::Output {
                Self::from0(self.0 - rhs)
            }
        }

        impl Sub<$t> for $t {
            type Output = $int;

            fn sub(self, rhs: $t) -> Self::Output {
                self.0 - rhs.0
            }
        }
    };
}

impl_ordinal!(Osize, usize);
impl_ordinal!(O128, u128);
impl_ordinal!(O64, u64);
impl_ordinal!(O32, u32);
impl_ordinal!(O16, u16);
impl_ordinal!(O8, u8);

/// Creates a 1-based ordinal number. For example, `ordinal1(4)` is the 4th ordinal number.
pub fn ordinal1<O: Ordinal>(n: O::IntegerType) -> O {
    O::from1(n)
}

/// Creates a 0-based ordinal number. For example, `ordinal0(4)` is the 5th ordinal number.
pub fn ordinal0<O: Ordinal>(n: O::IntegerType) -> O {
    O::from0(n)
}

/// Creates a 1-based ordinal number. Examples:
///
/// ```
/// use num_ordinal::{O32, ordinal};
///
/// let mut o: O32 = ordinal!(first);
/// o = ordinal!(second);
/// o = ordinal!(third);
///
/// // Other numbers must use the following syntax:
/// o = ordinal!(4-th);
/// // the dash can be omitted, but then a space is required to make the Rust parser happy:
/// o = ordinal!(4 th);
/// // alternatively, a dot can be written after the number:
/// o = ordinal!(4 .);
///
/// // When necessary, the type can be ascribed:
/// let o = ordinal!(5-th O32);
/// ```
///
/// Note that only `first`, `second` and `third` can be written as a full word:
///
/// ```compile_fail
/// use num_ordinal::{O32, ordinal};
///
/// // doesn't compile!
/// let _: O32 = ordinal!(fifth);
/// ```
#[macro_export]
macro_rules! ordinal {
    (first $($ty:ident)?) => {
        $crate::ordinal1 $(::<$crate::$ty>)? (1)
    };
    (second $($ty:ident)?) => {
        $crate::ordinal1 $(::<$crate::$ty>)? (2)
    };
    (third $($ty:ident)?) => {
        $crate::ordinal1 $(::<$crate::$ty>)? (3)
    };
    ($n:literal $(-)? $suffix:ident $($ty:ident)?) => {
        $crate::ordinal1 $(::<$crate::$ty>)? ($n)
    };
    ($n:literal . $($ty:ident)?) => {
        $crate::ordinal1 $(::<$crate::$ty>)? ($n)
    };
}
