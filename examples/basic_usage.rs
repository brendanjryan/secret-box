use secret_box::{ExposeSecret, SecretBox};

fn main() {
    // Create from String using .into() - shows asterisks in debug
    let password: SecretBox<String> = "my_password".to_string().into();
    println!("Password length: {}", password.expose_secret().len());
    println!("{:?}", password); // SecretBox<String>(***********)

    // Create with Box::new - shows [REDACTED] in debug
    let api_key: SecretBox<Vec<u8>> = SecretBox::new(Box::new(b"secret_key".to_vec()));
    println!("Key length: {}", api_key.expose_secret().len());
    println!("{:?}", api_key); // SecretBox<Vec<u8>>([REDACTED])

    // Automatically zeroized when dropped
}
