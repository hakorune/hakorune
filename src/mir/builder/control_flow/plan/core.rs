use super::branchn::CoreBranchNPlan;
use super::effect::CoreEffectPlan;
use super::exit::CoreExitPlan;
use crate::mir::builder::control_flow::edgecfg::api::Frag;
use crate::mir::{BasicBlockId, ValueId};

/// Phase 273 P1: CorePlan - Fixed vocabulary plan (structure nodes only)
///
/// CorePlan expressions use **ValueId references only** (no String parsing).
/// This prevents "second language processor" from growing inside Lowerer.
#[derive(Debug, Clone)]
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

/// M6 (CorePlan shrink): mechanical alias used by Recipe/Parts boundaries.
///
/// This keeps behavior unchanged while making the "semantic recipe" vs
/// "mechanical lowered form" boundary explicit.
pub(in crate::mir::builder) type LoweredRecipe = CorePlan;

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

/// Phase 29bq P2.5: Loop step placement (SSOT)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopStepMode {
    /// Extract step into step_bb (legacy/default)
    ExtractToStepBb,
    /// Inline step in body (step_bb is empty)
    InlineInBody,
}

/// Phase 273 P3: Loop plan with generalized fields (SSOT)
///
/// All fields are now REQUIRED (Option removed for structural SSOT).
/// Legacy fields (header_effects, step_effects, carriers) have been removed.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct CoreLoopPlan {
    // === Block IDs (pre-allocated by Normalizer) ===
    /// Preheader block (entry to loop)
    pub preheader_bb: BasicBlockId,
    /// Whether preheader is a fresh block (do not remap to current block)
    pub preheader_is_fresh: bool,

    /// Header block (loop condition check)
    pub header_bb: BasicBlockId,

    /// Body block (loop body start)
    pub body_bb: BasicBlockId,

    /// Step block (increment and back-edge)
    pub step_bb: BasicBlockId,

    /// Continue target (default: step_bb)
    pub continue_target: BasicBlockId,

    /// After block (loop exit)
    pub after_bb: BasicBlockId,

    /// Found block (early exit on match, or same as after_bb for patterns without early exit)
    pub found_bb: BasicBlockId,

    // === Body (for lowerer body emission) ===
    /// Body plans (emitted in body_bb, can contain If/Exit)
    pub body: Vec<LoweredRecipe>,

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

    /// Loop step placement (ExtractToStepBb / InlineInBody)
    pub step_mode: LoopStepMode,

    /// Whether this loop has an explicit step statement/effect in loop body semantics.
    /// S1 marker only (behavior-neutral): verifier V10c/V10d wiring uses this in follow-up.
    pub has_explicit_step: bool,
}

/// Phase 273 P1: Conditional plan
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct CoreIfPlan {
    /// Condition (ValueId reference, not String!)
    pub condition: ValueId,

    /// Then branch plans
    pub then_plans: Vec<LoweredRecipe>,

    /// Else branch plans (optional)
    pub else_plans: Option<Vec<LoweredRecipe>>,

    /// If-merge joins (optional; used to emit PHI at merge block)
    pub joins: Vec<CoreIfJoin>,
}

/// If-merge join entry (merge value from then/else into dst)
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct CoreIfJoin {
    pub name: String,
    pub dst: ValueId,
    /// Pre-if value for this name (when known). Used for analysis-only checks.
    ///
    /// Some(pre): join comes from 3-map diff (join_payload SSOT).
    /// None: synthetic join (e.g. short-circuit internal joins); pre is not meaningful.
    pub pre_val: Option<ValueId>,
    pub then_val: ValueId,
    pub else_val: ValueId,
}
