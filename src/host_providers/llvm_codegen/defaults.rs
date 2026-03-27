use std::path::PathBuf;

use super::Opts;

pub(super) const COMPILE_SYMBOL_DEFAULT: &[u8] = b"hako_llvmc_compile_json\0";

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

pub fn boundary_default_object_opts(
    out: Option<PathBuf>,
    nyrt: Option<PathBuf>,
    opt_level: Option<String>,
    timeout_ms: Option<u64>,
) -> Opts {
    Opts {
        out,
        nyrt,
        opt_level,
        timeout_ms,
        compile_recipe: None,
        compat_replay: None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        boundary_default_object_opts, ffi_library_default_candidates, ffi_library_filenames,
    };
    use std::path::PathBuf;

    #[test]
    fn boundary_default_object_opts_stays_transport_only() {
        let opts = boundary_default_object_opts(None, None, None, None);
        assert_eq!(opts.compile_recipe, None);
        assert_eq!(opts.compat_replay, None);
    }

    #[test]
    fn ffi_library_default_candidates_cover_release_and_lib_dirs() {
        let candidates = ffi_library_default_candidates();
        for name in ffi_library_filenames() {
            assert!(candidates.contains(&PathBuf::from("target/release").join(name)));
            assert!(candidates.contains(&PathBuf::from("lib").join(name)));
        }
    }
}
