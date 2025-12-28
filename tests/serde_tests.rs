//! Serde integration tests
//!
//! These tests verify that serialization and deserialization work correctly.

#![cfg(feature = "serde")]

mod common;

use common::TestSerializable;
use secret_box::{ExposeSecret, SecretBox};
use serde::Deserialize;

#[test]
fn test_deserialize_string() {
    let json = r#""secret_password""#;
    let secret: SecretBox<String> = serde_json::from_str(json).unwrap();
    assert_eq!(secret.expose_secret(), "secret_password");
}

#[test]
fn test_deserialize_in_struct() {
    #[derive(Deserialize)]
    struct Config {
        api_key: SecretBox<String>,
        debug: bool,
    }

    let json = r#"{"api_key": "my_api_key", "debug": true}"#;
    let config: Config = serde_json::from_str(json).unwrap();

    assert_eq!(config.api_key.expose_secret(), "my_api_key");
    assert!(config.debug);
}

#[test]
fn test_deserialize_nested_struct() {
    #[derive(Deserialize)]
    struct Credentials {
        username: String,
        password: SecretBox<String>,
    }

    #[derive(Deserialize)]
    struct Config {
        credentials: Credentials,
    }

    let json = r#"{"credentials": {"username": "user", "password": "pass"}}"#;
    let config: Config = serde_json::from_str(json).unwrap();

    assert_eq!(config.credentials.username, "user");
    assert_eq!(config.credentials.password.expose_secret(), "pass");
}

#[test]
fn test_deserialize_custom_type() {
    let json = r#"{"value": "custom_secret"}"#;
    let secret: SecretBox<TestSerializable> = serde_json::from_str(json).unwrap();
    assert_eq!(secret.expose_secret().value, "custom_secret");
}

#[test]
fn test_serialize_with_marker_trait() {
    let inner = TestSerializable {
        value: "serializable_secret".to_string(),
    };
    let secret = SecretBox::new(Box::new(inner));
    let json = serde_json::to_string(&secret).unwrap();
    assert!(json.contains("serializable_secret"));
}

#[test]
fn test_round_trip_serialization() {
    let original = TestSerializable {
        value: "round_trip_test".to_string(),
    };
    let secret = SecretBox::new(Box::new(original.clone()));
    let json = serde_json::to_string(&secret).unwrap();
    let restored: SecretBox<TestSerializable> = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.expose_secret(), &original);
}

#[test]
fn test_deserialize_vec() {
    let json = r#"[1, 2, 3, 4, 5]"#;
    let secret: SecretBox<Vec<u8>> = serde_json::from_str(json).unwrap();
    assert_eq!(secret.expose_secret(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_debug_still_redacts_after_deserialize() {
    let json = r#""super_secret_value""#;
    let secret: SecretBox<String> = serde_json::from_str(json).unwrap();

    let debug_str = format!("{:?}", secret);
    assert!(debug_str.contains("REDACTED"));
    assert!(!debug_str.contains("super_secret_value"));
}
