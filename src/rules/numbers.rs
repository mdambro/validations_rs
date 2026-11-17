use std::fmt::Display;

use super::codes;
use crate::error::Violation;
use crate::rule::Rule;

/// The value is at least the minimum. Works with any ordered, displayable type; `NaN` is rejected.
///
/// ```
/// use validations_rs::rules::Min;
/// use validations_rs::Rule;
///
/// assert!(Min(18).check(&18).is_ok());
/// assert_eq!(Min(18).check(&16).unwrap_err().detail("min"), Some("18"));
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Min<T>(pub T);

impl<T: PartialOrd + Display> Rule<T> for Min<T> {
    fn check(&self, value: &T) -> Result<(), Violation> {
        if *value >= self.0 {
            Ok(())
        } else {
            Err(Violation::new(codes::VALUE_BELOW_MIN, "The value is less than the minimum allowed.")
                .with_detail("value", value)
                .with_detail("min", &self.0))
        }
    }
}

/// The value is at most the maximum. Works with any ordered, displayable type; `NaN` is rejected.
///
/// ```
/// use validations_rs::rules::Max;
/// use validations_rs::Rule;
///
/// assert!(Max(10.0).check(&9.5).is_ok());
/// assert!(Max(10.0).check(&f64::NAN).is_err());
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Max<T>(pub T);

impl<T: PartialOrd + Display> Rule<T> for Max<T> {
    fn check(&self, value: &T) -> Result<(), Violation> {
        if *value <= self.0 {
            Ok(())
        } else {
            Err(Violation::new(codes::VALUE_ABOVE_MAX, "The value is greater than the maximum allowed.")
                .with_detail("value", value)
                .with_detail("max", &self.0))
        }
    }
}

/// The value is between `min` and `max`, inclusive. `NaN` is rejected.
///
/// ```
/// use validations_rs::rules::InRange;
/// use validations_rs::Rule;
///
/// assert!(InRange::new(1, 10).check(&10).is_ok());
/// assert!(InRange::new(1, 10).check(&11).is_err());
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct InRange<T> {
    min: T,
    max: T,
}

impl<T> InRange<T> {
    /// Between `min` and `max`, inclusive.
    #[must_use]
    pub const fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

impl<T: PartialOrd + Display> Rule<T> for InRange<T> {
    fn check(&self, value: &T) -> Result<(), Violation> {
        if *value >= self.min && *value <= self.max {
            Ok(())
        } else {
            Err(Violation::new(codes::VALUE_OUT_OF_RANGE, "The value is outside the allowed range.")
                .with_detail("value", value)
                .with_detail("min", &self.min)
                .with_detail("max", &self.max))
        }
    }
}

mod sealed {
    pub trait Sealed {}
}

/// A primitive number with a zero, used by the sign rules ([`Positive`], [`Negative`], and their
/// "or zero" variants). Implemented for every integer and floating-point primitive; sealed.
pub trait Number: sealed::Sealed + PartialOrd + Display + Copy {
    /// The zero of the type.
    const ZERO: Self;
}

macro_rules! impl_number {
    ($zero:expr => $($ty:ty),*) => {
        $(
            impl sealed::Sealed for $ty {}
            impl Number for $ty {
                const ZERO: Self = $zero;
            }
        )*
    };
}

impl_number!(0 => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_number!(0.0 => f32, f64);

macro_rules! sign_rule {
    ($(#[$doc:meta])* $name:ident, $holds:expr, $code:expr, $message:expr) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl<T: Number> Rule<T> for $name {
            fn check(&self, value: &T) -> Result<(), Violation> {
                let holds: fn(&T, &T) -> bool = $holds;
                if holds(value, &T::ZERO) {
                    Ok(())
                } else {
                    Err(Violation::new($code, $message).with_detail("value", value))
                }
            }
        }
    };
}

sign_rule!(
    /// The value is greater than zero. `NaN` is rejected.
    ///
    /// ```
    /// use validations_rs::rules::Positive;
    /// use validations_rs::Rule;
    ///
    /// assert!(Positive.check(&1).is_ok());
    /// assert!(Positive.check(&0).is_err());
    /// ```
    Positive, |value, zero| value > zero, codes::VALUE_NOT_POSITIVE, "The value must be greater than zero."
);
sign_rule!(
    /// The value is zero or greater. `NaN` is rejected.
    PositiveOrZero, |value, zero| value >= zero, codes::VALUE_NEGATIVE, "The value must not be negative."
);
sign_rule!(
    /// The value is less than zero. `NaN` is rejected.
    Negative, |value, zero| value < zero, codes::VALUE_NOT_NEGATIVE, "The value must be less than zero."
);
sign_rule!(
    /// The value is zero or less. `NaN` is rejected.
    NegativeOrZero, |value, zero| value <= zero, codes::VALUE_POSITIVE, "The value must not be positive."
);
