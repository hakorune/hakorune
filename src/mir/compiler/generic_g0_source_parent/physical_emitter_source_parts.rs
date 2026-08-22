//! Borrowed source facts shared by the Generic physical-emitter canaries.
//!
//! This is deliberately a validator transport, not another semantic receipt.
//! It borrows every sibling from one already-sealed source parent so entry,
//! effect, and shell validators cannot accidentally re-pair independently
//! issued products.  It owns no physical IDs, MIR objects, Builder state, or
//! session/lifecycle state.

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::generic_g0_function_effect::VerifiedGenericG0NoExternalEffectV1;
use crate::mir::compiler::generic_g0_result_abi::VerifiedGenericG0ResultAbiV1;
use crate::mir::compiler::generic_g0_storage_lane_source::
    VerifiedGenericG0StorageLaneSourceProjectionV1;
use crate::mir::compiler::generic_g0_top_level_declaration_header::
    VerifiedGenericG0TopLevelDeclarationHeaderV1;
use crate::mir::loop_recipe_contract::VerifiedGenericRecipeProductG0;
use crate::mir::resolved_semantics::VerifiedResolvedBodyShapeInventoryV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;

use super::{VerifiedGenericG0EntryBindingV1, VerifiedGenericG0SourceParentV1};

/// Borrowed sibling view issued only from one source parent.
pub(in crate::mir::compiler) struct GenericG0PhysicalEmitterSourcePartsRef<
    'parts,
    'source,
> {
    input: &'parts ResolvedFunctionLoweringInputV1<'source>,
    product: &'parts VerifiedGenericRecipeProductG0,
    entries: &'parts [VerifiedGenericG0EntryBindingV1],
    body_shape: &'parts VerifiedResolvedBodyShapeInventoryV1,
    declaration_header: &'parts VerifiedGenericG0TopLevelDeclarationHeaderV1,
    function_effect: &'parts VerifiedGenericG0NoExternalEffectV1,
    result_abi: &'parts VerifiedGenericG0ResultAbiV1,
    storage_lane: &'parts VerifiedGenericG0StorageLaneSourceProjectionV1,
    completion: &'parts VerifiedFunctionCompletionV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::compiler) enum GenericG0PhysicalEmitterSourcePartsRejectV1 {
    OwnerMismatch,
    OriginMismatch,
    SourceKindMismatch,
    BodyRootMismatch,
    FrameMismatch,
}

impl<'parts, 'source> GenericG0PhysicalEmitterSourcePartsRef<'parts, 'source> {
    pub(super) fn from_parent(parent: &'parts VerifiedGenericG0SourceParentV1<'source>) -> Self {
        Self {
            input: &parent.input,
            product: &parent.product,
            entries: &parent.entries,
            body_shape: parent.body_shape,
            declaration_header: &parent.declaration_header,
            function_effect: &parent.function_effect,
            result_abi: &parent.result_abi,
            storage_lane: &parent.storage_lane,
            completion: &parent.completion,
        }
    }

    pub(in crate::mir::compiler) fn input(
        &self,
    ) -> &'parts ResolvedFunctionLoweringInputV1<'source> {
        self.input
    }

    pub(in crate::mir::compiler) fn product(&self) -> &'parts VerifiedGenericRecipeProductG0 {
        self.product
    }

    pub(in crate::mir::compiler) fn entries(&self) -> &'parts [VerifiedGenericG0EntryBindingV1] {
        self.entries
    }

    pub(in crate::mir::compiler) fn body_shape(
        &self,
    ) -> &'parts VerifiedResolvedBodyShapeInventoryV1 {
        self.body_shape
    }

    pub(in crate::mir::compiler) fn declaration_header(
        &self,
    ) -> &'parts VerifiedGenericG0TopLevelDeclarationHeaderV1 {
        self.declaration_header
    }

    pub(in crate::mir::compiler) fn function_effect(
        &self,
    ) -> &'parts VerifiedGenericG0NoExternalEffectV1 {
        self.function_effect
    }

    pub(in crate::mir::compiler) fn result_abi(&self) -> &'parts VerifiedGenericG0ResultAbiV1 {
        self.result_abi
    }

    pub(in crate::mir::compiler) fn storage_lane(
        &self,
    ) -> &'parts VerifiedGenericG0StorageLaneSourceProjectionV1 {
        self.storage_lane
    }

    pub(in crate::mir::compiler) fn completion(&self) -> &'parts VerifiedFunctionCompletionV1 {
        self.completion
    }

    /// Validate only the shared source identity axes.  Variant-specific
    /// descriptor/effect/shell checks remain with their existing owners.
    pub(in crate::mir::compiler) fn validate_shared_axes(
        &self,
    ) -> Result<(), GenericG0PhysicalEmitterSourcePartsRejectV1> {
        let input = self.input;
        let owner = input.owner();
        if self.product.context().owner() != owner
            || self.declaration_header.owner() != owner
            || self.function_effect.owner() != owner
            || self.result_abi.owner() != owner
            || self.storage_lane.owner() != owner
            || self.completion.owner() != owner
            || self.body_shape.owner() != owner
        {
            return Err(GenericG0PhysicalEmitterSourcePartsRejectV1::OwnerMismatch);
        }

        let origin = input.function().function_origin();
        if self.product.context().origin() != origin
            || self.declaration_header.origin() != origin
            || self.function_effect.origin() != origin
            || self.result_abi.origin() != origin
            || self.storage_lane.origin() != origin
        {
            return Err(GenericG0PhysicalEmitterSourcePartsRejectV1::OriginMismatch);
        }

        let source_kind = input.function().source_kind();
        if self.product.context().source_kind() != source_kind
            || self.declaration_header.source_kind() != source_kind
            || self.function_effect.source_kind() != source_kind
            || self.result_abi.source_kind() != source_kind
            || self.storage_lane.source_kind() != source_kind
        {
            return Err(GenericG0PhysicalEmitterSourcePartsRejectV1::SourceKindMismatch);
        }

        if self.function_effect.body_root() != self.storage_lane.body_root()
            || self.function_effect.body_root() != self.body_shape.body_root()
        {
            return Err(GenericG0PhysicalEmitterSourcePartsRejectV1::BodyRootMismatch);
        }

        let frame = self.product.context().frame();
        if !self.function_effect.root_frame().matches(frame)
            || !self.storage_lane.frame().matches(frame)
        {
            return Err(GenericG0PhysicalEmitterSourcePartsRejectV1::FrameMismatch);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::GenericG0PhysicalEmitterSourcePartsRejectV1;
    use crate::mir::compiler::generic_g0_source_parent::with_generic_g0_source_parent_v1;
    use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;

    #[test]
    fn source_parts_borrow_one_complete_parent_without_repairing_siblings() {
        let (unit, selection) = generic_source_unit_and_selection_for_test();
        let input = unit.root_function_input().expect("root input");
        let result = with_generic_g0_source_parent_v1(input, selection, |parent| {
            let parts = parent.physical_emitter_source_parts();
            assert_eq!(parts.input().owner(), parent.owner());
            assert_eq!(parts.product().context().owner(), parent.owner());
            assert_eq!(parts.entries().len(), 2);
            assert_eq!(parts.body_shape().owner(), parent.owner());
            assert_eq!(parts.declaration_header().owner(), parent.owner());
            assert_eq!(parts.function_effect().owner(), parent.owner());
            assert_eq!(parts.result_abi().owner(), parent.owner());
            assert_eq!(parts.storage_lane().owner(), parent.owner());
            assert_eq!(parts.completion().owner(), parent.owner());
            assert!(parts.validate_shared_axes().is_ok());
            Ok::<(), GenericG0PhysicalEmitterSourcePartsRejectV1>(())
        });
        result.expect("source parent").expect("source parts");
    }
}
