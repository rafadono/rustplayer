use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=RPLAYER_MPV_LIB_DIR");

    let env_dir = env::var("RPLAYER_MPV_LIB_DIR").ok().map(PathBuf::from);
    let fallback_dir = env::var("CARGO_MANIFEST_DIR")
        .ok()
        .map(PathBuf::from)
        .map(|p| p.join("vendor").join("mpv"));

    let selected = env_dir.or(fallback_dir);
    if let Some(dir) = selected {
        if dir.exists() {
            println!("cargo:rustc-link-search=native={}", dir.display());
            println!("cargo:rerun-if-changed={}", dir.join("mpv.lib").display());
            println!(
                "cargo:rerun-if-changed={}",
                dir.join("libmpv-2.dll").display()
            );

            let target = env::var("TARGET").unwrap_or_default();
            if target.contains("windows") {
                let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".into());
                let target_dir = env::var("CARGO_TARGET_DIR")
                    .map(PathBuf::from)
                    .unwrap_or_else(|_| {
                        env::var("CARGO_MANIFEST_DIR")
                            .map(PathBuf::from)
                            .unwrap()
                            .join("target")
                    })
                    .join(&profile);

                let dll = dir.join("libmpv-2.dll");
                if dll.exists() {
                    let dest = target_dir.join("libmpv-2.dll");
                    if let Err(err) = fs::copy(&dll, &dest) {
                        println!(
                            "cargo:warning=Failed to copy {} to {}: {}",
                            dll.display(),
                            dest.display(),
                            err
                        );
                    } else {
                        println!(
                            "cargo:warning=Copied {} to {}",
                            dll.display(),
                            dest.display()
                        );
                    }
                }
            }
        }
    }
}
