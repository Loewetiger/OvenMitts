//! Password hashing, verification and random number generation

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use rand::distributions::{Alphanumeric, DistString};
use rand_core::{OsRng, RngCore};

/// Generate a random byte array of the given length with the OS's secure random number generator.
#[must_use]
pub fn random_data(size: usize) -> Vec<u8> {
    let mut key = vec![0; size];
    OsRng.fill_bytes(&mut key);
    key
}

/// Hash a password using Argon2id.
pub fn hash_password(password: &[u8]) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt)?.to_string();
    Ok(password_hash)
}

/// Verify a password against a given hash.
pub fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let result = Argon2::default()
        .verify_password(password, &parsed_hash)
        .is_ok();
    Ok(result)
}

/// Generate a random stream key.
pub fn gen_stream_key() -> String {
    format!(
        "stream_{}_{}",
        Alphanumeric.sample_string(&mut OsRng, 8),
        Alphanumeric.sample_string(&mut OsRng, 30)
    )
}
