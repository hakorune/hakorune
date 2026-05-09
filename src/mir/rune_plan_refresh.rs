use crate::mir::{MirFunction, MirModule};

/// Refresh every MIR-owned plan derived directly from declaration-local runes.
///
/// Keep this as the single entry point after `metadata.runes` changes so new
/// rune-derived plans cannot drift between parser, JSON bridge, and tests.
pub fn refresh_function_rune_plans(function: &mut MirFunction) {
    crate::mir::effect_capability_plan::refresh_function_effect_capability_plans(function);
    crate::mir::inline_plan::refresh_function_inline_plans(function);
}

/// Refresh rune-derived plans for every function after module construction.
///
/// Builder installs declaration runes before a function body is complete. Plans
/// such as required InlinePlan verification depend on the final body shape, so
/// compiler lanes must refresh once at the module boundary before optimizer
/// consumers inspect those plans.
pub fn refresh_module_rune_plans(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_rune_plans(function);
    }
}
