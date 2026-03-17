//! LLVM provider / AOT emission environment flags (SSOT)

use crate::config::env::env_bool;

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

/// Pure-first compile request for the current backend recipe.
pub fn backend_recipe_requests_pure_first() -> bool {
    matches!(backend_compile_recipe().as_deref(), Some("pure-first")) || env_bool("HAKO_CAPI_PURE")
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
