mod encryption;
use encryption::{decrypt, encrypt};
const ENCRYPTION_IV: &[u8] = b"47347d2f824ef9461b80a612bcb05e33"; // run: "openssl rand -hex 16" on your terminal
const ENCRYPTION_KEY: &[u8] = b"d3325226158920b2b547952bec81c2eba8d2129bb1359ace43c26958c79406b6"; // Run: "openssl rand -hex 32" on your terminal

fn main() {
    let iv = hex::decode(ENCRYPTION_IV).unwrap();
    let key = hex::decode(ENCRYPTION_KEY).unwrap();
    let plaintext = b"Eze";

    let encrypted_data = encrypt(&iv, &key, plaintext);
    let encrypted_hex = hex::encode(&encrypted_data);
    println!("{}", encrypted_hex);

    let decrypted_data = decrypt(&iv, &key, &encrypted_data).unwrap();
    let decrypted_text = String::from_utf8_lossy(&decrypted_data);
    println!("{}", decrypted_text);
}
