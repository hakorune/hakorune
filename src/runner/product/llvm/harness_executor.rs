//! LLVM compatibility-harness executor (native executable generation).
//!
//! The selected Dynamic Boundary owner lives in `boundary_executor`; this
//! module is the explicit compatibility lane and keeps its historical
//! feature-gated behavior until the feature recut is complete.

#[cfg(feature = "llvmlite-compat")]
use super::boundary_executor::run_emitted_executable;
use super::error::LlvmRunError;
use crate::config::env;
use crate::runtime::get_global_ring0;
use nyash_rust::mir::MirModule;

/// Compatibility harness executor.
pub struct HarnessExecutorBox;

impl HarnessExecutorBox {
    /// Execute via the explicit LLVM compatibility harness.
    #[cfg(feature = "llvmlite-compat")]
    pub fn try_execute(
        module: &MirModule,
        policy: &super::LlvmHarnessInvocationPolicyV1,
    ) -> Result<i32, LlvmRunError> {
        log_harness_runtime_state(policy);
        ensure_harness_requested(policy)?;
        let exe_out = "tmp/nyash_llvm_run";
        emit_executable_via_ny_llvmc(module, exe_out, policy)?;
        run_emitted_executable(exe_out)
    }

    #[cfg(not(feature = "llvmlite-compat"))]
    pub fn try_execute(
        _module: &MirModule,
        _policy: &super::LlvmHarnessInvocationPolicyV1,
    ) -> Result<i32, LlvmRunError> {
        if env::cli_verbose_enabled() {
            get_global_ring0()
                .log
                .warn("[llvm/harness] feature not enabled at compile time");
            get_global_ring0()
                .log
                .warn("[llvm/harness] rebuild with: cargo build --release --features llvm");
        }
        Err(LlvmRunError::fatal(
            "LLVM harness feature not enabled (built without --features llvm)",
        ))
    }
}

#[cfg(feature = "llvmlite-compat")]
fn log_harness_runtime_state(policy: &super::LlvmHarnessInvocationPolicyV1) {
    if env::cli_verbose_enabled() {
        get_global_ring0()
            .log
            .debug("[llvm/harness] feature enabled at compile time");
        get_global_ring0().log.debug(&format!(
            "[llvm/harness] llvm_use_harness() = {}",
            policy.harness_selector_enabled
        ));
        get_global_ring0().log.debug(&format!(
            "[llvm/harness] primary_failfast={} child_nyrt_precheck_bypass={}",
            policy.primary_request_failfast, policy.child_nyrt_precheck_bypass
        ));
    }
}

#[cfg(feature = "llvmlite-compat")]
fn ensure_harness_requested(
    policy: &super::LlvmHarnessInvocationPolicyV1,
) -> Result<(), LlvmRunError> {
    if policy.harness_selector_enabled {
        return Ok(());
    }
    Err(LlvmRunError::fatal(
        "LLVM harness not enabled (NYASH_LLVM_USE_HARNESS not set)",
    ))
}

#[cfg(feature = "llvmlite-compat")]
fn emit_executable_via_ny_llvmc(
    module: &MirModule,
    exe_out: &str,
    policy: &super::LlvmHarnessInvocationPolicyV1,
) -> Result<(), LlvmRunError> {
    let libs = env::env_string("NYASH_LLVM_EXE_LIBS");
    crate::runner::modes::common_util::exec::ny_llvmc_emit_exe_lib_with_harness_policy(
        module,
        exe_out,
        None,
        libs.as_deref(),
        policy.child_nyrt_precheck_bypass,
    )
    .map_err(|e| {
        LlvmRunError::fatal(format!(
            "ny-llvmc emit-exe error: {} (Hint: build ny-llvmc: cargo build -p nyash-llvm-compiler --release)",
            e
        ))
    })
}
