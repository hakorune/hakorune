//! Read-only initialized input admission for the canonical V1 draft.

use super::super::ids::{LoopBindingKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::super::input_source::LoopInitializedLocalInputSourceRelationV1;
use super::super::schema::LoopRecipeReadOnlyInputV1;
use super::{LoopRecipeDraftRejectV1, LoopRecipeDraftV1};
use crate::mir::resolved_semantics::SourceExprSiteV1;

impl LoopRecipeDraftV1 {
    /// Record a root input that may be read but has no carrier or After port.
    /// The semantic verifier rejects writes and overlap with carried inputs.
    pub(crate) fn read_only_input(
        &mut self,
        owner_loop: LoopNodeKeyV1,
        binding: LoopBindingKeyV1,
        initializer: SourceExprSiteV1,
    ) -> Result<LoopValueKeyV1, LoopRecipeDraftRejectV1> {
        self.loop_row(owner_loop)?;
        let class = self.binding_class(binding)?;
        let value = self.fresh_value(class);
        self.inputs.push(value);
        self.read_only_inputs.push(LoopRecipeReadOnlyInputV1 {
            owner_loop,
            binding,
            class,
            entry_value: value,
        });
        let (source_binding, declaration) = self.binding_sources[binding.raw() as usize].clone();
        self.input_relations
            .push(LoopInitializedLocalInputSourceRelationV1::new(
                declaration,
                initializer,
                source_binding,
                value,
                class,
            ));
        Ok(value)
    }
}
