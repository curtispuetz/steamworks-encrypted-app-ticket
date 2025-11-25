use std::env;
use std::path::PathBuf;

fn main() {
    // Tell Cargo to link the library
    println!("cargo:rustc-link-lib=sdkencryptedappticket");

    let lib_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("vendor")
        .join("lib");

    #[cfg(target_os = "windows")]
    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.join("win64").display()
    );
    #[cfg(target_os = "linux")]
    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.join("linux64").display()
    );
    #[cfg(target_os = "macos")]
    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.join("osx").display()
    );

    // Generate bindings from our clean header
    let bindings = bindgen::Builder::default()
        .header("wrapper/steamencryptedappticket_fixed.h")
        .allowlist_function("SteamEncryptedAppTicket_.*")
        .allowlist_type("SteamID_t")
        .allowlist_type("AppId_t")
        .allowlist_type("RTime32")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
