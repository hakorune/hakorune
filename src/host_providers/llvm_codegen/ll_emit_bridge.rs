use std::path::PathBuf;

use super::hako_ll_driver::{
    extract_contract_line, extract_ll, prepare_hako_driver_source, resolve_hakorune_bin,
    run_driver_via_vm,
};
use super::ll_tool_driver;
use super::normalize;
use super::Opts;

const COMPARE_TAG: &str = "compare";

fn mir_json_to_object_hako_ll(mir_json: &str, opts: &Opts) -> Result<PathBuf, String> {
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
    let acceptance_case =
        crate::config::env::backend_acceptance_case().unwrap_or_else(|| "unset".to_string());
    let legacy_daily_allowed =
        crate::config::env::backend_legacy_daily_allowed().unwrap_or_else(|| "unknown".to_string());
    let source_path = prepare_hako_driver_source(
        mir_json,
        &out_path,
        COMPARE_TAG,
        &acceptance_case,
        &legacy_daily_allowed,
    )?;
    let stdout = run_driver_via_vm(&hakorune, &source_path)?;
    let contract_line = extract_contract_line(&stdout, COMPARE_TAG)?;
    let ll_text = extract_ll(&stdout)?;
    println!("{}", contract_line);
    let compile_result = ll_tool_driver::ll_text_to_object(&ll_text, &out_path, COMPARE_TAG);
    let _ = std::fs::remove_file(&source_path);
    compile_result?;
    Ok(out_path)
}

pub(super) fn mir_json_to_object_hako_ll_compare(
    mir_json: &str,
    opts: &Opts,
) -> Result<PathBuf, String> {
    mir_json_to_object_hako_ll(mir_json, opts)
}
