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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key_length() {
        let psk = "my-secret-password";
        let key = derive_key(psk);
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_derive_key_is_deterministic() {
        let psk = "a-very-secure-password";
        let key1 = derive_key(psk);
        let key2 = derive_key(psk);
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_derive_key_is_different_for_different_psks() {
        let psk1 = "password-one";
        let psk2 = "password-two";
        let key1 = derive_key(psk1);
        let key2 = derive_key(psk2);
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_generate_nonce() {
        let seq_num = 1234567890;
        let nonce = generate_nonce(seq_num);
        // The first 4 bytes should be zero, the rest should be the sequence number
        let mut expected = [0u8; 12];
        expected[4..].copy_from_slice(&seq_num.to_be_bytes());
        assert_eq!(*nonce, expected);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip_success() {
        let psk = "roundtrip-psk";
        let key = derive_key(psk);
        let plaintext = b"hello onebox!";
        let sequence_number = 100;

        let ciphertext = encrypt(&key, plaintext, sequence_number).unwrap();
        let decrypted_plaintext = decrypt(&key, &ciphertext, sequence_number).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted_plaintext);
    }

    #[test]
    fn test_decrypt_tampered_ciphertext() {
        let psk = "tamper-test-psk";
        let key = derive_key(psk);
        let plaintext = b"this is a secret message";
        let sequence_number = 200;

        let mut ciphertext = encrypt(&key, plaintext, sequence_number).unwrap();

        // Flip a bit in the ciphertext
        let last_byte_index = ciphertext.len() - 1;
        ciphertext[last_byte_index] ^= 0x01;

        let result = decrypt(&key, &ciphertext, sequence_number);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Decryption failed"));
    }

    #[test]
    fn test_decrypt_wrong_key() {
        let psk1 = "correct-key";
        let psk2 = "wrong-key";
        let key1 = derive_key(psk1);
        let key2 = derive_key(psk2);
        let plaintext = b"message encrypted with key1";
        let sequence_number = 300;

        let ciphertext = encrypt(&key1, plaintext, sequence_number).unwrap();

        // Try to decrypt with the wrong key
        let result = decrypt(&key2, &ciphertext, sequence_number);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_wrong_sequence_number() {
        let psk = "sequence-psk";
        let key = derive_key(psk);
        let plaintext = b"this depends on the sequence number";
        let sequence_number = 400;
        let wrong_sequence_number = 401;

        let ciphertext = encrypt(&key, plaintext, sequence_number).unwrap();

        // Try to decrypt with the wrong sequence number (which generates a different nonce)
        let result = decrypt(&key, &ciphertext, wrong_sequence_number);
        assert!(result.is_err());
    }
}
