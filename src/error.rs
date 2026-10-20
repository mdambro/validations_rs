//! Error único de negocio y su código estable.

use std::fmt;

/// **Librería:** `validations` · **Funcionalidad:** código estable de una regla de negocio
/// (por ejemplo `"VALUE_BLANK"`). Quien llama decide por el código y la interfaz traduce
/// por código, nunca por el texto del mensaje.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RuleCode(pub &'static str);

impl RuleCode {
    /// Devuelve el código como texto.
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl fmt::Display for RuleCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// **Librería:** `validations` · **Funcionalidad:** único error de negocio. Equivale a
/// `BusinessRuleException` de la librería Java, con el agregado de `code` (reaccionar sin
/// leer el texto) y `details` (datos de contexto como pares clave-valor).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BusinessRuleError {
    /// Código estable de la regla violada.
    pub code: RuleCode,
    /// Mensaje legible para desarrolladores (no para el usuario final).
    pub message: String,
    /// Datos de contexto de la violación.
    pub details: Vec<(String, String)>,
}

impl BusinessRuleError {
    /// Crea un error sin detalles.
    pub fn new(code: RuleCode, message: impl Into<String>) -> Self {
        Self { code, message: message.into(), details: Vec::new() }
    }

    /// Devuelve una copia con un detalle más.
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.details.push((key.into(), value.into()));
        self
    }

    /// Busca un detalle por clave.
    pub fn detail(&self, key: &str) -> Option<&str> {
        self.details.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }
}

impl fmt::Display for BusinessRuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for BusinessRuleError {}
