//! Interfaz de validación y ejecutor ordenado.

use crate::error::BusinessRuleError;

/// **Librería:** `validations` · **Funcionalidad:** una regla de negocio sobre un objeto `T`.
/// Equivale a `BusinessLogicValidation<T>` de Java. Una validación, una regla, un código;
/// sin estado, sin I/O y sin modificar el objeto validado.
pub trait BusinessLogicValidation<T: ?Sized> {
    /// Orden de ejecución: de menor a mayor.
    fn execution_order(&self) -> i16;

    /// Valida el objeto; devuelve el error de negocio si la regla se viola.
    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError>;
}

/// **Librería:** `validations` · **Funcionalidad:** ejecuta una lista de validaciones en orden
/// ascendente de `execution_order` (empate: orden de la lista) y corta en la primera que falla.
/// Equivale a `BusinessLogicValidationUtils.startValidations`.
pub fn start_validations<T: ?Sized>(
    validations: &[&dyn BusinessLogicValidation<T>],
    to_validate: &T,
) -> Result<(), BusinessRuleError> {
    let mut ordered: Vec<&dyn BusinessLogicValidation<T>> = validations.to_vec();
    // `sort_by_key` es estable: los empates conservan el orden de la lista.
    ordered.sort_by_key(|v| v.execution_order());
    for validation in ordered {
        validation.validate(to_validate)?;
    }
    Ok(())
}
