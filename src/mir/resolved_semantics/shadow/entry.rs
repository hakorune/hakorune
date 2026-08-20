//! Public shadow traversal entries and observation-only adapters.

use std::collections::{BTreeMap, BTreeSet};

use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    EnumMatchDemandV1, EnumVariantDemandV1, FunctionOriginV1, FunctionSyntaxViewV1,
    RecordSchemaDemandV1, ScriptSyntaxViewV1, SourceExprSiteV1, VerifiedScriptRootDemandWindowV1,
};

use super::product::{
    ShadowMethodCallObservationV0, ShadowQualifiedReceiverDispositionV0, ShadowResolveErrorV0,
    ShadowResolvedFunctionV0, ShadowResolvedOwnerV0,
};
use super::resolver::{
    traverse_shadow_root_v1, ShadowLambdaModeV0, ShadowMethodCallObservationModeV0,
};
use super::root_traversal::ShadowRootTraversalInputV1;
use super::traversal_profile::ShadowTraversalProfileV1;

pub(super) fn resolve_function_shadow_v0(
    _function_origin: FunctionOriginV1,
    function: &ASTNode,
) -> Result<ShadowResolvedFunctionV0, ShadowResolveErrorV0> {
    let Some(view) = FunctionSyntaxViewV1::from_ast(function) else {
        return Err(ShadowResolveErrorV0::ExpectedFunctionDeclaration);
    };
    resolve_function_shadow_view_v0(view)
}

pub(in crate::mir::resolved_semantics) fn resolve_function_shadow_view_v0(
    view: FunctionSyntaxViewV1<'_>,
) -> Result<ShadowResolvedFunctionV0, ShadowResolveErrorV0> {
    resolve_shadow_view(
        view,
        ShadowLambdaModeV0::Reject,
        BTreeSet::new(),
        ShadowMethodCallObservationModeV0::Disabled,
    )
    .map(|owner| owner.function)
}

pub(in crate::mir::resolved_semantics) fn resolve_owner_shadow_view_v0<'ast>(
    view: FunctionSyntaxViewV1<'ast>,
    ancestor_names: BTreeSet<Box<str>>,
) -> Result<ShadowResolvedOwnerV0<'ast>, ShadowResolveErrorV0> {
    resolve_owner_shadow_view_with_profile_v0(
        view,
        ancestor_names,
        ShadowTraversalProfileV1::FullFunctionV1,
    )
}

pub(in crate::mir::resolved_semantics) fn resolve_owner_shadow_view_with_profile_v0<'ast>(
    view: FunctionSyntaxViewV1<'ast>,
    ancestor_names: BTreeSet<Box<str>>,
    traversal_profile: ShadowTraversalProfileV1,
) -> Result<ShadowResolvedOwnerV0<'ast>, ShadowResolveErrorV0> {
    resolve_shadow_view_with_profile(
        view,
        ShadowLambdaModeV0::Inventory,
        ancestor_names,
        ShadowMethodCallObservationModeV0::Disabled,
        traversal_profile,
    )
}

pub(in crate::mir::resolved_semantics) fn resolve_owner_shadow_view_with_profile_and_brand_catalog_v1<
    'ast,
    'catalog,
>(
    view: FunctionSyntaxViewV1<'ast>,
    ancestor_names: BTreeSet<Box<str>>,
    traversal_profile: ShadowTraversalProfileV1,
    brand_catalog: &'catalog VerifiedBrandProgramDeclarationCatalogV1,
) -> Result<ShadowResolvedOwnerV0<'ast>, ShadowResolveErrorV0> {
    let input = ShadowRootTraversalInputV1::dense_with_profile(view, traversal_profile)
        .with_brand_catalog(brand_catalog);
    let root_profile = input.root_profile();
    traverse_shadow_root_v1(
        input,
        ShadowLambdaModeV0::Inventory,
        ancestor_names,
        BTreeSet::new(),
        ShadowMethodCallObservationModeV0::Disabled,
        true,
    )
    .map(|resolver| resolver.finish_owner(root_profile))
}

pub(in crate::mir::resolved_semantics) fn resolve_script_shadow_view_v0<'ast>(
    view: ScriptSyntaxViewV1<'ast>,
    window: &'ast VerifiedScriptRootDemandWindowV1,
    record_schemas: &dyn RecordSchemaDemandV1,
    enum_variants: &dyn EnumVariantDemandV1,
    enum_matches: &dyn EnumMatchDemandV1,
) -> Result<ShadowResolvedFunctionV0, ShadowResolveErrorV0> {
    let input = ShadowRootTraversalInputV1::sparse_script(
        view,
        window,
        record_schemas,
        enum_variants,
        enum_matches,
    );
    let profile = input.root_profile();
    traverse_shadow_root_v1(
        input,
        ShadowLambdaModeV0::Reject,
        BTreeSet::new(),
        BTreeSet::new(),
        ShadowMethodCallObservationModeV0::Disabled,
        false,
    )
    .map(|resolver| resolver.finish_owner(profile).function)
}

pub(in crate::mir::resolved_semantics) fn resolve_script_owner_shadow_view_v0<'ast>(
    view: ScriptSyntaxViewV1<'ast>,
    window: &'ast VerifiedScriptRootDemandWindowV1,
    record_schemas: &dyn RecordSchemaDemandV1,
    enum_variants: &dyn EnumVariantDemandV1,
    enum_matches: &dyn EnumMatchDemandV1,
) -> Result<ShadowResolvedOwnerV0<'ast>, ShadowResolveErrorV0> {
    let input = ShadowRootTraversalInputV1::sparse_script(
        view,
        window,
        record_schemas,
        enum_variants,
        enum_matches,
    );
    let profile = input.root_profile();
    traverse_shadow_root_v1(
        input,
        ShadowLambdaModeV0::Inventory,
        BTreeSet::new(),
        BTreeSet::new(),
        ShadowMethodCallObservationModeV0::Disabled,
        false,
    )
    .map(|resolver| resolver.finish_owner(profile))
}

pub(in crate::mir::resolved_semantics) fn resolve_script_owner_shadow_view_with_brand_catalog_v1<
    'ast,
    'catalog,
>(
    view: ScriptSyntaxViewV1<'ast>,
    window: &'ast VerifiedScriptRootDemandWindowV1,
    record_schemas: &dyn RecordSchemaDemandV1,
    enum_variants: &dyn EnumVariantDemandV1,
    enum_matches: &dyn EnumMatchDemandV1,
    brand_catalog: &'catalog VerifiedBrandProgramDeclarationCatalogV1,
) -> Result<ShadowResolvedOwnerV0<'ast>, ShadowResolveErrorV0> {
    let input = ShadowRootTraversalInputV1::sparse_script(
        view,
        window,
        record_schemas,
        enum_variants,
        enum_matches,
    )
    .with_brand_catalog(brand_catalog);
    let profile = input.root_profile();
    traverse_shadow_root_v1(
        input,
        ShadowLambdaModeV0::Inventory,
        BTreeSet::new(),
        BTreeSet::new(),
        ShadowMethodCallObservationModeV0::Disabled,
        false,
    )
    .map(|resolver| resolver.finish_owner(profile))
}

fn resolve_shadow_view<'ast>(
    view: FunctionSyntaxViewV1<'ast>,
    lambda_mode: ShadowLambdaModeV0,
    ancestor_names: BTreeSet<Box<str>>,
    method_call_observation_mode: ShadowMethodCallObservationModeV0,
) -> Result<ShadowResolvedOwnerV0<'ast>, ShadowResolveErrorV0> {
    resolve_shadow_view_with_profile(
        view,
        lambda_mode,
        ancestor_names,
        method_call_observation_mode,
        ShadowTraversalProfileV1::FullFunctionV1,
    )
}

fn resolve_shadow_view_with_profile<'ast>(
    view: FunctionSyntaxViewV1<'ast>,
    lambda_mode: ShadowLambdaModeV0,
    ancestor_names: BTreeSet<Box<str>>,
    method_call_observation_mode: ShadowMethodCallObservationModeV0,
    traversal_profile: ShadowTraversalProfileV1,
) -> Result<ShadowResolvedOwnerV0<'ast>, ShadowResolveErrorV0> {
    let input = ShadowRootTraversalInputV1::dense_with_profile(view, traversal_profile);
    let root_profile = input.root_profile();
    traverse_shadow_root_v1(
        input,
        lambda_mode,
        ancestor_names,
        BTreeSet::new(),
        method_call_observation_mode,
        true,
    )
    .map(|resolver| resolver.finish_owner(root_profile))
}

pub(in crate::mir) fn observe_qualified_receiver_shadow_view_v0(
    view: FunctionSyntaxViewV1<'_>,
    requested_sites: BTreeSet<SourceExprSiteV1>,
) -> Result<BTreeMap<SourceExprSiteV1, ShadowQualifiedReceiverDispositionV0>, ShadowResolveErrorV0>
{
    traverse_shadow_root_v1(
        ShadowRootTraversalInputV1::dense(view),
        ShadowLambdaModeV0::Reject,
        BTreeSet::new(),
        requested_sites,
        ShadowMethodCallObservationModeV0::Disabled,
        false,
    )?
    .finish_qualified_receiver_observations()
}

pub(in crate::mir) fn observe_method_calls_shadow_view_v0(
    view: FunctionSyntaxViewV1<'_>,
) -> Result<BTreeMap<SourceExprSiteV1, ShadowMethodCallObservationV0>, ShadowResolveErrorV0> {
    traverse_shadow_root_v1(
        ShadowRootTraversalInputV1::dense(view),
        ShadowLambdaModeV0::Reject,
        BTreeSet::new(),
        BTreeSet::new(),
        ShadowMethodCallObservationModeV0::All,
        false,
    )?
    .finish_method_call_observations()
}

/// Observation-only Script traversal for the later direct-static target
/// catalog.  It reuses the shared resolver/path issuer but does not publish a
/// Script owner or widen the Script lexical acceptance profile.
pub(in crate::mir) fn observe_script_method_calls_shadow_view_v0(
    view: ScriptSyntaxViewV1<'_>,
    window: &VerifiedScriptRootDemandWindowV1,
) -> Result<BTreeMap<SourceExprSiteV1, ShadowMethodCallObservationV0>, ShadowResolveErrorV0> {
    traverse_shadow_root_v1(
        ShadowRootTraversalInputV1::sparse_script_static_target_observation(view, window),
        ShadowLambdaModeV0::Reject,
        BTreeSet::new(),
        BTreeSet::new(),
        ShadowMethodCallObservationModeV0::All,
        true,
    )?
    .finish_method_call_observations()
}
