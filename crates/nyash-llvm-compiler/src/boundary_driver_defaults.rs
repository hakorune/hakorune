use std::path::PathBuf;

use anyhow::{anyhow, Result};

pub(super) const COMPILE_SYMBOL_DEFAULT: &[u8] = b"hako_llvmc_compile_json\0";
pub(super) const COMPILE_SYMBOL_PURE_FIRST: &[u8] = b"hako_llvmc_compile_json_pure_first\0";
const BOUNDARY_DEFAULT_COMPILE_RECIPE: &str = "pure-first";
const BOUNDARY_DEFAULT_COMPAT_REPLAY: &str = "none";

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

pub(super) fn boundary_compile_prefers_pure_first(
    recipe: Option<&str>,
    _legacy_capi_pure: Option<&str>,
) -> bool {
    match recipe {
        Some("pure-first") => true,
        Some(_) => false,
        None => true,
    }
}

pub(super) fn boundary_compile_symbol(
    recipe: Option<&str>,
    legacy_capi_pure: Option<&str>,
) -> &'static [u8] {
    if boundary_compile_prefers_pure_first(recipe, legacy_capi_pure) {
        COMPILE_SYMBOL_PURE_FIRST
    } else {
        COMPILE_SYMBOL_DEFAULT
    }
}

pub(super) fn boundary_codegen_request_defaults(
    recipe: Option<&str>,
    compat_replay: Option<&str>,
) -> (Option<String>, Option<String>) {
    (
        Some(
            recipe
                .unwrap_or(BOUNDARY_DEFAULT_COMPILE_RECIPE)
                .to_string(),
        ),
        Some(
            compat_replay
                .unwrap_or(BOUNDARY_DEFAULT_COMPAT_REPLAY)
                .to_string(),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        boundary_codegen_request_defaults, boundary_compile_prefers_pure_first, boundary_compile_symbol,
        ffi_library_default_candidates, ffi_library_filenames, COMPILE_SYMBOL_DEFAULT,
        COMPILE_SYMBOL_PURE_FIRST,
    };
    use std::path::PathBuf;

    #[test]
    fn boundary_route_defaults_to_pure_first_symbol() {
        assert!(boundary_compile_prefers_pure_first(None, None));
        assert_eq!(
            boundary_compile_symbol(None, None),
            COMPILE_SYMBOL_PURE_FIRST
        );
    }

    #[test]
    fn boundary_route_keeps_generic_symbol_for_harness_recipe() {
        assert!(!boundary_compile_prefers_pure_first(Some("harness"), None));
        assert_eq!(
            boundary_compile_symbol(Some("harness"), None),
            COMPILE_SYMBOL_DEFAULT
        );
    }

    #[test]
    fn boundary_codegen_defaults_fill_missing_recipe_and_replay() {
        let (recipe, compat_replay) = boundary_codegen_request_defaults(None, None);
        assert_eq!(recipe.as_deref(), Some("pure-first"));
        assert_eq!(compat_replay.as_deref(), Some("none"));
    }

    #[test]
    fn boundary_codegen_defaults_preserve_explicit_values() {
        let (recipe, compat_replay) =
            boundary_codegen_request_defaults(Some("harness"), Some("native"));
        assert_eq!(recipe.as_deref(), Some("harness"));
        assert_eq!(compat_replay.as_deref(), Some("native"));
    }

    #[test]
    fn boundary_route_prefers_pure_first_only_for_explicit_recipe_or_legacy_alias() {
        assert!(boundary_compile_prefers_pure_first(
            Some("pure-first"),
            None
        ));
        assert!(boundary_compile_prefers_pure_first(None, Some("1")));
        assert_eq!(
            boundary_compile_symbol(Some("pure-first"), None),
            COMPILE_SYMBOL_PURE_FIRST
        );
        assert_eq!(
            boundary_compile_symbol(None, Some("1")),
            COMPILE_SYMBOL_PURE_FIRST
        );
    }

    #[test]
    fn boundary_route_harness_recipe_overrides_legacy_capi_pure_alias() {
        assert!(!boundary_compile_prefers_pure_first(
            Some("harness"),
            Some("1")
        ));
        assert_eq!(
            boundary_compile_symbol(Some("harness"), Some("1")),
            COMPILE_SYMBOL_DEFAULT
        );
    }

    #[test]
    fn generic_compile_symbol_stays_keep_only() {
        assert_eq!(
            boundary_compile_symbol(None, None),
            COMPILE_SYMBOL_PURE_FIRST
        );
        assert_eq!(
            boundary_compile_symbol(Some("pure-first"), None),
            COMPILE_SYMBOL_PURE_FIRST
        );
        assert_eq!(
            boundary_compile_symbol(None, Some("1")),
            COMPILE_SYMBOL_PURE_FIRST
        );
        assert_eq!(
            boundary_compile_symbol(Some("harness"), None),
            COMPILE_SYMBOL_DEFAULT
        );
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
