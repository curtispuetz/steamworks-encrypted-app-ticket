use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let lib_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("vendor")
        .join("lib");

    configure_linking(&lib_dir);

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

    println!("cargo:rerun-if-changed=vendor/lib");
}

fn configure_linking(lib_dir: &Path) {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    match target_os.as_str() {
        "windows" => link_windows(lib_dir, &target_arch),
        "linux" => link_unix(lib_dir, "linux", &target_arch),
        "macos" => link_unix(lib_dir, "osx", &target_arch),
        other => panic!("Unsupported target OS: {}", other),
    }
}

fn link_windows(lib_dir: &Path, target_arch: &str) {
    let (subdir, link_name, dll_name) = match target_arch {
        "x86_64" => (
            "win64",
            "sdkencryptedappticket64",
            "sdkencryptedappticket64.dll",
        ),
        "x86" | "i686" => (
            "win32",
            "sdkencryptedappticket",
            "sdkencryptedappticket.dll",
        ),
        other => panic!("Unsupported Windows arch: {}", other),
    };

    let platform_dir = lib_dir.join(subdir);
    println!("cargo:rustc-link-lib={}", link_name);
    println!("cargo:rustc-link-search=native={}", platform_dir.display());

    let dll_src = platform_dir.join(dll_name);
    copy_native_library(&dll_src, dll_name);
    println!("cargo:rerun-if-changed={}", dll_src.display());
}

fn link_unix(lib_dir: &Path, subdir: &str, target_arch: &str) {
    // Prefer 64-bit libs when available, fall back otherwise
    let platform_dir = match target_arch {
        "x86_64" => lib_dir.join(format!("{}64", subdir)),
        "aarch64" if subdir == "linux" => lib_dir.join("linuxarm64"),
        "x86" | "i686" => lib_dir.join(format!("{}32", subdir)),
        other => panic!("Unsupported {} arch: {}", subdir, other),
    };

    println!("cargo:rustc-link-lib=sdkencryptedappticket");
    println!("cargo:rustc-link-search=native={}", platform_dir.display());
    println!("cargo:rerun-if-changed={}", platform_dir.display());
}

fn copy_native_library(dll_src: &Path, dll_name: &str) {
    if !dll_src.exists() {
        panic!(
            "Expected Steamworks Encrypted App Ticket DLL to exist at {}",
            dll_src.display()
        );
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let profile_dir = locate_profile_dir(&out_dir)
        .unwrap_or_else(|| panic!("Failed to locate Cargo profile directory from OUT_DIR"));

    let mut last_dest = None;
    for target_dir in [&profile_dir, &profile_dir.join("deps"), &out_dir] {
        if let Err(err) = fs::create_dir_all(target_dir) {
            panic!("Failed to create {}: {}", target_dir.display(), err);
        }
        let dest = target_dir.join(dll_name);
        if let Err(err) = fs::copy(dll_src, &dest) {
            panic!(
                "Failed to copy {} to {}: {}",
                dll_src.display(),
                dest.display(),
                err
            );
        }
        last_dest = Some(dest);
    }

    if let Some(dest) = last_dest {
        println!(
            "cargo:rustc-env=STEAMWORKS_ENCRYPTED_APP_TICKET_SYS_DLL_PATH={}",
            dest.display()
        );
        if let Some(dir) = dest.parent() {
            println!(
                "cargo:rustc-env=STEAMWORKS_ENCRYPTED_APP_TICKET_SYS_DLL_DIR={}",
                dir.display()
            );
        }
    }
}

fn locate_profile_dir(out_dir: &Path) -> Option<PathBuf> {
    let mut current = out_dir;
    while let Some(parent) = current.parent() {
        if parent
            .file_name()
            .map(|name| name == "build")
            .unwrap_or(false)
        {
            return parent.parent().map(Path::to_path_buf);
        }
        current = parent;
    }
    None
}
