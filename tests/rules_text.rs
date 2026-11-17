use validations_rs::rules::{codes, Email, Length, NotBlank};
use validations_rs::Rule;

#[test]
fn not_blank_accepts_text_with_content() {
    for value in ["a", " a ", "ñ", "\u{00a0}x"] {
        assert!(NotBlank.check(value).is_ok(), "should accept {value:?}");
    }
}

#[test]
fn not_blank_rejects_empty_and_whitespace() {
    for value in ["", " ", "\t\n", "\u{2003}"] {
        assert_eq!(NotBlank.check(value).unwrap_err().code(), codes::VALUE_BLANK, "should reject {value:?}");
    }
}

#[test]
fn not_blank_works_with_string_and_str() {
    assert!(NotBlank.check(&String::from("Ada")).is_ok());
    assert!(NotBlank.check("Ada").is_ok());
}

#[test]
fn length_counts_characters_not_bytes() {
    assert!(Length::between(5, 5).check("ñandú").is_ok());
    assert!(Length::between(1, 1).check("é").is_ok());
}

#[test]
fn length_bounds_are_inclusive() {
    let rule = Length::between(2, 4);
    assert!(rule.check("ab").is_ok());
    assert!(rule.check("abcd").is_ok());
    assert!(rule.check("a").is_err());
    assert!(rule.check("abcde").is_err());
}

#[test]
fn length_reports_the_length_and_bounds() {
    let violation = Length::between(2, 4).check("abcde").unwrap_err();
    assert_eq!(violation.code(), codes::VALUE_LENGTH_OUT_OF_RANGE);
    assert_eq!(violation.detail("length"), Some("5"));
    assert_eq!(violation.detail("min"), Some("2"));
    assert_eq!(violation.detail("max"), Some("4"));
}

#[test]
fn length_at_least_and_at_most() {
    assert!(Length::at_least(3).check("abc").is_ok());
    assert!(Length::at_least(3).check("ab").is_err());
    assert!(Length::at_most(2).check("").is_ok());
    assert!(Length::at_most(2).check("abc").is_err());
}

#[test]
fn email_accepts_well_formed_addresses() {
    for email in [
        "ada@example.com",
        "ada.lovelace@example.com",
        "ada+news@sub.example.co.uk",
        "a_b-c@ex-ample.org",
        "1@2.io",
        "ñandú@exämple.com",
    ] {
        assert!(Email.check(email).is_ok(), "should accept {email}");
    }
}

#[test]
fn email_rejects_malformed_addresses() {
    let long_local = format!("{}@example.com", "a".repeat(65));
    let long_label = format!("ada@{}.com", "a".repeat(64));
    let too_long = format!("{}@{}.com", "a".repeat(64), "b".repeat(190));
    for email in [
        "",
        "ada",
        "ada@",
        "@example.com",
        "ada@example",
        "ada@@example.com",
        "ada@exa@mple.com",
        "a da@example.com",
        "ada@exa mple.com",
        ".ada@example.com",
        "ada.@example.com",
        "a..da@example.com",
        "ada@.example.com",
        "ada@example..com",
        "ada@example.com.",
        "ada@-example.com",
        "ada@example-.com",
        "ada(comment)@example.com",
        long_local.as_str(),
        long_label.as_str(),
        too_long.as_str(),
    ] {
        assert_eq!(Email.check(email).unwrap_err().code(), codes::EMAIL_INVALID, "should reject {email:?}");
    }
}

#[test]
fn email_reports_the_address() {
    assert_eq!(Email.check("ada").unwrap_err().detail("email"), Some("ada"));
}
