# secret-box

Safe boxing mechanism for sensitive values in Rust.

## Core features

- **Explicit secret access** via `ExposeSecret` trait - makes secret access auditable in code reviews
- **Automatic memory zeroization** on drop using the `zeroize` crate
- **Debug redaction** prevents accidental logging of secrets
- **Optional serde support** with opt-in serialization to prevent accidental secret exfiltration

## Installation

```toml
[dependencies]
secret-box = "0.1"

# with serde support 
# [dependencies]
# secret-box = { version = "0.1", features = ["serde"] }
```

## Basic Usage

```rust
use secret_box::{SecretBox, ExposeSecret};

// Create a secret password
let password: SecretBox<String> = SecretBox::new(Box::new("super_secret".to_string()));

// Access requires explicit expose_secret()
println!("Password length: {}", password.expose_secret().len());

// Debug output is redacted
println!("{:?}", password); // Prints: SecretBox<String>([REDACTED])

// Secret is automatically zeroized when dropped
```

## API Overview

### `SecretBox<S>`

The main wrapper type for secrets. Generic over any type that implements `Zeroize`.

```rust
// From pre-boxed value
let secret = SecretBox::new(Box::new("password".to_string()));

// Initialize in-place on heap (safest method)
let secret: SecretBox<Vec<u8>> = SecretBox::init_with_mut(|v: &mut Vec<u8>| {
    v.extend_from_slice(b"secret_bytes");
});

// From String or Vec<u8> directly
let secret: SecretBox<String> = "password".to_string().into();
let secret: SecretBox<Vec<u8>> = vec![1, 2, 3].into();
```

### `ExposeSecret<S>`

Trait for accessing the secret value. This is the only way to access the secret, making secret access explicit and auditable.

```rust
use secret_box::ExposeSecret;

let secret = SecretBox::new(Box::new("password".to_string()));
let value: &String = secret.expose_secret();
```

### Serde Support

With the `serde` feature enabled:

- **Deserialization**: `SecretBox<T>` can be deserialized from any type that implements `DeserializeOwned`
- **Serialization**: Requires implementing the `SerializableSecret` marker trait to prevent accidental secret exfiltration

```rust
use secret_box::{SecretBox, ExposeSecret, SerializableSecret};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Config {
    api_key: SecretBox<String>,
}

// Deserialize secrets from config
let json = r#"{"api_key":"secret123"}"#;
let config: Config = serde_json::from_str(json).unwrap();

// Note: SecretBox<String> cannot be serialized without implementing SerializableSecret
// This prevents accidental secret leakage via serialization
```

## License

Apache-2.0
