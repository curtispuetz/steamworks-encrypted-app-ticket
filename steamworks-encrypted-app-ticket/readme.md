# Steamworks Encrypted App Ticket

This create provides Rust-friendly bindings for the [SteamEncryptedAppTicket part of the Steamworks SDK](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket).

The [steamworks crate](https://crates.io/crates/steamworks), while providing Rust-friendly bindings for most of the Steamworks SDK, to my knowledge does not provide them for SteamEncryptedAppTicket. Probably because the steamworks crate is more focused on the part of the SDK which makes API calls to the SteamClient, whereas SteamEncryptedAppTicket can be used completely offline.

# Examples

The integration test uses each function: [tests/real_ticket.rs](tests/real_ticket.rs)

# License

This crate is dual-licensed under Apache and MIT, except for the files in [steamworks-encrypted-app-ticket-sys/vendor/]