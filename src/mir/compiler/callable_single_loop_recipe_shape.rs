//! Test-only parity wrapper for the production callable Recipe shape.

use crate::mir::loop_recipe_contract::LoopRecipeV1;

pub(super) fn callable_recipe() -> LoopRecipeV1 {
    crate::mir::compiler::callable_single_loop_recipe::canonical_callable_single_loop_recipe_v1()
}
