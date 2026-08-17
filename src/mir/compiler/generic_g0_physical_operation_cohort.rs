//! Source-owned Generic operation cohort.
//!
//! The source parent is consumed exactly once at this boundary.  The neutral
//! operation program owns its demand/evidence, while the mechanical mapping
//! is lent only for the duration of `with_mapping`; no parent borrow or
//! self-reference is retained.

mod emitter_admission;
#[cfg(test)]
mod emitter_admission_tests;

pub(crate) use emitter_admission::{
    issue_generic_g0_physical_emitter_admission_v1,
    GenericG0PhysicalEmitterAdmissionRefV1, GenericG0PhysicalEmitterAdmissionRejectV1,
    PreparedGenericG0FunctionShellPlanV1, PreparedGenericG0PhysicalEmitterAdmissionV1,
};

use crate::mir::loop_recipe_contract::{
    LoopOperationPhysicalDemandRejectV1, PreparedLoopOperationProgramV1,
    VerifiedGenericRecipeProductG0,
};
use crate::mir::loop_route_policy::CanonicalLoopFamilySelectionV1;
use crate::mir::numeric_substrate::NumericTarget;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, VerifiedResolvedBodyShapeInventoryV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::generic_g0_physical_operation_mapping::{
    issue_generic_g0_physical_operation_mapping_from_program_v1,
    GenericG0PhysicalOperationMappingRejectV1, GenericG0PhysicalOperationMappingV1,
};
use super::generic_g0_source_parent::{
    issue_generic_g0_source_parent_v1, GenericG0SourceParentRejectV1,
    VerifiedGenericG0EntryBindingV1,
};
use super::generic_g0_function_effect::VerifiedGenericG0NoExternalEffectV1;
use super::generic_g0_result_abi::VerifiedGenericG0ResultAbiV1;
use super::generic_g0_storage_lane_source::VerifiedGenericG0StorageLaneSourceProjectionV1;
use super::generic_g0_top_level_declaration_header::
    VerifiedGenericG0TopLevelDeclarationHeaderV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0PhysicalOperationCohortRejectV1 {
    SourceParent(GenericG0SourceParentRejectV1),
    Program(LoopOperationPhysicalDemandRejectV1),
    OwnerMismatch,
    BodyRootMismatch,
    FrameMismatch,
}

/// One source-owned operation cohort.  The source siblings are kept as
/// independent owned fields so later consumers cannot retain a parent borrow
/// across the program ownership transition.
#[derive(Debug)]
pub(crate) struct GenericG0PhysicalOperationCohortV1<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    program: PreparedLoopOperationProgramV1,
    entries: Box<[VerifiedGenericG0EntryBindingV1]>,
    body_shape: &'source VerifiedResolvedBodyShapeInventoryV1,
    declaration_header: VerifiedGenericG0TopLevelDeclarationHeaderV1,
    function_effect: VerifiedGenericG0NoExternalEffectV1,
    result_abi: VerifiedGenericG0ResultAbiV1,
    storage_lane: VerifiedGenericG0StorageLaneSourceProjectionV1,
    completion: VerifiedFunctionCompletionV1,
    target: NumericTarget,
}

impl<'source> GenericG0PhysicalOperationCohortV1<'source> {
    pub(super) fn from_parent_parts(
        input: ResolvedFunctionLoweringInputV1<'source>,
        product: VerifiedGenericRecipeProductG0,
        entries: Box<[VerifiedGenericG0EntryBindingV1]>,
        body_shape: &'source VerifiedResolvedBodyShapeInventoryV1,
        declaration_header: VerifiedGenericG0TopLevelDeclarationHeaderV1,
        function_effect: VerifiedGenericG0NoExternalEffectV1,
        result_abi: VerifiedGenericG0ResultAbiV1,
        storage_lane: VerifiedGenericG0StorageLaneSourceProjectionV1,
        completion: VerifiedFunctionCompletionV1,
    ) -> Result<Self, GenericG0PhysicalOperationCohortRejectV1> {
        let target = product.target();
        let program = product
            .into_prepared_operation_program()
            .map_err(GenericG0PhysicalOperationCohortRejectV1::Program)?;
        let context = program.demand().context();
        if context.owner() != input.owner()
            || program.demand().operation_effect().core().owner() != input.owner()
        {
            return Err(GenericG0PhysicalOperationCohortRejectV1::OwnerMismatch);
        }
        if context.scope_region().scope().owner() != input.owner()
            || context.scope_region().region().owner() != input.owner()
        {
            return Err(GenericG0PhysicalOperationCohortRejectV1::OwnerMismatch);
        }
        if *body_shape.body_root() != input.function().root_profile().body_root() {
            return Err(GenericG0PhysicalOperationCohortRejectV1::BodyRootMismatch);
        }
        let Ok((source, _)) = input
            .function()
            .resolved_loop_source_context(context.loop_site())
        else {
            return Err(GenericG0PhysicalOperationCohortRejectV1::FrameMismatch);
        };
        if !context.frame().matches(&source.frame_key()) {
            return Err(GenericG0PhysicalOperationCohortRejectV1::FrameMismatch);
        }
        Ok(Self {
            input,
            program,
            entries,
            body_shape,
            declaration_header,
            function_effect,
            result_abi,
            storage_lane,
            completion,
            target,
        })
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.input.owner()
    }

    pub(crate) const fn target(&self) -> NumericTarget {
        self.target
    }

    pub(crate) fn entries(&self) -> &[VerifiedGenericG0EntryBindingV1] {
        &self.entries
    }

    pub(crate) fn declaration_header(&self) -> &VerifiedGenericG0TopLevelDeclarationHeaderV1 {
        &self.declaration_header
    }

    pub(crate) fn function_effect(&self) -> &VerifiedGenericG0NoExternalEffectV1 {
        &self.function_effect
    }

    pub(crate) fn result_abi(&self) -> &VerifiedGenericG0ResultAbiV1 {
        &self.result_abi
    }

    pub(crate) fn storage_lane(&self) -> &VerifiedGenericG0StorageLaneSourceProjectionV1 {
        &self.storage_lane
    }

    pub(crate) fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.completion
    }

    /// Consume the cohort once and lend its program plus one complete
    /// mechanical mapping.  Neither borrow can escape this callback and the
    /// mapping never becomes a second owner of operation meaning.
    pub(crate) fn with_mapping<R>(
        self,
        callback: impl for<'mapping> FnOnce(
            &PreparedLoopOperationProgramV1,
            &GenericG0PhysicalOperationMappingV1<'mapping>,
        ) -> R,
    ) -> Result<R, GenericG0PhysicalOperationMappingRejectV1> {
        let mapping = issue_generic_g0_physical_operation_mapping_from_program_v1(&self.program)?;
        Ok(callback(&self.program, &mapping))
    }
}

/// Consume the source parent once and lend the resulting cohort through a
/// single callback.  No Builder/session or MIR effect is allowed here.
pub(crate) fn with_generic_g0_physical_operation_cohort_v1<'source, R>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    selection: CanonicalLoopFamilySelectionV1,
    callback: impl FnOnce(GenericG0PhysicalOperationCohortV1<'source>) -> R,
) -> Result<R, GenericG0PhysicalOperationCohortRejectV1> {
    let parent = issue_generic_g0_source_parent_v1(input, selection)
        .map_err(GenericG0PhysicalOperationCohortRejectV1::SourceParent)?;
    let cohort = parent.into_physical_operation_cohort()?;
    Ok(callback(cohort))
}

#[cfg(test)]
mod tests {
    use super::with_generic_g0_physical_operation_cohort_v1;
    use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;

    #[test]
    fn owns_program_and_lends_mapping_only_inside_callback() {
        let (unit, selection) = generic_source_unit_and_selection_for_test();
        let input = unit.root_function_input().expect("root input");
        let result = with_generic_g0_physical_operation_cohort_v1(input, selection, |cohort| {
            assert_eq!(cohort.entries().len(), 2);
            let owner = cohort.owner();
            cohort
                .with_mapping(|program, mapping| {
                    assert_eq!(program.coverage().operation_count(), 15);
                    assert_eq!(mapping.owner(), owner);
                    assert_eq!(mapping.operation_count(), 15);
                })
                .expect("transient mapping");
        });
        result.expect("Generic operation cohort");
    }
}
