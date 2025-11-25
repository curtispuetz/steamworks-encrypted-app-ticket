use std::slice;
use steamworks_encrypted_app_ticket_sys::{
    SteamEncryptedAppTicket_BDecryptTicket as BDecryptTicket,
    SteamEncryptedAppTicket_BIsTicketForApp as BIsTicketForApp,
    SteamEncryptedAppTicket_BUserIsVacBanned as BUserIsVacBanned,
    SteamEncryptedAppTicket_BUserOwnsAppInTicket as BUserOwnsAppInTicket,
    SteamEncryptedAppTicket_GetTicketAppID as GetTicketAppID,
    SteamEncryptedAppTicket_GetTicketIssueTime as GetTicketIssueTime,
    SteamEncryptedAppTicket_GetTicketSteamID as GetTicketSteamID,
    SteamEncryptedAppTicket_GetUserVariableData as GetUserVariableData, SteamID_t,
};

const KEY_LEN: usize = 32;

pub fn b_decrypt_ticket(
    encrypted_ticket: &[u8],
    key: &[u8; KEY_LEN],
) -> Result<(Vec<u8>, u32), ()> {
    unsafe {
        let mut decrypted = vec![0u8; 1024]; // Max ticket size from SDK docs
        let mut decrypted_len: u32 = decrypted.len() as u32;

        if !BDecryptTicket(
            encrypted_ticket.as_ptr(),
            encrypted_ticket.len() as u32,
            decrypted.as_mut_ptr(),
            &mut decrypted_len,
            key.as_ptr(),
            KEY_LEN as i32,
        ) {
            return Err(());
        }
        decrypted.truncate(decrypted_len as usize);
        Ok((decrypted, decrypted_len))
    }
}

pub fn b_is_ticket_for_app(
    decrypted_ticket: &mut Vec<u8>,
    decrypted_len: u32,
    app_id: u32,
) -> bool {
    unsafe { BIsTicketForApp(decrypted_ticket.as_mut_ptr(), decrypted_len, app_id) }
}

pub fn get_ticket_issue_time(decrypted_ticket: &mut Vec<u8>, decrypted_len: u32) -> u32 {
    unsafe { GetTicketIssueTime(decrypted_ticket.as_mut_ptr(), decrypted_len) }
}

pub fn get_ticket_steam_id(decrypted_ticket: &mut Vec<u8>, decrypted_len: u32) -> u64 {
    let mut steam_id: SteamID_t = 0;
    unsafe {
        GetTicketSteamID(decrypted_ticket.as_mut_ptr(), decrypted_len, &mut steam_id);
        steam_id
    }
}

pub fn get_ticket_app_id(decrypted_ticket: &mut Vec<u8>, decrypted_len: u32) -> u32 {
    unsafe { GetTicketAppID(decrypted_ticket.as_mut_ptr(), decrypted_len) }
}

pub fn b_user_owns_app_in_ticket(
    decrypted_ticket: &mut Vec<u8>,
    decrypted_len: u32,
    app_id: u32,
) -> bool {
    unsafe { BUserOwnsAppInTicket(decrypted_ticket.as_mut_ptr(), decrypted_len, app_id) }
}

pub fn b_user_is_vac_banned(decrypted_ticket: &mut Vec<u8>, decrypted_len: u32) -> bool {
    unsafe { BUserIsVacBanned(decrypted_ticket.as_mut_ptr(), decrypted_len) }
}

pub fn get_user_variable_data(
    decrypted_ticket: &mut Vec<u8>,
    decrypted_len: u32,
) -> Option<Vec<u8>> {
    unsafe {
        let mut data_len: u32 = 0;
        let data_ptr =
            GetUserVariableData(decrypted_ticket.as_mut_ptr(), decrypted_len, &mut data_len);
        if data_len > 0 && !data_ptr.is_null() {
            return Some(slice::from_raw_parts(data_ptr, data_len as usize).to_vec());
        }
        None
    }
}
