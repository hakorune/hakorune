//! LLVM pipeline report (diagnostic, opt-in)
//!
//! Emits dynamic runner pipeline evidence when requested. This is observation
//! only: it must not choose routes or change executor behavior.

use std::fmt::Write as _;

#[derive(Clone, Debug)]
pub struct LlvmPipelineReport {
    pub mir_future_rewrite_route: &'static str,
    pub method_id_injector_mutation_count: usize,
    pub pipeline_joinir_experiment_enabled: bool,
    pub execution_backend: &'static str,
    pub llvm_fallback_used: bool,
    pub llvm_fallback_reason: &'static str,
    pub pyvm_requested: bool,
    pub harness_requested: bool,
    pub mock_fallback_used: bool,
}

impl LlvmPipelineReport {
    pub fn new() -> Self {
        Self {
            mir_future_rewrite_route: "env_forced_llvm_future_externs",
            method_id_injector_mutation_count: 0,
            pipeline_joinir_experiment_enabled: joinir_experiment_enabled_for_llvm(),
            execution_backend: "not_selected",
            llvm_fallback_used: false,
            llvm_fallback_reason: "none",
            pyvm_requested: crate::config::env::env_bool("SMOKES_USE_PYVM"),
            harness_requested: crate::config::env::env_bool("NYASH_LLVM_USE_HARNESS"),
            mock_fallback_used: false,
        }
    }
}

pub struct PipelineReportBox;

impl PipelineReportBox {
    pub fn emit_if_requested(report: &LlvmPipelineReport) {
        let Some(path) = crate::config::env::llvm_pipeline_report_out() else {
            return;
        };
        if let Err(err) = write_report(&path, report) {
            crate::runtime::get_global_ring0().log.warn(&format!(
                "[llvm/pipeline-report] failed to write {}: {}",
                path, err
            ));
        }
    }
}

fn write_report(path: &str, report: &LlvmPipelineReport) -> std::io::Result<()> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let mut out = String::new();
    writeln!(
        out,
        "output_contract=hako-llvm-pipeline-runtime-report-v0"
    )
    .unwrap();
    writeln!(out, "tool_surface=llvm_runner_pipeline_report").unwrap();
    writeln!(out, "observation_only=1").unwrap();
    writeln!(out, "behavior_change=0").unwrap();
    writeln!(
        out,
        "mir_future_rewrite_route={}",
        report.mir_future_rewrite_route
    )
    .unwrap();
    writeln!(
        out,
        "pipeline_joinir_experiment_enabled={}",
        report.pipeline_joinir_experiment_enabled as u8
    )
    .unwrap();
    writeln!(
        out,
        "method_id_injector_mutation_count={}",
        report.method_id_injector_mutation_count
    )
    .unwrap();
    writeln!(out, "execution_backend={}", report.execution_backend).unwrap();
    writeln!(
        out,
        "llvm_fallback_used={}",
        report.llvm_fallback_used as u8
    )
    .unwrap();
    writeln!(out, "llvm_fallback_reason={}", report.llvm_fallback_reason).unwrap();
    writeln!(out, "pyvm_requested={}", report.pyvm_requested as u8).unwrap();
    writeln!(
        out,
        "harness_requested={}",
        report.harness_requested as u8
    )
    .unwrap();
    writeln!(
        out,
        "mock_fallback_used={}",
        report.mock_fallback_used as u8
    )
    .unwrap();
    writeln!(out, "product_activation=0").unwrap();
    writeln!(out, "hook_installed=0").unwrap();
    writeln!(out, "global_allocator_product_claim=0").unwrap();
    writeln!(out, "winner_claim=0").unwrap();
    writeln!(out, "summary=ok").unwrap();
    std::fs::write(path, out)
}

#[cfg(feature = "llvm-harness")]
fn joinir_experiment_enabled_for_llvm() -> bool {
    crate::config::env::joinir_experiment_enabled()
        && crate::config::env::joinir_llvm_experiment_enabled()
        && crate::config::env::llvm_use_harness()
}

#[cfg(not(feature = "llvm-harness"))]
fn joinir_experiment_enabled_for_llvm() -> bool {
    false
}
