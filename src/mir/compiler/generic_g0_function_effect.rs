//! Source-backed Generic G0 function-effect admission.
//!
//! The receipt below is deliberately weaker than a MIR `EffectMask`.  It is
//! issued from the resolver products before the Generic selection is consumed
//! and records only the bounded fact needed by the next physical-entry row:
//! this function has the exact local-write/tail-return shape and no external
//! effect.  It owns no physical ID, ValueId, block, or runtime behavior.

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::generic_g0_top_level_declaration_header::
    VerifiedGenericG0TopLevelDeclarationHeaderV1;
use crate::mir::loop_structural_facts::generic_g0::VerifiedGenericStructuralFactsG0;
use crate::mir::resolved_semantics::{
    BodyEffectKindV1, BodyStatementShapeV1, FunctionOriginV1, ResolvedAssignmentTargetV1,
    ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitSiteV1,
    SemanticOwnerSourceKindV1, SourcePathSegmentV1, VerifiedResolvedBodyShapeInventoryV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0FunctionEffectRejectV1 {
    OwnerMismatch,
    OriginMismatch,
    SourceKindMismatch,
    BodyRootMismatch,
    RootFrameMismatch,
    MetadataNotEmpty,
    BodyEffectCountMismatch,
    UnexpectedBodyEffect(BodyEffectKindV1),
    BodyEffectSiteMismatch,
    DirectCallPresent,
    MethodCallPresent,
    AssignmentCountMismatch,
    AssignmentTargetMismatch,
    AssignmentNotLocal,
    ExitCountMismatch,
    ExitNotTailReturn,
    ReturnStatementMismatch,
}

/// Opaque source receipt for the bounded Generic G0 effect shape.
///
/// The source rows are checked at issuance and are intentionally not copied
/// here.  Later consumers must retain the source parent/cohort that issued
/// this receipt; they may not reconstruct its meaning from the count.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericG0NoExternalEffectV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    body_root: SourcePathSegmentV1,
    root_frame: crate::mir::resolved_semantics::LoopExecutionFrameKeyV1,
    local_write_count: u8,
    tail_return_count: u8,
}

impl VerifiedGenericG0NoExternalEffectV1 {
    pub(crate) const fn owner(
        &self,
    ) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
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

    pub(crate) fn root_frame(
        &self,
    ) -> &crate::mir::resolved_semantics::LoopExecutionFrameKeyV1 {
        &self.root_frame
    }

    pub(crate) const fn local_write_count(&self) -> u8 {
        self.local_write_count
    }

    pub(crate) const fn tail_return_count(&self) -> u8 {
        self.tail_return_count
    }
}

pub(crate) fn issue_generic_g0_no_external_effect_v1(
    input: &ResolvedFunctionLoweringInputV1<'_>,
    body_shape: &VerifiedResolvedBodyShapeInventoryV1,
    declaration_header: &VerifiedGenericG0TopLevelDeclarationHeaderV1,
    structural: &VerifiedGenericStructuralFactsG0,
) -> Result<VerifiedGenericG0NoExternalEffectV1, GenericG0FunctionEffectRejectV1> {
    let function = input.function();
    if body_shape.owner() != input.owner() {
        return Err(GenericG0FunctionEffectRejectV1::OwnerMismatch);
    }
    if function.function_origin() != structural.origin() {
        return Err(GenericG0FunctionEffectRejectV1::OriginMismatch);
    }
    if function.source_kind() != structural.source_kind()
        || declaration_header.source_kind() != function.source_kind()
    {
        return Err(GenericG0FunctionEffectRejectV1::SourceKindMismatch);
    }
    if body_shape.body_root() != &function.root_profile().body_root() {
        return Err(GenericG0FunctionEffectRejectV1::BodyRootMismatch);
    }
    let (root_source, _) = function
        .resolved_loop_source_context(structural.root_loop())
        .map_err(|_| GenericG0FunctionEffectRejectV1::RootFrameMismatch)?;
    if structural.owner() != input.owner()
        || !structural.root_frame().matches(&root_source.frame_key())
    {
        return Err(GenericG0FunctionEffectRejectV1::RootFrameMismatch);
    }
    if !declaration_header.metadata_is_empty() {
        return Err(GenericG0FunctionEffectRejectV1::MetadataNotEmpty);
    }

    let expected_targets = [
        structural.outer_update().target.clone(),
        structural.inner_update().target.clone(),
    ];
    if body_shape.effects().len() != expected_targets.len() {
        return Err(GenericG0FunctionEffectRejectV1::BodyEffectCountMismatch);
    }
    for effect in body_shape.effects() {
        if effect.kind != BodyEffectKindV1::Write {
            return Err(GenericG0FunctionEffectRejectV1::UnexpectedBodyEffect(effect.kind));
        }
        if !expected_targets.iter().any(|target| target == &effect.site) {
            return Err(GenericG0FunctionEffectRejectV1::BodyEffectSiteMismatch);
        }
    }
    if expected_targets[0] == expected_targets[1] {
        return Err(GenericG0FunctionEffectRejectV1::BodyEffectSiteMismatch);
    }
    if function.direct_call_targets().next().is_some() {
        return Err(GenericG0FunctionEffectRejectV1::DirectCallPresent);
    }
    if function.method_calls().next().is_some() {
        return Err(GenericG0FunctionEffectRejectV1::MethodCallPresent);
    }

    let mut assignment_count = 0usize;
    for (site, target) in function.assignment_targets() {
        assignment_count += 1;
        let ResolvedAssignmentTargetV1::BindingRebind(binding) = target else {
            return Err(GenericG0FunctionEffectRejectV1::AssignmentNotLocal);
        };
        let matches_expected = (*site == expected_targets[0]
            && *binding == structural.outer_update().binding)
            || (*site == expected_targets[1] && *binding == structural.inner_update().binding);
        if !matches_expected {
            return Err(GenericG0FunctionEffectRejectV1::AssignmentTargetMismatch);
        }
    }
    if assignment_count != expected_targets.len() {
        return Err(GenericG0FunctionEffectRejectV1::AssignmentCountMismatch);
    }

    let exits = function.resolved_exits().collect::<Vec<_>>();
    if exits.len() != 1 {
        return Err(GenericG0FunctionEffectRejectV1::ExitCountMismatch);
    }
    let (exit_site, exit) = exits[0];
    if exit.origin() != ResolvedExitOriginV1::ExplicitReturn
        || !matches!(
            exit.transfer(),
            ResolvedControlTransferV1::Return { target_function }
                if target_function == function.function_region()
        )
    {
        return Err(GenericG0FunctionEffectRejectV1::ExitNotTailReturn);
    }
    let ResolvedExitSiteV1::Statement(return_site) = exit_site else {
        return Err(GenericG0FunctionEffectRejectV1::ExitNotTailReturn);
    };
    let return_count = body_shape
        .statements()
        .iter()
        .filter_map(|statement| match statement {
            BodyStatementShapeV1::Return { site, .. } => Some(site),
            BodyStatementShapeV1::SequenceItem { .. } => None,
        })
        .filter(|site| *site == return_site)
        .count();
    if return_count != 1 {
        return Err(GenericG0FunctionEffectRejectV1::ReturnStatementMismatch);
    }

    Ok(VerifiedGenericG0NoExternalEffectV1 {
        owner: input.owner(),
        origin: function.function_origin(),
        source_kind: function.source_kind(),
        body_root: body_shape.body_root().clone(),
        root_frame: structural.root_frame().clone(),
        local_write_count: 2,
        tail_return_count: 1,
    })
}
