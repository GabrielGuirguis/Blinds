use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let swift_file = "swift/ScreenCapture.swift";
        let lib_name = "screencapture_swift";

        println!("cargo:rerun-if-changed={}", swift_file);

        // 1. Compile Swift to a static library
        // We force the deployment target to 13.0+ to ensure it uses the SYSTEM swift libraries
        // instead of trying to bundle them (which caused your error).
        let status = Command::new("swiftc")
            .args(&[
                "-emit-library",
                "-static",
                "-o", out_dir.join(format!("lib{}.a", lib_name)).to_str().unwrap(),
                "-target", "arm64-apple-macosx13.0", // Adjust to x86_64 if on Intel
                swift_file
            ])
            .status()
            .expect("Failed to run swiftc");

        if !status.success() {
            panic!("Swift compilation failed");
        }

        // 2. Link the library we just built
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=static={}", lib_name);

        // 3. Link System Frameworks
        println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=Foundation");
        
        // 4. THE FIX: Explicitly link the Swift Runtime from the system
        // This prevents the linker from looking for @rpath local copies
        println!("cargo:rustc-link-search=native=/usr/lib/swift");
    }
    tauri_build::build()
}
