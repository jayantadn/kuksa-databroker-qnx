// QNX build script for errno compatibility
use std::env;
use std::path::PathBuf;

fn main() {
    // Only compile the errno shim for QNX targets
    let target = env::var("TARGET").unwrap();
    if target.contains("nto-qnx") {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        
        // Get the QNX compiler from environment
        let qcc = env::var("CC").unwrap_or_else(|_| "qcc".to_string());
        
        // Compile the errno compatibility shim
        let status = std::process::Command::new(&qcc)
            .args(&[
                "-Vgcc_ntox86_64_cxx",
                "-c", 
                "../qnx_errno_compat.c",
                "-o"
            ])
            .arg(out_dir.join("qnx_errno_compat.o"))
            .status()
            .expect("Failed to compile QNX errno compatibility shim");
            
        if !status.success() {
            panic!("Failed to compile QNX errno compatibility object");
        }
        
        // Create a static library from the object file
        let ar = env::var("AR").unwrap_or_else(|_| "ar".to_string());
        let lib_path = out_dir.join("libqnx_errno_compat.a");
        
        let status = std::process::Command::new(&ar)
            .args(&["rcs"])
            .arg(&lib_path)
            .arg(out_dir.join("qnx_errno_compat.o"))
            .status()
            .expect("Failed to create static library");
            
        if !status.success() {
            panic!("Failed to create QNX errno compatibility library");
        }
        
        // Tell cargo to link against our compatibility library
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=static=qnx_errno_compat");
        
        // Re-run if the C file changes
        println!("cargo:rerun-if-changed=../qnx_errno_compat.c");
    }
}
