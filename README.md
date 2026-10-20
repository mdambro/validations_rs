# validations-rs

Port a Rust de [`mdambro/validations`](https://github.com/mdambro/validations) (Java): validaciones de
negocio genéricas y ordenadas. Sin dependencias (solo `std`).

- `BusinessLogicValidation<T>`: una regla (`execution_order` + `validate`) y `AsyncBusinessLogicValidation<T>`.
- `start_validations` (y `start_validations_async`): corren la lista en orden ascendente (empate:
  orden de la lista) y cortan en la primera falla.
- `BusinessRuleError`: único error de negocio, con `RuleCode` estable y `details`.
- Validaciones genéricas: `NotBlankValidation`, `LengthBetweenValidation`, `RangeValidation`,
  `PredicateValidation`.

Hermano en Dart: [`validations-dart`](https://github.com/mdambro/validations-dart).

```sh
cargo test
```
