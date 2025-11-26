# Steamworks Encrypted App Ticket

This create provides Rust-friendly bindings for the [SteamEncryptedAppTicket part of the Steamworks SDK](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket).

The [steamworks crate](https://crates.io/crates/steamworks), while providing Rust-friendly bindings for most of the Steamworks SDK, to my knowledge does not provide them for SteamEncryptedAppTicket. But if the steamworks create does provide them, this crate is still a smaller dependency if all you need is the SteamEncryptedAppTicket part of the SDK.

# Examples

The integration test uses each function: [tests/real_ticket.rs](tests/real_ticket.rs)

# License

This crate is dual-licensed under Apache and MIT, except for the files in steamworks-encrypted-app-ticket-sys/vendor/

# Testing

As of now, its only been tested on my Windows computer. It will probably work on linux and mac as well since I added the binaries for them, but if it does not: submit a ticket on github.
