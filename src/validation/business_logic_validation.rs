use crate::error::BusinessRuleError;

/// A synchronous business rule over an object of type `T`.
pub trait BusinessLogicValidation<T: ?Sized> {
    /// Validates the object; returns the business error if the rule is violated.
    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError>;
}
