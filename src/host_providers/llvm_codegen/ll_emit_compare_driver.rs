use std::path::PathBuf;

use super::ll_emit_compare_source;
use super::ll_tool_driver;
use super::normalize;
use super::ll_emit_compare_stdout;
use super::ll_emit_compare_vm;
use super::Opts;

const COMPARE_TAG: &str = "compare";

pub(super) fn mir_json_to_object_hako_ll_compare(
    mir_json: &str,
    opts: &Opts,
) -> Result<PathBuf, String> {
    normalize::validate_backend_mir_shape(mir_json)?;
    let hakorune = ll_emit_compare_vm::resolve_hakorune_bin();
    if !hakorune.exists() {
        return Err(format!(
            "[llvmemit/hako-ll/not-found] path={}",
            hakorune.display()
        ));
    }

    let out_path = super::transport_paths::resolve_backend_object_output(opts);
    super::transport_io::ensure_backend_output_parent(&out_path);
    let acceptance_case =
        crate::config::env::backend_acceptance_case().unwrap_or_else(|| "unset".to_string());
    let legacy_daily_allowed =
        crate::config::env::backend_legacy_daily_allowed().unwrap_or_else(|| "unknown".to_string());
    let source_path = ll_emit_compare_source::prepare_hako_driver_source(
        mir_json,
        &out_path,
        COMPARE_TAG,
        &acceptance_case,
        &legacy_daily_allowed,
    )?;
    let stdout = ll_emit_compare_vm::run_driver_via_vm(&hakorune, &source_path)?;
    let contract_line = ll_emit_compare_stdout::extract_contract_line(&stdout, COMPARE_TAG)?;
    let ll_text = ll_emit_compare_stdout::extract_ll(&stdout)?;
    println!("{}", contract_line);
    let compile_result = ll_tool_driver::ll_text_to_object(&ll_text, &out_path, COMPARE_TAG);
    let _ = std::fs::remove_file(&source_path);
    compile_result?;
    Ok(out_path)
}
