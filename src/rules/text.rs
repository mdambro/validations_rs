use super::codes;
use crate::error::Violation;
use crate::rule::Rule;

const MAX_EMAIL_LEN: usize = 254;
const MAX_LOCAL_LEN: usize = 64;
const MAX_LABEL_LEN: usize = 63;
const LOCAL_SPECIAL_CHARS: &str = "!#$%&'*+/=?^_`{|}~.-";

/// The text has at least one character that is not whitespace.
///
/// ```
/// use validations_rs::rules::NotBlank;
/// use validations_rs::Rule;
///
/// assert!(NotBlank.check("Ada").is_ok());
/// assert!(NotBlank.check(" \t").is_err());
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct NotBlank;

impl<T: AsRef<str> + ?Sized> Rule<T> for NotBlank {
    fn check(&self, value: &T) -> Result<(), Violation> {
        if value.as_ref().trim().is_empty() {
            Err(Violation::new(codes::VALUE_BLANK, "The value must not be blank."))
        } else {
            Ok(())
        }
    }
}

/// The text has between `min` and `max` characters, inclusive. Counts Unicode scalar values, not bytes.
///
/// ```
/// use validations_rs::rules::Length;
/// use validations_rs::Rule;
///
/// assert!(Length::between(2, 4).check("ñandú").is_err()); // five characters
/// assert!(Length::between(2, 5).check("ñandú").is_ok());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Length {
    min: usize,
    max: usize,
}

impl Length {
    /// Between `min` and `max` characters, inclusive.
    #[must_use]
    pub const fn between(min: usize, max: usize) -> Self {
        Self { min, max }
    }

    /// At least `min` characters.
    #[must_use]
    pub const fn at_least(min: usize) -> Self {
        Self { min, max: usize::MAX }
    }

    /// At most `max` characters.
    #[must_use]
    pub const fn at_most(max: usize) -> Self {
        Self { min: 0, max }
    }
}

impl<T: AsRef<str> + ?Sized> Rule<T> for Length {
    fn check(&self, value: &T) -> Result<(), Violation> {
        let length = value.as_ref().chars().count();
        if length < self.min || length > self.max {
            Err(Violation::new(codes::VALUE_LENGTH_OUT_OF_RANGE, "The length must be within the allowed range.")
                .with_detail("length", length)
                .with_detail("min", self.min)
                .with_detail("max", self.max))
        } else {
            Ok(())
        }
    }
}

/// The text looks like an email address.
///
/// A pragmatic structural check, not a full RFC 5322 implementation: it requires a local part and a
/// domain with at least two dot-separated labels, without spaces or consecutive dots. It does not check
/// that the domain or the mailbox exist.
///
/// ```
/// use validations_rs::rules::Email;
/// use validations_rs::Rule;
///
/// assert!(Email.check("ada@example.com").is_ok());
/// assert!(Email.check("ada@example").is_err());
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Email;

impl<T: AsRef<str> + ?Sized> Rule<T> for Email {
    fn check(&self, value: &T) -> Result<(), Violation> {
        let email = value.as_ref();
        if is_valid_email(email) {
            Ok(())
        } else {
            Err(Violation::new(codes::EMAIL_INVALID, "The email address does not have a valid format.")
                .with_detail("email", email))
        }
    }
}

fn is_valid_email(email: &str) -> bool {
    if email.len() > MAX_EMAIL_LEN {
        return false;
    }
    let Some((local, domain)) = email.split_once('@') else {
        return false;
    };
    is_valid_local_part(local) && is_valid_domain(domain)
}

fn is_valid_local_part(local: &str) -> bool {
    !local.is_empty()
        && local.len() <= MAX_LOCAL_LEN
        && !local.starts_with('.')
        && !local.ends_with('.')
        && !local.contains("..")
        && local.chars().all(|c| c.is_alphanumeric() || LOCAL_SPECIAL_CHARS.contains(c))
}

fn is_valid_domain(domain: &str) -> bool {
    let mut labels = 0;
    for label in domain.split('.') {
        let is_valid_label = !label.is_empty()
            && label.len() <= MAX_LABEL_LEN
            && !label.starts_with('-')
            && !label.ends_with('-')
            && label.chars().all(|c| c.is_alphanumeric() || c == '-');
        if !is_valid_label {
            return false;
        }
        labels += 1;
    }
    labels >= 2
}
