// NOTE: Adapted from cortex-m/build.rs
// NOTE: Further adapted from riscv-rt/build.rs
extern crate riscv_target;

use riscv_target::Target;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let name = env::var("CARGO_PKG_NAME").unwrap();

    if target.starts_with("riscv") {
        let mut target = Target::from_target_str(&target);
        target.retain_extensions("imc");

        let target = target.to_string();

        fs::copy(
            format!("bin/{}.a", target),
            out_dir.join(format!("lib{}.a", name)),
        )
        .unwrap();

        println!("cargo:rustc-link-lib=static={}", name);
        println!("cargo:rustc-link-search={}", out_dir.display());
    }

    if cfg!(feature = "direct-boot") {
        // Put the directboot linker script somewhere the linker can find it
        fs::File::create(out_dir.join("link_direct_boot.x"))
            .unwrap()
            .write_all(include_bytes!("link_direct_boot.x"))
            .unwrap();
    } else {
        // Put the baseline linker script somewhere the linker can find it
        fs::File::create(out_dir.join("link.x"))
            .unwrap()
            .write_all(include_bytes!("link.x"))
            .unwrap();
    };

    if cfg!(feature = "esp32c3") {
        // Put the esp32-c3 memory layout linker script somewhere the linker can find it
        fs::File::create(out_dir.join("memory.x"))
            .unwrap()
            .write_all(include_bytes!("esp32_c3_memory.x"))
            .unwrap();
    }

    println!("cargo:rustc-link-search={}", out_dir.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link.x");
    println!("cargo:rerun-if-changed=link_direct_boot.x");
    println!("cargo:rerun-if-changed=esp32_c3_memory.x");
}
