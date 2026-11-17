# validations_rs

**English** · [Español](README.es.md)

**Ordered business-rule validation with stable error codes** for Rust. A rule is a plain function; the
library gives you the error type, a catalog of common checks, adapters to compose them, and async rules
for checks that wait on I/O. No required dependencies, no `unsafe`, no runtime.

```rust
use validations_rs::prelude::*;
use validations_rs::rules::{Email, Min, NotBlank};

struct Signup {
    name: String,
    email: String,
    age: u8,
    accepted_terms: bool,
}

const TERMS_NOT_ACCEPTED: RuleCode = RuleCode("TERMS_NOT_ACCEPTED");

// A business rule is a function.
fn terms_accepted(signup: &Signup) -> Result<(), Violation> {
    ensure(signup.accepted_terms, TERMS_NOT_ACCEPTED, "you must accept the terms")
}

let signup = Signup { name: "Ada".into(), email: "ada@example".into(), age: 36, accepted_terms: true };

let result = validate(&signup, &[
    &terms_accepted,
    &NotBlank.on("name", |s: &Signup| &s.name),
    &Email.on("email", |s: &Signup| &s.email),
    &Min(18).on("age", |s: &Signup| &s.age),
]);

let violation = result.unwrap_err();
assert_eq!(violation.code().as_str(), "EMAIL_INVALID");
assert_eq!(violation.field(), Some("email"));
assert_eq!(violation.to_string(), "[EMAIL_INVALID] email: The email address does not have a valid format.");
```

## Why use it

- **Rules are functions.** Any `fn(&T) -> Result<(), Violation>`, or closure with that signature, is a
  rule. Write a business rule in three lines with `ensure`, test it by calling it, and put it in a list.
  Implement the `Rule` trait yourself only when a rule carries configuration.
- **Stable codes, not strings.** A `Violation` carries a `RuleCode` that does not change when the message
  does, the field it applies to, and details (the value received, the bounds). Your API, your frontend, and
  your logs decide by code, without parsing text.
- **Explicit order.** Rules run in list order. `validate` stops at the first violation, so a cheap rule
  fails before an expensive one runs; `validate_all` runs every rule and returns every violation, for forms.
- **Composable.** `.with_code(...)`, `.with_message(...)`, and `.on("field", getter)` adapt any rule,
  including your functions, without writing a new type.
- **"Breaks the rule" is not "could not check".** An async rule returns `AsyncError::Violation` when the
  data breaks it and `AsyncError::CheckFailed` when the check itself failed (the database is down), with
  the cause kept as the error `source`. You do not answer a server problem with a 422.
- **Correct at the edges.** Length counts characters, not bytes; `NaN` never passes a numeric rule; date
  rules take a clock, so tests are deterministic.
- **Fits any project.** The standard library only, an optional `serde` feature, no async runtime,
  `#![forbid(unsafe_code)]`, Rust 1.75 or later. Every public type implements `Debug`, and errors
  implement `std::error::Error + Send + Sync`.

### When to use something else

`validations_rs` is for **business rules**: ordered checks with stable codes that live in your domain or
application layer and can run async. If what you need is to annotate a DTO with `#[validate(length(min = 1))]`
and check its shape at the edge, a derive-based crate such as [`validator`](https://crates.io/crates/validator)
or [`garde`](https://crates.io/crates/garde) fits better. The two approaches work well together: derive at
the edge, `validations_rs` in the use case.

## Installation

```sh
cargo add validations_rs
# with `serde::Serialize` for the error types:
cargo add validations_rs --features serde
```

## Concepts

| Item | What it is |
|---|---|
| `Rule<T>` | Checks a `&T` and returns `Ok(())` or a `Violation`. Implemented for every `Fn(&T) -> Result<(), Violation>`. |
| `ensure(condition, code, message)` | The one-line way to write a rule's body. |
| `Violation` | The broken rule: `code()`, `message()`, `field()`, `details()`. |
| `Violations` | Every violation from `validate_all`; never empty. |
| `validate` / `validate_all` | Run a list of rules: first violation, or all of them. |
| `RuleExt` | Adapters for any rule: `with_code`, `with_message`, `on`. |
| `AsyncRule<T>`, `validate_async`, `AsyncError` | Rules that wait on I/O. |
| `rules` | The catalog: text, numbers, presence, collections, dates. |

## Writing rules

Most rules are functions. Give each business rule its own `RuleCode` constant, so callers can match on it:

```rust
use validations_rs::{ensure, validate, RuleCode, Violation};

pub struct Transfer {
    pub amount_cents: u64,
    pub from: String,
    pub to: String,
}

pub const SAME_ACCOUNT: RuleCode = RuleCode("TRANSFER_SAME_ACCOUNT");
pub const AMOUNT_ZERO: RuleCode = RuleCode("TRANSFER_AMOUNT_ZERO");

fn different_accounts(t: &Transfer) -> Result<(), Violation> {
    ensure(t.from != t.to, SAME_ACCOUNT, "the source and destination accounts must differ")
}

fn has_amount(t: &Transfer) -> Result<(), Violation> {
    ensure(t.amount_cents > 0, AMOUNT_ZERO, "the amount must be greater than zero")
}

let transfer = Transfer { amount_cents: 0, from: "A".into(), to: "B".into() };
let violation = validate(&transfer, &[&different_accounts, &has_amount]).unwrap_err();
assert_eq!(violation.code(), AMOUNT_ZERO);
```

A closure that captures configuration is a rule too. When a rule needs configuration that callers set,
and you want it reusable and named, make it a type and implement `Rule`:

```rust
use validations_rs::{ensure, validate, Rule, RuleCode, Violation};

pub struct Transfer { pub amount_cents: u64 }

pub const OVER_LIMIT: RuleCode = RuleCode("TRANSFER_OVER_LIMIT");

pub struct DailyLimit { pub cents: u64 }

impl Rule<Transfer> for DailyLimit {
    fn check(&self, t: &Transfer) -> Result<(), Violation> {
        ensure(t.amount_cents <= self.cents, OVER_LIMIT, "the transfer exceeds the daily limit")
            .map_err(|v| v.with_detail("limit", self.cents))
    }
}

let violation = validate(&Transfer { amount_cents: 900 }, &[&DailyLimit { cents: 500 }]).unwrap_err();
assert_eq!(violation.detail("limit"), Some("500"));
```

## The catalog

The `rules` module has the checks every project writes. Each one reports a stable code from
`rules::codes`.

| Rule | Checks | Code |
|---|---|---|
| `NotBlank` | text with a non-whitespace character | `VALUE_BLANK` |
| `Length::between(min, max)`, `at_least`, `at_most` | number of characters | `VALUE_LENGTH_OUT_OF_RANGE` |
| `Email` | a well-formed address (structural, not RFC 5322) | `EMAIL_INVALID` |
| `Min(x)`, `Max(x)` | any `PartialOrd + Display` value | `VALUE_BELOW_MIN`, `VALUE_ABOVE_MAX` |
| `InRange::new(min, max)` | inclusive range | `VALUE_OUT_OF_RANGE` |
| `Positive`, `PositiveOrZero`, `Negative`, `NegativeOrZero` | sign of a primitive number | `VALUE_NOT_POSITIVE`, `VALUE_NEGATIVE`, `VALUE_NOT_NEGATIVE`, `VALUE_POSITIVE` |
| `IsSome`, `IsNone` | an `Option` is present or absent | `VALUE_MISSING`, `VALUE_PRESENT` |
| `IsTrue`, `IsFalse` | a flag | `VALUE_NOT_TRUE`, `VALUE_NOT_FALSE` |
| `NotEmpty` | a collection with an element | `COLLECTION_EMPTY` |
| `Count::between(min, max)`, `at_least`, `at_most` | number of elements | `SIZE_OUT_OF_RANGE` |
| `Past::new()`, `Future::new()` | a `SystemTime` before or after now; `.with_clock(...)` for tests | `DATE_NOT_PAST`, `DATE_NOT_FUTURE` |

Text rules accept anything that is `AsRef<str>`; collection rules accept anything a shared reference can
iterate (slices, arrays, `Vec`, maps, sets).

```rust
use validations_rs::rules::{Count, Length, NotBlank, Positive};
use validations_rs::{validate_all, Rule};

assert!(Positive.check(&3_i64).is_ok());
assert!(Count::at_most(2).check(&vec!["a", "b", "c"]).is_err());

let violations = validate_all("  ", &[&NotBlank, &Length::at_least(3)]).unwrap_err();
assert_eq!(violations.len(), 2);
```

Not included on purpose, to stay dependency-free: regular expressions (use the `regex` crate in your own
rule) and arbitrary-precision decimals (`Min` and `Max` work with any ordered type, so your decimal type
works if it implements `PartialOrd + Display`).

## Adapting rules

`RuleExt` is implemented for every rule. Change the code to one of your domain, change the message, or
check a field of a larger value. `on` records the field name in the violation, and nested `on` builds a
path such as `address.city`:

```rust
use validations_rs::rules::{Min, NotBlank};
use validations_rs::{validate_all, Rule, RuleCode, RuleExt};

struct Address { city: String }
struct Customer { age: u8, address: Address }

let city = NotBlank.with_message("Tell us your city.").on("city", |a: &Address| &a.city);

let violations = validate_all(&Customer { age: 16, address: Address { city: "".into() } }, &[
    &Min(18).with_code(RuleCode("UNDERAGE")).on("age", |c: &Customer| &c.age),
    &city.on("address", |c: &Customer| &c.address),
])
.unwrap_err();

let fields: Vec<_> = violations.iter().map(|v| v.field()).collect();
assert_eq!(fields, [Some("age"), Some("address.city")]);
assert_eq!(violations.first().code().as_str(), "UNDERAGE");
assert_eq!(violations.first().detail("min"), Some("18"));
```

## Reading the error

Decide by code; show the message; use the field and the details to point at the input:

```rust
use validations_rs::rules::{codes, Length};
use validations_rs::Rule;

let violation = Length::between(2, 40).check("A").unwrap_err();

match violation.code() {
    codes::VALUE_LENGTH_OUT_OF_RANGE => {
        assert_eq!(violation.detail("length"), Some("1"));
        assert_eq!(violation.detail("min"), Some("2"));
    }
    other => panic!("unexpected rule {other}"),
}
```

With the `serde` feature, `Violation` serializes as
`{"code": "...", "message": "...", "field": "..." | null, "details": {"key": "value"}}`, and `Violations`
as a list of them, ready for an HTTP error body.

## Asynchronous rules

A rule that queries a database or another service implements `AsyncRule`. The trait returns a boxed
future, so it works behind `&dyn` with any runtime. Report a broken rule with a `Violation` and a failed
check with `AsyncError::check_failed(cause)`:

```rust
use validations_rs::{validate_async, AsyncError, AsyncRule, RuleCode, RuleFuture, Violation};

pub trait Users: Send + Sync {
    fn email_exists(&self, email: &str) -> Result<bool, std::io::Error>;
}

pub const EMAIL_TAKEN: RuleCode = RuleCode("EMAIL_TAKEN");

pub struct EmailNotTaken<'r> {
    pub users: &'r dyn Users,
}

impl AsyncRule<str> for EmailNotTaken<'_> {
    fn check<'a>(&'a self, email: &'a str) -> RuleFuture<'a> {
        Box::pin(async move {
            if self.users.email_exists(email).map_err(AsyncError::check_failed)? {
                return Err(Violation::new(EMAIL_TAKEN, "the email is already registered").into());
            }
            Ok(())
        })
    }
}

async fn register(users: &dyn Users, email: &str) -> Result<(), AsyncError> {
    validate_async(email, &[&EmailNotTaken { users }]).await
}
```

Run the synchronous rules first with `validate`, so cheap checks fail before any I/O. At the boundary,
map `AsyncError::Violation` to a client error (such as HTTP 422) and `AsyncError::CheckFailed` to a server
error (such as HTTP 503).

## Migrating from 0.2

See the [CHANGELOG](CHANGELOG.md) for the full table. In short: `BusinessRuleError` is `Violation`,
`BusinessLogicValidation` is `Rule`, `start_validations(&rules, value)` is `validate(value, &rules)`,
`collect_validations` is `validate_all`, the `*Validation` structs are the `rules::*` catalog, and
`PredicateValidation` is a plain function with `ensure`.

## Development

```sh
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

The examples in this README and in the API docs compile and run as tests.

## License

Apache-2.0. See the [LICENSE](LICENSE) file.
