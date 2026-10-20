//! # validations
//!
//! Port a Rust de la librería Java `mdambro/validations`: una interfaz genérica de validación
//! con orden de ejecución, un ejecutor que las corre en orden cortando en el primer error y un
//! único error de negocio con código estable.
//!
//! ```
//! use validations::{start_validations, BusinessLogicValidation, NotBlankValidation, RuleCode};
//!
//! let not_blank = NotBlankValidation { order: 10, code: RuleCode("VALUE_BLANK") };
//! let list: [&dyn BusinessLogicValidation<str>; 1] = [&not_blank];
//! assert!(start_validations(&list, "hola").is_ok());
//! assert_eq!(start_validations(&list, "  ").unwrap_err().code, RuleCode("VALUE_BLANK"));
//! ```

pub mod async_validation;
pub mod error;
pub mod generic;
pub mod validation;

pub use async_validation::{start_validations_async, AsyncBusinessLogicValidation, ValidationFuture};
pub use error::{BusinessRuleError, RuleCode};
pub use generic::{LengthBetweenValidation, NotBlankValidation, PredicateValidation, RangeValidation};
pub use validation::{start_validations, BusinessLogicValidation};
