use std::{env, fs::File, path::PathBuf, process::Command};

fn main() { Nasm::build(); }

struct Nasm;

impl Nasm {
    fn build() {
        let archive = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("vendor/nasm-3.02.tar.xz");
        println!("cargo:rerun-if-changed={}", archive.display());
        for variable in ["CC", "CFLAGS", "DEVELOPER_DIR"] {
            println!("cargo:rerun-if-env-changed={variable}");
        }
        let output = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        let mut source = tar::Archive::new(lzma_rust2::XzReader::new(
            File::open(archive).unwrap(), false,
        ));
        source.set_preserve_mtime(true);
        source.unpack(&output).unwrap();
        let source = output.join("nasm-3.02");

        let mut configure = Command::new("sh");
        configure.arg("configure").arg("--without-zlib");
        if env::var("HOST").unwrap().ends_with("-apple-darwin") {
            configure.env("CC", "xcrun --sdk macosx clang");
            configure.env("CFLAGS", format!("-O2 --target={}", env::var("HOST").unwrap()));
        }

        let mut make = Command::new("make");
        make.args(["-j", &env::var("NUM_JOBS").unwrap(), "nasm"]);
        for command in [&mut configure, &mut make] {
            command.current_dir(&source);
            for variable in [
                "CPPFLAGS", "LDFLAGS", "LIBS", "AR", "RANLIB", "SDKROOT",
                "MACOSX_DEPLOYMENT_TARGET", "IPHONEOS_DEPLOYMENT_TARGET",
                "TVOS_DEPLOYMENT_TARGET", "WATCHOS_DEPLOYMENT_TARGET", "XROS_DEPLOYMENT_TARGET",
            ] {
                command.env_remove(variable);
            }
            assert!(command.status().unwrap().success(), "NASM build failed: {command:?}");
        }
        println!("cargo:bin={}", source.join("nasm").display());
    }
}
