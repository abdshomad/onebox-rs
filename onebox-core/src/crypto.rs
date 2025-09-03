//! Cryptographic operations for onebox-rs

use aead::{Aead, KeyInit};
use anyhow::Result;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};

const KEY_CONTEXT: &str = "onebox-rs-encryption-key-context";

/// Derives a 256-bit key from a Pre-Shared Key (PSK) string.
/// Uses BLAKE3 in its Key Derivation Function (KDF) mode.
pub fn derive_key(psk: &str) -> Key {
    let mut hasher = blake3::Hasher::new_keyed(blake3::hash(KEY_CONTEXT.as_bytes()).as_bytes());
    hasher.update(psk.as_bytes());
    let mut output_reader = hasher.finalize_xof();
    let mut key = [0u8; 32];
    output_reader.fill(&mut key);
    key.into()
}

/// Generates a 12-byte nonce from a 64-bit sequence number.
/// The sequence number is written as big-endian bytes and padded with zeros.
pub fn generate_nonce(sequence_number: u64) -> Nonce {
    let mut nonce_bytes = [0u8; 12];
    nonce_bytes[4..].copy_from_slice(&sequence_number.to_be_bytes());
    nonce_bytes.into()
}

/// Encrypts a plaintext payload using ChaCha20-Poly1305.
pub fn encrypt(key: &Key, plaintext: &[u8], sequence_number: u64) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = generate_nonce(sequence_number);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
    Ok(ciphertext)
}

/// Decrypts a ciphertext payload using ChaCha20-Poly1305.
/// Returns an error if authentication fails.
pub fn decrypt(key: &Key, ciphertext: &[u8], sequence_number: u64) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = generate_nonce(sequence_number);
    let plaintext = cipher
        .decrypt(&nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed (authentication tag mismatch): {}", e))?;
    Ok(plaintext)
}
