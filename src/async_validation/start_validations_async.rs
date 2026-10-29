use super::AsyncBusinessLogicValidation;
use crate::error::BusinessRuleError;

/// Runs asynchronous validations in list order and stops at the first one that fails.
pub async fn start_validations_async<T: Sync + ?Sized>(
    validations: &[&dyn AsyncBusinessLogicValidation<T>],
    to_validate: &T,
) -> Result<(), BusinessRuleError> {
    for validation in validations {
        validation.validate(to_validate).await?;
    }
    Ok(())
}
