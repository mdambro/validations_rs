//! Validaciones de uso general, reutilizables en cualquier contexto.

use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

/// **Librería:** `validations` · **Funcionalidad:** exige que un texto no esté vacío ni tenga
/// solo espacios en blanco.
#[derive(Debug, Clone, Copy)]
pub struct NotBlankValidation {
    /// Orden de ejecución.
    pub order: i16,
    /// Código con el que falla.
    pub code: RuleCode,
}

impl<T: AsRef<str> + ?Sized> BusinessLogicValidation<T> for NotBlankValidation {
    fn execution_order(&self) -> i16 {
        self.order
    }

    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError> {
        if to_validate.as_ref().trim().is_empty() {
            return Err(BusinessRuleError::new(self.code, "el valor no puede estar vacío"));
        }
        Ok(())
    }
}

/// **Librería:** `validations` · **Funcionalidad:** exige que la cantidad de caracteres de un
/// texto esté entre `min` y `max` (inclusive).
#[derive(Debug, Clone, Copy)]
pub struct LengthBetweenValidation {
    /// Orden de ejecución.
    pub order: i16,
    /// Mínimo de caracteres.
    pub min: usize,
    /// Máximo de caracteres.
    pub max: usize,
    /// Código con el que falla (por ejemplo `VALUE_TOO_LONG`).
    pub code: RuleCode,
}

impl<T: AsRef<str> + ?Sized> BusinessLogicValidation<T> for LengthBetweenValidation {
    fn execution_order(&self) -> i16 {
        self.order
    }

    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError> {
        let len = to_validate.as_ref().chars().count();
        if len < self.min || len > self.max {
            return Err(BusinessRuleError::new(
                self.code,
                format!("longitud {len} fuera de [{}, {}]", self.min, self.max),
            )
            .with_detail("length", len.to_string()));
        }
        Ok(())
    }
}

/// **Librería:** `validations` · **Funcionalidad:** exige que un valor ordenable esté entre
/// `min` y `max` (inclusive).
#[derive(Debug, Clone, Copy)]
pub struct RangeValidation<T> {
    /// Orden de ejecución.
    pub order: i16,
    /// Cota inferior.
    pub min: T,
    /// Cota superior.
    pub max: T,
    /// Código con el que falla (por ejemplo `VALUE_OUT_OF_RANGE`).
    pub code: RuleCode,
}

impl<T: PartialOrd + std::fmt::Display> BusinessLogicValidation<T> for RangeValidation<T> {
    fn execution_order(&self) -> i16 {
        self.order
    }

    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError> {
        if *to_validate < self.min || *to_validate > self.max {
            return Err(BusinessRuleError::new(
                self.code,
                format!("{to_validate} fuera de [{}, {}]", self.min, self.max),
            ));
        }
        Ok(())
    }
}

/// **Librería:** `validations` · **Funcionalidad:** validación definida por un predicado: falla
/// con `code` y `message` cuando el predicado devuelve `false`.
pub struct PredicateValidation<T: ?Sized> {
    /// Orden de ejecución.
    pub order: i16,
    /// Código con el que falla.
    pub code: RuleCode,
    /// Mensaje del error.
    pub message: &'static str,
    /// Predicado: `true` si el objeto cumple la regla.
    pub predicate: fn(&T) -> bool,
}

impl<T: ?Sized> BusinessLogicValidation<T> for PredicateValidation<T> {
    fn execution_order(&self) -> i16 {
        self.order
    }

    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError> {
        if (self.predicate)(to_validate) {
            Ok(())
        } else {
            Err(BusinessRuleError::new(self.code, self.message))
        }
    }
}
