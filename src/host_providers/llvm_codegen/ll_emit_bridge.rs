use std::path::PathBuf;

use super::hako_ll_driver::{
    compile_ll_to_object, extract_contract_line, extract_ll, prepare_hako_driver_source,
    resolve_hakorune_bin, run_driver_via_vm, temporary_ll_output_path, verify_ll_file,
};
use super::normalize;
use super::Opts;

enum BridgeLane {
    Daily,
    Compare,
}

impl BridgeLane {
    fn tag(&self) -> &'static str {
        match self {
            Self::Daily => "daily",
            Self::Compare => "compare",
        }
    }
}

fn mir_json_to_object_hako_ll(
    mir_json: &str,
    opts: &Opts,
    lane: BridgeLane,
) -> Result<PathBuf, String> {
    normalize::validate_backend_mir_shape(mir_json)?;
    let hakorune = resolve_hakorune_bin();
    if !hakorune.exists() {
        return Err(format!(
            "[llvmemit/hako-ll/not-found] path={}",
            hakorune.display()
        ));
    }

    let out_path = super::transport::resolve_backend_object_output(opts);
    super::transport::ensure_backend_output_parent(&out_path);
    let ll_path = temporary_ll_output_path(&out_path, lane.tag());
    let acceptance_case = crate::config::env::backend_acceptance_case()
        .unwrap_or_else(|| "unset".to_string());
    let legacy_daily_allowed = crate::config::env::backend_legacy_daily_allowed()
        .unwrap_or_else(|| "unknown".to_string());
    let source_path = prepare_hako_driver_source(
        mir_json,
        &out_path,
        lane.tag(),
        &acceptance_case,
        &legacy_daily_allowed,
    )?;
    let stdout = run_driver_via_vm(&hakorune, &source_path)?;
    let contract_line = extract_contract_line(&stdout, lane.tag())?;
    let ll_text = extract_ll(&stdout)?;
    std::fs::write(&ll_path, ll_text).map_err(|e| {
        format!(
            "[llvmemit/hako-ll/output-write-failed] path={} error={}",
            ll_path.display(),
            e
        )
    })?;
    println!("{}", contract_line);
    verify_ll_file(&ll_path)?;
    compile_ll_to_object(&ll_path, &out_path)?;
    let _ = std::fs::remove_file(&source_path);
    let _ = std::fs::remove_file(&ll_path);
    Ok(out_path)
}

pub(super) fn mir_json_to_object_hako_ll_daily(
    mir_json: &str,
    opts: &Opts,
) -> Result<PathBuf, String> {
    mir_json_to_object_hako_ll(mir_json, opts, BridgeLane::Daily)
}

pub(super) fn mir_json_to_object_hako_ll_compare(
    mir_json: &str,
    opts: &Opts,
) -> Result<PathBuf, String> {
    mir_json_to_object_hako_ll(mir_json, opts, BridgeLane::Compare)
}
