use secret_box::{ExposeSecret, SecretBox};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    api_key: SecretBox<String>,
}

fn main() {
    // Deserialize secrets from config
    let json = r#"{"api_key":"secret123"}"#;
    let config: Config = serde_json::from_str(json).unwrap();

    println!(
        "Loaded key length: {}",
        config.api_key.expose_secret().len()
    );
    println!("{:?}", config.api_key); // SecretBox<String>([REDACTED])

    // Note: Cannot serialize SecretBox<String> without implementing SerializableSecret
    // This is intentional to prevent accidental secret leakage
}
