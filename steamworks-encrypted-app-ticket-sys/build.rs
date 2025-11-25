use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Tell Cargo to link the library
    let lib_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("vendor")
        .join("lib");
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=sdkencryptedappticket64");
        println!(
            "cargo:rustc-link-search=native={}",
            lib_dir.join("win64").display()
        );

        // Copy the DLLs to the workspace's target/debug directory (for tests)
        let dll_dir =
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("vendor/lib/win64");
        println!("Resolved DLL directory: {}", dll_dir.display()); // Debug print
        let workspace_target_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .expect("Failed to find workspace root")
            .join("target/debug");

        if !workspace_target_dir.exists() {
            fs::create_dir_all(&workspace_target_dir)
                .expect("Failed to create target/debug directory");
        }

        for entry in fs::read_dir(&dll_dir).expect("Failed to read DLL directory") {
            let entry = entry.expect("Failed to read entry in DLL directory");
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("dll") {
                let file_name = path.file_name().expect("Failed to get file name");
                let target_path = workspace_target_dir.join(file_name);
                println!(
                    "Copying from {} to {}",
                    path.display(),
                    target_path.display()
                ); // Debug print
                fs::copy(&path, &target_path).unwrap_or_else(|err| {
                    panic!("Failed to copy DLL to target directory: {}", err);
                });
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=libsdkencryptedappticket");
        println!(
            "cargo:rustc-link-search=native={}",
            lib_dir.join("linux64").display()
        );
    }
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=libsdkencryptedappticket");
        println!(
            "cargo:rustc-link-search=native={}",
            lib_dir.join("osx").display()
        );
    }

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

    println!("cargo:rerun-if-changed=vendor/lib/win64");
}
