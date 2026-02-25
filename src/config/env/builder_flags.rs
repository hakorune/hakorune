//! MIR builder-related environment flags
//!
//! Centralizes builder/debug toggles to avoid direct env reads.

use super::{env_bool, env_present, env_string};

pub fn builder_me_call_arity_strict() -> bool {
    env_bool("NYASH_ME_CALL_ARITY_STRICT")
}

pub fn builder_static_call_trace() -> bool {
    env_bool("NYASH_STATIC_CALL_TRACE")
}

pub fn builder_static_method_trace() -> bool {
    env_bool("NYASH_STATIC_METHOD_TRACE")
}

pub fn builder_conservative_phi_trace() -> bool {
    env_bool("NYASH_CONSERVATIVE_PHI_TRACE")
}

pub fn builder_type_registry_trace() -> bool {
    env_bool("NYASH_TYPE_REGISTRY_TRACE")
}

pub fn builder_source_file_hint() -> Option<String> {
    env_string("NYASH_SOURCE_FILE_HINT")
}

pub fn builder_router_trace() -> bool {
    env_string("NYASH_ROUTER_TRACE")
        .map(|v| matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "on" | "yes"))
        .unwrap_or(false)
}

pub fn builder_loopform_debug() -> bool {
    env_present("NYASH_LOOPFORM_DEBUG")
}

pub fn builder_use_type_registry() -> bool {
    env_bool("NYASH_USE_TYPE_REGISTRY")
}

pub fn builder_rewrite_known_default() -> Option<String> {
    env_string("NYASH_REWRITE_KNOWN_DEFAULT")
}

pub fn builder_rewrite_instance_mode() -> Option<String> {
    env_string("NYASH_BUILDER_REWRITE_INSTANCE")
}

pub fn builder_dev_rewrite_userbox() -> bool {
    env_bool("NYASH_DEV_REWRITE_USERBOX")
}

pub fn builder_dev_rewrite_new_origin() -> bool {
    env_bool("NYASH_DEV_REWRITE_NEW_ORIGIN")
}

pub fn builder_typefacts_debug() -> bool {
    env_present("NYASH_TYPEFACTS_DEBUG")
}

pub fn builder_birth_inject_builtins() -> bool {
    env_bool("NYASH_DEV_BIRTH_INJECT_BUILTINS")
}

pub fn builder_p3d_debug() -> bool {
    env_present("NYASH_P3D_DEBUG")
}

pub fn builder_p4_debug() -> bool {
    env_present("NYASH_P4_DEBUG")
}

pub fn builder_p3c_debug() -> bool {
    env_present("NYASH_P3C_DEBUG")
}

pub fn builder_operator_box_all_call() -> bool {
    env_bool("NYASH_BUILDER_OPERATOR_BOX_ALL_CALL")
}

pub fn builder_operator_box_add_call() -> bool {
    env_bool("NYASH_BUILDER_OPERATOR_BOX_ADD_CALL")
}

pub fn builder_operator_box_compare_call() -> bool {
    env_bool("NYASH_BUILDER_OPERATOR_BOX_COMPARE_CALL")
}

pub fn builder_local_ssa_trace() -> bool {
    env_bool("NYASH_LOCAL_SSA_TRACE")
}

pub fn builder_schedule_trace() -> bool {
    env_bool("NYASH_SCHEDULE_TRACE")
}

pub fn builder_block_schedule_verify() -> bool {
    env_bool("NYASH_BLOCK_SCHEDULE_VERIFY")
}

pub fn builder_trace_recv() -> bool {
    env_bool("NYASH_BUILDER_TRACE_RECV")
}

pub fn builder_mir_compile_trace() -> bool {
    env_bool("NYASH_MIR_COMPILE_TRACE")
}

pub fn builder_mir_type_trace() -> bool {
    env_bool("NYASH_MIR_TYPE_TRACE")
}

pub fn builder_debug_enabled() -> bool {
    env_present("NYASH_BUILDER_DEBUG")
}

pub fn builder_debug_limit() -> Option<usize> {
    env_string("NYASH_BUILDER_DEBUG_LIMIT").and_then(|s| s.parse::<usize>().ok())
}

pub fn builder_201a_debug() -> bool {
    env_present("NYASH_201A_DEBUG")
}

pub fn builder_if_trace() -> bool {
    env_bool("NYASH_IF_TRACE")
}

pub fn builder_build_static_main_entry() -> bool {
    env_present("NYASH_BUILD_STATIC_MAIN_ENTRY")
}

pub fn builder_script_args_json() -> Option<String> {
    env_string("NYASH_SCRIPT_ARGS_JSON").or_else(|| env_string("HAKO_SCRIPT_ARGS_JSON"))
}

pub fn builder_disable_trycatch() -> bool {
    env_bool("NYASH_BUILDER_DISABLE_TRYCATCH")
}

pub fn builder_trycatch_debug() -> bool {
    env_bool("NYASH_DEBUG_TRYCATCH")
}

pub fn builder_disable_throw() -> bool {
    env_bool("NYASH_BUILDER_DISABLE_THROW")
}

pub fn builder_boxcall_type_debug() -> bool {
    env_bool("NYASH_BOXCALL_TYPE_DEBUG")
}

pub fn builder_boxcall_type_trace() -> bool {
    env_bool("NYASH_BOXCALL_TYPE_TRACE")
}

pub fn builder_debug_kpi_known() -> bool {
    env_bool("NYASH_DEBUG_KPI_KNOWN")
}

pub fn builder_debug_sample_every() -> Option<usize> {
    env_string("NYASH_DEBUG_SAMPLE_EVERY").and_then(|s| s.parse::<usize>().ok())
}

pub fn builder_pin_trace() -> bool {
    env_bool("NYASH_PIN_TRACE")
}

pub fn builder_callee_resolve_trace() -> bool {
    env_bool("NYASH_CALLEE_RESOLVE_TRACE")
}

pub fn builder_debug_annotation() -> bool {
    env_bool("NYASH_DEBUG_ANNOTATION")
}

pub fn builder_debug_param_receiver() -> bool {
    env_bool("NYASH_DEBUG_PARAM_RECEIVER")
}

pub fn builder_tail_resolve() -> bool {
    env_bool("NYASH_BUILDER_TAIL_RESOLVE")
}

pub fn builder_methodize_trace() -> bool {
    env_bool("NYASH_METHODIZE_TRACE")
}

pub fn builder_call_resolve_trace() -> bool {
    env_bool("NYASH_CALL_RESOLVE_TRACE")
}

pub fn builder_methodize_mode() -> Option<String> {
    env_string("HAKO_MIR_BUILDER_METHODIZE")
}

pub fn builder_unified_call_mode() -> Option<String> {
    env_string("NYASH_MIR_UNIFIED_CALL")
}

pub fn builder_trace_normalize() -> bool {
    env_present("NYASH_TRACE_NORMALIZE")
}

pub fn builder_trace_varmap() -> bool {
    env_present("NYASH_TRACE_VARMAP")
}

pub fn builder_option_c_debug() -> bool {
    env_present("NYASH_OPTION_C_DEBUG")
}

pub fn builder_capture_debug() -> bool {
    env_present("NYASH_CAPTURE_DEBUG")
}

pub fn builder_carrier_phi_debug() -> bool {
    env_bool("NYASH_CARRIER_PHI_DEBUG")
}

pub fn builder_safepoint_entry() -> bool {
    env_bool("NYASH_BUILDER_SAFEPOINT_ENTRY")
}
