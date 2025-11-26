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
        8, 2, 16, 232, 209, 228, 137, 14, 24, 17, 32, 95, 42, 144, 1, 224, 130, 41, 204, 195, 129,
        240, 251, 11, 10, 73, 88, 230, 36, 139, 225, 184, 166, 41, 135, 157, 136, 78, 37, 203, 238,
        24, 102, 56, 15, 48, 97, 137, 153, 131, 186, 51, 120, 3, 166, 237, 128, 57, 53, 252, 188,
        21, 84, 13, 96, 2, 72, 111, 150, 243, 127, 88, 10, 10, 106, 189, 206, 27, 141, 38, 69, 45,
        115, 27, 206, 25, 114, 255, 249, 122, 90, 201, 54, 34, 27, 168, 81, 253, 119, 239, 224,
        158, 180, 174, 66, 144, 220, 156, 236, 59, 42, 18, 33, 134, 100, 153, 163, 170, 188, 77,
        37, 71, 106, 4, 162, 103, 191, 160, 44, 55, 12, 148, 131, 111, 224, 111, 78, 219, 83, 81,
        185, 84, 178, 120, 248, 129, 169, 132, 173, 49, 48, 140, 77, 100, 40, 232, 190, 219, 124,
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
