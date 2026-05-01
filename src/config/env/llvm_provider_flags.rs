//! LLVM provider / AOT emission environment flags (SSOT)

use super::{env_bool, warn_alias_once};

/// Path to ny-llvmc compiler (NYASH_NY_LLVM_COMPILER).
pub fn ny_llvm_compiler_path() -> Option<String> {
    std::env::var("NYASH_NY_LLVM_COMPILER")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Enable LLVM C-API provider (NYASH_LLVM_USE_CAPI=1).
pub fn llvm_use_capi() -> bool {
    env_bool("NYASH_LLVM_USE_CAPI")
}

/// Enable external provider C-ABI (HAKO_V1_EXTERN_PROVIDER_C_ABI=1).
pub fn extern_provider_c_abi() -> bool {
    env_bool("HAKO_V1_EXTERN_PROVIDER_C_ABI")
}

/// Provider selector for MIR→obj compat keeps (HAKO_LLVM_EMIT_PROVIDER=llvmlite|ny-llvmc).
pub fn llvm_emit_provider() -> Option<String> {
    std::env::var("HAKO_LLVM_EMIT_PROVIDER")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// AOT FFI library path (HAKO_AOT_FFI_LIB).
pub fn aot_ffi_lib_path() -> Option<String> {
    std::env::var("HAKO_AOT_FFI_LIB")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// AOT FFI enable env snapshot (HAKO_AOT_USE_FFI).
pub fn aot_use_ffi_env() -> Option<String> {
    std::env::var("HAKO_AOT_USE_FFI").ok()
}

/// Backend compile recipe selector (HAKO_BACKEND_COMPILE_RECIPE).
pub fn backend_compile_recipe() -> Option<String> {
    std::env::var("HAKO_BACKEND_COMPILE_RECIPE")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Backend compat replay selector (HAKO_BACKEND_COMPAT_REPLAY).
pub fn backend_compat_replay() -> Option<String> {
    std::env::var("HAKO_BACKEND_COMPAT_REPLAY")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Backend acceptance-case bridge selector (HAKO_BACKEND_ACCEPTANCE_CASE).
pub fn backend_acceptance_case() -> Option<String> {
    std::env::var("HAKO_BACKEND_ACCEPTANCE_CASE")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Backend transport-owner bridge selector (HAKO_BACKEND_TRANSPORT_OWNER).
pub fn backend_transport_owner() -> Option<String> {
    std::env::var("HAKO_BACKEND_TRANSPORT_OWNER")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Backend legacy-daily allowance bridge selector (HAKO_BACKEND_LEGACY_DAILY_ALLOWED).
pub fn backend_legacy_daily_allowed() -> Option<String> {
    std::env::var("HAKO_BACKEND_LEGACY_DAILY_ALLOWED")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Canonical caller-side backend codegen defaults.
///
/// Daily callers may pass explicit values. Compat callers may leave either
/// field empty, in which case the current env override is applied here.
pub fn backend_codegen_request_defaults(
    compile_recipe: Option<String>,
    compat_replay: Option<String>,
) -> (Option<String>, Option<String>) {
    (
        compile_recipe.or_else(backend_compile_recipe),
        compat_replay.or_else(backend_compat_replay),
    )
}

/// Pure-first compile request for the current backend recipe.
/// Recipe-aware callers may bind an explicit pure-first FFI export from this.
pub fn backend_recipe_requests_pure_first() -> bool {
    let legacy_capi_pure = env_bool("HAKO_CAPI_PURE");
    if legacy_capi_pure {
        warn_alias_once("HAKO_CAPI_PURE", "HAKO_BACKEND_COMPILE_RECIPE=pure-first");
    }
    matches!(backend_compile_recipe().as_deref(), Some("pure-first"))
}

/// AOT ldflags override (HAKO_AOT_LDFLAGS).
pub fn aot_ldflags() -> Option<String> {
    std::env::var("HAKO_AOT_LDFLAGS")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// C-ABI trace (HAKO_CABI_TRACE=1).
pub fn cabi_trace() -> bool {
    env_bool("HAKO_CABI_TRACE")
}

#[cfg(test)]
mod tests {
    use super::backend_codegen_request_defaults;
    use std::ffi::OsString;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .expect("env lock")
    }

    struct EnvRestore {
        compile_recipe: Option<OsString>,
        compat_replay: Option<OsString>,
        acceptance_case: Option<OsString>,
        transport_owner: Option<OsString>,
        legacy_daily_allowed: Option<OsString>,
        legacy_capi_pure: Option<OsString>,
    }

    impl Drop for EnvRestore {
        fn drop(&mut self) {
            match self.compile_recipe.take() {
                Some(v) => std::env::set_var("HAKO_BACKEND_COMPILE_RECIPE", v),
                None => std::env::remove_var("HAKO_BACKEND_COMPILE_RECIPE"),
            }
            match self.compat_replay.take() {
                Some(v) => std::env::set_var("HAKO_BACKEND_COMPAT_REPLAY", v),
                None => std::env::remove_var("HAKO_BACKEND_COMPAT_REPLAY"),
            }
            match self.acceptance_case.take() {
                Some(v) => std::env::set_var("HAKO_BACKEND_ACCEPTANCE_CASE", v),
                None => std::env::remove_var("HAKO_BACKEND_ACCEPTANCE_CASE"),
            }
            match self.transport_owner.take() {
                Some(v) => std::env::set_var("HAKO_BACKEND_TRANSPORT_OWNER", v),
                None => std::env::remove_var("HAKO_BACKEND_TRANSPORT_OWNER"),
            }
            match self.legacy_daily_allowed.take() {
                Some(v) => std::env::set_var("HAKO_BACKEND_LEGACY_DAILY_ALLOWED", v),
                None => std::env::remove_var("HAKO_BACKEND_LEGACY_DAILY_ALLOWED"),
            }
            match self.legacy_capi_pure.take() {
                Some(v) => std::env::set_var("HAKO_CAPI_PURE", v),
                None => std::env::remove_var("HAKO_CAPI_PURE"),
            }
        }
    }

    #[test]
    fn backend_codegen_request_defaults_preserves_explicit_values() {
        let _guard = env_lock();
        let (compile_recipe, compat_replay) = backend_codegen_request_defaults(
            Some("pure-first".to_string()),
            Some("harness".to_string()),
        );
        assert_eq!(compile_recipe.as_deref(), Some("pure-first"));
        assert_eq!(compat_replay.as_deref(), Some("harness"));
    }

    #[test]
    fn backend_codegen_request_defaults_fills_missing_from_env() {
        let _guard = env_lock();
        let _restore = EnvRestore {
            compile_recipe: std::env::var_os("HAKO_BACKEND_COMPILE_RECIPE"),
            compat_replay: std::env::var_os("HAKO_BACKEND_COMPAT_REPLAY"),
            acceptance_case: std::env::var_os("HAKO_BACKEND_ACCEPTANCE_CASE"),
            transport_owner: std::env::var_os("HAKO_BACKEND_TRANSPORT_OWNER"),
            legacy_daily_allowed: std::env::var_os("HAKO_BACKEND_LEGACY_DAILY_ALLOWED"),
            legacy_capi_pure: std::env::var_os("HAKO_CAPI_PURE"),
        };
        std::env::set_var("HAKO_BACKEND_COMPILE_RECIPE", "pure-first");
        std::env::set_var("HAKO_BACKEND_COMPAT_REPLAY", "harness");

        let (compile_recipe, compat_replay) = backend_codegen_request_defaults(None, None);
        assert_eq!(compile_recipe.as_deref(), Some("pure-first"));
        assert_eq!(compat_replay.as_deref(), Some("harness"));
    }

    #[test]
    fn backend_owner_bridge_fields_ignore_empty_values() {
        let _guard = env_lock();
        let _restore = EnvRestore {
            compile_recipe: std::env::var_os("HAKO_BACKEND_COMPILE_RECIPE"),
            compat_replay: std::env::var_os("HAKO_BACKEND_COMPAT_REPLAY"),
            acceptance_case: std::env::var_os("HAKO_BACKEND_ACCEPTANCE_CASE"),
            transport_owner: std::env::var_os("HAKO_BACKEND_TRANSPORT_OWNER"),
            legacy_daily_allowed: std::env::var_os("HAKO_BACKEND_LEGACY_DAILY_ALLOWED"),
            legacy_capi_pure: std::env::var_os("HAKO_CAPI_PURE"),
        };

        std::env::set_var(
            "HAKO_BACKEND_ACCEPTANCE_CASE",
            "hello-simple-llvm-native-probe-v1",
        );
        std::env::set_var("HAKO_BACKEND_TRANSPORT_OWNER", "hako_ll_emitter");
        std::env::set_var("HAKO_BACKEND_LEGACY_DAILY_ALLOWED", "no");
        assert_eq!(
            super::backend_acceptance_case().as_deref(),
            Some("hello-simple-llvm-native-probe-v1")
        );
        assert_eq!(
            super::backend_transport_owner().as_deref(),
            Some("hako_ll_emitter")
        );
        assert_eq!(super::backend_legacy_daily_allowed().as_deref(), Some("no"));

        std::env::set_var("HAKO_BACKEND_ACCEPTANCE_CASE", "");
        std::env::set_var("HAKO_BACKEND_TRANSPORT_OWNER", " ");
        std::env::set_var("HAKO_BACKEND_LEGACY_DAILY_ALLOWED", "");
        assert_eq!(super::backend_acceptance_case(), None);
        assert_eq!(super::backend_transport_owner(), None);
        assert_eq!(super::backend_legacy_daily_allowed(), None);
    }

    #[test]
    fn backend_recipe_requests_pure_first_ignores_legacy_alias() {
        let _guard = env_lock();
        let _restore = EnvRestore {
            compile_recipe: std::env::var_os("HAKO_BACKEND_COMPILE_RECIPE"),
            compat_replay: std::env::var_os("HAKO_BACKEND_COMPAT_REPLAY"),
            acceptance_case: std::env::var_os("HAKO_BACKEND_ACCEPTANCE_CASE"),
            transport_owner: std::env::var_os("HAKO_BACKEND_TRANSPORT_OWNER"),
            legacy_daily_allowed: std::env::var_os("HAKO_BACKEND_LEGACY_DAILY_ALLOWED"),
            legacy_capi_pure: std::env::var_os("HAKO_CAPI_PURE"),
        };
        std::env::remove_var("HAKO_BACKEND_COMPILE_RECIPE");
        std::env::set_var("HAKO_CAPI_PURE", "1");

        assert!(!super::backend_recipe_requests_pure_first());
    }
}
