//! Shared test utilities

#![allow(dead_code)]

use zeroize::Zeroize;

/// Test secret string constant
pub const TEST_SECRET: &str = "super_secret_password_123";

/// Custom type for zeroization verification
#[derive(Debug, Zeroize)]
pub struct ZeroizeVerifier {
    pub data: Vec<u8>,
}

impl Default for ZeroizeVerifier {
    fn default() -> Self {
        Self {
            data: vec![0xAB; 32],
        }
    }
}

/// Helper to check debug redaction
pub fn assert_debug_redacted<T: std::fmt::Debug>(secret: &T) {
    let debug_str = format!("{:?}", secret);
    assert!(
        debug_str.contains("REDACTED"),
        "Debug output should contain REDACTED: {}",
        debug_str
    );
    assert!(
        !debug_str.contains(TEST_SECRET),
        "Debug output should not contain the secret: {}",
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
