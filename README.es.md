# validations_rs

[English](README.md) · **Español**

**Validación ordenada de reglas de negocio con códigos de error estables** para Rust. Una regla es una
función común; la librería aporta el tipo de error, un catálogo de comprobaciones frecuentes, adaptadores
para componerlas y reglas asíncronas para las comprobaciones que esperan I/O. Sin dependencias
obligatorias, sin `unsafe`, sin runtime.

```rust
use validations_rs::prelude::*;
use validations_rs::rules::{Email, Min, NotBlank};

struct Registro {
    nombre: String,
    email: String,
    edad: u8,
    acepto_terminos: bool,
}

const TERMINOS_NO_ACEPTADOS: RuleCode = RuleCode("TERMINOS_NO_ACEPTADOS");

// Una regla de negocio es una función.
fn acepto_terminos(registro: &Registro) -> Result<(), Violation> {
    ensure(registro.acepto_terminos, TERMINOS_NO_ACEPTADOS, "tienes que aceptar los términos")
}

let registro = Registro { nombre: "Ada".into(), email: "ada@example".into(), edad: 36, acepto_terminos: true };

let resultado = validate(&registro, &[
    &acepto_terminos,
    &NotBlank.on("nombre", |r: &Registro| &r.nombre),
    &Email.on("email", |r: &Registro| &r.email),
    &Min(18).on("edad", |r: &Registro| &r.edad),
]);

let violacion = resultado.unwrap_err();
assert_eq!(violacion.code().as_str(), "EMAIL_INVALID");
assert_eq!(violacion.field(), Some("email"));
```

## Por qué usarla

- **Las reglas son funciones.** Cualquier `fn(&T) -> Result<(), Violation>`, o closure con esa firma, es
  una regla. Escribes una regla de negocio en tres líneas con `ensure`, la pruebas llamándola y la pones en
  una lista. Implementas el trait `Rule` a mano solo cuando la regla lleva configuración.
- **Códigos estables, no textos.** Una `Violation` lleva un `RuleCode` que no cambia aunque cambie el
  mensaje, el campo al que aplica y detalles (el valor recibido, los límites). Tu API, tu frontend y tus
  logs deciden por código, sin parsear texto.
- **Orden explícito.** Las reglas corren en el orden de la lista. `validate` se detiene en la primera
  violación, así una regla barata falla antes de que corra una cara; `validate_all` corre todas y devuelve
  todas las violaciones, para formularios.
- **Componible.** `.with_code(...)`, `.with_message(...)` y `.on("campo", getter)` adaptan cualquier
  regla, incluidas tus funciones, sin escribir un tipo nuevo.
- **"Rompe la regla" no es "no se pudo comprobar".** Una regla asíncrona devuelve `AsyncError::Violation`
  cuando el dato la rompe y `AsyncError::CheckFailed` cuando la comprobación misma falló (la base de datos
  no responde), con la causa guardada como `source` del error. No se responde un problema del servidor
  con un 422.
- **Correcta en los bordes.** La longitud cuenta caracteres, no bytes; `NaN` nunca pasa una regla
  numérica; las reglas de fechas reciben un reloj, así los tests son deterministas.
- **Sirve para cualquier proyecto.** Solo la biblioteca estándar, una feature `serde` opcional, ningún
  runtime async, `#![forbid(unsafe_code)]`, Rust 1.75 o posterior. Todos los tipos públicos implementan
  `Debug`, y los errores implementan `std::error::Error + Send + Sync`.

### Cuándo usar otra cosa

`validations_rs` es para **reglas de negocio**: comprobaciones ordenadas, con códigos estables, que viven
en la capa de dominio o de aplicación y pueden ser asíncronas. Si lo que necesitas es anotar un DTO con
`#[validate(length(min = 1))]` y comprobar su forma en el borde, te conviene un crate basado en derive como
[`validator`](https://crates.io/crates/validator) o [`garde`](https://crates.io/crates/garde). Los dos
enfoques se complementan: derive en el borde, `validations_rs` en el caso de uso.

## Instalación

```sh
cargo add validations_rs
# con `serde::Serialize` para los tipos de error:
cargo add validations_rs --features serde
```

## Conceptos

| Elemento | Qué es |
|---|---|
| `Rule<T>` | Comprueba un `&T` y devuelve `Ok(())` o una `Violation`. Implementado para todo `Fn(&T) -> Result<(), Violation>`. |
| `ensure(condición, código, mensaje)` | La forma de una línea de escribir el cuerpo de una regla. |
| `Violation` | La regla rota: `code()`, `message()`, `field()`, `details()`. |
| `Violations` | Todas las violaciones de `validate_all`; nunca está vacía. |
| `validate` / `validate_all` | Corren una lista de reglas: la primera violación, o todas. |
| `RuleExt` | Adaptadores para cualquier regla: `with_code`, `with_message`, `on`. |
| `AsyncRule<T>`, `validate_async`, `AsyncError` | Reglas que esperan I/O. |
| `rules` | El catálogo: texto, números, presencia, colecciones, fechas. |

## Escribir reglas

La mayoría de las reglas son funciones. Dale a cada regla de negocio su propia constante `RuleCode`, así
quien llama puede hacer `match` sobre ella:

```rust
use validations_rs::{ensure, validate, RuleCode, Violation};

pub struct Transferencia {
    pub monto_centavos: u64,
    pub origen: String,
    pub destino: String,
}

pub const MISMA_CUENTA: RuleCode = RuleCode("TRANSFERENCIA_MISMA_CUENTA");
pub const MONTO_CERO: RuleCode = RuleCode("TRANSFERENCIA_MONTO_CERO");

fn cuentas_distintas(t: &Transferencia) -> Result<(), Violation> {
    ensure(t.origen != t.destino, MISMA_CUENTA, "la cuenta de origen y la de destino deben ser distintas")
}

fn tiene_monto(t: &Transferencia) -> Result<(), Violation> {
    ensure(t.monto_centavos > 0, MONTO_CERO, "el monto debe ser mayor que cero")
}

let transferencia = Transferencia { monto_centavos: 0, origen: "A".into(), destino: "B".into() };
let violacion = validate(&transferencia, &[&cuentas_distintas, &tiene_monto]).unwrap_err();
assert_eq!(violacion.code(), MONTO_CERO);
```

Un closure que captura configuración también es una regla. Cuando una regla necesita configuración que
fija quien la usa, y la quieres reutilizable y con nombre, hazla un tipo e implementa `Rule`:

```rust
use validations_rs::{ensure, validate, Rule, RuleCode, Violation};

pub struct Transferencia { pub monto_centavos: u64 }

pub const SUPERA_LIMITE: RuleCode = RuleCode("TRANSFERENCIA_SUPERA_LIMITE");

pub struct LimiteDiario { pub centavos: u64 }

impl Rule<Transferencia> for LimiteDiario {
    fn check(&self, t: &Transferencia) -> Result<(), Violation> {
        ensure(t.monto_centavos <= self.centavos, SUPERA_LIMITE, "la transferencia supera el límite diario")
            .map_err(|v| v.with_detail("limite", self.centavos))
    }
}

let violacion = validate(&Transferencia { monto_centavos: 900 }, &[&LimiteDiario { centavos: 500 }]).unwrap_err();
assert_eq!(violacion.detail("limite"), Some("500"));
```

## El catálogo

El módulo `rules` trae las comprobaciones que todo proyecto termina escribiendo. Cada una informa un
código estable de `rules::codes`.

| Regla | Comprueba | Código |
|---|---|---|
| `NotBlank` | texto con algún carácter que no sea espacio | `VALUE_BLANK` |
| `Length::between(min, max)`, `at_least`, `at_most` | cantidad de caracteres | `VALUE_LENGTH_OUT_OF_RANGE` |
| `Email` | una dirección bien formada (estructural, no RFC 5322) | `EMAIL_INVALID` |
| `Min(x)`, `Max(x)` | cualquier valor `PartialOrd + Display` | `VALUE_BELOW_MIN`, `VALUE_ABOVE_MAX` |
| `InRange::new(min, max)` | rango inclusivo | `VALUE_OUT_OF_RANGE` |
| `Positive`, `PositiveOrZero`, `Negative`, `NegativeOrZero` | signo de un número primitivo | `VALUE_NOT_POSITIVE`, `VALUE_NEGATIVE`, `VALUE_NOT_NEGATIVE`, `VALUE_POSITIVE` |
| `IsSome`, `IsNone` | un `Option` presente o ausente | `VALUE_MISSING`, `VALUE_PRESENT` |
| `IsTrue`, `IsFalse` | un booleano | `VALUE_NOT_TRUE`, `VALUE_NOT_FALSE` |
| `NotEmpty` | una colección con algún elemento | `COLLECTION_EMPTY` |
| `Count::between(min, max)`, `at_least`, `at_most` | cantidad de elementos | `SIZE_OUT_OF_RANGE` |
| `Past::new()`, `Future::new()` | un `SystemTime` anterior o posterior a ahora; `.with_clock(...)` para tests | `DATE_NOT_PAST`, `DATE_NOT_FUTURE` |

Las reglas de texto aceptan todo lo que sea `AsRef<str>`; las de colecciones, todo lo que una referencia
compartida pueda iterar (slices, arrays, `Vec`, mapas, sets).

```rust
use validations_rs::rules::{Count, Length, NotBlank, Positive};
use validations_rs::{validate_all, Rule};

assert!(Positive.check(&3_i64).is_ok());
assert!(Count::at_most(2).check(&vec!["a", "b", "c"]).is_err());

let violaciones = validate_all("  ", &[&NotBlank, &Length::at_least(3)]).unwrap_err();
assert_eq!(violaciones.len(), 2);
```

No se incluye a propósito, para no tener dependencias: expresiones regulares (usa el crate `regex` en tu
propia regla) ni decimales de precisión arbitraria (`Min` y `Max` aceptan cualquier tipo ordenado, así que
tu tipo decimal funciona si implementa `PartialOrd + Display`).

## Adaptar reglas

`RuleExt` está implementado para toda regla. Cambia el código por uno de tu dominio, cambia el mensaje o
comprueba un campo de un valor más grande. `on` registra el nombre del campo en la violación, y `on`
anidado arma una ruta como `direccion.ciudad`:

```rust
use validations_rs::rules::{Min, NotBlank};
use validations_rs::{validate_all, Rule, RuleCode, RuleExt};

struct Direccion { ciudad: String }
struct Cliente { edad: u8, direccion: Direccion }

let ciudad = NotBlank.with_message("Indica tu ciudad.").on("ciudad", |d: &Direccion| &d.ciudad);

let violaciones = validate_all(&Cliente { edad: 16, direccion: Direccion { ciudad: "".into() } }, &[
    &Min(18).with_code(RuleCode("MENOR_DE_EDAD")).on("edad", |c: &Cliente| &c.edad),
    &ciudad.on("direccion", |c: &Cliente| &c.direccion),
])
.unwrap_err();

let campos: Vec<_> = violaciones.iter().map(|v| v.field()).collect();
assert_eq!(campos, [Some("edad"), Some("direccion.ciudad")]);
assert_eq!(violaciones.first().code().as_str(), "MENOR_DE_EDAD");
assert_eq!(violaciones.first().detail("min"), Some("18"));
```

## Leer el error

Decide por código; muestra el mensaje; usa el campo y los detalles para señalar la entrada:

```rust
use validations_rs::rules::{codes, Length};
use validations_rs::Rule;

let violacion = Length::between(2, 40).check("A").unwrap_err();

match violacion.code() {
    codes::VALUE_LENGTH_OUT_OF_RANGE => {
        assert_eq!(violacion.detail("length"), Some("1"));
        assert_eq!(violacion.detail("min"), Some("2"));
    }
    otro => panic!("regla inesperada {otro}"),
}
```

Con la feature `serde`, `Violation` se serializa como
`{"code": "...", "message": "...", "field": "..." | null, "details": {"clave": "valor"}}`, y `Violations`
como una lista de ellas, lista para el cuerpo de un error HTTP.

## Reglas asíncronas

Una regla que consulta una base de datos u otro servicio implementa `AsyncRule`. El trait devuelve un
future en un `Box`, así funciona detrás de `&dyn` y con cualquier runtime. Informa una regla rota con una
`Violation` y una comprobación fallida con `AsyncError::check_failed(causa)`:

```rust
use validations_rs::{validate_async, AsyncError, AsyncRule, RuleCode, RuleFuture, Violation};

pub trait Usuarios: Send + Sync {
    fn existe_email(&self, email: &str) -> Result<bool, std::io::Error>;
}

pub const EMAIL_EN_USO: RuleCode = RuleCode("EMAIL_EN_USO");

pub struct EmailLibre<'r> {
    pub usuarios: &'r dyn Usuarios,
}

impl AsyncRule<str> for EmailLibre<'_> {
    fn check<'a>(&'a self, email: &'a str) -> RuleFuture<'a> {
        Box::pin(async move {
            if self.usuarios.existe_email(email).map_err(AsyncError::check_failed)? {
                return Err(Violation::new(EMAIL_EN_USO, "el email ya está registrado").into());
            }
            Ok(())
        })
    }
}

async fn registrar(usuarios: &dyn Usuarios, email: &str) -> Result<(), AsyncError> {
    validate_async(email, &[&EmailLibre { usuarios }]).await
}
```

Corre primero las reglas síncronas con `validate`, así las comprobaciones baratas fallan antes de
cualquier I/O. En el borde, mapea `AsyncError::Violation` a un error del cliente (como HTTP 422) y
`AsyncError::CheckFailed` a un error del servidor (como HTTP 503).

## Migrar desde la 0.2

La tabla completa está en el [CHANGELOG](CHANGELOG.md). En resumen: `BusinessRuleError` es `Violation`,
`BusinessLogicValidation` es `Rule`, `start_validations(&reglas, valor)` es `validate(valor, &reglas)`,
`collect_validations` es `validate_all`, los structs `*Validation` son el catálogo `rules::*` y
`PredicateValidation` es una función común con `ensure`.

## Desarrollo

```sh
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

Los ejemplos de este README y de la documentación de la API se compilan y ejecutan como tests.

## Licencia

Apache-2.0. Ver el archivo [LICENSE](LICENSE).
