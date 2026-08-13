use std::path::PathBuf;

use super::capi_transport::compile_via_capi_keep;
use super::defaults::COMPILE_SYMBOL_DEFAULT;
use super::ll_emit_compare_driver::mir_json_to_object_hako_ll_compare;
use super::normalize::validate_backend_mir_shape;
use super::provider_keep::{mir_json_to_object_llvmlite, mir_json_to_object_ny_llvmc};
use super::{CodegenRouteRequestV1, Opts};

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
    validate_route_request(opts)?;
    if opts.route_request == CodegenRouteRequestV1::ExplicitHarnessCompat {
        // The named compatibility admission is intentionally a direct
        // harness/provider lane.  Do not let a generic C-ABI probe silently
        // change the meaning of that explicit request.
        return Ok(None);
    }
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
    let compile_recipe = opts.compile_recipe.clone();
    let compat_replay = opts.compat_replay.clone();
    let compile_symbol = compile_symbol_for_keep_recipe(compile_recipe.as_deref());
    match compile_via_capi_keep(
        mir_json,
        compile_symbol,
        compile_recipe.as_deref(),
        compat_replay.as_deref(),
        opts,
    ) {
        Ok(out_path) => Ok(out_path),
        Err(e) => Err(e),
    }
}

pub(super) fn try_compile_via_explicit_provider_keep(
    mir_json: &str,
    opts: &Opts,
) -> Result<Option<PathBuf>, String> {
    match opts.route_request {
        CodegenRouteRequestV1::ExplicitHarnessCompat => {
            return mir_json_to_object_llvmlite(mir_json, opts).map(Some);
        }
        CodegenRouteRequestV1::BoundaryPureFirst => return Ok(None),
        CodegenRouteRequestV1::LegacyAmbientKeep => {}
    }
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
    validate_route_request(opts)?;
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

fn validate_route_request(opts: &Opts) -> Result<(), String> {
    match opts.route_request {
        CodegenRouteRequestV1::LegacyAmbientKeep => Ok(()),
        CodegenRouteRequestV1::BoundaryPureFirst => {
            if opts.compile_recipe.as_deref() != Some("pure-first") {
                return Err(
                    "[llvmemit/request] Boundary route requires compile_recipe=pure-first"
                        .to_string(),
                );
            }
            if matches!(opts.compat_replay.as_deref(), Some(value) if value != "none") {
                return Err(
                    "[llvmemit/request] Boundary route rejects compat replay inheritance"
                        .to_string(),
                );
            }
            Ok(())
        }
        CodegenRouteRequestV1::ExplicitHarnessCompat => {
            if opts.compile_recipe.as_deref() != Some("pure-first")
                || opts.compat_replay.as_deref() != Some("harness")
            {
                return Err(
                    "[llvmemit/request] explicit harness route requires pure-first/harness"
                        .to_string(),
                );
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        compile_symbol_for_keep_recipe, hako_ll_bridge_lane, llvm_route_trace_enabled,
        required_hako_ll_context_field, validate_route_request, CodegenRouteRequestV1,
        HakoLlBridgeLane, COMPILE_SYMBOL_DEFAULT, COMPILE_SYMBOL_PURE_FIRST,
    };
    use crate::host_providers::llvm_codegen::defaults::boundary_default_object_opts;

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
        let err =
            required_hako_ll_context_field("acceptance_case", None, HakoLlBridgeLane::Compare)
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

    #[test]
    fn boundary_request_rejects_inherited_harness_replay() {
        let mut opts = boundary_default_object_opts(None, None, None, None);
        opts.route_request = CodegenRouteRequestV1::BoundaryPureFirst;
        opts.compile_recipe = Some("pure-first".to_string());
        opts.compat_replay = Some("harness".to_string());
        let err = validate_route_request(&opts).expect_err("ambient replay must stop");
        assert!(err.contains("rejects compat replay inheritance"));
    }

    #[test]
    fn explicit_harness_request_requires_exact_recipe_and_replay() {
        let mut opts = boundary_default_object_opts(None, None, None, None);
        opts.route_request = CodegenRouteRequestV1::ExplicitHarnessCompat;
        opts.compile_recipe = Some("pure-first".to_string());
        opts.compat_replay = Some("harness".to_string());
        validate_route_request(&opts).expect("named compat admission should validate");
    }
}
