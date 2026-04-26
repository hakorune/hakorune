//! Baseline if-only normalized-shadow entry.
//!
//! This facade lets callers depend on route intent instead of the legacy
//! storage module. The implementation delegates until the physical move lands.

use crate::mir::control_tree::normalized_shadow::env_layout::EnvLayout;
use crate::mir::control_tree::normalized_shadow::legacy::LegacyLowerer;
use crate::mir::control_tree::step_tree::StepTree;
use crate::mir::join_ir::lowering::carrier_info::JoinFragmentMeta;
use crate::mir::join_ir::JoinModule;

pub(crate) fn lower_if_only_to_normalized(
    step_tree: &StepTree,
    env_layout: &EnvLayout,
) -> Result<Option<(JoinModule, JoinFragmentMeta)>, String> {
    LegacyLowerer::lower_if_only_to_normalized(step_tree, env_layout)
}
