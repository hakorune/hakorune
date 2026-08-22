//! Generic G0 source-to-physical function-effect projection.
//!
//! The source parent remains the semantic authority.  This module only
//! projects its already-verified finite operation contract into the physical
//! function-header effect mask needed by a later skeleton; it creates no MIR,
//! `ValueId`, Builder state, or session effect.

use crate::mir::loop_recipe_contract::{LoopOperationV1, LoopRecipeItemV1};
use crate::mir::numeric_substrate::NumericTarget;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1,
    SemanticOwnerSourceKindV1, SourcePathSegmentV1,
};
use crate::mir::EffectMask;

use super::generic_g0_source_parent::{
    physical_emitter_source_parts::GenericG0PhysicalEmitterSourcePartsRejectV1,
    GenericG0SourceParentRefV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0PhysicalFunctionEffectRejectV1 {
    OwnerMismatch,
    OriginMismatch,
    SourceKindMismatch,
    BodyRootMismatch,
    FrameMismatch,
    ResultAbiMismatch,
    HeaderMismatch,
    StorageMismatch,
    SourceEffectMismatch,
    OperationCoverageMismatch,
    UnsupportedOperation,
}

/// Private physical header-effect projection for the bounded Generic G0
/// contract.  `EffectMask` is a realization detail, not a source authority.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericG0PhysicalFunctionEffectsV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    body_root: SourcePathSegmentV1,
    frame: LoopExecutionFrameKeyV1,
    target: NumericTarget,
    operation_count: u32,
    effect_mask: EffectMask,
}

impl VerifiedGenericG0PhysicalFunctionEffectsV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn origin(&self) -> FunctionOriginV1 {
        self.origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn body_root(&self) -> &SourcePathSegmentV1 {
        &self.body_root
    }

    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }

    pub(crate) const fn target(&self) -> NumericTarget {
        self.target
    }

    pub(crate) const fn operation_count(&self) -> u32 {
        self.operation_count
    }

    pub(crate) const fn effect_mask(&self) -> EffectMask {
        self.effect_mask
    }
}

/// Issue the one Generic-only physical effect projection from the retained
/// source parent.  All checks are pre-effect and the parent remains usable
/// after this borrowed, mechanical projection.
pub(crate) fn issue_generic_g0_physical_function_effects_v1<'loan, 'source>(
    parent: &GenericG0SourceParentRefV1<'loan, 'source>,
) -> Result<
    VerifiedGenericG0PhysicalFunctionEffectsV1,
    GenericG0PhysicalFunctionEffectRejectV1,
> {
    let parts = parent.physical_emitter_source_parts();
    parts
        .validate_shared_axes()
        .map_err(map_source_parts_reject)?;
    let context = parts.product().context();
    let source_effect = parts.function_effect();
    let result = parts.result_abi();
    let header = parts.declaration_header();
    let storage = parts.storage_lane();
    let operation_effect = parent.product().operation_effect();
    if header.return_type_name() != Some(result.abi().source_type_name())
        || result.abi().source_type_name() != "i64"
    {
        return Err(GenericG0PhysicalFunctionEffectRejectV1::ResultAbiMismatch);
    }
    if header.parameters().len() != storage.formals().len()
        || header.parameters().len() != parent.entries().len()
        || !header.metadata_is_empty()
    {
        return Err(GenericG0PhysicalFunctionEffectRejectV1::HeaderMismatch);
    }
    if storage.physical_callable_lane_count()
        != storage.receiver_lane_count() + storage.physical_formal_lane_count()
    {
        return Err(GenericG0PhysicalFunctionEffectRejectV1::StorageMismatch);
    }
    if source_effect.local_write_count() != 2 || source_effect.tail_return_count() != 1 {
        return Err(GenericG0PhysicalFunctionEffectRejectV1::SourceEffectMismatch);
    }

    let mut operation_count = 0u32;
    for row in &operation_effect.core().recipe().as_recipe().items {
        let LoopRecipeItemV1::Operation { operation } = &row.item else {
            continue;
        };
        operation_count = operation_count
            .checked_add(1)
            .ok_or(GenericG0PhysicalFunctionEffectRejectV1::OperationCoverageMismatch)?;
        if !is_mir_pure_generic_operation(*operation) {
            return Err(GenericG0PhysicalFunctionEffectRejectV1::UnsupportedOperation);
        }
    }
    if usize::try_from(operation_count).ok() != Some(operation_effect.evidence().len())
        || operation_count == 0
    {
        return Err(GenericG0PhysicalFunctionEffectRejectV1::OperationCoverageMismatch);
    }

    Ok(VerifiedGenericG0PhysicalFunctionEffectsV1 {
        owner: parent.owner(),
        origin: context.origin(),
        source_kind: context.source_kind(),
        body_root: source_effect.body_root().clone(),
        frame: context.frame().clone(),
        target: parent.product().target(),
        operation_count,
        effect_mask: EffectMask::PURE,
    })
}

fn map_source_parts_reject(
    reject: GenericG0PhysicalEmitterSourcePartsRejectV1,
) -> GenericG0PhysicalFunctionEffectRejectV1 {
    match reject {
        GenericG0PhysicalEmitterSourcePartsRejectV1::OwnerMismatch => {
            GenericG0PhysicalFunctionEffectRejectV1::OwnerMismatch
        }
        GenericG0PhysicalEmitterSourcePartsRejectV1::OriginMismatch => {
            GenericG0PhysicalFunctionEffectRejectV1::OriginMismatch
        }
        GenericG0PhysicalEmitterSourcePartsRejectV1::SourceKindMismatch => {
            GenericG0PhysicalFunctionEffectRejectV1::SourceKindMismatch
        }
        GenericG0PhysicalEmitterSourcePartsRejectV1::BodyRootMismatch => {
            GenericG0PhysicalFunctionEffectRejectV1::BodyRootMismatch
        }
        GenericG0PhysicalEmitterSourcePartsRejectV1::FrameMismatch => {
            GenericG0PhysicalFunctionEffectRejectV1::FrameMismatch
        }
    }
}

fn is_mir_pure_generic_operation(operation: LoopOperationV1) -> bool {
    matches!(
        operation,
        LoopOperationV1::ReadBinding { .. }
            | LoopOperationV1::ConstI64 { .. }
            | LoopOperationV1::BinaryI64 { .. }
            | LoopOperationV1::CompareI64 { .. }
            | LoopOperationV1::WriteBinding { .. }
    )
}

#[cfg(test)]
mod tests {
    use super::issue_generic_g0_physical_function_effects_v1;
    use crate::mir::compiler::generic_g0_source_parent::with_generic_g0_source_parent_v1;
    use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;
    use crate::mir::EffectMask;

    #[test]
    fn generic_physical_effect_projects_finite_operation_contract_without_effect() {
        let (unit, selection) = generic_source_unit_and_selection_for_test();
        let input = unit.root_function_input().expect("root input");
        let owner = input.owner();
        let result = with_generic_g0_source_parent_v1(input, selection, |parent| {
            let projection = issue_generic_g0_physical_function_effects_v1(&parent)?;
            assert_eq!(projection.owner(), owner);
            assert_eq!(projection.operation_count(), 15);
            assert_eq!(projection.effect_mask(), EffectMask::PURE);
            assert_eq!(projection.target(), parent.product().target());
            assert_eq!(projection.body_root(), parent.body_shape().body_root());
            assert_eq!(projection.frame(), parent.product().context().frame());
            // The projection is borrowed and mechanical; the source parent is
            // still available and no MirFunction/ValueId was created.
            assert_eq!(parent.owner(), owner);
            Ok::<(), super::GenericG0PhysicalFunctionEffectRejectV1>(())
        });
        let _ = result.expect("Generic physical effect projection");
    }
}
