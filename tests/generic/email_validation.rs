use validations_rs::{BusinessLogicValidation, EmailValidation};

#[test]
fn accepts_well_formed_addresses() {
    let rule = EmailValidation::new();
    for email in [
        "ada@example.com",
        "ada.lovelace@example.com",
        "ada+news@sub.example.co.uk",
        "a_b-c@ex-ample.org",
        "ñandú@exämple.com",
        "1@2.io",
    ] {
        assert!(rule.validate(email).is_ok(), "should accept {email}");
    }
}

#[test]
fn rejects_malformed_addresses() {
    let rule = EmailValidation::new();
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
        "ada@-example.com",
        "ada@example-.com",
        "ada@example..com",
        "ada@example.com.",
        "ada@.example.com",
        "ada(comment)@example.com",
    ] {
        let error = rule.validate(email).unwrap_err();
        assert_eq!(error.code, EmailValidation::CODE, "should reject {email:?}");
    }
}

#[test]
fn rejects_addresses_over_the_length_limits() {
    let rule = EmailValidation::new();
    assert!(rule.validate(&format!("{}@example.com", "a".repeat(64))).is_ok());
    assert!(rule.validate(&format!("{}@example.com", "a".repeat(65))).is_err());
    assert!(rule.validate(&format!("ada@{}.com", "a".repeat(63))).is_ok());
    assert!(rule.validate(&format!("ada@{}.com", "a".repeat(64))).is_err());
    assert!(rule.validate(&format!("ada@{}.com", "a.".repeat(130))).is_err());
}

#[test]
fn reports_the_rejected_value_and_default_message() {
    let error = EmailValidation::new().validate("ada").unwrap_err();
    assert_eq!(error.message, "The email address does not have a valid format.");
    assert_eq!(error.detail("email"), Some("ada"));
}

#[test]
fn custom_message_keeps_the_code() {
    let error = EmailValidation::new().with_message("Check your email.").validate("ada").unwrap_err();
    assert_eq!(error.code, EmailValidation::CODE);
    assert_eq!(error.message, "Check your email.");
}
