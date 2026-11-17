//! Adapters that change what a rule reports or what it checks, in the style of iterator adapters.

use std::borrow::Cow;
use std::fmt;
use std::marker::PhantomData;

use crate::error::{RuleCode, Violation};
use crate::rule::Rule;

/// Adapter methods available on every rule: bring them into scope with `use validations_rs::RuleExt`
/// (or the [`prelude`](crate::prelude)).
///
/// ```
/// use validations_rs::rules::{Min, NotBlank};
/// use validations_rs::{validate, RuleCode, RuleExt};
///
/// struct Signup {
///     name: String,
///     age: u8,
/// }
///
/// let signup = Signup { name: "Ada".into(), age: 16 };
/// let violation = validate(&signup, &[
///     &NotBlank.on("name", |s: &Signup| &s.name),
///     &Min(18).with_code(RuleCode("UNDERAGE")).on("age", |s: &Signup| &s.age),
/// ])
/// .unwrap_err();
///
/// assert_eq!(violation.code(), RuleCode("UNDERAGE"));
/// assert_eq!(violation.field(), Some("age"));
/// ```
///
/// The trait is implemented for every type, so its methods appear wherever it is imported; they only
/// produce a rule when the wrapped value is one.
pub trait RuleExt: Sized {
    /// Reports `message` instead of the rule's own message.
    fn with_message(self, message: impl Into<Cow<'static, str>>) -> WithMessage<Self> {
        WithMessage { rule: self, message: message.into() }
    }

    /// Reports `code` instead of the rule's own code, for example to give a catalog rule a business
    /// name.
    fn with_code(self, code: RuleCode) -> WithCode<Self> {
        WithCode { rule: self, code }
    }

    /// Applies the rule to one field of a larger value and records the field name in the violation.
    fn on<O, T, F>(self, field: &'static str, get: F) -> On<Self, F, O, T>
    where
        O: ?Sized,
        T: ?Sized,
        F: Fn(&O) -> &T,
        Self: Rule<T>,
    {
        On { rule: self, field, get, _types: PhantomData }
    }
}

impl<R> RuleExt for R {}

/// A rule that reports a custom message. Created by [`RuleExt::with_message`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithMessage<R> {
    rule: R,
    message: Cow<'static, str>,
}

impl<T: ?Sized, R: Rule<T>> Rule<T> for WithMessage<R> {
    fn check(&self, value: &T) -> Result<(), Violation> {
        self.rule.check(value).map_err(|violation| violation.with_message(self.message.clone()))
    }
}

/// A rule that reports a custom code. Created by [`RuleExt::with_code`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithCode<R> {
    rule: R,
    code: RuleCode,
}

impl<T: ?Sized, R: Rule<T>> Rule<T> for WithCode<R> {
    fn check(&self, value: &T) -> Result<(), Violation> {
        self.rule.check(value).map_err(|violation| violation.with_code(self.code))
    }
}

/// A rule applied to one field of a larger value. Created by [`RuleExt::on`].
pub struct On<R, F, O: ?Sized, T: ?Sized> {
    rule: R,
    field: &'static str,
    get: F,
    _types: PhantomData<fn(&O) -> &T>,
}

impl<O, T, R, F> Rule<O> for On<R, F, O, T>
where
    O: ?Sized,
    T: ?Sized,
    R: Rule<T>,
    F: Fn(&O) -> &T,
{
    fn check(&self, value: &O) -> Result<(), Violation> {
        self.rule.check((self.get)(value)).map_err(|violation| violation.at_field(self.field))
    }
}

impl<R: Clone, F: Clone, O: ?Sized, T: ?Sized> Clone for On<R, F, O, T> {
    fn clone(&self) -> Self {
        Self { rule: self.rule.clone(), field: self.field, get: self.get.clone(), _types: PhantomData }
    }
}

impl<R: fmt::Debug, F, O: ?Sized, T: ?Sized> fmt::Debug for On<R, F, O, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("On").field("rule", &self.rule).field("field", &self.field).finish_non_exhaustive()
    }
}
