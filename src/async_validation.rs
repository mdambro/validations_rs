//! Variante asíncrona, para validaciones de la capa de aplicación que consultan puertos.

use std::future::Future;
use std::pin::Pin;

use crate::error::BusinessRuleError;

/// Futuro devuelto por una validación asíncrona.
pub type ValidationFuture<'a> = Pin<Box<dyn Future<Output = Result<(), BusinessRuleError>> + Send + 'a>>;

/// **Librería:** `validations` · **Funcionalidad:** misma interfaz que `BusinessLogicValidation`
/// pero con `validate` asíncrono, para reglas que necesitan leer un puerto (existencia,
/// autorización). Sin dependencias: usa solo `std::future`.
pub trait AsyncBusinessLogicValidation<T: Sync + ?Sized>: Send + Sync {
    /// Orden de ejecución: de menor a mayor.
    fn execution_order(&self) -> i16;

    /// Valida el objeto; devuelve el error de negocio si la regla se viola.
    fn validate<'a>(&'a self, to_validate: &'a T) -> ValidationFuture<'a>;
}

/// **Librería:** `validations` · **Funcionalidad:** ejecuta validaciones asíncronas en orden
/// ascendente y corta en la primera que falla (misma semántica que `start_validations`).
pub async fn start_validations_async<T: Sync + ?Sized>(
    validations: &[&dyn AsyncBusinessLogicValidation<T>],
    to_validate: &T,
) -> Result<(), BusinessRuleError> {
    let mut ordered: Vec<&dyn AsyncBusinessLogicValidation<T>> = validations.to_vec();
    ordered.sort_by_key(|v| v.execution_order());
    for validation in ordered {
        validation.validate(to_validate).await?;
    }
    Ok(())
}
