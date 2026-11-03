use crate::error::BusinessRuleError;

/// A synchronous business rule over an object of type `T`.
pub trait BusinessLogicValidation<T: ?Sized> {
    /// Validates the object.
    ///
    /// # Errors
    ///
    /// Returns the business error if the object breaks the rule.
    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError>;
}
