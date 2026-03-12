use super::exit::CoreExitPlan;
use crate::mir::{BinaryOp, CompareOp, ConstValue, EffectMask, ValueId};

/// Phase 273 P1: Effect plan (side effects already lowered to ValueId)
///
/// Effect vocabulary is minimal (scan-specific variants forbidden):
/// - MethodCall, GlobalCall, BinOp, Compare, Const
/// - ExitIf (loop-only)
/// - IfEffect (loop-only, leaf effects only, no else)
///
/// Phase 273 P2: MethodCall now supports:
/// - dst: Option<ValueId> for void methods (e.g., push)
/// - effects: EffectMask for side effects (e.g., MUT for push)
#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum CoreEffectPlan {
    /// Method call (args are ValueIds, not Strings!)
    ///
    /// Phase 273 P2: dst is Option for void methods, effects for side effects
    MethodCall {
        dst: Option<ValueId>, // P2: Option for void methods (push)
        object: ValueId,
        method: String, // Method name only (OK as String)
        args: Vec<ValueId>,
        effects: EffectMask, // P2: Side effect mask (PURE+Io or MUT)
    },
    /// Global/static call (box-level or free function)
    GlobalCall {
        dst: Option<ValueId>,
        func: String,
        args: Vec<ValueId>,
    },
    /// Indirect call (callee is a ValueId)
    ValueCall {
        dst: Option<ValueId>,
        callee: ValueId,
        args: Vec<ValueId>,
    },
    /// External call (env.* and other extern interfaces)
    ExternCall {
        dst: Option<ValueId>,
        iface_name: String,
        method_name: String,
        args: Vec<ValueId>,
        effects: EffectMask,
    },

    /// New box allocation
    NewBox {
        dst: ValueId,
        box_type: String,
        args: Vec<ValueId>,
    },

    /// Binary operation
    BinOp {
        dst: ValueId,
        lhs: ValueId,
        op: BinaryOp,
        rhs: ValueId,
    },

    /// Comparison
    Compare {
        dst: ValueId,
        lhs: ValueId,
        op: CompareOp,
        rhs: ValueId,
    },

    /// Conditional select (ternary)
    Select {
        dst: ValueId,
        cond: ValueId,
        then_val: ValueId,
        else_val: ValueId,
    },

    /// Conditional exit inside loop body (minimal ExitIf)
    ///
    /// - exit must be Return/Break/Continue only
    /// - Return requires payload (Some)
    ExitIf { cond: ValueId, exit: CoreExitPlan },

    /// Conditional effects inside loop body (optional else, no join)
    ///
    /// - then/else effects must be leaf effects or nested IfEffect
    /// - tail ExitIf(Continue) is allowed (single, last)
    /// - loop-only (cannot appear outside loop body)
    IfEffect {
        cond: ValueId,
        then_effects: Vec<CoreEffectPlan>,
        else_effects: Option<Vec<CoreEffectPlan>>,
    },

    /// Constant
    Const { dst: ValueId, value: ConstValue },

    /// Copy (SSA value assignment)
    ///
    /// **SSOT for short-circuit + joins**: Used by `cond_lowering.rs` to ensure
    /// all paths through `&&`/`||` expansion define the same intermediate value
    /// for outer PHI merge. This is not a workaround but the canonical solution
    /// for the 3-path problem (short-circuit creates 3 paths, joins expects 2 states).
    Copy { dst: ValueId, src: ValueId },
}
