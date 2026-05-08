use crate::mir::MirFunction;

/// Refresh every MIR-owned plan derived directly from declaration-local runes.
///
/// Keep this as the single entry point after `metadata.runes` changes so new
/// rune-derived plans cannot drift between parser, JSON bridge, and tests.
pub fn refresh_function_rune_plans(function: &mut MirFunction) {
    crate::mir::effect_capability_plan::refresh_function_effect_capability_plans(function);
    crate::mir::inline_plan::refresh_function_inline_plans(function);
}
