use super::*;

#[test]
fn hash_admin() {
    assert_eq!(
        Plaintext
            .hash_password_simple("admin".as_bytes(), "salt")
            .unwrap(),
        PasswordHash::new("$plain$salt$AAAAAABhZG1pbg").unwrap()
    );
}

#[test]
fn verify_admin() {
    PasswordHash::new("$plain$salt$AAAAAABhZG1pbg")
        .unwrap()
        .verify_password(&[&Plaintext], "admin".as_bytes())
        .unwrap();
}

#[test]
fn hash_test_with_pad() {
    assert_eq!(
        Plaintext
            .hash_password(
                "test".as_bytes(),
                None,
                PlaintextParams::new(b'*'),
                Salt::new("salt").unwrap()
            )
            .unwrap(),
        PasswordHash::new("$plain$pad=42$salt$KioqKioqdGVzdA").unwrap()
    );
}

#[test]
fn verify_test_with_pad() {
    PasswordHash::new("$plain$pad=42$salt$KioqKioqdGVzdA")
        .unwrap()
        .verify_password(&[&Plaintext], "test".as_bytes())
        .unwrap();
}
