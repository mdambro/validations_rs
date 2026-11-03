use super::BusinessLogicValidation;
use crate::error::BusinessRuleError;

/// Runs every validation in list order and returns all the failures.
///
/// Use it where the caller should see every problem at once, such as a form or an API request with
/// several fields. Use [`start_validations`](super::start_validations) when later rules are expensive or
/// only make sense if the earlier ones passed.
///
/// ```
/// use validations_rs::{collect_validations, LengthBetweenValidation, NotBlankValidation};
///
/// let errors = collect_validations::<str>(
///     &[&NotBlankValidation::new(), &LengthBetweenValidation::new(2, 40)],
///     " ",
/// )
/// .unwrap_err();
/// assert_eq!(errors.len(), 2);
/// ```
///
/// # Errors
///
/// Returns the errors of all the validations that fail, in list order. The vector is never empty.
pub fn collect_validations<T: ?Sized>(
    validations: &[&dyn BusinessLogicValidation<T>],
    to_validate: &T,
) -> Result<(), Vec<BusinessRuleError>> {
    let errors: Vec<BusinessRuleError> =
        validations.iter().filter_map(|validation| validation.validate(to_validate).err()).collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
