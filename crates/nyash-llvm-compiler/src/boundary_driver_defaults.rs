use std::path::PathBuf;

use anyhow::{anyhow, Result};

fn ffi_library_filenames() -> &'static [&'static str] {
    if cfg!(target_os = "windows") {
        &["hako_llvmc_ffi.dll", "libhako_llvmc_ffi.dll"]
    } else if cfg!(target_os = "macos") {
        &[
            "libhako_llvmc_ffi.dylib",
            "hako_llvmc_ffi.dylib",
            "libhako_llvmc_ffi.so",
        ]
    } else {
        &[
            "libhako_llvmc_ffi.so",
            "hako_llvmc_ffi.so",
            "libhako_llvmc_ffi.dylib",
        ]
    }
}

pub(super) fn ffi_library_default_candidates() -> Vec<PathBuf> {
    let mut out = Vec::new();
    for name in ffi_library_filenames() {
        out.push(PathBuf::from("target/release").join(name));
        out.push(PathBuf::from("lib").join(name));
    }
    out
}

pub(super) fn resolve_ffi_library() -> Result<PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(path) = std::env::var("HAKO_AOT_FFI_LIB") {
        let path = path.trim();
        if !path.is_empty() {
            candidates.push(PathBuf::from(path));
        }
    }
    candidates.extend(ffi_library_default_candidates());
    candidates
        .into_iter()
        .find(|path| path.exists())
        .ok_or_else(|| {
            anyhow!("FFI library not found (set HAKO_AOT_FFI_LIB or build libhako_llvmc_ffi)")
        })
}

#[cfg(test)]
mod tests {
    use super::{ffi_library_default_candidates, ffi_library_filenames};
    use std::path::PathBuf;

    #[test]
    fn ffi_library_default_candidates_cover_release_and_lib_dirs() {
        let candidates = ffi_library_default_candidates();
        for name in ffi_library_filenames() {
            assert!(candidates.contains(&PathBuf::from("target/release").join(name)));
            assert!(candidates.contains(&PathBuf::from("lib").join(name)));
        }
    }
}
