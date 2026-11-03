# Changelog

All notable changes to this project are documented in this file. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/).

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

[0.2.0]: https://github.com/mdambro/validations_rs/compare/2f4ae15...release/0.2.0
[0.1.0]: https://github.com/mdambro/validations_rs/tree/2f4ae15
