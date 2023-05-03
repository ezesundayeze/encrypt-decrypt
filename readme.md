## A basic encryption and decryption implementation with Rust

<p>You can clone the repo to run it.</p>

```rust
use hex::FromHexError;
use openssl::symm::{Cipher, Crypter, Mode};

/// This function encrypts a plaintext message using AES-256 in CBC mode with the provided initialization vector and secret key.
///
/// # Arguments
///
/// * `iv` - An initialization vector for the encryption process.
/// * `key` - A secret key used to encrypt the message.
/// * `text` - The plaintext message to be encrypted.
///
/// # Example
///
/// ```
/// use aes::encrypt;
/// let iv = b"1234567890123456";
/// let key = b"01234567890123456789012345678901";
/// let text = b"Hello, world!";
/// let ciphertext = encrypt(iv, key, text);
/// assert_eq!(ciphertext, [197, 221, 61, 37, 184, 139, 38, 189, 182, 53, 144, 45, 170, 182, 220, 210, 64, 155, 36, 239, 138, 38, 33, 48, 25, 101, 160, 99, 8, 24, 111, 137]);
/// ```
pub fn encrypt(iv: &[u8], key: &[u8], text: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_256_cbc();
    let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv)).unwrap();

    let block_size = cipher.block_size();
    let mut encrypted_data = vec![0; text.len() + block_size];
    let count = encrypter.update(text, &mut encrypted_data).unwrap();
    let rest = encrypter.finalize(&mut encrypted_data[count..]).unwrap();
    encrypted_data.truncate(count + rest);

    encrypted_data
}

/// This function decrypts an AES-256 encrypted message in CBC mode with the provided initialization vector and secret key.
/// 
/// # Arguments
///
/// * `iv` - An initialization vector for the decryption process.
/// * `key` - A secret key used to decrypt the message.
/// * `encrypted_data` - The ciphertext message to be decrypted.
///
/// # Example
///
/// ```
/// use aes::decrypt;
/// use hex::FromHexError;
/// let iv = b"1234567890123456";
/// let key = b"01234567890123456789012345678901";
/// let ciphertext = [202, 234, 210, 161, 130, 10, 237, 118, 76, 66, 52, 178, 12, 8, 2, 241];
/// let plaintext = decrypt(iv, key, &ciphertext).unwrap();
/// assert_eq!(plaintext, b"Hello, world!");
/// ```
pub fn decrypt(iv: &[u8], key: &[u8], encrypted_data: &[u8]) -> Result<Vec<u8>, FromHexError> {
    let cipher = Cipher::aes_256_cbc();
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv)).unwrap();

    let block_size = cipher.block_size();
    let mut decrypted_data = vec![0; encrypted_data.len() + block_size];
    let count = decrypter
        .update(encrypted_data, &mut decrypted_data)
        .unwrap();
    let rest = decrypter.finalize(&mut decrypted_data[count..]).unwrap();
    decrypted_data.truncate(count + rest);

    Ok(decrypted_data)
}

```

