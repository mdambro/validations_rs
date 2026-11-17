//! Running a list of synchronous rules: [`validate`] and [`validate_all`].

use crate::error::{Violation, Violations};
use crate::rule::Rule;

/// Checks the rules in list order and stops at the first violation.
///
/// Put cheap rules first: the ones after a violation do not run. The value goes first so that the
/// list's element type is known, and a list of plain functions needs no type annotation.
///
/// ```
/// use validations_rs::rules::{Length, NotBlank};
/// use validations_rs::validate;
///
/// assert!(validate("Ada", &[&NotBlank, &Length::between(2, 40)]).is_ok());
/// assert_eq!(validate("  ", &[&NotBlank, &Length::between(2, 40)]).unwrap_err().code().as_str(), "VALUE_BLANK");
/// ```
///
/// # Errors
///
/// Returns the violation of the first rule that fails.
pub fn validate<T: ?Sized>(value: &T, rules: &[&dyn Rule<T>]) -> Result<(), Violation> {
    for rule in rules {
        rule.check(value)?;
    }
    Ok(())
}

/// Checks every rule and returns all the violations, in list order.
///
/// Use it where the caller should see every problem at once, such as a form or a request with several
/// fields. Use [`validate`] when later rules are expensive or only make sense if earlier ones passed.
///
/// # Errors
///
/// Returns the violations of all the rules that fail.
pub fn validate_all<T: ?Sized>(value: &T, rules: &[&dyn Rule<T>]) -> Result<(), Violations> {
    let violations: Vec<Violation> = rules.iter().filter_map(|rule| rule.check(value).err()).collect();
    match Violations::from_vec(violations) {
        None => Ok(()),
        Some(violations) => Err(violations),
    }
}
