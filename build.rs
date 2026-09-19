//! Build script.
//!
//! On every target except Emscripten this does nothing at all.
//!
//! For `wasm32-unknown-emscripten` it compiles `src/web/shim.c` - the handful of
//! platform functions miniquad and quad-snd import from JS (`init_webgl`,
//! `run_animation_loop`, `fs_load_file`, `audio_*`, ...) - and hands the object
//! file to the final link. See the long comment at the top of `src/web/shim.c`
//! for why those 23 symbols, and only those, need supplying.

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    // `pq_std_fs`: this target has a real, writable filesystem behind `std::fs`.
    //
    // Emscripten does (MEMFS), which is easy to miss because it is also
    // `target_arch = "wasm32"` - the plain wasm targets and iOS do not, and the
    // engine has separate, engine-mediated code paths for those. Having one cfg
    // for it keeps `Loading.rs` from repeating a three-clause predicate on every
    // single function.
    println!("cargo::rustc-check-cfg=cfg(pq_std_fs)");
    let no_std_fs = target_os == "ios" || (target_arch == "wasm32" && target_os != "emscripten");
    if !no_std_fs {
        println!("cargo:rustc-cfg=pq_std_fs");
    }

    if target_os != "emscripten" {
        return;
    }

    println!("cargo:rerun-if-changed=src/web/shim.c");
    println!("cargo:rerun-if-env-changed=EMCC");
    println!("cargo:rerun-if-env-changed=EMSDK");

    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let src = manifest_dir.join("src/web/shim.c");
    let obj = out_dir.join("pyroquad_shim.o");

    let emcc = find_emcc(&manifest_dir);

    let status = Command::new(&emcc)
        .arg("-c")
        .arg(&src)
        .arg("-o")
        .arg(&obj)
        // -fPIC is mandatory: the result is linked into an Emscripten SIDE_MODULE.
        .arg("-fPIC")
        .arg("-O2")
        .arg("-Wall")
        .status()
        .unwrap_or_else(|e| panic!("failed to run {}: {e}", emcc.display()));

    assert!(status.success(), "{} failed to compile {}", emcc.display(), src.display());

    println!("cargo:rustc-link-arg={}", obj.display());
}

/// `emcc` is looked for in the same places the project already expects it:
/// an explicit `EMCC`, then an activated `EMSDK`, then the project-local
/// `emsdk/` checkout that `.cargo/config.toml` points the linker at, then PATH.
fn find_emcc(manifest_dir: &Path) -> PathBuf {
    if let Ok(p) = std::env::var("EMCC") {
        return PathBuf::from(p);
    }

    let exe_names: &[&str] = if cfg!(windows) {
        &["emcc.exe", "emcc.bat"]
    } else {
        &["emcc"]
    };

    let mut roots = Vec::new();
    if let Ok(emsdk) = std::env::var("EMSDK") {
        roots.push(PathBuf::from(emsdk).join("upstream/emscripten"));
    }
    roots.push(manifest_dir.join("emsdk/upstream/emscripten"));

    for root in roots {
        for name in exe_names {
            let candidate = root.join(name);
            if candidate.is_file() {
                return candidate;
            }
        }
    }

    // Last resort: whatever an activated emsdk put on PATH.
    PathBuf::from("emcc")
}
