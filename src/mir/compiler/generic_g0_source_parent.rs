//! Production Generic G0 source-parent issuer.
//!
//! This is the first fast-row seam after the design-only source-cohort stop.
//! It consumes one resolver input and one canonical family selection, issues
//! the Generic demand/Recipe product exactly once, and lends only a
//! callback-scoped typed view.  It does not create physical IDs, a Builder,
//! session state, CFG, or a route decision.

use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::loop_recipe_contract::{
    issue_generic_g0_recipe_demand_v1, produce_generic_g0_recipe_v1,
    GenericG0RecipeDemandIssueV1, GenericG0RecipeProducerRejectV1, LoopBindingKeyV1,
    LoopValueClassV1, LoopValueKeyV1, VerifiedGenericRecipeProductG0,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::generic_g0_completion::{
    issue_generic_g0_completion_transport_v1, GenericG0CompletionRejectV1,
};
use super::generic_g0_function_effect::{
    issue_generic_g0_no_external_effect_v1, GenericG0FunctionEffectRejectV1,
    VerifiedGenericG0NoExternalEffectV1,
};
use super::generic_g0_result_abi::{
    issue_generic_g0_result_abi_transport_v1, GenericG0ResultAbiRejectV1,
    VerifiedGenericG0ResultAbiV1,
};
use super::generic_g0_storage_lane_source::{
    issue_generic_g0_storage_lane_source_projection_v1,
    GenericG0StorageLaneSourceRejectV1,
    VerifiedGenericG0StorageLaneSourceProjectionV1,
};
use super::generic_g0_top_level_declaration_header::{
    issue_generic_g0_top_level_declaration_header_v1,
    GenericG0TopLevelDeclarationHeaderRejectV1,
    VerifiedGenericG0TopLevelDeclarationHeaderV1,
};
use crate::mir::loop_route_policy::{
    CanonicalLoopFamilyCandidateV1, CanonicalLoopFamilySelectionV1,
};
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, BindingRefV1, FunctionOwnerIdV1,
    SourceBindingSiteV1, SourceStmtSiteV1, VerifiedResolvedBodyShapeInventoryV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use super::generic_g0_physical_operation_cohort::{
    GenericG0PhysicalOperationCohortRejectV1, GenericG0PhysicalOperationCohortV1,
};

#[path = "generic_g0_source_parent/physical_emitter_source_parts.rs"]
pub(in crate::mir::compiler) mod physical_emitter_source_parts;
pub(in crate::mir::compiler) use physical_emitter_source_parts::{
    GenericG0PhysicalEmitterSourcePartsRef,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0SourceParentRejectV1 {
    SelectionOwnerMismatch,
    SelectionOriginMismatch,
    SelectionSourceKindMismatch,
    SelectionSiteMismatch,
    SelectionFrameMismatch,
    SelectionScopeRegionMismatch,
    SelectionFamilyMismatch,
    SelectionForestMismatch,
    LoopSiteNotUnique,
    LoopForestUnavailable,
    ProductOwnerMismatch,
    ProductOriginMismatch,
    ProductSourceKindMismatch,
    ProductSiteMismatch,
    ProductFrameMismatch,
    ProductScopeRegionMismatch,
    EntryCountMismatch,
    EntryBindingMissing,
    EntryBindingOwnerMismatch,
    EntryBindingKindMismatch,
    EntryBindingOriginMismatch,
    EntryBindingIndexMismatch,
    EntryBindingClassMismatch,
    BodyShapeMissing,
    BodyShapeOwnerMismatch,
    BodyShapeRootMismatch,
    DeclarationHeader(GenericG0TopLevelDeclarationHeaderRejectV1),
    FunctionEffect(GenericG0FunctionEffectRejectV1),
    ResultAbi(GenericG0ResultAbiRejectV1),
    StorageLane(GenericG0StorageLaneSourceRejectV1),
    Completion(GenericG0CompletionRejectV1),
    Demand(GenericG0RecipeDemandIssueV1),
    Product(GenericG0RecipeProducerRejectV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericG0EntryBindingV1 {
    recipe_value: LoopValueKeyV1,
    binding: BindingRefV1,
    parameter_index: u32,
    abi: ExactTrivialReturnAbiV1,
}

impl VerifiedGenericG0EntryBindingV1 {
    pub(crate) const fn recipe_value(&self) -> LoopValueKeyV1 {
        self.recipe_value
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn parameter_index(&self) -> u32 {
        self.parameter_index
    }

    pub(crate) const fn abi(&self) -> ExactTrivialReturnAbiV1 {
        self.abi
    }
}

/// Non-Clone parent retained only for the duration of the source cohort
/// callback.  The resolver input is intentionally private; consumers receive
/// typed rows and the already-produced source-bound product instead.
#[derive(Debug)]
pub(crate) struct VerifiedGenericG0SourceParentV1<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    product: VerifiedGenericRecipeProductG0,
    entries: Box<[VerifiedGenericG0EntryBindingV1]>,
    body_shape: &'source VerifiedResolvedBodyShapeInventoryV1,
    declaration_header: VerifiedGenericG0TopLevelDeclarationHeaderV1,
    function_effect: VerifiedGenericG0NoExternalEffectV1,
    result_abi: VerifiedGenericG0ResultAbiV1,
    storage_lane: VerifiedGenericG0StorageLaneSourceProjectionV1,
    completion: VerifiedFunctionCompletionV1,
}

impl<'source> VerifiedGenericG0SourceParentV1<'source> {
    pub(in crate::mir::compiler) fn physical_emitter_source_parts(
        &self,
    ) -> GenericG0PhysicalEmitterSourcePartsRef<'_, 'source> {
        GenericG0PhysicalEmitterSourcePartsRef::from_parent(self)
    }

    pub(crate) fn source_input(&self) -> ResolvedFunctionLoweringInputV1<'source> {
        self.input
    }

    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.input.owner()
    }

    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 {
        self.product.context().loop_site()
    }

    pub(crate) fn product(&self) -> &VerifiedGenericRecipeProductG0 {
        &self.product
    }

    pub(crate) fn entries(&self) -> &[VerifiedGenericG0EntryBindingV1] {
        &self.entries
    }

    pub(crate) fn body_shape(&self) -> &VerifiedResolvedBodyShapeInventoryV1 {
        self.body_shape
    }

    pub(crate) fn declaration_header(
        &self,
    ) -> &VerifiedGenericG0TopLevelDeclarationHeaderV1 {
        &self.declaration_header
    }

    pub(crate) fn function_effect(&self) -> &VerifiedGenericG0NoExternalEffectV1 {
        &self.function_effect
    }

    pub(crate) fn result_abi(&self) -> &VerifiedGenericG0ResultAbiV1 {
        &self.result_abi
    }

    pub(crate) fn storage_lane(
        &self,
    ) -> &VerifiedGenericG0StorageLaneSourceProjectionV1 {
        &self.storage_lane
    }

    pub(crate) fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.completion
    }

    pub(super) fn into_physical_operation_cohort(
        self,
    ) -> Result<GenericG0PhysicalOperationCohortV1<'source>, GenericG0PhysicalOperationCohortRejectV1>
    {
        let Self {
            input,
            product,
            entries,
            body_shape,
            declaration_header,
            function_effect,
            result_abi,
            storage_lane,
            completion,
        } = self;
        GenericG0PhysicalOperationCohortV1::from_parent_parts(
            input,
            product,
            entries,
            body_shape,
            declaration_header,
            function_effect,
            result_abi,
            storage_lane,
            completion,
        )
    }
}

/// Callback-scoped combined view.  No raw `(input, selection)` tuple or
/// independently reacquirable demand is exposed to the callback.
pub(crate) struct GenericG0SourceParentRefV1<'loan, 'source> {
    parent: &'loan VerifiedGenericG0SourceParentV1<'source>,
}

impl<'loan, 'source> GenericG0SourceParentRefV1<'loan, 'source> {
    pub(in crate::mir::compiler) fn physical_emitter_source_parts(
        &self,
    ) -> GenericG0PhysicalEmitterSourcePartsRef<'_, 'source> {
        self.parent.physical_emitter_source_parts()
    }

    pub(crate) fn source_input(&self) -> ResolvedFunctionLoweringInputV1<'source> {
        self.parent.source_input()
    }

    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.parent.owner()
    }

    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 {
        self.parent.loop_site()
    }

    pub(crate) fn product(&self) -> &VerifiedGenericRecipeProductG0 {
        self.parent.product()
    }

    pub(crate) fn entries(&self) -> &[VerifiedGenericG0EntryBindingV1] {
        self.parent.entries()
    }

    pub(crate) fn body_shape(&self) -> &VerifiedResolvedBodyShapeInventoryV1 {
        self.parent.body_shape()
    }

    pub(crate) fn declaration_header(
        &self,
    ) -> &VerifiedGenericG0TopLevelDeclarationHeaderV1 {
        self.parent.declaration_header()
    }

    pub(crate) fn function_effect(&self) -> &VerifiedGenericG0NoExternalEffectV1 {
        self.parent.function_effect()
    }

    pub(crate) fn result_abi(&self) -> &VerifiedGenericG0ResultAbiV1 {
        self.parent.result_abi()
    }

    pub(crate) fn storage_lane(
        &self,
    ) -> &VerifiedGenericG0StorageLaneSourceProjectionV1 {
        self.parent.storage_lane()
    }

    pub(crate) fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        self.parent.completion()
    }
}

pub(super) fn issue_generic_g0_source_parent_v1<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    selection: CanonicalLoopFamilySelectionV1,
)-> Result<VerifiedGenericG0SourceParentV1<'source>, GenericG0SourceParentRejectV1> {
    validate_selection_input(&input, &selection)?;
    let body_shape = input
        .body_shape()
        .ok_or(GenericG0SourceParentRejectV1::BodyShapeMissing)?;
    validate_body_shape_input(&input, body_shape)?;
    let declaration_header = issue_generic_g0_top_level_declaration_header_v1(&input)
        .map_err(GenericG0SourceParentRejectV1::DeclarationHeader)?;
    let structural = match selection.candidate() {
        CanonicalLoopFamilyCandidateV1::GenericG0(candidate) => candidate
            .observation()
            .handoff()
            .bundle()
            .source()
            .structural(),
        _ => return Err(GenericG0SourceParentRejectV1::SelectionFamilyMismatch),
    };
    let result_abi = issue_generic_g0_result_abi_transport_v1(
        &input,
        &selection,
        &declaration_header,
    )
    .map_err(GenericG0SourceParentRejectV1::ResultAbi)?;
    let completion = issue_generic_g0_completion_transport_v1(input, &result_abi)
        .map_err(GenericG0SourceParentRejectV1::Completion)?;
    let function_effect = issue_generic_g0_no_external_effect_v1(
        &input,
        body_shape,
        &declaration_header,
        structural,
    )
    .map_err(GenericG0SourceParentRejectV1::FunctionEffect)?;
    let demand = issue_generic_g0_recipe_demand_v1(selection)
        .map_err(GenericG0SourceParentRejectV1::Demand)?;
    let product = produce_generic_g0_recipe_v1(demand)
        .map_err(GenericG0SourceParentRejectV1::Product)?;
    validate_product_input(&input, &product)?;
    let entries = issue_entry_rows(&input, &product)?;
    let storage_lane = issue_generic_g0_storage_lane_source_projection_v1(
        &input,
        &product,
        &declaration_header,
        body_shape,
        &entries,
    )
    .map_err(GenericG0SourceParentRejectV1::StorageLane)?;
    let parent = VerifiedGenericG0SourceParentV1 {
        input,
        product,
        entries,
        body_shape,
        declaration_header,
        function_effect,
        result_abi,
        storage_lane,
        completion,
    };
    Ok(parent)
}

pub(crate) fn with_generic_g0_source_parent_v1<'source, R>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    selection: CanonicalLoopFamilySelectionV1,
    callback: impl for<'loan> FnOnce(GenericG0SourceParentRefV1<'loan, 'source>) -> R,
) -> Result<R, GenericG0SourceParentRejectV1> {
    let parent = issue_generic_g0_source_parent_v1(input, selection)?;
    Ok(callback(GenericG0SourceParentRefV1 { parent: &parent }))
}

fn validate_body_shape_input(
    input: &ResolvedFunctionLoweringInputV1<'_>,
    body_shape: &VerifiedResolvedBodyShapeInventoryV1,
) -> Result<(), GenericG0SourceParentRejectV1> {
    if body_shape.owner() != input.owner() {
        return Err(GenericG0SourceParentRejectV1::BodyShapeOwnerMismatch);
    }
    if *body_shape.body_root() != input.function().root_profile().body_root() {
        return Err(GenericG0SourceParentRejectV1::BodyShapeRootMismatch);
    }
    Ok(())
}

fn validate_selection_input(
    input: &ResolvedFunctionLoweringInputV1<'_>,
    selection: &CanonicalLoopFamilySelectionV1,
) -> Result<(), GenericG0SourceParentRejectV1> {
    let lease = selection.lease();
    if lease.owner() != input.owner() {
        return Err(GenericG0SourceParentRejectV1::SelectionOwnerMismatch);
    }
    if lease.function_origin() != input.function().function_origin() {
        return Err(GenericG0SourceParentRejectV1::SelectionOriginMismatch);
    }
    if lease.source_kind() != input.function().source_kind() {
        return Err(GenericG0SourceParentRejectV1::SelectionSourceKindMismatch);
    }
    let site = lease.site();
    if input.function().loop_region_bundle(site).is_err() {
        return Err(GenericG0SourceParentRejectV1::SelectionSiteMismatch);
    }
    let (source, pair) = input
        .function()
        .resolved_loop_source_context(site)
        .map_err(|_| GenericG0SourceParentRejectV1::LoopSiteNotUnique)?;
    if !lease.frame().matches(&source.frame_key()) {
        return Err(GenericG0SourceParentRejectV1::SelectionFrameMismatch);
    }
    if lease.scope_region() != pair {
        return Err(GenericG0SourceParentRejectV1::SelectionScopeRegionMismatch);
    }
    let input_forest = input
        .function()
        .resolved_loop_source_forest(site)
        .map_err(|_| GenericG0SourceParentRejectV1::LoopForestUnavailable)?;
    let CanonicalLoopFamilyCandidateV1::GenericG0(candidate) = selection.candidate() else {
        return Err(GenericG0SourceParentRejectV1::SelectionFamilyMismatch);
    };
    if candidate
        .observation()
        .handoff()
        .bundle()
        .source()
        .structural()
        .forest()
        != &input_forest
    {
        return Err(GenericG0SourceParentRejectV1::SelectionForestMismatch);
    }
    Ok(())
}

fn validate_product_input(
    input: &ResolvedFunctionLoweringInputV1<'_>,
    product: &VerifiedGenericRecipeProductG0,
) -> Result<(), GenericG0SourceParentRejectV1> {
    let context = product.context();
    if context.owner() != input.owner() {
        return Err(GenericG0SourceParentRejectV1::ProductOwnerMismatch);
    }
    if context.origin() != input.function().function_origin() {
        return Err(GenericG0SourceParentRejectV1::ProductOriginMismatch);
    }
    if context.source_kind() != input.function().source_kind() {
        return Err(GenericG0SourceParentRejectV1::ProductSourceKindMismatch);
    }
    let site = context.loop_site();
    if input.function().loop_region_bundle(site).is_err() {
        return Err(GenericG0SourceParentRejectV1::ProductSiteMismatch);
    }
    let (source, pair) = input
        .function()
        .resolved_loop_source_context(site)
        .map_err(|_| GenericG0SourceParentRejectV1::LoopSiteNotUnique)?;
    if !context.frame().matches(&source.frame_key()) {
        return Err(GenericG0SourceParentRejectV1::ProductFrameMismatch);
    }
    if context.scope_region() != pair {
        return Err(GenericG0SourceParentRejectV1::ProductScopeRegionMismatch);
    }
    if product.core().binding_relations().is_empty()
        || product.core().owner() != input.owner()
    {
        return Err(GenericG0SourceParentRejectV1::LoopForestUnavailable);
    }
    Ok(())
}

fn issue_entry_rows<'source>(
    input: &ResolvedFunctionLoweringInputV1<'source>,
    product: &VerifiedGenericRecipeProductG0,
) -> Result<Box<[VerifiedGenericG0EntryBindingV1]>, GenericG0SourceParentRejectV1> {
    let relations = product.core().binding_relations();
    if relations.len() != 2 {
        return Err(GenericG0SourceParentRejectV1::EntryCountMismatch);
    }
    let mut rows = Vec::with_capacity(2);
    for (index, expected_key) in [0u32, 1u32].into_iter().enumerate() {
        let key = LoopBindingKeyV1::new(expected_key);
        let relation = relations
            .iter()
            .find(|row| row.recipe_binding() == key)
            .ok_or(GenericG0SourceParentRejectV1::EntryBindingMissing)?;
        if relation.class() != LoopValueClassV1::I64 {
            return Err(GenericG0SourceParentRejectV1::EntryBindingClassMismatch);
        }
        let binding = relation.source_binding();
        if binding.owner() != input.owner() {
            return Err(GenericG0SourceParentRejectV1::EntryBindingOwnerMismatch);
        }
        let record = input
            .function()
            .binding(binding)
            .ok_or(GenericG0SourceParentRejectV1::EntryBindingMissing)?;
        let BindingKindV1::Parameter { index: parameter_index } = record.kind() else {
            return Err(GenericG0SourceParentRejectV1::EntryBindingKindMismatch);
        };
        if parameter_index != index as u32 {
            return Err(GenericG0SourceParentRejectV1::EntryBindingIndexMismatch);
        }
        if !matches!(
            record.origin(),
            BindingOriginV1::Source(SourceBindingSiteV1::Parameter { index: origin_index })
                if *origin_index == index as u32
        ) {
            return Err(GenericG0SourceParentRejectV1::EntryBindingOriginMismatch);
        }
        rows.push(VerifiedGenericG0EntryBindingV1 {
            recipe_value: LoopValueKeyV1::new(relation.recipe_binding().raw()),
            binding,
            parameter_index,
            abi: ExactTrivialReturnAbiV1::I64,
        });
    }
    Ok(rows.into_boxed_slice())
}
