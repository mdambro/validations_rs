//! Ready-made rules for common checks, grouped by the kind of value they check.
//!
//! Rules without configuration are unit structs used as values (`NotBlank`); rules with configuration
//! are built with their values (`Min(18)`, `Length::between(2, 40)`). Every rule reports a stable code
//! from [`codes`]; change it with [`RuleExt::with_code`](crate::RuleExt::with_code) and the message with
//! [`RuleExt::with_message`](crate::RuleExt::with_message).
//!
//! The catalog is not re-exported at the crate root, so its names (such as `Email`) do not collide
//! with your own types: write `rules::Email`, or import the ones you use.

mod collections;
mod numbers;
mod presence;
mod text;
mod time;

pub use collections::{Count, NotEmpty};
pub use numbers::{InRange, Max, Min, Negative, NegativeOrZero, Number, Positive, PositiveOrZero};
pub use presence::{IsFalse, IsNone, IsSome, IsTrue};
pub use text::{Email, Length, NotBlank};
pub use time::{Future, Past};

/// The stable codes the catalog rules report.
pub mod codes {
    use crate::RuleCode;

    /// [`NotBlank`](super::NotBlank): the text is empty or only whitespace.
    pub const VALUE_BLANK: RuleCode = RuleCode("VALUE_BLANK");
    /// [`Length`](super::Length): the number of characters is out of range.
    pub const VALUE_LENGTH_OUT_OF_RANGE: RuleCode = RuleCode("VALUE_LENGTH_OUT_OF_RANGE");
    /// [`Email`](super::Email): the text is not a well-formed email address.
    pub const EMAIL_INVALID: RuleCode = RuleCode("EMAIL_INVALID");
    /// [`Min`](super::Min): the value is below the minimum.
    pub const VALUE_BELOW_MIN: RuleCode = RuleCode("VALUE_BELOW_MIN");
    /// [`Max`](super::Max): the value is above the maximum.
    pub const VALUE_ABOVE_MAX: RuleCode = RuleCode("VALUE_ABOVE_MAX");
    /// [`InRange`](super::InRange): the value is outside the range.
    pub const VALUE_OUT_OF_RANGE: RuleCode = RuleCode("VALUE_OUT_OF_RANGE");
    /// [`Positive`](super::Positive): the value is not greater than zero.
    pub const VALUE_NOT_POSITIVE: RuleCode = RuleCode("VALUE_NOT_POSITIVE");
    /// [`PositiveOrZero`](super::PositiveOrZero): the value is negative.
    pub const VALUE_NEGATIVE: RuleCode = RuleCode("VALUE_NEGATIVE");
    /// [`Negative`](super::Negative): the value is not less than zero.
    pub const VALUE_NOT_NEGATIVE: RuleCode = RuleCode("VALUE_NOT_NEGATIVE");
    /// [`NegativeOrZero`](super::NegativeOrZero): the value is positive.
    pub const VALUE_POSITIVE: RuleCode = RuleCode("VALUE_POSITIVE");
    /// [`IsSome`](super::IsSome): a required value is missing.
    pub const VALUE_MISSING: RuleCode = RuleCode("VALUE_MISSING");
    /// [`IsNone`](super::IsNone): a value that must be absent is present.
    pub const VALUE_PRESENT: RuleCode = RuleCode("VALUE_PRESENT");
    /// [`IsTrue`](super::IsTrue): the value is not `true`.
    pub const VALUE_NOT_TRUE: RuleCode = RuleCode("VALUE_NOT_TRUE");
    /// [`IsFalse`](super::IsFalse): the value is not `false`.
    pub const VALUE_NOT_FALSE: RuleCode = RuleCode("VALUE_NOT_FALSE");
    /// [`NotEmpty`](super::NotEmpty): the collection is empty.
    pub const COLLECTION_EMPTY: RuleCode = RuleCode("COLLECTION_EMPTY");
    /// [`Count`](super::Count): the number of elements is out of range.
    pub const SIZE_OUT_OF_RANGE: RuleCode = RuleCode("SIZE_OUT_OF_RANGE");
    /// [`Past`](super::Past): the date is not in the past.
    pub const DATE_NOT_PAST: RuleCode = RuleCode("DATE_NOT_PAST");
    /// [`Future`](super::Future): the date is not in the future.
    pub const DATE_NOT_FUTURE: RuleCode = RuleCode("DATE_NOT_FUTURE");
}
