# validations_rs

**English** · [Español](README.es.md)

**Ordered, composable business validations with stable errors** for Rust. No dependencies, no `unsafe`,
with both synchronous and asynchronous support.

It is the Rust port of [`mdambro/validations`](https://github.com/mdambro/validations) (Java).

```rust
use validations_rs::{
    start_validations, LengthBetweenValidation, NotBlankValidation,
};

let not_blank = NotBlankValidation::new();
let length = LengthBetweenValidation::new(2, 40);

for name in ["Ada", "   "] {
    match start_validations(&[&not_blank, &length], name) {
        Ok(()) => println!("{name:?}: valid"),
        Err(error) => println!("{name:?}: {} ({})", error.message, error.code), // VALUE_BLANK
    }
}
```

## Why use it

- **Explicit order.** Rules run in the order of the list. Whoever builds the list decides the order,
  not each rule, so changing it is just reordering an array.
- **Stops at the first failure.** If a cheap rule fails, the following ones (for example a database
  query) are not executed.
- **A single error, with a stable code.** `BusinessRuleError` carries a `RuleCode` that does not change
  even if you change the message, plus `details` with the context of the violation (received value,
  bounds, etc.). Your API, your frontend or your logs can decide by code, without parsing text.
- **Customizable messages that break nothing.** `.with_message("...")` changes the text; the code
  stays the same.
- **Synchronous and asynchronous.** Local rules with no runtime or `async`; rules that wait on I/O with
  `AsyncBusinessLogicValidation`. The library depends on no runtime: use the one you already have.
- **One rule, one responsibility.** Each rule is a small struct that can be tested on its own. Your own
  rules and the built-in ones mix in the same list.
- **A ready-to-use catalog.** 19 generic validations: nulls,
  text, collections, numbers, booleans, emails and dates.
- **Works with unsized types.** They validate `str` and `[T]` directly, without forcing you to have a
  `String` or a `Vec`.
- **Correct behavior at the edges.** Length counts characters, not bytes; a `NaN` never passes a
  numeric rule.
- **Lightweight and safe.** `std` only, `#![forbid(unsafe_code)]` and Rust 1.75 or later.
- **Well tested.** More than 100 tests, one test file for each file in `src/`.

## Installation

```sh
cargo add validations_rs
```

or in your `Cargo.toml`:

```toml
[dependencies]
validations_rs = "0.1.0"
```

## Concepts

| Piece | What it is for |
|---|---|
| `BusinessLogicValidation<T>` | Trait for a **synchronous** rule over a value of type `T`. One method: `validate(&self, &T) -> Result<(), BusinessRuleError>`. |
| `AsyncBusinessLogicValidation<T>` | The same, but **asynchronous**: for rules that wait on I/O (database, HTTP). |
| `start_validations` / `start_validations_async` | Run a list of rules in order and return the first error. |
| `BusinessRuleError` | The business error: `code`, `message` and `details`. |
| `RuleCode` | Stable code of the rule (for example `"VALUE_BLANK"`). |
| Generic validations | Ready-to-use rules. See the [catalog](#catalog-of-generic-validations). |

## Basic usage

Build a list of rules and pass it to `start_validations` together with the value to validate:

```rust
use validations_rs::{
    start_validations, LengthBetweenValidation, NotBlankValidation,
};

let not_blank = NotBlankValidation::new();
let length = LengthBetweenValidation::new(2, 40)
    .with_message("the name must be between 2 and 40 characters long");

// The order of the list is the order of execution.
for name in ["Ada", "   "] {
    match start_validations(&[&not_blank, &length], name) {
        Ok(()) => println!("{name:?}: valid"),
        Err(error) => println!("{name:?}: {}", error.message), // stops at the first failure
    }
}
```

You can also use a single rule, without a runner:

```rust
use validations_rs::{BusinessLogicValidation, MinValidation};

let adult = MinValidation::new(18);
match adult.validate(&10) {
    Ok(()) => println!("adult"),
    Err(error) => println!("{}", error.message),
}
```

## Catalog of generic validations

All of them are created with `::new(...)`, accept `.with_message("...")` and expose their code as
`Rule::CODE`.

**Presence**

| Rule | Validates | Code |
|---|---|---|
| `NotNullValidation::new()` | An `Option` holds a value. | `VALUE_MISSING` |
| `NullValidation::new()` | An `Option` is empty. | `VALUE_PRESENT` |

**Text**

| Rule | Validates | Code | Details |
|---|---|---|---|
| `NotBlankValidation::new()` | It is neither empty nor only whitespace. | `VALUE_BLANK` | — |
| `LengthBetweenValidation::new(min, max)` | Number of **characters** between `min` and `max`, inclusive. | `VALUE_LENGTH_OUT_OF_RANGE` | `length`, `min`, `max` |
| `EmailValidation::new()` | It looks like an email address. | `EMAIL_INVALID` | `email` |

**Collections** (`Vec`, slices, arrays, `HashMap`, `HashSet`, `BTreeMap`…)

| Rule | Validates | Code | Details |
|---|---|---|---|
| `NotEmptyValidation::new()` | It has at least one element. | `COLLECTION_EMPTY` | — |
| `SizeBetweenValidation::new(min, max)` | Number of elements between `min` and `max`, inclusive. | `SIZE_OUT_OF_RANGE` | `size`, `min`, `max` |

**Numbers** (any `PartialOrd + Display` type; the sign rules also require `Default`)

| Rule | Validates | Code | Details |
|---|---|---|---|
| `MinValidation::new(min)` | Value `>= min`. | `VALUE_BELOW_MIN` | `value`, `min` |
| `MaxValidation::new(max)` | Value `<= max`. | `VALUE_ABOVE_MAX` | `value`, `max` |
| `RangeValidation::new(min, max)` | Value between `min` and `max`, inclusive. | `VALUE_OUT_OF_RANGE` | `value`, `min`, `max` |
| `PositiveValidation::new()` | Value `> 0`. | `VALUE_NOT_POSITIVE` | `value` |
| `PositiveOrZeroValidation::new()` | Value `>= 0`. | `VALUE_NEGATIVE` | `value` |
| `NegativeValidation::new()` | Value `< 0`. | `VALUE_NOT_NEGATIVE` | `value` |
| `NegativeOrZeroValidation::new()` | Value `<= 0`. | `VALUE_POSITIVE` | `value` |

**Booleans**

| Rule | Validates | Code |
|---|---|---|
| `AssertTrueValidation::new()` | The `bool` is `true`. | `VALUE_NOT_TRUE` |
| `AssertFalseValidation::new()` | The `bool` is `false`. | `VALUE_NOT_FALSE` |

**Dates** (`std::time::SystemTime`, compared against the moment of validation)

| Rule | Validates | Code |
|---|---|---|
| `PastValidation::new()` | The instant is before now. | `DATE_NOT_PAST` |
| `FutureValidation::new()` | The instant is after now. | `DATE_NOT_FUTURE` |

**Generic**

| Rule | Validates | Code |
|---|---|---|
| `PredicateValidation::new(fn)` | A predicate `fn(&T) -> bool` returns `true`. | `PREDICATE_FAILED` |

Each rule comes with a default message (in English). Change it with `.with_message("...")`; **the code does not
change**, so whoever consumes the error can keep relying on it.

```rust
use validations_rs::{BusinessLogicValidation, PredicateValidation};

let even = PredicateValidation::new(|n: &i32| n % 2 == 0).with_message("must be even");
match even.validate(&3) {
    Ok(()) => println!("even"),
    Err(error) => println!("{}", error.message), // "must be even"
}
```

### What it does not include (on purpose)

To keep the library free of dependencies, it does not include rules that require an external engine.
They can still be solved with `PredicateValidation` or with your own rule:

- **Regular expressions**: use the `regex` crate inside your own rule.
- **Arbitrary-precision decimals**: `MinValidation` and `MaxValidation` work with `f32`, `f64` and integers.
- **"Past or present" / "future or present"**: a `SystemTime` is an instant, and "present" makes no
  sense at that resolution; use `PastValidation` and `FutureValidation`.

## Writing your own rules

Implement the trait for your type. A rule is a small struct with a single responsibility:

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
            Err(BusinessRuleError::new(Self::CODE, "that domain is not allowed")
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
            Err(BusinessRuleError::new(RuleCode("UNDERAGE"), "you must be of legal age")
                .with_detail("age", signup.age.to_string()))
        }
    }
}

let signup = Signup { email: "ada@example.com".into(), age: 16 };
match start_validations(&[&EmailDomainNotBlocked, &AdultOnly], &signup) {
    Ok(()) => println!("valid signup"),
    Err(error) => println!("{}: {}", error.code, error.message), // UNDERAGE: ...
}
```

### Validating an object's fields with the catalog rules

Delegate from your own rule to the catalog ones. Each field gets a rule with its own name, and the
whole object is validated with an ordered list:

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
        MinValidation::<u8>::new(18).with_message("you must be of legal age").validate(&signup.age)
    }
}

let invalid = Signup { name: "Ada".into(), email: "ada".into(), age: 36 };

match start_validations(&[&ValidName, &ValidEmail, &AdultOnly], &invalid) {
    Ok(()) => println!("valid signup"),
    Err(error) if error.code == EmailValidation::CODE => println!("invalid email: {}", error.message),
    Err(error) => println!("{error}"),
}
```

## Reading the error

`BusinessRuleError` has three public fields and implements `std::error::Error` and `Display`
(`"[CODE] message"`):

```rust
use validations_rs::{
    start_validations, LengthBetweenValidation,
};

let length = LengthBetweenValidation::new(2, 5);

match start_validations(&[&length], "far too long") {
    Ok(()) => println!("valid"),
    Err(error) if error.code == LengthBetweenValidation::CODE => {
        println!("{}", error.message);               // to show or log
        println!("max: {:?}", error.detail("max"));  // Some("5"): context of the violation
    }
    Err(error) => println!("{error}"),               // any other rule: "[CODE] message"
}
```

Use `code` for logic (mapping to an HTTP 422, translating the message, etc.) and `message` only for
people. The `code` is what stays stable between versions.

## Asynchronous validations

If a rule needs I/O, implement `AsyncBusinessLogicValidation<T>` and run it with
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
    // for example, a database pool or an HTTP client
}

impl AsyncBusinessLogicValidation<Signup> for EmailAvailable {
    fn validate<'a>(&'a self, signup: &'a Signup) -> ValidationFuture<'a> {
        Box::pin(async move {
            let taken = false; // the query would go here: repo.exists(&signup.email).await
            if taken {
                Err(BusinessRuleError::new(RuleCode("EMAIL_TAKEN"), "the email is already registered"))
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
        Ok(()) => println!("email available"),
        Err(error) => println!("{error}"),
    }
}
```

A common pattern is to **combine both**: first the synchronous (cheap) rules and, only if they pass,
the asynchronous ones (which hit the network or the database).

> Asynchronous rules must be `Send + Sync` and their future is `Send`, so they can be used with
> multi-threaded runtimes.

## Synchronous or asynchronous?

Both variants are kept because they address different needs:

- **Synchronous**: local rules that do not wait on I/O. No runtime or `async` required.
- **Asynchronous**: rules that query services or persistence.

## Code structure

Modules group the types by function and each struct has its own file under `src/`. The tests follow
the same structure under `tests/`:

```text
src/                          tests/
├── validation/               ├── validation/
├── async_validation/         ├── async_validation/
├── error/                    ├── error/
└── generic/                  ├── generic/
                              └── support/        shared utilities
```

| Package | Contents |
|---|---|
| `validation` | `BusinessLogicValidation`, `start_validations` |
| `async_validation` | `AsyncBusinessLogicValidation`, `start_validations_async` |
| `error` | `BusinessRuleError`, `RuleCode` |
| `generic` | The 19 validations of the catalog |

## Development

```sh
cargo test
```

## License

See the [LICENSE](LICENSE) file.
