use steamworks_encrypted_app_ticket::{b_decrypt_ticket, get_user_variable_data};

#[test]
fn test_decrypt_fails_on_garbage() {
    let garbage = vec![0xDE, 0xAD, 0xBE, 0xEF];
    let key = [0u8; 32];

    assert!(b_decrypt_ticket(&garbage, &key).is_err());
}

#[test]
fn test_user_variable_data_returns_none_when_empty() {
    // You can't easily craft a valid decrypted ticket without the key,
    // but we can test that the function doesn't panic on zero length
    let mut dummy = vec![0u8; 100];
    let result = get_user_variable_data(&mut dummy, 0);
    assert!(result.is_none());
}
