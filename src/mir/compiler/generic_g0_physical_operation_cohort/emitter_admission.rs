//! One-shot Generic G0 source/prephysical emitter admission.
//!
//! One source parent supplies every sibling.  The resulting owner retains a
//! neutral program/layout, a declaration-only shell plan, resolver control,
//! canonical Completion, target, and one full mechanical stamp.  Actual
//! function state and instruction identities are deliberately absent.

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::generic_g0_physical_entry_admission::{
    issue_generic_g0_entry_control_facts_v1, GenericG0DetachedEntryCanaryRejectV1,
    PreparedGenericG0EntryControlFactsV1,
};
use crate::mir::compiler::generic_g0_physical_function_effect::{
    issue_generic_g0_physical_function_effects_v1,
    GenericG0PhysicalFunctionEffectRejectV1, VerifiedGenericG0PhysicalFunctionEffectsV1,
};
use crate::mir::compiler::generic_g0_physical_function_entry_input::{
    issue_generic_g0_physical_function_entry_input_v1,
    GenericG0PhysicalFunctionEntryRejectV1, GenericG0PhysicalParameterDescriptorV1,
};
use crate::mir::compiler::generic_g0_physical_function_skeleton::{
    validate_generic_g0_physical_function_shell_facts,
    GenericG0PhysicalFunctionSkeletonRejectV1,
};
use crate::mir::compiler::generic_g0_physical_operation_mapping::{
    issue_generic_g0_physical_operation_mapping_from_program_v1,
    GenericG0PhysicalOperationMappingRejectV1, GenericG0PhysicalOperationMappingV1,
};
use crate::mir::compiler::generic_g0_source_parent::{
    issue_generic_g0_source_parent_v1, GenericG0SourceParentRejectV1,
};
use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::loop_recipe_contract::{
    LoopPhysicalLayoutRejectV1, PreparedLoopPhysicalLayoutV1,
};
use crate::mir::loop_route_policy::CanonicalLoopFamilySelectionV1;
use crate::mir::numeric_substrate::NumericTarget;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{
    CanonicalCallableSymbolV1, FunctionOriginV1, FunctionOwnerIdV1,
    LoopExecutionFrameKeyV1, RegionId, ResolvedScopeRegionPairV1,
    SemanticOwnerSourceKindV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

use super::{GenericG0PhysicalOperationCohortRejectV1, GenericG0PhysicalOperationCohortV1};
use super::super::generic_g0_source_parent::VerifiedGenericG0EntryBindingV1;

const GENERIC_G0_OPERATION_PROGRAM_REVISION_V1: u16 = 1;
const GENERIC_G0_PHYSICAL_LAYOUT_REVISION_V1: u16 = 1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0PhysicalEmitterAdmissionRejectV1 {
    SourceParent(GenericG0SourceParentRejectV1),
    Entry(GenericG0PhysicalFunctionEntryRejectV1),
    Effect(GenericG0PhysicalFunctionEffectRejectV1),
    Shell(GenericG0PhysicalFunctionSkeletonRejectV1),
    Control(GenericG0DetachedEntryCanaryRejectV1),
    Cohort(GenericG0PhysicalOperationCohortRejectV1),
    Mapping(GenericG0PhysicalOperationMappingRejectV1),
    Layout(LoopPhysicalLayoutRejectV1),
    CountOverflow,
    CohortDrift,
}

/// Declaration-only physical shell plan.  It owns no executable function
/// state and reserves no instruction identity.
#[derive(Debug)]
pub(crate) struct PreparedGenericG0FunctionShellPlanV1 {
    symbol: CanonicalCallableSymbolV1,
    descriptors: Box<[GenericG0PhysicalParameterDescriptorV1]>,
    result_abi: ExactTrivialReturnAbiV1,
    effects: VerifiedGenericG0PhysicalFunctionEffectsV1,
}

impl PreparedGenericG0FunctionShellPlanV1 {
    pub(crate) fn symbol(&self) -> &CanonicalCallableSymbolV1 {
        &self.symbol
    }

    pub(crate) fn descriptors(&self) -> &[GenericG0PhysicalParameterDescriptorV1] {
        &self.descriptors
    }

    pub(crate) const fn result_abi(&self) -> ExactTrivialReturnAbiV1 {
        self.result_abi
    }

    pub(crate) fn effects(&self) -> &VerifiedGenericG0PhysicalFunctionEffectsV1 {
        &self.effects
    }
}

#[derive(Debug)]
struct GenericG0PhysicalEmitterCohortStampV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    body_root: SourcePathSegmentV1,
    loop_site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    completion_target: RegionId,
    source_logical_arity: u32,
    receiver_lane_count: u32,
    physical_callable_lane_count: u32,
    operation_count: u32,
    layout_item_count: u32,
    layout_segment_count: u32,
    target: NumericTarget,
    program_revision: u16,
    layout_revision: u16,
}

#[derive(Debug)]
struct VerifiedGenericG0PhysicalLayoutBindingV1 {
    layout: PreparedLoopPhysicalLayoutV1,
    stamp: GenericG0PhysicalEmitterCohortStampV1,
}

/// Complete prephysical admission.  It is non-Clone and exposes only scoped
/// borrowed views, preventing independently owned siblings from being paired.
pub(crate) struct PreparedGenericG0PhysicalEmitterAdmissionV1<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    /// Source-backed LoopValueKey/BindingRef rows retained for the later
    /// canonical preheader read. These rows belong to the same one-shot
    /// admission as the program/layout; downstream must not reconstruct them
    /// from arity, operation counts, or physical identifiers.
    entries: Box<[VerifiedGenericG0EntryBindingV1]>,
    layout_binding: VerifiedGenericG0PhysicalLayoutBindingV1,
    shell_plan: PreparedGenericG0FunctionShellPlanV1,
    control: PreparedGenericG0EntryControlFactsV1,
    completion: VerifiedFunctionCompletionV1,
}

pub(crate) struct GenericG0PhysicalEmitterAdmissionRefV1<'loan, 'source> {
    admission: &'loan PreparedGenericG0PhysicalEmitterAdmissionV1<'source>,
}

impl<'loan, 'source> GenericG0PhysicalEmitterAdmissionRefV1<'loan, 'source> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.admission.layout_binding.stamp.owner
    }

    pub(crate) const fn target(&self) -> NumericTarget {
        self.admission.layout_binding.stamp.target
    }

    pub(crate) fn layout(&self) -> &PreparedLoopPhysicalLayoutV1 {
        &self.admission.layout_binding.layout
    }

    pub(crate) fn shell_plan(&self) -> &PreparedGenericG0FunctionShellPlanV1 {
        &self.admission.shell_plan
    }

    pub(crate) fn control(&self) -> &PreparedGenericG0EntryControlFactsV1 {
        &self.admission.control
    }

    pub(crate) fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.admission.completion
    }

    pub(crate) fn entries(&self) -> &[VerifiedGenericG0EntryBindingV1] {
        &self.admission.entries
    }

    pub(crate) const fn program_revision(&self) -> u16 {
        self.admission.layout_binding.stamp.program_revision
    }

    pub(crate) const fn layout_revision(&self) -> u16 {
        self.admission.layout_binding.stamp.layout_revision
    }
}

impl<'source> PreparedGenericG0PhysicalEmitterAdmissionV1<'source> {
    pub(crate) fn consume<R>(
        self,
        callback: impl for<'loan> FnOnce(GenericG0PhysicalEmitterAdmissionRefV1<'loan, 'source>) -> R,
    ) -> R {
        callback(GenericG0PhysicalEmitterAdmissionRefV1 { admission: &self })
    }

    pub(crate) fn with_mapping<R>(
        self,
        callback: impl for<'loan> FnOnce(
            GenericG0PhysicalEmitterAdmissionRefV1<'loan, 'source>,
            GenericG0PhysicalOperationMappingV1<'loan>,
        ) -> R,
    ) -> Result<R, GenericG0PhysicalOperationMappingRejectV1> {
        let mapping = issue_generic_g0_physical_operation_mapping_from_program_v1(
            self.layout_binding.layout.program(),
        )?;
        Ok(callback(
            GenericG0PhysicalEmitterAdmissionRefV1 { admission: &self },
            mapping,
        ))
    }
}

pub(crate) fn issue_generic_g0_physical_emitter_admission_v1<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    selection: CanonicalLoopFamilySelectionV1,
) -> Result<PreparedGenericG0PhysicalEmitterAdmissionV1<'source>, GenericG0PhysicalEmitterAdmissionRejectV1>
{
    let parent = issue_generic_g0_source_parent_v1(input, selection)
        .map_err(GenericG0PhysicalEmitterAdmissionRejectV1::SourceParent)?;
    let descriptors = issue_generic_g0_physical_function_entry_input_v1(
        parent.borrow_for_physical_emitter(),
    )
    .map_err(GenericG0PhysicalEmitterAdmissionRejectV1::Entry)?
    .consume(|_, descriptors| descriptors);
    let parent_ref = parent.borrow_for_physical_emitter();
    let effects = issue_generic_g0_physical_function_effects_v1(&parent_ref)
        .map_err(GenericG0PhysicalEmitterAdmissionRejectV1::Effect)?;
    validate_generic_g0_physical_function_shell_facts(&parent_ref, &effects, &descriptors)
        .map_err(GenericG0PhysicalEmitterAdmissionRejectV1::Shell)?;
    let header = parent_ref.declaration_header();
    let shell_plan = PreparedGenericG0FunctionShellPlanV1 {
        symbol: CanonicalCallableSymbolV1::from_name_arity(
            header.name(),
            header.parameters().len(),
        ),
        descriptors,
        result_abi: parent_ref.result_abi().abi(),
        effects,
    };
    let control = issue_generic_g0_entry_control_facts_v1(&parent_ref)
        .map_err(GenericG0PhysicalEmitterAdmissionRejectV1::Control)?;
    drop(parent_ref);

    let cohort = parent
        .into_physical_operation_cohort()
        .map_err(GenericG0PhysicalEmitterAdmissionRejectV1::Cohort)?;
    seal_admission(cohort, shell_plan, control)
}

fn seal_admission<'source>(
    cohort: GenericG0PhysicalOperationCohortV1<'source>,
    shell_plan: PreparedGenericG0FunctionShellPlanV1,
    control: PreparedGenericG0EntryControlFactsV1,
) -> Result<PreparedGenericG0PhysicalEmitterAdmissionV1<'source>, GenericG0PhysicalEmitterAdmissionRejectV1>
{
    let GenericG0PhysicalOperationCohortV1 {
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
    } = cohort;
    let mapping_count = {
        let mapping = issue_generic_g0_physical_operation_mapping_from_program_v1(&program)
            .map_err(GenericG0PhysicalEmitterAdmissionRejectV1::Mapping)?;
        mapping.operation_count()
    };
    let layout = program
        .prepare_physical_layout()
        .map_err(GenericG0PhysicalEmitterAdmissionRejectV1::Layout)?;
    let context = layout.program().demand().context();
    let coverage = layout.coverage();
    let operation_count = u32::try_from(mapping_count)
        .map_err(|_| GenericG0PhysicalEmitterAdmissionRejectV1::CountOverflow)?;
    let layout_item_count = u32::try_from(coverage.item_count())
        .map_err(|_| GenericG0PhysicalEmitterAdmissionRejectV1::CountOverflow)?;
    let layout_segment_count = u32::try_from(coverage.segment_count())
        .map_err(|_| GenericG0PhysicalEmitterAdmissionRejectV1::CountOverflow)?;

    let expected_symbol = CanonicalCallableSymbolV1::from_name_arity(
        declaration_header.name(),
        declaration_header.parameters().len(),
    );
    let exact = context.owner() == input.owner()
        && context.origin() == input.function().function_origin()
        && context.source_kind() == input.function().source_kind()
        && body_shape.owner() == input.owner()
        && body_shape.body_root() == &input.function().root_profile().body_root()
        && function_effect.owner() == input.owner()
        && function_effect.body_root() == body_shape.body_root()
        && result_abi.owner() == input.owner()
        && storage_lane.owner() == input.owner()
        && completion.owner() == input.owner()
        && completion.target_function() == input.function().function_region()
        && control.expectation().owner() == input.owner()
        && control.expectation().function_origin() == context.origin()
        && control.expectation().body_root() == *body_shape.body_root()
        && control.outer_if().owner() == input.owner()
        && control.outer_if().row_count() == 0
        && shell_plan.symbol() == &expected_symbol
        && shell_plan.result_abi() == result_abi.abi()
        && shell_plan.effects().owner() == input.owner()
        && shell_plan.effects().target() == target
        && shell_plan.effects().operation_count() == operation_count
        && shell_plan.descriptors().len()
            == usize::try_from(storage_lane.physical_callable_lane_count()).unwrap_or(usize::MAX)
        && entries.len() == declaration_header.parameters().len()
        && storage_lane.source_logical_arity()
            == u32::try_from(declaration_header.parameters().len()).unwrap_or(u32::MAX)
        && coverage.operation_count() == mapping_count
        && operation_count > 0
        && layout_segment_count > 0;
    if !exact {
        return Err(GenericG0PhysicalEmitterAdmissionRejectV1::CohortDrift);
    }

    let stamp = GenericG0PhysicalEmitterCohortStampV1 {
        owner: input.owner(),
        origin: context.origin(),
        source_kind: context.source_kind(),
        body_root: body_shape.body_root().clone(),
        loop_site: context.loop_site().clone(),
        frame: context.frame().clone(),
        scope_region: context.scope_region(),
        completion_target: completion.target_function(),
        source_logical_arity: storage_lane.source_logical_arity(),
        receiver_lane_count: storage_lane.receiver_lane_count(),
        physical_callable_lane_count: storage_lane.physical_callable_lane_count(),
        operation_count,
        layout_item_count,
        layout_segment_count,
        target,
        program_revision: GENERIC_G0_OPERATION_PROGRAM_REVISION_V1,
        layout_revision: GENERIC_G0_PHYSICAL_LAYOUT_REVISION_V1,
    };
    debug_assert!(
        stamp.owner == input.owner()
            && stamp.origin == context.origin()
            && stamp.source_kind == context.source_kind()
            && stamp.body_root == *body_shape.body_root()
            && stamp.loop_site == *context.loop_site()
            && stamp.frame.matches(context.frame())
            && stamp.scope_region == context.scope_region()
            && stamp.completion_target == completion.target_function()
            && stamp.source_logical_arity == storage_lane.source_logical_arity()
            && stamp.receiver_lane_count == storage_lane.receiver_lane_count()
            && stamp.physical_callable_lane_count
                == storage_lane.physical_callable_lane_count()
            && stamp.operation_count == operation_count
            && stamp.layout_item_count == layout_item_count
            && stamp.layout_segment_count == layout_segment_count
            && stamp.target == target
            && stamp.program_revision == GENERIC_G0_OPERATION_PROGRAM_REVISION_V1
            && stamp.layout_revision == GENERIC_G0_PHYSICAL_LAYOUT_REVISION_V1
    );
    Ok(PreparedGenericG0PhysicalEmitterAdmissionV1 {
        input,
        entries,
        layout_binding: VerifiedGenericG0PhysicalLayoutBindingV1 { layout, stamp },
        shell_plan,
        control,
        completion,
    })
}
