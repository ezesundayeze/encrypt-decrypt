#[cfg(test)]
use super::*;
use hex::FromHexError;

#[test]
fn test_encrypt_decrypt() -> Result<(), FromHexError> {
    let iv = b"1234567890123456";
    let key = b"01234567890123456789012345678901";
    let plaintext = b"Eze";

    let ciphertext = encrypt(iv, key, plaintext);
    let decrypted_text = decrypt(iv, key, &ciphertext)?;

    assert_eq!(decrypted_text, plaintext);

    Ok(())
}
