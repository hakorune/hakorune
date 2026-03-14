//! Object file emitter for LLVM mode
//!
//! Handles LLVM object file generation when requested (feature-gated).

#[cfg(feature = "llvm-harness")]
use crate::config::env;
use nyash_rust::mir::MirModule;
#[cfg(feature = "llvm-harness")]
use std::path::PathBuf;

/// Object emitter Box
///
/// **Responsibility**: Emit LLVM object file if requested
/// **Input**: &MirModule
/// **Output**: Result<bool, String> (Ok(true) if emitted, Ok(false) if not requested, Err on failure)
#[allow(dead_code)]
pub struct ObjectEmitterBox;

impl ObjectEmitterBox {
    /// Emit LLVM object file if requested
    ///
    /// Checks NYASH_LLVM_OBJ_OUT environment variable.
    /// If set, emits object file and verifies it's not empty.
    #[cfg(feature = "llvm-harness")]
    pub fn try_emit(module: &MirModule) -> Result<bool, String> {
        let Some(out_path) = env::env_string("NYASH_LLVM_OBJ_OUT") else {
            return Ok(false); // Not requested
        };

        if crate::config::env::llvm_use_harness() {
            emit_object_via_boundary_llvmlite_keep(module, &out_path)?;

            // Verify object file
            Self::verify_object(&out_path)?;
            return Ok(true);
        }

        // Verify object presence and size (>0)
        Self::verify_object(&out_path)?;
        Ok(true)
    }

    #[cfg(feature = "llvm-harness")]
    fn verify_object(path: &str) -> Result<(), String> {
        match std::fs::metadata(path) {
            Ok(meta) if meta.len() > 0 => {
                if env::cli_verbose_enabled() {
                    crate::console_println!(
                        "[LLVM] object emitted: {} ({} bytes)",
                        path,
                        meta.len()
                    );
                }
                Ok(())
            }
            Ok(_) => Err(format!("harness object is empty: {}", path)),
            Err(e) => Err(format!("harness output not found: {} ({})", path, e)),
        }
    }

    #[cfg(not(feature = "llvm-harness"))]
    #[allow(dead_code)]
    pub fn try_emit(_module: &MirModule) -> Result<bool, String> {
        Ok(false)
    }
}

#[cfg(feature = "llvm-harness")]
struct ScopedEnvOverride {
    key: &'static str,
    prev: Option<String>,
}

#[cfg(feature = "llvm-harness")]
impl ScopedEnvOverride {
    fn set(key: &'static str, value: &'static str) -> Self {
        let prev = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, prev }
    }
}

#[cfg(feature = "llvm-harness")]
impl Drop for ScopedEnvOverride {
    fn drop(&mut self) {
        match self.prev.take() {
            Some(value) => std::env::set_var(self.key, value),
            None => std::env::remove_var(self.key),
        }
    }
}

#[cfg(feature = "llvm-harness")]
fn emit_object_via_boundary_llvmlite_keep(
    module: &MirModule,
    out_path: &str,
) -> Result<(), String> {
    let _provider_keep = ScopedEnvOverride::set("HAKO_LLVM_EMIT_PROVIDER", "llvmlite");
    let mir_json = emit_module_mir_json_for_backend_boundary(module)?;
    let opts = crate::host_providers::llvm_codegen::Opts {
        out: Some(PathBuf::from(out_path)),
        nyrt: None,
        opt_level: crate::config::env::llvm_opt_level_env(),
        timeout_ms: Some(20_000),
    };
    crate::host_providers::llvm_codegen::mir_json_to_object(&mir_json, opts).map(|_| ())
}

#[cfg(feature = "llvm-harness")]
fn emit_module_mir_json_for_backend_boundary(module: &MirModule) -> Result<String, String> {
    let tmp_path = temporary_mir_json_path();
    crate::runner::mir_json_emit::emit_mir_json_for_harness(module, &tmp_path)?;
    let mir_json_result = std::fs::read_to_string(&tmp_path)
        .map_err(|error| format!("read boundary mir json: {}", error));
    let _ = std::fs::remove_file(&tmp_path);
    mir_json_result
}

#[cfg(feature = "llvm-harness")]
fn temporary_mir_json_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "llvm_object_emitter-{}-{}.json",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default()
    ))
}
