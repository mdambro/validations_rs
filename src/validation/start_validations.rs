use super::BusinessLogicValidation;
use crate::error::BusinessRuleError;

/// Runs validations in list order and stops at the first one that fails.
pub fn start_validations<T: ?Sized>(
    validations: &[&dyn BusinessLogicValidation<T>],
    to_validate: &T,
) -> Result<(), BusinessRuleError> {
    for validation in validations {
        validation.validate(to_validate)?;
    }
    Ok(())
}
