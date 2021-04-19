use std::env;
use std::path::Path;
use std::process::Command;

#[cfg(any(target_os = "windows", feature = "no_build"))]
fn main() {}

#[cfg(all(not(target_os = "windows"), not(feature = "no_build")))]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dist");
    println!("{:?}", dest_path);

    let _ = match Command::new("yarn").arg("--cwd").arg("ui").spawn() {
        Ok(x) => x,
        Err(_) => {
            if cfg!(feature = "embed_ui") {
                panic!("Could not find `yarn`.");
            } else {
                println!("cargo:warnings=Could not find `yarn`.");
                return;
            }
        }
    }
    .wait();

    let mut build_log = match Command::new("yarn")
        .arg("--cwd")
        .arg("ui")
        .arg("build")
        .spawn()
    {
        Ok(x) => x,
        Err(_) => {
            if cfg!(feature = "embed_ui") {
                panic!("Could not find `yarn`.");
            } else {
                println!("cargo:warnings=Could not find `yarn`.");
                return;
            }
        }
    };

    if !build_log.wait().unwrap().success() {
        if cfg!(feature = "embed_ui") {
            panic!("Failed to build the UI.");
        } else {
            println!("cargo:warnings=Failed to build the UI.");
            return;
        }
    }

    println!("cargo:rerun-if-changed=ui/src");
    println!("cargo:rerun-if-changed=ui/node_modules");
    println!("cargo:rustc-cfg=feature=\"embed_ui\"");
}
