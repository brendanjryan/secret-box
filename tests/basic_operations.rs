//! Basic operations integration tests

mod common;

use common::{assert_debug_redacted, TEST_SECRET};
use secret_box::{ExposeSecret, SecretBox};
use zeroize::Zeroize;

#[test]
fn test_secret_box_with_string() {
    let secret: SecretBox<String> = SecretBox::new(Box::new(TEST_SECRET.to_string()));
    assert_eq!(secret.expose_secret(), TEST_SECRET);
}

#[test]
fn test_secret_box_with_vec() {
    let data = vec![1u8, 2, 3, 4, 5];
    let secret: SecretBox<Vec<u8>> = SecretBox::new(Box::new(data.clone()));
    assert_eq!(secret.expose_secret(), &data);
}

#[test]
fn test_secret_box_with_custom_type() {
    #[derive(Debug, Default, PartialEq, Zeroize)]
    struct Credentials {
        username: String,
        password: String,
    }

    let creds = Credentials {
        username: "user".to_string(),
        password: "pass".to_string(),
    };

    let secret = SecretBox::new(Box::new(creds));
    assert_eq!(secret.expose_secret().username, "user");
    assert_eq!(secret.expose_secret().password, "pass");
}

#[test]
fn test_from_box_conversion() {
    let boxed = Box::new(TEST_SECRET.to_string());
    let secret: SecretBox<String> = boxed.into();
    assert_eq!(secret.expose_secret(), TEST_SECRET);
}

#[test]
fn test_from_string_conversion() {
    let secret: SecretBox<String> = TEST_SECRET.to_string().into();
    assert_eq!(secret.expose_secret(), TEST_SECRET);
}

#[test]
fn test_from_vec_conversion() {
    let data = vec![1u8, 2, 3];
    let secret: SecretBox<Vec<u8>> = data.clone().into();
    assert_eq!(secret.expose_secret(), &data);
}

#[test]
fn test_init_with_mut() {
    let secret: SecretBox<String> = SecretBox::init_with_mut(|s: &mut String| {
        s.push_str(TEST_SECRET);
    });
    assert_eq!(secret.expose_secret(), TEST_SECRET);
}

#[test]
fn test_init_with_mut_vec() {
    let secret: SecretBox<Vec<u8>> = SecretBox::init_with_mut(|v: &mut Vec<u8>| {
        v.extend_from_slice(&[1, 2, 3, 4, 5]);
    });
    assert_eq!(secret.expose_secret(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_debug_does_not_leak_string() {
    let secret: SecretBox<String> = SecretBox::new(Box::new(TEST_SECRET.to_string()));
    assert_debug_redacted(&secret);
}

#[test]
fn test_debug_does_not_leak_vec() {
    let secret: SecretBox<Vec<u8>> = SecretBox::new(Box::new(vec![1, 2, 3]));
    let debug_str = format!("{:?}", secret);
    assert!(debug_str.contains("REDACTED"));
    assert!(!debug_str.contains("[1, 2, 3]"));
}

#[test]
fn test_debug_shows_type_name() {
    let secret: SecretBox<String> = SecretBox::new(Box::new("test".to_string()));
    let debug_str = format!("{:?}", secret);
    assert!(debug_str.contains("String"));
    assert!(debug_str.contains("SecretBox"));
}

#[test]
fn test_multiple_secrets_are_independent() {
    let secret1: SecretBox<String> = SecretBox::new(Box::new("secret1".to_string()));
    let secret2: SecretBox<String> = SecretBox::new(Box::new("secret2".to_string()));

    assert_eq!(secret1.expose_secret(), "secret1");
    assert_eq!(secret2.expose_secret(), "secret2");
}

#[test]
fn test_empty_string_secret() {
    let secret: SecretBox<String> = SecretBox::new(Box::new(String::new()));
    assert_eq!(secret.expose_secret(), "");
}

#[test]
fn test_empty_vec_secret() {
    let secret: SecretBox<Vec<u8>> = SecretBox::new(Box::new(Vec::new()));
    assert!(secret.expose_secret().is_empty());
}
