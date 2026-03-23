/*!
 * Unified Call System
 *
 * ChatGPT5 Pro A++ design for complete call unification
 * Replaces 6 different call instructions with a single unified system
 */

use crate::mir::definitions::call_unified::{CallFlags, MirCall};
use crate::mir::{Callee, EffectMask, ValueId};

/// Check if unified call system is enabled
pub fn is_unified_call_enabled() -> bool {
    match crate::config::env::builder_unified_call_mode()
        .as_deref()
        .map(|s| s.to_ascii_lowercase())
    {
        Some(s) if s == "0" || s == "false" || s == "off" => false,
        _ => true, // default ON during development; explicit opt-out supported
    }
}

/// Classify box type to prevent static/runtime mixing
/// Prevents Stage-B/Stage-1 compiler boxes from being confused with runtime data boxes
pub fn classify_box_kind(box_name: &str) -> crate::mir::definitions::call_unified::CalleeBoxKind {
    use crate::mir::definitions::call_unified::CalleeBoxKind;

    // Static compiler boxes (Stage-B, Stage-1, parsers, resolvers)
    // These should ONLY appear in static method lowering, never in runtime method dispatch
    match box_name {
        // Stage-B compiler boxes
        "StageBArgsBox" | "StageBBodyExtractorBox" | "StageBDriverBox" |
        // Stage-1 using/namespace resolver boxes
        "Stage1UsingResolverBox" | "BundleResolver" |
        // Parser boxes
        "ParserBox" | "ParserStmtBox" | "ParserExprBox" | "ParserControlBox" |
        "ParserLiteralBox" | "ParserTokenBox" |
        // Scanner/builder boxes
        "FuncScannerBox" | "MirBuilderBox" |
        // Selfhost builder helper boxes
        "JsonFragBox" | "JsonCursorBox" | "JsonScanBox" |
        "LowerReturnMethodArrayMapBox" | "PatternUtilBox" | "MethodAliasPolicy" |
        "StringHelpers" | "StringOps" | "StringScanBox" | "StringifyOperator" |
        "AddOperator" | "CompareOperator"
        => CalleeBoxKind::StaticCompiler,

        // Runtime data boxes (built-in types that handle actual runtime values)
        "MapBox" | "ArrayBox" | "StringBox" | "IntegerBox" | "BoolBox" |
        "FloatBox" | "NullBox" | "VoidBox" | "UnknownBox" |
        "FileBox" | "ConsoleBox" | "PathBox"
        => CalleeBoxKind::RuntimeData,

        // Everything else is user-defined
        _ => CalleeBoxKind::UserDefined,
    }
}

/// Convert CallTarget to Callee
/// Main translation layer between builder and MIR representations
/// Convert CallTarget to Callee with type resolution
/// 🎯 TypeRegistry 対応: NYASH_USE_TYPE_REGISTRY=1 で registry 優先
// DEPRECATED: Moved to CalleeResolverBox::resolve() in resolver.rs
// This function is kept for reference but should not be used.
// Use CalleeResolverBox instead for all callee resolution.

// DEPRECATED: Moved to EffectsAnalyzerBox in effects_analyzer.rs
// Use EffectsAnalyzerBox::compute_call_effects() instead.
pub fn compute_call_effects(callee: &Callee) -> EffectMask {
    super::effects_analyzer::EffectsAnalyzerBox::compute_call_effects(callee)
}

/// Create CallFlags based on callee type
pub fn create_call_flags(callee: &Callee) -> CallFlags {
    if callee.is_constructor() {
        CallFlags::constructor()
    } else if matches!(callee, Callee::Closure { .. }) {
        CallFlags::constructor() // Closures are also constructors
    } else {
        CallFlags::new()
    }
}

/// Create a MirCall instruction from components
pub fn create_mir_call(dst: Option<ValueId>, callee: Callee, args: Vec<ValueId>) -> MirCall {
    let effects = compute_call_effects(&callee);
    let flags = create_call_flags(&callee);

    MirCall {
        dst,
        callee,
        args,
        flags,
        effects,
    }
}

// DEPRECATED: validate_call_args moved to CalleeResolverBox::validate_args() in resolver.rs
// Use CalleeResolverBox instead for argument validation.
