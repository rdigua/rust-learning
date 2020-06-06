//Uses ring::hmac to creates a hmac::Signature of a string then verifies the signature is correct.
extern crate ring;

use ring::{digest, hmac, rand};
use ring::rand::SecureRandom;
use ring::error::Unspecified;

fn main() -> Result<(), Unspecified> {
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key = hmac::SigningKey::new(&digest::SHA256, &key_value);

    let message = "Legitimate and important message.";
    let signature = hmac::sign(&key, message.as_bytes());
    ring::hmac::verify_with_own_key(&key, message.as_bytes(), signature.as_ref())?;

    Ok(())
}

