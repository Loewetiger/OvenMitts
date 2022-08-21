//! Password hashing, verification and random number generation

use argon2::Config;
use rand_core::{OsRng, RngCore};

/// Generate a random byte array of the given length with the OS's secure random number generator.
pub fn random_data(size: usize) -> Vec<u8> {
    let mut key = vec![0; size];
    OsRng.fill_bytes(&mut key);
    key
}

/// Hash a password using Argon2id.
pub fn hash_password(password: &[u8]) -> Result<String, argon2::Error> {
    let salt = random_data(16);
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config)
}

/// Verify a password against a given hash.
pub fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}
