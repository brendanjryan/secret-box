//! Shared test utilities

#![allow(dead_code)]

use zeroize::Zeroize;

pub const TEST_SECRET: &str = "super_secret_password_123";

#[derive(Debug, Default, Zeroize)]
pub struct ZeroizeVerifier {
    pub data: Vec<u8>,
}

pub fn assert_debug_redacted<T: std::fmt::Debug>(secret: &T) {
    let debug_str = format!("{:?}", secret);
    assert!(
        debug_str.contains("REDACTED"),
        "Expected REDACTED: {}",
        debug_str
    );
    assert!(
        !debug_str.contains(TEST_SECRET),
        "Secret leaked: {}",
        debug_str
    );
}

/// For serde tests - a serializable secret type
#[cfg(feature = "serde")]
#[derive(Debug, Clone, Zeroize, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct TestSerializable {
    pub value: String,
}

#[cfg(feature = "serde")]
impl secret_box::SerializableSecret for TestSerializable {}
