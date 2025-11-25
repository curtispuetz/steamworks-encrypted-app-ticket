use std::vec;

// tests/real_ticket.rs
use steamworks_encrypted_app_ticket::*;

// 64 hex chars, from Steamworks App Admin → "App Encryption Key"
// This is the publicly available one for SpaceWars.
const APP_KEY_HEX: &str = "ED9386073647CEA58B7721490D59ED445723F0F66E7414E1533BA33CD803BDBD";
const APP_ID: u32 = 480; // SpaceWars AppID

#[test]
fn test_full_decryption_and_validation() {
    let encrypted_ticket: Vec<u8> = vec![
        // paste a real encrypted ticket here (do not commit real ticket!).
        // (Use a ticket generated for SpaceWars with some user variable data set.)
    ];

    let mut key_bytes = [0u8; 32];
    hex::decode_to_slice(APP_KEY_HEX, &mut key_bytes).expect("Invalid hex key");

    // 2. Decrypt
    let (mut decrypted, len) =
        b_decrypt_ticket(&encrypted_ticket, &key_bytes).expect("Failed to decrypt real ticket");

    assert!(
        b_is_ticket_for_app(&mut decrypted, len, APP_ID),
        "Ticket is not for this app!"
    );

    assert!(
        b_user_owns_app_in_ticket(&mut decrypted, len, APP_ID),
        "User does not own the app!"
    );

    assert!(
        !b_user_is_vac_banned(&mut decrypted, len),
        "User is VAC banned!"
    );

    let steam_id = get_ticket_steam_id(&mut decrypted, len);
    assert!(steam_id > 76561197960265728, "Invalid SteamID");

    let issue_time = get_ticket_issue_time(&mut decrypted, len);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    assert!(
        issue_time <= now && issue_time > now - 86400,
        "Issue time seems wrong"
    );

    let user_data = get_user_variable_data(&mut decrypted, len);
    println!("User variable data: {:?}", user_data);
    if let Some(data) = user_data {
        assert!(!data.is_empty(), "User data should not be empty");
        match String::from_utf8(data) {
            Ok(text) => {
                println!("User variable data as text: {}", text);
            }
            Err(err) => {
                panic!("Failed to convert user variable data to text: {}", err);
            }
        }
    } else {
        panic!("Expected some user variable data");
    }

    println!("All tests passed! SteamID: {}", steam_id);
}
