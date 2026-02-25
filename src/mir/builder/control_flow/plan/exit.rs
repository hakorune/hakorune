use crate::mir::ValueId;

/// Phase 273 P1: Exit plan (control flow exit)
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(in crate::mir::builder) enum CoreExitPlan {
    /// Return with optional value
    Return(Option<ValueId>),

    /// Break from loop
    Break(usize),

    /// Break from loop, providing per-edge PHI inputs for the loop's after-bb merge.
    ///
    /// This is used by strict/dev-only planner-required plans to preserve updated carrier values
    /// on early-break exits without rewriting execution order.
    ///
    /// Each entry is `(phi_dst, value_on_this_edge)`.
    BreakWithPhiArgs {
        depth: usize,
        phi_args: Vec<(ValueId, ValueId)>,
    },

    /// Continue to next iteration
    Continue(usize),

    /// Continue to next iteration, providing per-edge PHI inputs for the loop's step-join.
    ///
    /// This is used by strict/dev-only planner-required plans to merge carrier values
    /// across multiple early-continue edges without rewriting execution order.
    ///
    /// Each entry is `(phi_dst, value_on_this_edge)`.
    ContinueWithPhiArgs {
        depth: usize,
        phi_args: Vec<(ValueId, ValueId)>,
    },
}
