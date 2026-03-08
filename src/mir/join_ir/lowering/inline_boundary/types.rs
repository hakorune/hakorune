use crate::mir::join_ir::lowering::carrier_info::{CarrierInfo, CarrierRole, ExitReconnectMode};
use crate::mir::join_ir::lowering::condition_to_joinir::ConditionBinding;
use crate::mir::ValueId;
use std::collections::BTreeSet;

/// Explicit binding between JoinIR exit value and host variable
///
/// This structure formalizes the connection between a JoinIR exit PHI value
/// and the host variable it should update. This eliminates implicit assumptions
/// about which variable a ValueId represents.
///
/// # IfPhiJoin Example
///
/// For `loop(i < 3) { sum = sum + i; i = i + 1 }`:
///
/// ```text
/// LoopExitBinding {
///     carrier_name: "sum",
///     join_exit_value: ValueId(18),  // k_exit's return value (JoinIR-local)
///     host_slot: ValueId(5),          // variable_map["sum"] in host
/// }
/// ```
///
/// # Multi-Carrier Support (`loop_continue_only` and friends)
///
/// Multiple carriers can be represented as a vector:
///
/// ```text
/// vec![
///     LoopExitBinding { carrier_name: "sum", join_exit_value: ValueId(18), host_slot: ValueId(5), role: LoopState },
///     LoopExitBinding { carrier_name: "count", join_exit_value: ValueId(19), host_slot: ValueId(6), role: LoopState },
/// ]
/// ```
#[derive(Debug, Clone)]
pub struct LoopExitBinding {
    /// Carrier variable name (e.g., "sum", "count", "is_digit_pos")
    ///
    /// This is the variable name in the host's variable_map that should
    /// receive the exit value.
    pub carrier_name: String,

    /// JoinIR-side ValueId from k_exit (or exit parameter)
    ///
    /// This is the **JoinIR-local** ValueId that represents the exit value.
    /// It will be remapped when merged into the host function.
    pub join_exit_value: ValueId,

    /// Host-side variable_map slot to reconnect
    ///
    /// This is the host function's ValueId for the variable that should be
    /// updated with the exit PHI result.
    pub host_slot: ValueId,

    /// Phase 227: Role of this carrier (LoopState or ConditionOnly)
    ///
    /// Determines whether this carrier should participate in exit PHI:
    /// - LoopState: Needs exit PHI (value used after loop)
    /// - ConditionOnly: No exit PHI (only used in loop condition)
    pub role: CarrierRole,
}

/// Layout policy for JoinIR jump_args (SSOT)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JumpArgsLayout {
    /// jump_args = [carriers...]
    CarriersOnly,
    /// jump_args = [expr_result, carriers...]
    ExprResultPlusCarriers,
}

/// Boundary information for inlining a JoinIR fragment into a host function
///
/// This structure captures the "interface" between a JoinIR fragment and the
/// host function, allowing the merger to inject necessary Copy instructions
/// to connect the two SSA value spaces.
///
/// # Design Note
///
/// This is a **pure data structure** with no logic. All transformation logic
/// lives in the merger (merge_joinir_mir_blocks).
#[derive(Debug, Clone)]
pub struct JoinInlineBoundary {
    /// JoinIR-local ValueIds that act as "input slots"
    ///
    /// These are the ValueIds used **inside** the JoinIR fragment to refer
    /// to values that come from the host. They should be in the JoinValueSpace
    /// Param region (100-999). (They are typically allocated sequentially.)
    ///
    /// Example: For a loop variable `i`, JoinIR uses ValueId(100) as the parameter.
    pub join_inputs: Vec<ValueId>,

    /// Host-function ValueIds that provide the input values
    ///
    /// These are the ValueIds from the **host function** that correspond to
    /// the join_inputs. The merger will inject Copy instructions to connect
    /// host_inputs[i] → join_inputs[i].
    ///
    /// Example: If host has `i` as ValueId(4), then host_inputs = [ValueId(4)].
    pub host_inputs: Vec<ValueId>,

    /// Phase 255 P2: Loop invariant variables
    ///
    /// Variables that are referenced inside the loop body but do not change
    /// across iterations. These variables need header PHI nodes (with the same
    /// value from all incoming edges) but do NOT need exit PHI nodes.
    ///
    /// # Format
    ///
    /// Each entry is `(variable_name, host_value_id)`.
    pub loop_invariants: Vec<(String, ValueId)>,

    /// Explicit exit bindings for loop carriers (Phase 190+)
    ///
    /// Each binding explicitly names which variable is being updated and
    /// where the value comes from. This eliminates ambiguity and prepares
    /// for multi-carrier support.
    pub exit_bindings: Vec<LoopExitBinding>,

    /// Phase 171-fix: Condition bindings with explicit JoinIR ValueIds
    ///
    /// Each binding explicitly specifies:
    /// - Variable name
    /// - HOST ValueId (source)
    /// - JoinIR ValueId (destination)
    ///
    /// This replaces legacy condition-only input plumbing and ensures proper
    /// ValueId separation.
    pub condition_bindings: Vec<ConditionBinding>,

    /// Phase 33-14: Expression result ValueId (JoinIR-local)
    ///
    /// If the loop is used as an expression (like `return loop(...)`), this field
    /// contains the JoinIR-local ValueId of k_exit's return value.
    ///
    /// - `Some(ValueId)`: Loop returns a value → k_exit return goes to exit_phi_inputs
    /// - `None`: Loop only updates carriers → no exit_phi_inputs generation
    pub expr_result: Option<ValueId>,

    /// Phase 256 P1.12: jump_args layout (SSOT)
    ///
    /// This prevents merge from guessing whether jump_args contains a leading
    /// expr_result slot.
    pub jump_args_layout: JumpArgsLayout,

    /// Phase 33-16: Loop variable name (for LoopHeaderPhiBuilder)
    ///
    /// The name of the loop control variable (e.g., "i" in `loop(i < 3)`).
    /// Used to track which PHI corresponds to the loop variable.
    pub loop_var_name: Option<String>,

    /// Phase 287 P2: Loop header function name (SSOT)
    ///
    /// Merge must not guess the loop header function from "first non-main non-continuation".
    /// For loop routes, set this explicitly (typically "loop_step").
    pub loop_header_func_name: Option<String>,

    /// Phase 228: Carrier metadata (for header PHI generation)
    ///
    /// Contains full carrier information including initialization policies.
    /// This allows header PHI generation to handle ConditionOnly carriers
    /// with explicit bool initialization.
    pub carrier_info: Option<CarrierInfo>,

    /// Phase 132 P1: Continuation contract (SSOT)
    /// Phase 256 P1.7: Changed from BTreeSet<JoinFuncId> to BTreeSet<String>
    ///
    /// JoinIR merge must not infer/guess continuation functions. The router/lowerer
    /// must declare continuation function names here.
    pub continuation_func_ids: BTreeSet<String>,

    /// Phase 131 P1.5: Exit reconnection mode
    ///
    /// Controls whether exit values are reconnected via PHI generation (Phi)
    /// or direct variable_map update (DirectValue).
    pub exit_reconnect_mode: ExitReconnectMode,
}
