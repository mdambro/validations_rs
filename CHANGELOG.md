# Changelog

All notable changes to this project are documented in this file. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/).

## [0.3.0] - 2026-11-16

A redesign around the Rust API Guidelines. The behavior of the catalog is the same and every error code
keeps its text; the API around it changed, so this release breaks every caller.

### Added

- Plain functions and closures are rules: `Rule<T>` is implemented for every
  `Fn(&T) -> Result<(), Violation>`.
- `ensure(condition, code, message)` writes a rule's body in one line.
- `RuleExt`, implemented for every rule: `.with_code(code)`, `.with_message(text)`, and
  `.on("field", getter)`, which checks a field of a larger value and records its name. Nested `on` builds
  a path such as `address.city`.
- `Violation::field()`, `Violation::at_field(name)`, and `Violation::detail(key)`.
- `Violations`, the error of `validate_all`: a list that is never empty, with `first()`, `iter()`,
  `Display`, and `Error`.
- `Length::at_least`/`at_most` and `Count::at_least`/`at_most`.
- An optional `serde` feature that implements `Serialize` for `RuleCode`, `Violation`, and `Violations`.
- `prelude` module.
- Every public type implements `Debug`; unit rules also implement `Clone`, `Copy`, `Default`, `PartialEq`,
  `Eq`, and `Hash`. `RuleCode` implements `Hash` and `Ord`, so it works as a map key.

### Changed (breaking)

- Runners take the value first: `validate(value, &rules)`. With the value first, a list of plain functions
  needs no type annotations.
- `Violation` keeps its fields private behind accessors (`code()`, `message()`, `details()`), so the type
  can grow without another breaking release. Messages are `Cow<'static, str>`: static text is not copied.
- Catalog rules live in the `rules` module, are named by what they check, and are built as values
  (`NotBlank`, `Min(18)`, `Length::between(2, 40)`) instead of through `new()`.
- `AsyncRule::check` returns `RuleFuture`, a boxed `Send` future, and the trait requires `Send + Sync`.
- Sign rules (`Positive` and the others) work on the primitive numbers through the sealed `Number` trait,
  instead of on any type whose `Default` is zero.

### Removed

- `PredicateValidation`: write a function with `ensure`, which also gives the rule its own code.
- The `CODE` associated constants: the codes are in `rules::codes`.
- `NotNullValidation` and `NullValidation`: replaced by `IsSome` and `IsNone`, which name what they check.

## Migrating from 0.2

| 0.2 | 0.3 |
|---|---|
| `BusinessRuleError` | `Violation` |
| `error.code`, `error.message`, `error.details` | `violation.code()`, `violation.message()`, `violation.details()`, `violation.detail(key)` |
| `BusinessLogicValidation<T>` with `fn validate(&self, &T)` | `Rule<T>` with `fn check(&self, &T)`, or a plain function |
| `start_validations(&rules, value)` | `validate(value, &rules)` |
| `collect_validations(&rules, value)` → `Vec<BusinessRuleError>` | `validate_all(value, &rules)` → `Violations` |
| `AsyncBusinessLogicValidation<T>` | `AsyncRule<T>` |
| `AsyncValidationError` | `AsyncError` |
| `ValidationFuture` | `RuleFuture` |
| `start_validations_async(&rules, value)` | `validate_async(value, &rules)` |
| `PredicateValidation::new(code, msg, predicate)` | `fn rule(v: &T) -> Result<(), Violation> { ensure(predicate(v), code, msg) }` |
| `NotBlankValidation::new()` | `rules::NotBlank` |
| `LengthBetweenValidation::new(min, max)` | `rules::Length::between(min, max)` |
| `EmailValidation::new()` | `rules::Email` |
| `MinValidation::new(x)`, `MaxValidation::new(x)` | `rules::Min(x)`, `rules::Max(x)` |
| `RangeValidation::new(min, max)` | `rules::InRange::new(min, max)` |
| `PositiveValidation::new()` (and the other sign rules) | `rules::Positive` (`PositiveOrZero`, `Negative`, `NegativeOrZero`) |
| `NotNullValidation::new()`, `NullValidation::new()` | `rules::IsSome`, `rules::IsNone` |
| `AssertTrueValidation::new()`, `AssertFalseValidation::new()` | `rules::IsTrue`, `rules::IsFalse` |
| `NotEmptyValidation::new()` | `rules::NotEmpty` |
| `SizeBetweenValidation::new(min, max)` | `rules::Count::between(min, max)` |
| `PastValidation::new()`, `FutureValidation::new()` | `rules::Past::new()`, `rules::Future::new()` (same `with_clock`) |
| `.with_message(text)` on a catalog rule | `.with_message(text)` from `RuleExt`, on any rule |

A rule in 0.2 and the same rule in 0.3:

```rust,ignore
// 0.2.0
pub struct AdultValidation;
impl BusinessLogicValidation<User> for AdultValidation {
    fn validate(&self, user: &User) -> Result<(), BusinessRuleError> {
        if user.age >= 18 { Ok(()) } else { Err(BusinessRuleError::new(UNDERAGE, "must be an adult")) }
    }
}
start_validations(&[&AdultValidation], &user)?;

// 0.3.0
fn adult(user: &User) -> Result<(), Violation> {
    ensure(user.age >= 18, UNDERAGE, "must be an adult")
}
validate(&user, &[&adult])?;
```

## [0.2.0] - 2026-11-03

### Added

- `collect_validations`: runs every synchronous rule and returns all the errors, in list order, for
  forms and requests with several fields. `start_validations` keeps stopping at the first failure.
- `AsyncValidationError`: the error of asynchronous rules, with two variants:
  `Violation(BusinessRuleError)` when the data breaks a rule, and `CheckFailed(cause)` when the rule
  could not be checked (for example, the database is unavailable). The cause is kept as the error
  `source`.
- `PastValidation::with_clock` and `FutureValidation::with_clock`: take the current time from a
  function instead of the system clock, so date rules are deterministic in tests.
- `PredicateValidation` accepts any `Fn(&T) -> bool`, including closures that capture variables. It now
  implements `Debug`, and `Clone` when its predicate does.
- The README examples (English and Spanish) compile and run as doctests.

### Changed (breaking)

- `ValidationFuture` resolves to `Result<(), AsyncValidationError>` instead of
  `Result<(), BusinessRuleError>`, and `start_validations_async` returns `AsyncValidationError`.
- `PredicateValidation` has a second type parameter for the predicate. It defaults to
  `fn(&T) -> bool`, so `PredicateValidation<T>` written out by hand keeps meaning what it meant.

### Other

- `#[must_use]` on constructors, `with_*` methods, and accessors.
- `# Errors` sections in the documentation of the traits and runners.

## Migrating from 0.1.0

Synchronous rules, `start_validations`, `BusinessRuleError`, `RuleCode`, and the catalog rules need no
change.

Asynchronous rules return the business error through `.into()` (or `?`), and report I/O failures as
`CheckFailed`:

```rust,ignore
// 0.1.0
Err(BusinessRuleError::new(RuleCode("EMAIL_TAKEN"), "taken"))

// 0.2.0
Err(BusinessRuleError::new(RuleCode("EMAIL_TAKEN"), "taken").into())
let taken = repo.exists(email).await.map_err(AsyncValidationError::check_failed)?;
```

Code that reads the result of `start_validations_async` matches on the variant:

```rust,ignore
// 0.1.0
Err(error) if error.code == EMAIL_TAKEN => ...

// 0.2.0
Err(AsyncValidationError::Violation(error)) if error.code == EMAIL_TAKEN => ...
Err(AsyncValidationError::CheckFailed(cause)) => ... // usually HTTP 503 or gRPC Unavailable
```

`error.violation()` and `error.into_violation()` return the business error when there is one.

## [0.1.0]

- First release: synchronous and asynchronous business validations with stable error codes and a
  catalog of 19 generic rules.

[0.3.0]: https://github.com/mdambro/validations_rs/compare/9b284f1...release/0.3.0
[0.2.0]: https://github.com/mdambro/validations_rs/compare/2f4ae15...release/0.2.0
[0.1.0]: https://github.com/mdambro/validations_rs/tree/2f4ae15
