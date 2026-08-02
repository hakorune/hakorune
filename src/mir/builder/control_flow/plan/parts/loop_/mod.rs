//! Loop parts (scaffold).
//!
//! Purpose (L0):
//! - Provide a Parts entry for lowering a loop body represented as `RecipeBlock`.
//! - Keep the contract explicit and fail-fast (no silent fallback).
//!
//! NOTE:
//! - This is an implementation-prep step. Producers are unchanged.

mod analysis;
mod body_block;
mod debug;
mod final_values;
mod loop_v0;
mod nested_depth1;
mod vars;

#[cfg(test)]
pub(crate) mod test_probe {
    use std::cell::Cell;

    thread_local! {
        static LOOP_PHYSICAL_EFFECTS: Cell<usize> = const { Cell::new(0) };
    }

    pub(crate) fn reset() {
        LOOP_PHYSICAL_EFFECTS.with(|count| count.set(0));
    }

    pub(crate) fn record_after_frame() {
        LOOP_PHYSICAL_EFFECTS.with(|count| count.set(count.get() + 1));
    }

    pub(crate) fn take() -> usize {
        LOOP_PHYSICAL_EFFECTS.with(|count| count.replace(0))
    }
}

pub(in crate::mir::builder) type LoopBodyContractKind =
    crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

#[allow(unused_imports)]
pub(in crate::mir::builder) use body_block::lower_loop_with_body_block;
pub(in crate::mir::builder) use final_values::apply_loop_final_values_to_bindings;
#[allow(unused_imports)]
pub(in crate::mir::builder) use loop_v0::lower_loop_v0;
#[allow(unused_imports)]
pub(in crate::mir::builder) use nested_depth1::{
    lower_nested_loop_depth1_stmt_only, try_lower_nested_loop_depth1_stmt_only_fastpath,
};
