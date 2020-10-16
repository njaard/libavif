use std::env;
#[cfg(feature = "codec-rav1e")]
use std::fs;
use std::path::{Path, PathBuf};

use cmake::Config;
use std::ffi::OsString;

fn main() {
    if env::var_os("DOCS_RS").is_some() {
        return;
    }

    let out_dir_ = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_);
    let mut _built_products_paths: Vec<PathBuf> = vec![];
    let mut avif = Config::new("libavif");

    let mut pc_paths: Vec<_> = env::var("PKG_CONFIG_PATH")
        .map(|v| env::split_paths(&v).collect())
        .unwrap_or_default();

    avif.define("BUILD_SHARED_LIBS", "0");
    // Required for clang 12 on macOS, and likely all future compilers libavif hasn't been tweaked for yet
    avif.define("AVIF_ENABLE_WERROR", "0");

    if env::var_os("CI").is_some() {
        avif.very_verbose(true);
    }

    #[cfg(feature = "codec-aom")]
    {
        let include =
            env::var_os("DEP_AOM_INCLUDE").expect("libaom-sys should have set pkgconfig path");
        avif.define("AVIF_CODEC_AOM", "1");
        avif.define("AOM_INCLUDE_DIR", include);

        if let Some(pc_path) = env::var_os("DEP_AOM_PKGCONFIG") {
            let p = PathBuf::from(pc_path);

            _built_products_paths.push(p.parent().unwrap().to_path_buf());
            pc_paths.insert(0, p);
        }
    }

    #[cfg(feature = "codec-rav1e")]
    {
        let crate_dir_ = env::var("CARGO_MANIFEST_DIR").unwrap();
        let crate_dir = Path::new(&crate_dir_);
        fs::create_dir_all(out_dir.join("include").join("rav1e")).expect("mkdir");
        fs::copy(
            crate_dir.join("rav1e.h"),
            out_dir.join("include").join("rav1e").join("rav1e.h"),
        )
        .expect("copy rav1e.h");

        avif.define("AVIF_CODEC_RAV1E", "1")
            .define("AVIF_CODEC_LIBRARIES", "rav1e")
            .define("RAV1E_LIBRARY", "-rav1e");
    }

    #[cfg(feature = "codec-dav1d")]
    {
        let include =
            env::var_os("DEP_DAV1D_INCLUDE").expect("libdav1d-sys should have set pkgconfig path");
        avif.define("AVIF_CODEC_DAV1D", "1");
        avif.define("DAV1D_INCLUDE_DIR", include);

        if let Some(pc_path) = env::var_os("DEP_DAV1D_PKGCONFIG") {
            pc_paths.insert(0, pc_path.into());
        }

        if let Some(staticlib) = env::var_os("DEP_DAV1D_STATICLIB") {
            avif.define("DAV1D_LIBRARY", staticlib);
        }
    }

    eprintln!("building libavif");

    let local_pc_files = env::join_paths(pc_paths).unwrap();
    let mut cmake_prefix_path = OsString::new();
    for s in _built_products_paths {
        if !cmake_prefix_path.is_empty() {
            cmake_prefix_path.push(";");
        }
        cmake_prefix_path.push(s);
    }

    eprintln!(
        "pc=\"{:?}\"; bp=\"{:?}\"",
        local_pc_files, cmake_prefix_path
    );
    avif.define("CMAKE_PREFIX_PATH", cmake_prefix_path);
    avif.env("PKG_CONFIG_PATH", local_pc_files);

    let avif_built = avif.profile("Release").build();

    println!(
        "cargo:rustc-link-search=native={}",
        avif_built.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=avif");
    println!("cargo:outdir={}", out_dir.display());
}
