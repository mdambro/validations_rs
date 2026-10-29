# validations_rs

[English](README.md) · **Español**

Validaciones de negocio **ordenadas, combinables y con errores estables** para Rust. Sin dependencias,
sin `unsafe`, con soporte síncrono y asíncrono.

Es el port a Rust de [`mdambro/validations`](https://github.com/mdambro/validations) (Java).

```rust
use validations_rs::{
    start_validations, LengthBetweenValidation, NotBlankValidation,
};

let not_blank = NotBlankValidation::new();
let length = LengthBetweenValidation::new(2, 40);

for name in ["Ada", "   "] {
    match start_validations(&[&not_blank, &length], name) {
        Ok(()) => println!("{name:?}: válido"),
        Err(error) => println!("{name:?}: {} ({})", error.message, error.code), // VALUE_BLANK
    }
}
```

## Por qué usarla

- **Orden explícito.** Las reglas se ejecutan en el orden de la lista. El orden lo decide quien arma
  la lista, no cada regla, así que cambiarlo es reordenar un array.
- **Corta en la primera falla.** Si una regla barata falla, las siguientes (por ejemplo una consulta a
  la base de datos) no se ejecutan.
- **Un único error, con código estable.** `BusinessRuleError` lleva un `RuleCode` que no cambia aunque
  cambies el mensaje, más `details` con el contexto de la violación (valor recibido, límites, etc.).
  Tu API, tu frontend o tus logs deciden por el código, sin parsear textos.
- **Mensajes personalizables sin romper nada.** `.with_message("...")` cambia el texto; el código
  queda igual.
- **Síncrona y asíncrona.** Reglas locales sin runtime ni `async`; reglas que esperan I/O con
  `AsyncBusinessLogicValidation`. La librería no depende de ningún runtime: usa el que ya tengas.
- **Una regla, una responsabilidad.** Cada regla es una struct pequeña y testeable por separado. Las
  reglas propias y las incluidas se mezclan en la misma lista.
- **Catálogo listo para usar.** 19 validaciones genéricas: nulos, textos,
  colecciones, números, booleanos, emails y fechas.
- **Trabaja con tipos sin tamaño.** Validan `str` y `[T]` directamente, sin obligarte a tener un
  `String` o un `Vec`.
- **Comportamiento correcto en los bordes.** La longitud cuenta caracteres, no bytes; los `NaN` nunca
  pasan una regla numérica.
- **Ligera y segura.** Solo `std`, `#![forbid(unsafe_code)]` y Rust 1.75 o superior.
- **Bien probada.** Más de 100 pruebas, un archivo de pruebas por cada archivo de `src/`.

## Instalación

```sh
cargo add validations_rs
```

o en tu `Cargo.toml`:

```toml
[dependencies]
validations_rs = "0.1.0"
```

## Conceptos

| Pieza | Para qué sirve |
|---|---|
| `BusinessLogicValidation<T>` | Trait de una regla **síncrona** sobre un valor de tipo `T`. Un método: `validate(&self, &T) -> Result<(), BusinessRuleError>`. |
| `AsyncBusinessLogicValidation<T>` | Lo mismo, pero **asíncrona**: para reglas que esperan I/O (base de datos, HTTP). |
| `start_validations` / `start_validations_async` | Ejecutan una lista de reglas en orden y devuelven el primer error. |
| `BusinessRuleError` | El error de negocio: `code`, `message` y `details`. |
| `RuleCode` | Código estable de la regla (por ejemplo `"VALUE_BLANK"`). |
| Validaciones genéricas | Reglas listas para usar. Ver el [catálogo](#catálogo-de-validaciones-genéricas). |

## Uso básico

Arma una lista de reglas y pásala a `start_validations` junto con el valor a validar:

```rust
use validations_rs::{
    start_validations, LengthBetweenValidation, NotBlankValidation,
};

let not_blank = NotBlankValidation::new();
let length = LengthBetweenValidation::new(2, 40)
    .with_message("el nombre debe tener entre 2 y 40 caracteres");

// El orden de la lista es el orden de ejecución.
for name in ["Ada", "   "] {
    match start_validations(&[&not_blank, &length], name) {
        Ok(()) => println!("{name:?}: válido"),
        Err(error) => println!("{name:?}: {}", error.message), // corta en la primera falla
    }
}
```

También puedes usar una regla suelta, sin ejecutor:

```rust
use validations_rs::{BusinessLogicValidation, MinValidation};

let adult = MinValidation::new(18);
match adult.validate(&10) {
    Ok(()) => println!("mayor de edad"),
    Err(error) => println!("{}", error.message),
}
```

## Catálogo de validaciones genéricas

Todas se crean con `::new(...)`, aceptan `.with_message("...")` y exponen su código como
`Regla::CODE`.

**Presencia**

| Regla | Valida | Código |
|---|---|---|
| `NotNullValidation::new()` | Un `Option` tiene valor. | `VALUE_MISSING` |
| `NullValidation::new()` | Un `Option` no tiene valor. | `VALUE_PRESENT` |

**Textos**

| Regla | Valida | Código | Detalles |
|---|---|---|---|
| `NotBlankValidation::new()` | No está vacío ni solo con espacios. | `VALUE_BLANK` | — |
| `LengthBetweenValidation::new(min, max)` | Cantidad de **caracteres** entre `min` y `max`, inclusive. | `VALUE_LENGTH_OUT_OF_RANGE` | `length`, `min`, `max` |
| `EmailValidation::new()` | Tiene forma de dirección de email. | `EMAIL_INVALID` | `email` |

**Colecciones** (`Vec`, slices, arrays, `HashMap`, `HashSet`, `BTreeMap`…)

| Regla | Valida | Código | Detalles |
|---|---|---|---|
| `NotEmptyValidation::new()` | Tiene al menos un elemento. | `COLLECTION_EMPTY` | — |
| `SizeBetweenValidation::new(min, max)` | Cantidad de elementos entre `min` y `max`, inclusive. | `SIZE_OUT_OF_RANGE` | `size`, `min`, `max` |

**Números** (cualquier tipo `PartialOrd + Display`; los de signo también piden `Default`)

| Regla | Valida | Código | Detalles |
|---|---|---|---|
| `MinValidation::new(min)` | Valor `>= min`. | `VALUE_BELOW_MIN` | `value`, `min` |
| `MaxValidation::new(max)` | Valor `<= max`. | `VALUE_ABOVE_MAX` | `value`, `max` |
| `RangeValidation::new(min, max)` | Valor entre `min` y `max`, inclusive. | `VALUE_OUT_OF_RANGE` | `value`, `min`, `max` |
| `PositiveValidation::new()` | Valor `> 0`. | `VALUE_NOT_POSITIVE` | `value` |
| `PositiveOrZeroValidation::new()` | Valor `>= 0`. | `VALUE_NEGATIVE` | `value` |
| `NegativeValidation::new()` | Valor `< 0`. | `VALUE_NOT_NEGATIVE` | `value` |
| `NegativeOrZeroValidation::new()` | Valor `<= 0`. | `VALUE_POSITIVE` | `value` |

**Booleanos**

| Regla | Valida | Código |
|---|---|---|
| `AssertTrueValidation::new()` | El `bool` es `true`. | `VALUE_NOT_TRUE` |
| `AssertFalseValidation::new()` | El `bool` es `false`. | `VALUE_NOT_FALSE` |

**Fechas** (`std::time::SystemTime`, comparado contra el momento de validar)

| Regla | Valida | Código |
|---|---|---|
| `PastValidation::new()` | El instante es anterior a ahora. | `DATE_NOT_PAST` |
| `FutureValidation::new()` | El instante es posterior a ahora. | `DATE_NOT_FUTURE` |

**Genérica**

| Regla | Valida | Código |
|---|---|---|
| `PredicateValidation::new(fn)` | Un predicado `fn(&T) -> bool` devuelve `true`. | `PREDICATE_FAILED` |

Cada regla trae un mensaje predeterminado (en inglés). Cámbialo con `.with_message("...")`; **el código no
cambia**, así que quien consume el error puede seguir confiando en él.

```rust
use validations_rs::{BusinessLogicValidation, PredicateValidation};

let even = PredicateValidation::new(|n: &i32| n % 2 == 0).with_message("debe ser par");
match even.validate(&3) {
    Ok(()) => println!("par"),
    Err(error) => println!("{}", error.message), // "debe ser par"
}
```

### Qué no incluye (a propósito)

Para mantener la librería sin dependencias, no incluye reglas que requieren un motor externo. Se
resuelven igual con `PredicateValidation` o con una regla propia:

- **Expresiones regulares**: usa el crate `regex` dentro de una regla propia.
- **Decimales de precisión arbitraria**: `MinValidation` y `MaxValidation` funcionan con `f32`, `f64` y enteros.
- **"Pasado o presente" / "futuro o presente"**: un `SystemTime` es un instante, y "presente" no tiene
  sentido a esa resolución; usa `PastValidation` y `FutureValidation`.

## Escribir tus propias reglas

Implementa el trait para tu tipo. Una regla es una struct pequeña con una sola responsabilidad:

```rust
use validations_rs::{
    start_validations, BusinessLogicValidation, BusinessRuleError, RuleCode,
};

struct Signup {
    email: String,
    age: u8,
}

struct EmailDomainNotBlocked;

impl EmailDomainNotBlocked {
    const CODE: RuleCode = RuleCode("EMAIL_DOMAIN_BLOCKED");
}

impl BusinessLogicValidation<Signup> for EmailDomainNotBlocked {
    fn validate(&self, signup: &Signup) -> Result<(), BusinessRuleError> {
        if signup.email.ends_with("@spam.test") {
            Err(BusinessRuleError::new(Self::CODE, "ese dominio no está permitido")
                .with_detail("email", signup.email.clone()))
        } else {
            Ok(())
        }
    }
}

struct AdultOnly;

impl BusinessLogicValidation<Signup> for AdultOnly {
    fn validate(&self, signup: &Signup) -> Result<(), BusinessRuleError> {
        if signup.age >= 18 {
            Ok(())
        } else {
            Err(BusinessRuleError::new(RuleCode("UNDERAGE"), "debes ser mayor de edad")
                .with_detail("age", signup.age.to_string()))
        }
    }
}

let signup = Signup { email: "ada@example.com".into(), age: 16 };
match start_validations(&[&EmailDomainNotBlocked, &AdultOnly], &signup) {
    Ok(()) => println!("registro válido"),
    Err(error) => println!("{}: {}", error.code, error.message), // UNDERAGE: ...
}
```

### Validar los campos de un objeto con las reglas del catálogo

Delega desde tu propia regla en las del catálogo. Cada campo queda en una regla con nombre propio, y el
objeto completo se valida con una lista ordenada:

```rust
use validations_rs::{
    start_validations, BusinessLogicValidation, BusinessRuleError, EmailValidation,
    LengthBetweenValidation, MinValidation, NotBlankValidation,
};

struct Signup {
    name: String,
    email: String,
    age: u8,
}

struct ValidName;

impl BusinessLogicValidation<Signup> for ValidName {
    fn validate(&self, signup: &Signup) -> Result<(), BusinessRuleError> {
        start_validations::<str>(
            &[&NotBlankValidation::new(), &LengthBetweenValidation::new(2, 40)],
            &signup.name,
        )
    }
}

struct ValidEmail;

impl BusinessLogicValidation<Signup> for ValidEmail {
    fn validate(&self, signup: &Signup) -> Result<(), BusinessRuleError> {
        EmailValidation::new().validate(&signup.email)
    }
}

struct AdultOnly;

impl BusinessLogicValidation<Signup> for AdultOnly {
    fn validate(&self, signup: &Signup) -> Result<(), BusinessRuleError> {
        MinValidation::<u8>::new(18).with_message("debes ser mayor de edad").validate(&signup.age)
    }
}

let invalid = Signup { name: "Ada".into(), email: "ada".into(), age: 36 };

match start_validations(&[&ValidName, &ValidEmail, &AdultOnly], &invalid) {
    Ok(()) => println!("registro válido"),
    Err(error) if error.code == EmailValidation::CODE => println!("email inválido: {}", error.message),
    Err(error) => println!("{error}"),
}
```

## Leer el error

`BusinessRuleError` tiene tres campos públicos y implementa `std::error::Error` y `Display`
(`"[CODIGO] mensaje"`):

```rust
use validations_rs::{
    start_validations, LengthBetweenValidation,
};

let length = LengthBetweenValidation::new(2, 5);

match start_validations(&[&length], "demasiado largo") {
    Ok(()) => println!("válido"),
    Err(error) if error.code == LengthBetweenValidation::CODE => {
        println!("{}", error.message);               // para mostrar o loguear
        println!("max: {:?}", error.detail("max"));  // Some("5"): contexto de la violación
    }
    Err(error) => println!("{error}"),               // cualquier otra regla: "[CODIGO] mensaje"
}
```

Usa `code` para la lógica (mapear a un HTTP 422, traducir el mensaje, etc.) y `message` solo para
personas. El `code` es lo que se mantiene estable entre versiones.

## Validaciones asíncronas

Si una regla necesita I/O, implementa `AsyncBusinessLogicValidation<T>` y ejecútala con
`start_validations_async`:

```rust,ignore
use validations_rs::{
    start_validations_async, AsyncBusinessLogicValidation, BusinessRuleError, RuleCode,
    ValidationFuture,
};

struct Signup {
    email: String,
}

struct EmailAvailable {
    // por ejemplo, un pool de base de datos o un cliente HTTP
}

impl AsyncBusinessLogicValidation<Signup> for EmailAvailable {
    fn validate<'a>(&'a self, signup: &'a Signup) -> ValidationFuture<'a> {
        Box::pin(async move {
            let taken = false; // acá iría la consulta: repo.exists(&signup.email).await
            if taken {
                Err(BusinessRuleError::new(RuleCode("EMAIL_TAKEN"), "el email ya está registrado"))
            } else {
                Ok(())
            }
        })
    }
}

#[tokio::main]
async fn main() {
    let signup = Signup { email: "ada@example.com".into() };
    match start_validations_async(&[&EmailAvailable {}], &signup).await {
        Ok(()) => println!("email disponible"),
        Err(error) => println!("{error}"),
    }
}
```

Un patrón habitual es **combinar ambas**: primero las reglas síncronas (baratas) y, solo si pasan,
las asíncronas (que tocan la red o la base de datos).

> Las reglas asíncronas deben ser `Send + Sync` y su futuro es `Send`, para que se puedan usar con
> runtimes multihilo.

## ¿Síncrono o asíncrono?

Se mantienen las dos variantes porque resuelven necesidades distintas:

- **Síncrona**: reglas locales que no esperan I/O. No requiere runtime ni `async`.
- **Asíncrona**: reglas que consultan servicios o persistencia.

## Estructura del código

Los módulos agrupan los tipos por función y cada struct tiene su propio archivo bajo `src/`. Las
pruebas siguen la misma estructura bajo `tests/`:

```text
src/                          tests/
├── validation/               ├── validation/
├── async_validation/         ├── async_validation/
├── error/                    ├── error/
└── generic/                  ├── generic/
                              └── support/        utilidades compartidas
```

| Paquete | Contenido |
|---|---|
| `validation` | `BusinessLogicValidation`, `start_validations` |
| `async_validation` | `AsyncBusinessLogicValidation`, `start_validations_async` |
| `error` | `BusinessRuleError`, `RuleCode` |
| `generic` | Las 19 validaciones del catálogo |

## Desarrollo

```sh
cargo test
```

## Licencia

Consulta el archivo [LICENSE](LICENSE).
