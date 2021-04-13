use super::*;

#[test]
fn symetric_encrypt_decrypt_with_zeros() {
    let key = [0; 16];
    let iv = [0; 16];
    let plaintext = [0; 24];

    let ciphertext = symetric_encrypt(&key, &iv, &plaintext);
    let decrypted_ciphertext = symetric_decrypt(&key, &iv, &ciphertext).unwrap();

    assert_eq!(plaintext, decrypted_ciphertext.as_slice());
}

#[test]
fn symetric_encrypt_decrypt_with_random() {
    let mut key = [0; 16];
    fill_rand_array(&mut key);
    let mut iv = [0; 16];
    fill_rand_array(&mut iv);
    let plaintext = "Hello World, 42!";

    let ciphertext = symetric_encrypt(&key, &iv, plaintext.as_bytes());
    let decrypted_cipher = symetric_decrypt(&key, &iv, &ciphertext).unwrap();
    let decrypted_ciphertext = std::str::from_utf8(decrypted_cipher.as_slice()).unwrap();

    assert_eq!(plaintext, decrypted_ciphertext);
}

#[test]
fn symetric_encrypt_mutated_key_decrypt() {
    let key = [0; 16];
    let iv = [0; 16];
    let plaintext = [0; 24];

    let ciphertext = symetric_encrypt(&key, &iv, &plaintext);
    let mutated_key = [1; 16];
    let decrypted_ciphertext = symetric_decrypt(&mutated_key, &iv, &ciphertext);

    assert_matches!(decrypted_ciphertext, Err(_));
}

#[test]
fn symetric_encrypt_mutated_iv_decrypt() {
    let key = [0; 16];
    let iv = [0; 16];
    let plaintext = [0; 24];

    let ciphertext = symetric_encrypt(&key, &iv, &plaintext);
    let mutated_iv = [1; 16];
    let decrypted_ciphertext = symetric_decrypt(&key, &mutated_iv, &ciphertext);

    assert_matches!(decrypted_ciphertext, Err(_));
}

#[test]
fn symetric_encrypt_mutated_ciphertext_decrypt() {
    let key = [0; 16];
    let iv = [0; 16];
    let plaintext = [0; 24];

    let _ciphertext = symetric_encrypt(&key, &iv, &plaintext);
    let mutated_ciphertext = [1; 24];
    let decrypted_ciphertext = symetric_decrypt(&key, &iv, &mutated_ciphertext);

    assert_matches!(decrypted_ciphertext, Err(_));
}
