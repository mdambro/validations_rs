//! The result of a broken rule: [`RuleCode`], [`Violation`], and [`Violations`].

use std::borrow::Cow;
use std::error::Error;
use std::fmt;

/// Stable, machine-readable code of a business rule, such as `"EMAIL_TAKEN"`.
///
/// Callers decide by code (an HTTP status, a translated message, a retry), never by the message text.
///
/// ```
/// use validations_rs::RuleCode;
///
/// const EMAIL_TAKEN: RuleCode = RuleCode("EMAIL_TAKEN");
/// assert_eq!(EMAIL_TAKEN.as_str(), "EMAIL_TAKEN");
/// assert_eq!(EMAIL_TAKEN.to_string(), "EMAIL_TAKEN");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RuleCode(pub &'static str);

impl RuleCode {
    /// The code as text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl fmt::Display for RuleCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl AsRef<str> for RuleCode {
    fn as_ref(&self) -> &str {
        self.0
    }
}

/// A broken business rule: its stable [`RuleCode`], a message for people, the field it applies to (if
/// any), and context details.
///
/// ```
/// use validations_rs::{RuleCode, Violation};
///
/// let violation = Violation::new(RuleCode("UNDERAGE"), "you must be of legal age")
///     .with_detail("min", 18)
///     .at_field("age");
///
/// assert_eq!(violation.code(), RuleCode("UNDERAGE"));
/// assert_eq!(violation.field(), Some("age"));
/// assert_eq!(violation.detail("min"), Some("18"));
/// assert_eq!(violation.to_string(), "[UNDERAGE] age: you must be of legal age");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Violation {
    code: RuleCode,
    message: Cow<'static, str>,
    field: Option<Cow<'static, str>>,
    details: Vec<(&'static str, String)>,
}

impl Violation {
    /// Creates a violation with a code and a message. A `&'static str` message is not copied.
    #[must_use]
    pub fn new(code: RuleCode, message: impl Into<Cow<'static, str>>) -> Self {
        Self { code, message: message.into(), field: None, details: Vec::new() }
    }

    /// Adds a context detail, such as the limit that was crossed.
    #[must_use]
    pub fn with_detail(mut self, key: &'static str, value: impl fmt::Display) -> Self {
        self.details.push((key, value.to_string()));
        self
    }

    /// Replaces the message and keeps the code.
    #[must_use]
    pub fn with_message(mut self, message: impl Into<Cow<'static, str>>) -> Self {
        self.message = message.into();
        self
    }

    /// Replaces the code and keeps the message.
    #[must_use]
    pub fn with_code(mut self, code: RuleCode) -> Self {
        self.code = code;
        self
    }

    /// Records the field the violation applies to. Applied again from an outer value, it builds a path:
    /// `.at_field("city").at_field("address")` gives `address.city`.
    #[must_use]
    pub fn at_field(mut self, name: &'static str) -> Self {
        self.field = Some(match self.field.take() {
            None => Cow::Borrowed(name),
            Some(inner) => Cow::Owned(format!("{name}.{inner}")),
        });
        self
    }

    /// The stable code of the broken rule.
    #[must_use]
    pub fn code(&self) -> RuleCode {
        self.code
    }

    /// The message for people.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// The field path the violation applies to, if any.
    #[must_use]
    pub fn field(&self) -> Option<&str> {
        self.field.as_deref()
    }

    /// The context details, in the order they were added.
    pub fn details(&self) -> impl Iterator<Item = (&'static str, &str)> + '_ {
        self.details.iter().map(|(key, value)| (*key, value.as_str()))
    }

    /// The value of one detail.
    #[must_use]
    pub fn detail(&self, key: &str) -> Option<&str> {
        self.details.iter().find(|(k, _)| *k == key).map(|(_, value)| value.as_str())
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.field {
            Some(field) => write!(f, "[{}] {field}: {}", self.code, self.message),
            None => write!(f, "[{}] {}", self.code, self.message),
        }
    }
}

impl Error for Violation {}

/// All the violations found by [`validate_all`](crate::validate_all), in rule order. Never empty.
///
/// ```
/// use validations_rs::rules::{Length, NotBlank};
/// use validations_rs::validate_all;
///
/// let violations = validate_all(" ", &[&NotBlank, &Length::between(2, 40)]).unwrap_err();
/// assert_eq!(violations.len(), 2);
/// for violation in &violations {
///     println!("{violation}");
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Violations(Vec<Violation>);

impl Violations {
    /// Wraps a list of violations; returns `None` if it is empty.
    #[must_use]
    pub fn from_vec(violations: Vec<Violation>) -> Option<Self> {
        if violations.is_empty() {
            None
        } else {
            Some(Self(violations))
        }
    }

    /// The number of violations, at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Always `false`: a `Violations` holds at least one violation.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The first violation, in rule order.
    #[must_use]
    pub fn first(&self) -> &Violation {
        &self.0[0]
    }

    /// The violations as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[Violation] {
        &self.0
    }

    /// Iterates over the violations.
    pub fn iter(&self) -> std::slice::Iter<'_, Violation> {
        self.0.iter()
    }

    /// Converts into the underlying vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<Violation> {
        self.0
    }
}

impl From<Violation> for Violations {
    fn from(violation: Violation) -> Self {
        Self(vec![violation])
    }
}

impl IntoIterator for Violations {
    type Item = Violation;
    type IntoIter = std::vec::IntoIter<Violation>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Violations {
    type Item = &'a Violation;
    type IntoIter = std::slice::Iter<'a, Violation>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl fmt::Display for Violations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, violation) in self.0.iter().enumerate() {
            if index > 0 {
                f.write_str("; ")?;
            }
            fmt::Display::fmt(violation, f)?;
        }
        Ok(())
    }
}

impl Error for Violations {}

#[cfg(feature = "serde")]
mod serde_impls {
    use serde::ser::{Serialize, SerializeMap, SerializeStruct, Serializer};

    use super::{RuleCode, Violation, Violations};

    impl Serialize for RuleCode {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_str(self.0)
        }
    }

    /// Serializes the details as a map: `{"min": "18"}`.
    struct Details<'a>(&'a [(&'static str, String)]);

    impl Serialize for Details<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut map = serializer.serialize_map(Some(self.0.len()))?;
            for (key, value) in self.0 {
                map.serialize_entry(key, value)?;
            }
            map.end()
        }
    }

    impl Serialize for Violation {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct("Violation", 4)?;
            state.serialize_field("code", &self.code)?;
            state.serialize_field("message", &self.message)?;
            state.serialize_field("field", &self.field)?;
            state.serialize_field("details", &Details(&self.details))?;
            state.end()
        }
    }

    impl Serialize for Violations {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.0.serialize(serializer)
        }
    }
}
