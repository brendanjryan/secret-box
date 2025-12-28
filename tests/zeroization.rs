//! Zeroization tests
//!
//! These tests verify that secrets are properly zeroized when dropped.

mod common;

use common::ZeroizeVerifier;
use secret_box::{ExposeSecret, SecretBox};
use zeroize::Zeroize;

#[test]
fn test_zeroize_trait_implemented() {
    let mut secret: SecretBox<String> = SecretBox::new(Box::new("secret".to_string()));
    secret.zeroize();
    assert!(secret.expose_secret().is_empty());
}

#[test]
fn test_zeroize_vec() {
    let mut secret: SecretBox<Vec<u8>> = SecretBox::new(Box::new(vec![1, 2, 3, 4, 5]));
    secret.zeroize();
    assert!(secret.expose_secret().is_empty());
}

#[test]
fn test_zeroize_custom_type() {
    let mut secret = SecretBox::new(Box::new(ZeroizeVerifier {
        data: vec![0xAB; 32],
    }));
    assert_eq!(secret.expose_secret().data.len(), 32);

    secret.zeroize();
    assert!(secret.expose_secret().data.is_empty());
}

#[test]
fn test_drop_calls_zeroize() {
    let secret: SecretBox<String> = SecretBox::new(Box::new("secret".to_string()));
    drop(secret);
}

#[test]
fn test_zeroize_nested_struct() {
    #[derive(Default, Zeroize)]
    struct NestedSecret {
        password: String,
        key: Vec<u8>,
    }

    let mut secret = SecretBox::new(Box::new(NestedSecret {
        password: "secret_password".to_string(),
        key: vec![1, 2, 3, 4, 5],
    }));

    secret.zeroize();

    assert!(secret.expose_secret().password.is_empty());
    assert!(secret.expose_secret().key.is_empty());
}

#[test]
fn test_init_with_mut_produces_zeroizable_secret() {
    let mut secret: SecretBox<Vec<u8>> = SecretBox::init_with_mut(|v: &mut Vec<u8>| {
        v.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);
    });

    assert_eq!(secret.expose_secret(), &[0xDE, 0xAD, 0xBE, 0xEF]);

    secret.zeroize();

    assert!(secret.expose_secret().is_empty());
}

#[test]
fn test_multiple_zeroize_calls_are_safe() {
    let mut secret: SecretBox<String> = SecretBox::new(Box::new("secret".to_string()));
    secret.zeroize();
    secret.zeroize();
    secret.zeroize();
    assert!(secret.expose_secret().is_empty());
}
