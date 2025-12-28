//! Safe boxing mechanism for sensitive values
//!
//! # Example
//!
//! ```
//! use secret_box::{SecretBox, ExposeSecret};
//!
//! let password: SecretBox<String> = "my_password".to_string().into();
//! println!("Length: {}", password.expose_secret().len());
//! println!("{:?}", password); // Prints: SecretBox<alloc::string::String>(***********)
//! ```
//!
//! # Features
//!
//! - `serde`: Enable serialization/deserialization support

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::{
    any,
    fmt::{self, Debug},
};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub use zeroize;

/// Wrapper type for values that contain secrets, which attempts to limit
/// accidental exposure and ensure secrets are wiped from memory when dropped.
///
/// Access to the secret inner value occurs through the [`ExposeSecret`] trait,
/// which provides a method for accessing the inner secret value.
///
/// # Example
///
/// ```
/// use secret_box::{SecretBox, ExposeSecret};
///
/// // Create a secret password
/// let password = SecretBox::new(Box::new("super_secret".to_string()));
///
/// // Access requires explicit expose_secret()
/// assert_eq!(password.expose_secret(), "super_secret");
///
/// // Debug output masks the secret (shows asterisks when length is known)
/// let debug_output = format!("{:?}", password);
/// assert!(!debug_output.contains("super_secret"));
/// ```
pub struct SecretBox<S: Zeroize> {
    inner: Box<S>,
    length: Option<usize>,
}

impl<S: Zeroize> SecretBox<S> {
    /// Create a secret value using a pre-boxed value.
    ///
    /// This is the primary constructor. The value must already be on the heap
    /// (in a `Box`) to avoid leaving copies on the stack.
    ///
    /// # Example
    ///
    /// ```
    /// use secret_box::SecretBox;
    ///
    /// let secret = SecretBox::new(Box::new("password".to_string()));
    /// ```
    pub fn new(boxed: Box<S>) -> Self {
        Self {
            inner: boxed,
            length: None,
        }
    }
}

impl<S: Zeroize + Default> SecretBox<S> {
    /// Create a secret value by initializing it in-place on the heap.
    ///
    /// This is the safest construction method as the secret value never
    /// exists on the stack - it's initialized directly on the heap.
    ///
    /// # Example
    ///
    /// ```
    /// use secret_box::{SecretBox, ExposeSecret};
    ///
    /// let secret: SecretBox<Vec<u8>> = SecretBox::init_with_mut(|v: &mut Vec<u8>| {
    ///     v.extend_from_slice(b"secret_bytes");
    /// });
    ///
    /// assert_eq!(secret.expose_secret(), b"secret_bytes");
    /// ```
    pub fn init_with_mut(f: impl FnOnce(&mut S)) -> Self {
        let mut secret = Self {
            inner: Box::default(),
            length: None,
        };
        f(&mut secret.inner);
        secret
    }
}

impl<S: Zeroize> Zeroize for SecretBox<S> {
    fn zeroize(&mut self) {
        self.inner.zeroize()
    }
}

impl<S: Zeroize> Drop for SecretBox<S> {
    fn drop(&mut self) {
        self.zeroize()
    }
}

impl<S: Zeroize> ZeroizeOnDrop for SecretBox<S> {}

impl<S: Zeroize> Debug for SecretBox<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.length {
            Some(len) => {
                write!(f, "SecretBox<{}>(", any::type_name::<S>())?;
                for _ in 0..len {
                    write!(f, "*")?;
                }
                write!(f, ")")
            }
            None => write!(f, "SecretBox<{}>([REDACTED])", any::type_name::<S>()),
        }
    }
}

impl<S: Zeroize> From<Box<S>> for SecretBox<S> {
    fn from(boxed: Box<S>) -> Self {
        Self::new(boxed)
    }
}

/// Expose a reference to an inner secret.
///
/// This trait provides the only method for accessing a secret value,
/// making secret access explicit and auditable in code reviews.
pub trait ExposeSecret<S> {
    /// Expose the secret value.
    ///
    /// This is the only method providing access to a secret.
    fn expose_secret(&self) -> &S;
}

impl<S: Zeroize> ExposeSecret<S> for SecretBox<S> {
    fn expose_secret(&self) -> &S {
        &self.inner
    }
}

impl From<String> for SecretBox<String> {
    fn from(s: String) -> Self {
        let length = s.len();
        Self {
            inner: Box::new(s),
            length: Some(length),
        }
    }
}

impl From<Vec<u8>> for SecretBox<Vec<u8>> {
    fn from(v: Vec<u8>) -> Self {
        let length = v.len();
        Self {
            inner: Box::new(v),
            length: Some(length),
        }
    }
}

/// Marker trait for secret types which can be serialized by serde.
///
/// By default, `SecretBox<T>` does NOT implement `Serialize` to prevent
/// accidental exfiltration of secrets via serialization.
///
/// To allow serialization, implement this marker trait on `T`:
///
/// ```
/// use secret_box::SerializableSecret;
/// use serde::Serialize;
/// use zeroize::Zeroize;
///
/// #[derive(Serialize, Zeroize)]
/// struct MySecret {
///     key: String,
/// }
///
/// impl SerializableSecret for MySecret {}
/// ```
#[cfg(feature = "serde")]
pub trait SerializableSecret: Serialize {}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for SecretBox<T>
where
    T: Zeroize + DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(Self::new(Box::new(value)))
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for SecretBox<T>
where
    T: Zeroize + SerializableSecret,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn test_debug_redaction() {
        let secret = SecretBox::new(Box::new("super_secret".to_string()));
        let debug = alloc::format!("{:?}", secret);
        assert!(debug.contains("REDACTED") && !debug.contains("super_secret"));
    }

    #[test]
    fn test_debug_asterisks() {
        let secret: SecretBox<String> = "hello".to_string().into();
        let debug = alloc::format!("{:?}", secret);
        assert!(debug.contains("*****") && !debug.contains("hello"));

        let secret: SecretBox<Vec<u8>> = vec![1, 2, 3].into();
        let debug = alloc::format!("{:?}", secret);
        assert!(debug.contains("***"));
    }
}
