//! Ordered business-rule validation with stable error codes.
//!
//! A **rule** checks one value and returns `Ok(())` or a [`Violation`] with a stable [`RuleCode`]. Any
//! function with the signature `fn(&T) -> Result<(), Violation>` is a rule, so most rules are plain
//! functions written with [`ensure`]. The [`rules`] module has a catalog of common checks, and
//! [`RuleExt`] adapts any rule: a business code, a message, or a field of a larger value.
//!
//! ```
//! use validations_rs::prelude::*;
//! use validations_rs::rules::{Email, Min, NotBlank};
//!
//! struct Signup {
//!     name: String,
//!     email: String,
//!     age: u8,
//!     accepted_terms: bool,
//! }
//!
//! fn terms_accepted(signup: &Signup) -> Result<(), Violation> {
//!     ensure(signup.accepted_terms, RuleCode("TERMS_NOT_ACCEPTED"), "you must accept the terms")
//! }
//!
//! let signup = Signup { name: " ".into(), email: "ada".into(), age: 16, accepted_terms: true };
//!
//! let violations = validate_all(&signup, &[
//!     &terms_accepted,
//!     &NotBlank.on("name", |s: &Signup| &s.name),
//!     &Email.on("email", |s: &Signup| &s.email),
//!     &Min(18).with_code(RuleCode("UNDERAGE")).on("age", |s: &Signup| &s.age),
//! ])
//! .unwrap_err();
//!
//! let codes: Vec<&str> = violations.iter().map(|v| v.code().as_str()).collect();
//! assert_eq!(codes, ["VALUE_BLANK", "EMAIL_INVALID", "UNDERAGE"]);
//! ```
//!
//! Rules that wait on I/O implement [`AsyncRule`] and run with [`validate_async`], which tells a broken
//! rule ([`AsyncError::Violation`]) apart from a rule that could not be checked
//! ([`AsyncError::CheckFailed`]).
//!
//! # Features
//!
//! - `serde`: implements `serde::Serialize` for [`RuleCode`], [`Violation`], and [`Violations`].

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod adapters;
mod async_rule;
mod error;
mod rule;
pub mod rules;
mod validate;

pub use adapters::{On, RuleExt, WithCode, WithMessage};
pub use async_rule::{validate_async, AsyncError, AsyncRule, RuleFuture};
pub use error::{RuleCode, Violation, Violations};
pub use rule::{ensure, Rule};
pub use validate::{validate, validate_all};

/// The traits and functions most programs use: `use validations_rs::prelude::*;`.
pub mod prelude {
    pub use crate::{
        ensure, validate, validate_all, validate_async, AsyncError, AsyncRule, Rule, RuleCode, RuleExt, RuleFuture,
        Violation, Violations,
    };
}

// Compile and run the README examples as doctests, so they cannot drift from the API.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

#[doc = include_str!("../README.es.md")]
#[cfg(doctest)]
pub struct ReadmeEsDoctests;
