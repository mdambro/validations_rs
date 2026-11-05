use super::{AsyncBusinessLogicValidation, AsyncValidationError};

/// Runs asynchronous validations in list order and stops at the first one that fails.
///
/// # Errors
///
/// Returns the error of the first validation that fails, unchanged: a
/// [`Violation`](AsyncValidationError::Violation) or a [`CheckFailed`](AsyncValidationError::CheckFailed).
/// The validations after it do not run.
pub async fn start_validations_async<T: Sync + ?Sized>(
    validations: &[&dyn AsyncBusinessLogicValidation<T>],
    to_validate: &T,
) -> Result<(), AsyncValidationError> {
    for validation in validations {
        validation.validate(to_validate).await?;
    }
    Ok(())
}
