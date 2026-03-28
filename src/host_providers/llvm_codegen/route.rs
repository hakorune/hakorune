use std::path::PathBuf;

use super::defaults::COMPILE_SYMBOL_DEFAULT;
use super::ll_emit_compare_driver::mir_json_to_object_hako_ll_compare;
use super::capi_transport::compile_via_capi;
use super::normalize::validate_backend_mir_shape;
use super::provider_keep::{mir_json_to_object_llvmlite, mir_json_to_object_ny_llvmc};
use super::transport_io::{ensure_backend_output_parent, prepare_backend_input_json_file};
use super::transport_paths::resolve_backend_object_output;
use super::Opts;

const COMPILE_SYMBOL_PURE_FIRST: &[u8] = b"hako_llvmc_compile_json_pure_first\0";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HakoLlBridgeLane {
    Compare,
}

impl HakoLlBridgeLane {
    fn tag(self) -> &'static str {
        "compare"
    }
}

fn hako_ll_bridge_lane(recipe: Option<&str>) -> Option<HakoLlBridgeLane> {
    match recipe {
        Some("hako-ll-compare-v0") => Some(HakoLlBridgeLane::Compare),
        _ => None,
    }
}

struct HakoLlRouteContext {
    acceptance_case: String,
    transport_owner: String,
    legacy_daily_allowed: String,
}

fn llvm_route_trace_enabled() -> bool {
    matches!(
        std::env::var("NYASH_LLVM_ROUTE_TRACE").ok().as_deref(),
        Some("1" | "on" | "true" | "yes")
    )
}

fn required_hako_ll_context_field(
    field_name: &str,
    value: Option<String>,
    lane: HakoLlBridgeLane,
) -> Result<String, String> {
    value.ok_or_else(|| {
        format!(
            "[llvmemit/hako-ll/context-missing] lane={} field={}",
            lane.tag(),
            field_name
        )
    })
}

fn validate_hako_ll_route_context(lane: HakoLlBridgeLane) -> Result<HakoLlRouteContext, String> {
    let acceptance_case = required_hako_ll_context_field(
        "acceptance_case",
        crate::config::env::backend_acceptance_case(),
        lane,
    )?;
    let transport_owner = required_hako_ll_context_field(
        "transport_owner",
        crate::config::env::backend_transport_owner(),
        lane,
    )?;
    if transport_owner != "hako_ll_emitter" {
        return Err(format!(
            "[llvmemit/hako-ll/context-mismatch] lane={} field=transport_owner expected=hako_ll_emitter got={}",
            lane.tag(),
            transport_owner
        ));
    }
    let legacy_daily_allowed = required_hako_ll_context_field(
        "legacy_daily_allowed",
        crate::config::env::backend_legacy_daily_allowed(),
        lane,
    )?;
    if legacy_daily_allowed != "no" {
        return Err(format!(
            "[llvmemit/hako-ll/context-mismatch] lane={} field=legacy_daily_allowed expected=no got={}",
            lane.tag(),
            legacy_daily_allowed
        ));
    }
    Ok(HakoLlRouteContext {
        acceptance_case,
        transport_owner,
        legacy_daily_allowed,
    })
}

fn emit_hako_ll_route_trace(
    recipe: Option<&str>,
    compat_replay: Option<&str>,
    ctx: &HakoLlRouteContext,
) {
    if !llvm_route_trace_enabled() {
        return;
    }
    eprintln!(
        "[llvm-route/select] owner={} recipe={} compat_replay={} acceptance_case={} legacy_daily_allowed={}",
        ctx.transport_owner,
        recipe.unwrap_or("unset"),
        compat_replay.unwrap_or("unset"),
        ctx.acceptance_case,
        ctx.legacy_daily_allowed
    );
    eprintln!(
        "[llvm-route/replay] lane=compare reason=explicit_bridge acceptance_case={}",
        ctx.acceptance_case
    );
}

pub(super) fn try_compile_via_hako_ll_bridge(
    mir_json: &str,
    opts: &Opts,
) -> Result<Option<PathBuf>, String> {
    match hako_ll_bridge_lane(opts.compile_recipe.as_deref()) {
        Some(HakoLlBridgeLane::Compare) => {
            let ctx = validate_hako_ll_route_context(HakoLlBridgeLane::Compare)?;
            emit_hako_ll_route_trace(
                opts.compile_recipe.as_deref(),
                opts.compat_replay.as_deref(),
                &ctx,
            );
            validate_backend_mir_shape(mir_json)?;
            mir_json_to_object_hako_ll_compare(mir_json, opts).map(Some)
        }
        _ => Ok(None),
    }
}

pub(super) fn try_compile_via_capi_keep(
    mir_json: &str,
    opts: &Opts,
) -> Result<Option<PathBuf>, String> {
    if !(crate::config::env::llvm_use_capi() && crate::config::env::extern_provider_c_abi()) {
        return Ok(None);
    }
    match compile_via_capi_keep_internal(mir_json, opts) {
        Ok(out_path) => Ok(Some(out_path)),
        Err(e) => {
            llvm_emit_error!("[llvmemit/capi/failed] {}", e);
            Err(format!("[llvmemit/capi/failed] {}", e))
        }
    }
}

fn compile_via_capi_keep_internal(mir_json: &str, opts: &Opts) -> Result<PathBuf, String> {
    validate_backend_mir_shape(mir_json)?;
    let in_path = prepare_backend_input_json_file(mir_json)?;
    let out_path = resolve_backend_object_output(opts);
    ensure_backend_output_parent(&out_path);
    let compile_recipe = opts.compile_recipe.clone();
    let compat_replay = opts.compat_replay.clone();
    let compile_symbol = compile_symbol_for_keep_recipe(compile_recipe.as_deref());
    match compile_via_capi(
        &in_path,
        &out_path,
        compile_symbol,
        compile_recipe.as_deref(),
        compat_replay.as_deref(),
        opts,
    ) {
        Ok(()) => Ok(out_path),
        Err(e) => Err(e),
    }
}

pub(super) fn try_compile_via_explicit_provider_keep(
    mir_json: &str,
    opts: &Opts,
) -> Result<Option<PathBuf>, String> {
    match crate::config::env::llvm_emit_provider().as_deref() {
        Some("llvmlite") => mir_json_to_object_llvmlite(mir_json, opts).map(Some),
        Some("ny-llvmc") => mir_json_to_object_ny_llvmc(mir_json, opts).map(Some),
        _ => Ok(None),
    }
}

pub(super) fn try_compile_via_boundary_default(
    mir_json: &str,
    opts: &Opts,
) -> Result<Option<PathBuf>, String> {
    match compile_via_capi_keep_internal(mir_json, opts) {
        Ok(out_path) => Ok(Some(out_path)),
        Err(error) if capi_boundary_unavailable(&error) => Ok(None),
        Err(error) => {
            llvm_emit_error!("[llvmemit/capi/default-failed] {}", error);
            Err(format!("[llvmemit/capi/default-failed] {}", error))
        }
    }
}

pub(super) fn boundary_default_unavailable_tag() -> String {
    "[llvmemit/capi/default-unavailable] build libhako_llvmc_ffi.so or set HAKO_LLVM_EMIT_PROVIDER=llvmlite".into()
}

fn compile_symbol_for_keep_recipe(recipe: Option<&str>) -> &'static [u8] {
    // Keep lanes may still reuse the historical generic export.
    // Daily pure-first callers should already be explicit before reaching here.
    match recipe {
        Some("pure-first") => COMPILE_SYMBOL_PURE_FIRST,
        _ => COMPILE_SYMBOL_DEFAULT,
    }
}

fn capi_boundary_unavailable(error: &str) -> bool {
    error.contains("FFI library not found")
        || error.contains("capi not available")
        || error.contains("dlopen failed")
        || error.contains("dlsym failed")
}

#[cfg(test)]
mod tests {
    use super::{
        compile_symbol_for_keep_recipe, hako_ll_bridge_lane, llvm_route_trace_enabled,
        required_hako_ll_context_field, HakoLlBridgeLane, COMPILE_SYMBOL_DEFAULT,
        COMPILE_SYMBOL_PURE_FIRST,
    };

    #[test]
    fn keep_recipe_prefers_pure_first_symbol_when_explicit() {
        assert_eq!(
            compile_symbol_for_keep_recipe(Some("pure-first")),
            COMPILE_SYMBOL_PURE_FIRST
        );
    }

    #[test]
    fn keep_recipe_uses_generic_symbol_for_missing_or_compat_values() {
        assert_eq!(compile_symbol_for_keep_recipe(None), COMPILE_SYMBOL_DEFAULT);
        assert_eq!(
            compile_symbol_for_keep_recipe(Some("harness")),
            COMPILE_SYMBOL_DEFAULT
        );
    }

    #[test]
    fn hako_ll_bridge_lane_stays_explicit() {
        assert_eq!(
            hako_ll_bridge_lane(Some("hako-ll-compare-v0")),
            Some(HakoLlBridgeLane::Compare)
        );
        assert_eq!(hako_ll_bridge_lane(Some("hako-ll-min-v0")), None);
        assert_eq!(hako_ll_bridge_lane(Some("pure-first")), None);
        assert_eq!(hako_ll_bridge_lane(None), None);
    }

    #[test]
    fn required_hako_ll_context_field_is_fail_fast() {
        let err = required_hako_ll_context_field("acceptance_case", None, HakoLlBridgeLane::Compare)
            .expect_err("missing acceptance_case should fail");
        assert!(err.contains("field=acceptance_case"));
        assert!(err.contains("lane=compare"));
    }

    #[test]
    fn llvm_route_trace_enabled_accepts_explicit_truthy_values_only() {
        std::env::remove_var("NYASH_LLVM_ROUTE_TRACE");
        assert!(!llvm_route_trace_enabled());
        std::env::set_var("NYASH_LLVM_ROUTE_TRACE", "1");
        assert!(llvm_route_trace_enabled());
        std::env::set_var("NYASH_LLVM_ROUTE_TRACE", "yes");
        assert!(llvm_route_trace_enabled());
        std::env::set_var("NYASH_LLVM_ROUTE_TRACE", "0");
        assert!(!llvm_route_trace_enabled());
        std::env::remove_var("NYASH_LLVM_ROUTE_TRACE");
    }
}
