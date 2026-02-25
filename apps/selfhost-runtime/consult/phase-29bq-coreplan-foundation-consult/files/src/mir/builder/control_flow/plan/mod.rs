//! Phase 273 P1: DomainPlan/CorePlan 二層構造 + PlanNormalizer + PlanVerifier
//!
//! This module provides a two-layer Plan architecture for loop pattern lowering:
//!
//! # Architecture
//!
//! ```text
//! DomainPlan (Pattern固有)
//!     ↓ PlanNormalizer (SSOT)
//! CorePlan (固定語彙 - 構造ノードのみ)
//!     ↓ PlanLowerer
//! MIR (block/value/phi)
//! ```
//!
//! - **DomainPlan**: Pattern-specific plans (ScanWithInit etc.)
//! - **PlanNormalizer**: DomainPlan → CorePlan conversion (SSOT, scan knowledge here)
//! - **CorePlan**: Fixed vocabulary, expressions as ValueId references (no String parsing)
//! - **PlanVerifier**: Fail-fast validation for CorePlan invariants
//! - **PlanLowerer**: Processes CorePlan only (no string interpretation)
//!
//! # Key Design Decision (String式禁止)
//!
//! CorePlan expressions use **ValueId references only** (String expressions forbidden).
//! This prevents "second language processor" from growing inside Lowerer.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::loop_cond_break_continue::facts::LoopCondBreakContinueFacts;
use crate::mir::builder::control_flow::plan::loop_true_break_continue::facts::LoopTrueBreakContinueFacts;
use crate::mir::builder::control_flow::plan::facts::pattern2_loopbodylocal_facts::Pattern2LoopBodyLocalFacts;
use crate::mir::builder::control_flow::plan::generic_loop::facts::{
    GenericLoopV0Facts, GenericLoopV1Facts,
};
use crate::mir::{BasicBlockId, BinaryOp, CompareOp, ConstValue, EffectMask, ValueId};
use crate::mir::builder::control_flow::edgecfg::api::Frag;

pub(in crate::mir::builder) mod lowerer;
pub(in crate::mir::builder) mod normalizer;
pub(in crate::mir::builder) mod verifier;
pub(in crate::mir::builder) mod branchn;
// Phase 29ai P0: Facts SSOT + Single Planner skeleton (parallel footing, unused in P0)
pub(in crate::mir::builder) mod facts;
// Phase 29ca P1: Core loop body effect contract (SSOT)
pub(in crate::mir::builder) mod coreloop_body_contract;
// Phase 29ca P1: Generic loop v0 module (facts/normalizer SSOT)
pub(in crate::mir::builder) mod generic_loop;
pub(in crate::mir::builder) mod normalize;
pub(in crate::mir::builder) mod planner;
// Phase 29ao P0: CorePlan composer scaffold (unused)
pub(in crate::mir::builder) mod composer;
pub(in crate::mir::builder) mod emit;
// Phase 29ai P6: Extractors moved into plan layer
pub(in crate::mir::builder) mod extractors;
// Phase 29ao P21: Pattern1 subset policy (SSOT gate)
pub(in crate::mir::builder) mod policies;
// Phase 29av P1: FlowBox observability tags (strict/dev only)
pub(in crate::mir::builder) mod observability;
// Phase 29ai P5: JoinIR router → single plan extraction entrypoint
pub(in crate::mir::builder) mod single_planner;
// Phase 29bq P2: loop(true) break/continue coverage
pub(in crate::mir::builder) mod loop_true_break_continue;
// Phase 29bq P2: loop(cond) break/continue coverage
pub(in crate::mir::builder) mod loop_cond_break_continue;

pub(in crate::mir::builder) use branchn::CoreBranchNPlan;

// ============================================================================
// DomainPlan (Pattern固有)
// ============================================================================

/// Phase 273 P1: DomainPlan - Pattern-specific plan vocabulary
///
/// DomainPlan contains pattern-specific knowledge (e.g., scan semantics).
/// Normalizer converts DomainPlan → CorePlan with ValueId generation.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum DomainPlan {
    /// Pattern6: index_of / find scan
    ScanWithInit(ScanWithInitPlan),
    /// Pattern7: split / tokenization scan
    SplitScan(SplitScanPlan),
    /// Pattern4: Loop with Continue (Phase 286 P2)
    Pattern4Continue(Pattern4ContinuePlan),
    /// Pattern1: Simple While Loop (Phase 286 P2.1)
    Pattern1SimpleWhile(Pattern1SimpleWhilePlan),
    /// Pattern1: Char map loop (Phase 29ap P2)
    Pattern1CharMap(Pattern1CharMapPlan),
    /// Pattern1: Array join loop (Phase 29ap P3)
    Pattern1ArrayJoin(Pattern1ArrayJoinPlan),
    /// Generic loop v0 (Phase 29ca P1)
    GenericLoopV0(GenericLoopV0Facts),
    /// Generic loop v1 (Phase 29bs P3)
    GenericLoopV1(GenericLoopV1Facts),
    /// Pattern9: Accumulator Const Loop (Phase 286 P2.3)
    Pattern9AccumConstLoop(Pattern9AccumConstLoopPlan),
    /// Pattern8: Boolean Predicate Scan (Phase 286 P2.4)
    Pattern8BoolPredicateScan(Pattern8BoolPredicateScanPlan),
    /// Pattern3: Loop with If-Phi (Phase 286 P2.6)
    Pattern3IfPhi(Pattern3IfPhiPlan),
    /// Pattern2: Loop with Conditional Break (Phase 286 P3.1)
    Pattern2Break(Pattern2BreakPlan),
    /// Pattern5: Infinite Loop with Early Exit (Phase 286 P3.2)
    Pattern5InfiniteEarlyExit(Pattern5InfiniteEarlyExitPlan),
    /// LoopTrue: Multiple break/continue ifs (Phase 29bq P2)
    LoopTrueBreakContinue(LoopTrueBreakContinueFacts),
    /// LoopCond: Multiple break/continue ifs (Phase 29bq P2)
    LoopCondBreakContinue(LoopCondBreakContinueFacts),
}

/// Phase 273 P0: Scan direction for forward/reverse scan
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum ScanDirection {
    /// Forward scan: i < s.length(), i = i + 1
    Forward,
    /// Reverse scan: i >= 0, i = i - 1
    Reverse,
}

pub(in crate::mir::builder) fn scan_direction_from_step_lit(
    step_lit: i64,
) -> Option<ScanDirection> {
    match step_lit {
        1 => Some(ScanDirection::Forward),
        -1 => Some(ScanDirection::Reverse),
        _ => None,
    }
}

/// Phase 273 P0: Extracted structure for scan-with-init pattern
///
/// This structure contains all the information needed to lower an index_of-style loop.
/// Moved from pattern6_scan_with_init.rs for centralization.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(in crate::mir::builder) struct ScanWithInitPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Haystack variable name (e.g., "s")
    pub haystack: String,
    /// Needle variable name (e.g., "ch")
    pub needle: String,
    /// Step literal (Phase 257: can be 1 forward or -1 reverse)
    pub step_lit: i64,
    /// Early return expression (P0: must be Variable(loop_var))
    pub early_return_expr: ASTNode,
    /// Not-found return literal (P0: must be -1)
    pub not_found_return_lit: i64,
    /// Scan direction (Phase 257 P0)
    pub scan_direction: ScanDirection,
    /// Phase 258 P0: True if dynamic needle (substr.length()), false if fixed (ch)
    pub dynamic_needle: bool,
}

/// Phase 273 P2: Extracted structure for split-scan pattern
///
/// This structure contains all the information needed to lower a split-style loop.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct SplitScanPlan {
    /// Haystack variable name (e.g., "s")
    pub s_var: String,
    /// Separator variable name (e.g., "separator")
    pub sep_var: String,
    /// Accumulator variable name (e.g., "result", ArrayBox)
    pub result_var: String,
    /// Loop index variable name (e.g., "i")
    pub i_var: String,
    /// Segment start position variable name (e.g., "start")
    pub start_var: String,
}

/// Phase 286 P2: Extracted structure for Pattern4 (Loop with Continue)
///
/// This structure contains all the information needed to lower a continue-style loop.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern4ContinuePlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Carrier variable names (e.g., ["sum"])
    pub carrier_vars: Vec<String>,
    /// Loop condition AST (e.g., `i < 6`)
    pub condition: ASTNode,
    /// Continue condition AST (e.g., `i == 0`)
    pub continue_condition: ASTNode,
    /// Carrier update expressions (var -> update AST)
    pub carrier_updates: std::collections::BTreeMap<String, ASTNode>,
    /// Loop increment expression (e.g., `i + 1`)
    pub loop_increment: ASTNode,
}

/// Phase 286 P2.1: Extracted structure for Pattern1 (Simple While Loop)
///
/// This structure contains all the information needed to lower a simple while loop.
/// Pattern1 is the simplest loop: no break, no continue, no if-else-phi.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern1SimpleWhilePlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Loop condition AST (e.g., `i < 3`)
    pub condition: ASTNode,
    /// Loop increment expression AST (e.g., `i + 1`)
    pub loop_increment: ASTNode,
}

/// Phase 29ap P2: Extracted structure for Pattern1 char-map loop
///
/// This structure captures the stdlib-style `to_lower`/`to_upper` loop shape.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern1CharMapPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Loop condition AST (e.g., `i < s.length()`)
    pub condition: ASTNode,
    /// Loop increment expression AST (e.g., `i + 1`)
    pub loop_increment: ASTNode,
    /// Haystack variable name (e.g., "s")
    pub haystack_var: String,
    /// Result accumulator variable name (e.g., "result")
    pub result_var: String,
    /// Receiver variable name for the transform method (e.g., "me")
    pub receiver_var: String,
    /// Transform method name (e.g., "char_to_lower")
    pub transform_method: String,
}

/// Phase 29ap P3: Extracted structure for Pattern1 array join loop
///
/// This structure captures the stdlib-style `StringUtils.join` loop shape.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern1ArrayJoinPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Loop condition AST (e.g., `i < arr.length()`)
    pub condition: ASTNode,
    /// Guard condition AST (e.g., `i > 0`)
    pub if_condition: ASTNode,
    /// Loop increment expression AST (e.g., `i + 1`)
    pub loop_increment: ASTNode,
    /// Array variable name (e.g., "arr")
    pub array_var: String,
    /// Result accumulator variable name (e.g., "result")
    pub result_var: String,
    /// Separator variable name (e.g., "separator")
    pub separator_var: String,
}

/// Phase 286 P2.3: Extracted structure for Pattern9 (Accumulator Const Loop)
///
/// This structure contains all the information needed to lower an accumulator loop.
/// Pattern9 extends Pattern1 with an accumulator variable (e.g., sum = sum + 1).
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern9AccumConstLoopPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Accumulator variable name (e.g., "sum")
    pub acc_var: String,
    /// Loop condition AST (e.g., `i < 3`)
    pub condition: ASTNode,
    /// Accumulator update expression AST (e.g., `sum + 1` or `sum + i`)
    pub acc_update: ASTNode,
    /// Loop increment expression AST (e.g., `i + 1`)
    pub loop_increment: ASTNode,
}

/// Phase 286 P2.4: Extracted structure for Pattern8 (BoolPredicateScan)
///
/// This structure contains all the information needed to lower a boolean predicate scan loop.
/// Pattern8 scans a string with a predicate check (e.g., is_digit) and returns false on first failure.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern8BoolPredicateScanPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Haystack variable name (e.g., "s")
    pub haystack: String,
    /// Predicate receiver name (e.g., "me")
    pub predicate_receiver: String,
    /// Predicate method name (e.g., "is_digit")
    pub predicate_method: String,
    /// Loop condition AST (e.g., `i < s.length()`)
    pub condition: ASTNode,
    /// Loop increment literal (P0: must be 1)
    pub step_lit: i64,
}

/// Phase 286 P2.6: Extracted structure for Pattern3 (Loop with If-Phi)
///
/// This structure contains all the information needed to lower an if-phi merge loop.
/// Pattern3 is a loop with conditional carrier update via if-else branching.
///
/// # Structure
/// ```text
/// loop(i < N) {
///     if (condition) {
///         carrier = then_update
///     } else {
///         carrier = else_update
///     }
///     i = i + step
/// }
/// ```
///
/// # CFG Layout
/// ```text
/// preheader → header(PHI: i, carrier) → body(if_condition)
///              ↓                            ↓
///            after                     then | else
///                                        ↓     ↓
///                                       merge(PHI: carrier)
///                                          ↓
///                                        step(i_next)
///                                          ↓
///                                      back-edge to header
/// ```
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern3IfPhiPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Carrier variable name (e.g., "sum")
    pub carrier_var: String,
    /// Loop condition AST (e.g., `i < 3`)
    pub condition: ASTNode,
    /// If condition AST (e.g., `i > 0`)
    pub if_condition: ASTNode,
    /// Then branch update AST (e.g., `sum + 1`)
    pub then_update: ASTNode,
    /// Else branch update AST (e.g., `sum + 0`)
    pub else_update: ASTNode,
    /// Loop increment expression AST (e.g., `i + 1`)
    pub loop_increment: ASTNode,
}

/// Phase 286 P3.1: Extracted structure for Pattern2 (Loop with Conditional Break)
///
/// This structure contains all the information needed to lower a break-style loop.
///
/// Key insight: after_bb PHI merges break path and natural exit path carrier values.
/// - break path: carrier_break = carrier_update_in_break (if Some) or carrier_current (if None)
/// - natural exit: carrier_out = carrier_current (from header PHI)
/// - after_bb PHI: carrier_out = PHI(header: carrier_current, break_then: carrier_break)
///
/// CFG structure (6 blocks):
/// ```
/// preheader → header(PHI: i_current, carrier_current)
///               ↓
///            body(break_cond check)
///               ↓
///          ┌────┴────┐
///     break_then    step
///     (optional      ↓
///      update)    header (back-edge)
///          ↓
///        after_bb(PHI: carrier_out)
///          ↑
///        header (natural exit when !cond_loop)
/// ```
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern2BreakPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Carrier variable name (e.g., "sum", "result")
    pub carrier_var: String,
    /// Loop condition AST (e.g., `i < 3`)
    pub loop_condition: ASTNode,
    /// Break condition AST (e.g., `i == 1`)
    pub break_condition: ASTNode,
    /// Carrier update in break path (None if no update before break)
    pub carrier_update_in_break: Option<ASTNode>,
    /// Carrier update in normal body path (e.g., `sum + 1`)
    pub carrier_update_in_body: ASTNode,
    /// Loop increment expression AST (e.g., `i + 1`)
    pub loop_increment: ASTNode,
    /// Optional promotion hint for LoopBodyLocal handling (planner-only metadata)
    #[allow(dead_code)]
    pub promotion: Option<Pattern2PromotionHint>,
}

/// Phase 29ai P14: Promotion hint metadata for Pattern2BreakPlan
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum Pattern2PromotionHint {
    LoopBodyLocal(Pattern2LoopBodyLocalFacts),
}

/// Phase 286 P3.2: Exit kind for Pattern5 infinite loop
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum Pattern5ExitKind {
    /// Early return from function
    Return,
    /// Break from loop
    Break,
}

/// Phase 286 P3.2: Extracted structure for Pattern5 (Infinite Loop with Early Exit)
///
/// This structure contains all the information needed to lower a loop(true) pattern
/// with early exit (return or break).
///
/// # PoC Subset
///
/// - `loop(true)` literal only (not `loop(1)` or truthy)
/// - Return version: `if (cond) { return <expr> }` + `i = i + 1`
/// - Break version: `if (cond) { break }` + `sum = sum + 1` + `i = i + 1` (carrier_update required)
///
/// # CFG Structure (Return version)
/// ```text
/// preheader → header(PHI: i_current) → body(exit_cond)
///               ↑                           ↓
///               └───── step ←────────  else path
///                                           ↓
///                                then path: CoreExitPlan::Return
/// ```
///
/// # CFG Structure (Break version)
/// ```text
/// preheader → header(PHI: i, carrier) → body(exit_cond)
///               ↑                             ↓
///               └───── step ←──────────  else path
///                                             ↓
///                                  then path → after_bb(PHI: carrier_out)
/// ```
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern5InfiniteEarlyExitPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Exit kind (Return or Break)
    pub exit_kind: Pattern5ExitKind,
    /// Exit condition AST (e.g., `i == 3`)
    pub exit_condition: ASTNode,
    /// Return value expression (Some for Return, None for Break)
    pub exit_value: Option<ASTNode>,
    /// Carrier variable name (Some for Break with carrier, None for Return)
    pub carrier_var: Option<String>,
    /// Carrier update expression (Some for Break, None for Return)
    pub carrier_update: Option<ASTNode>,
    /// Loop increment expression AST (e.g., `i + 1`)
    pub loop_increment: ASTNode,
}

// ============================================================================
// CorePlan (固定語彙 - 構造ノードのみ)
// ============================================================================

/// Phase 273 P1: CorePlan - Fixed vocabulary plan (structure nodes only)
///
/// CorePlan expressions use **ValueId references only** (no String parsing).
/// This prevents "second language processor" from growing inside Lowerer.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(in crate::mir::builder) enum CorePlan {
    /// Sequence: execute plans in order
    Seq(Vec<CorePlan>),

    /// Loop with carriers (PHI variables)
    Loop(CoreLoopPlan),

    /// Conditional branching
    If(CoreIfPlan),

    /// Multi-branch conditional (match/switch)
    BranchN(CoreBranchNPlan),

    /// Side effect (already lowered to ValueId)
    Effect(CoreEffectPlan),

    /// Control flow exit (Return/Break/Continue)
    Exit(CoreExitPlan),
}

/// Phase 273 P2: PHI information for generalized CoreLoopPlan
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct CorePhiInfo {
    /// Block where PHI is located
    pub block: BasicBlockId,
    /// Destination ValueId for PHI
    pub dst: ValueId,
    /// PHI inputs: (predecessor_block, value)
    pub inputs: Vec<(BasicBlockId, ValueId)>,
    /// Tag for debugging (e.g., "loop_carrier_i")
    pub tag: String,
}

/// Phase 273 P3: Loop plan with generalized fields (SSOT)
///
/// All fields are now REQUIRED (Option removed for structural SSOT).
/// Legacy fields (header_effects, step_effects, carriers) have been removed.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(in crate::mir::builder) struct CoreLoopPlan {
    // === Block IDs (pre-allocated by Normalizer) ===

    /// Preheader block (entry to loop)
    pub preheader_bb: BasicBlockId,

    /// Header block (loop condition check)
    pub header_bb: BasicBlockId,

    /// Body block (loop body start)
    pub body_bb: BasicBlockId,

    /// Step block (increment and back-edge)
    pub step_bb: BasicBlockId,

    /// After block (loop exit)
    pub after_bb: BasicBlockId,

    /// Found block (early exit on match, or same as after_bb for patterns without early exit)
    pub found_bb: BasicBlockId,

    // === Body (for lowerer body emission) ===

    /// Body plans (emitted in body_bb, can contain If/Exit)
    pub body: Vec<CorePlan>,

    // === Loop control (ValueId references for Frag) ===

    /// Loop condition (for header→body/after branch)
    pub cond_loop: ValueId,

    /// Match condition (for body→found/step branch)
    pub cond_match: ValueId,

    // === Phase 273 P3: Generalized fields (REQUIRED - SSOT) ===

    /// Block-level effects (generalized for multiple blocks)
    /// Order: SSOT - preheader, header, body, then, else, step (pattern dependent)
    pub block_effects: Vec<(BasicBlockId, Vec<CoreEffectPlan>)>,

    /// PHI information (generalized for multiple blocks)
    pub phis: Vec<CorePhiInfo>,

    /// Edge CFG fragment (generalized terminator structure)
    pub frag: Frag,

    /// Final values for variable_map update (after loop exit)
    pub final_values: Vec<(String, ValueId)>,
}

// Phase 273 P3: CoreCarrierInfo removed (replaced by CorePhiInfo)

/// Phase 273 P1: Conditional plan
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct CoreIfPlan {
    /// Condition (ValueId reference, not String!)
    pub condition: ValueId,

    /// Then branch plans
    pub then_plans: Vec<CorePlan>,

    /// Else branch plans (optional)
    pub else_plans: Option<Vec<CorePlan>>,
}

// Phase 29at P1: BranchN plan moved to branchn.rs (SSOT)

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
        dst: Option<ValueId>,   // P2: Option for void methods (push)
        object: ValueId,
        method: String,         // Method name only (OK as String)
        args: Vec<ValueId>,
        effects: EffectMask,    // P2: Side effect mask (PURE+Io or MUT)
    },
    /// Global/static call (box-level or free function)
    GlobalCall {
        dst: Option<ValueId>,
        func: String,
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
    ExitIf {
        cond: ValueId,
        exit: CoreExitPlan,
    },

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
}

/// Phase 273 P1: Exit plan (control flow exit)
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(in crate::mir::builder) enum CoreExitPlan {
    /// Return with optional value
    Return(Option<ValueId>),

    /// Break from loop
    Break(usize),

    /// Continue to next iteration
    Continue(usize),
}
