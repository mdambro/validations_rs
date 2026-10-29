//! # validations_rs
//!
//! Rust port of the Java library `mdambro/validations`: generic validation interfaces, synchronous and
//! asynchronous runners that follow the list order and stop at the first error, and a single business
//! error with a stable code.
//!
//! ```
//! use validations_rs::{start_validations, NotBlankValidation};
//!
//! let not_blank = NotBlankValidation::new();
//! assert!(start_validations(&[&not_blank], "hello").is_ok());
//! assert_eq!(start_validations(&[&not_blank], "  ").unwrap_err().code, NotBlankValidation::CODE);
//! ```

#![forbid(unsafe_code)]

pub mod async_validation;
pub mod error;
pub mod generic;
pub mod validation;

pub use async_validation::{start_validations_async, AsyncBusinessLogicValidation, ValidationFuture};
pub use error::{BusinessRuleError, RuleCode};
pub use generic::{
    AssertFalseValidation, AssertTrueValidation, EmailValidation, FutureValidation, LengthBetweenValidation,
    MaxValidation, MinValidation, NegativeOrZeroValidation, NegativeValidation, NotBlankValidation, NotEmptyValidation,
    NotNullValidation, NullValidation, PastValidation, PositiveOrZeroValidation, PositiveValidation,
    PredicateValidation, RangeValidation, SizeBetweenValidation,
};
pub use validation::{start_validations, BusinessLogicValidation};
