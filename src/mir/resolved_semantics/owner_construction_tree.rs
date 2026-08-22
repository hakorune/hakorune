//! Fallible recursive shadow construction before canonical owner issuance.
//!
//! This private tree gives every nested Lambda one shared shadow traversal
//! before the owner forest assigns any canonical ID.

use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use std::collections::BTreeSet;

use super::function_view::FunctionSyntaxViewV1;
use super::script_view::ScriptSyntaxViewV1;
use super::shadow::ShadowTraversalProfileV1;
use super::shadow::{
    resolve_owner_shadow_view_v0, resolve_owner_shadow_view_with_profile_and_brand_catalog_v1,
    resolve_owner_shadow_view_with_profile_v0, resolve_script_owner_shadow_view_v0,
    resolve_script_owner_shadow_view_with_brand_catalog_v1, ShadowLambdaSyntaxV0,
    ShadowResolveErrorV0, ShadowResolvedFunctionV0, ShadowResolvedOwnerV0,
    VerifiedScriptRootDemandWindowV1,
};
use super::{EnumMatchDemandV1, EnumVariantDemandV1, RecordSchemaDemandV1};

pub(super) struct ShadowOwnerConstructionTreeV1<'ast> {
    pub(super) function: ShadowResolvedFunctionV0,
    pub(super) children: Vec<ShadowOwnerConstructionChildV1<'ast>>,
}

pub(super) struct ShadowOwnerConstructionChildV1<'ast> {
    pub(super) lambda: ShadowLambdaSyntaxV0<'ast>,
    pub(super) tree: Box<ShadowOwnerConstructionTreeV1<'ast>>,
}

pub(super) fn construct_function_owner_tree_v1<'ast>(
    view: FunctionSyntaxViewV1<'ast>,
    ancestor_names: &BTreeSet<Box<str>>,
) -> Result<ShadowOwnerConstructionTreeV1<'ast>, ShadowResolveErrorV0> {
    let ShadowResolvedOwnerV0 { function, lambdas } =
        resolve_owner_shadow_view_v0(view, ancestor_names.clone())?;
    construct_owner_tree_v1(
        function,
        lambdas,
        ancestor_names,
        ShadowTraversalProfileV1::FullFunctionV1,
        None,
    )
}

pub(super) fn construct_selected_callable_owner_tree_v1<'ast>(
    view: FunctionSyntaxViewV1<'ast>,
    brand_catalog: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
) -> Result<ShadowOwnerConstructionTreeV1<'ast>, ShadowResolveErrorV0> {
    let profile = ShadowTraversalProfileV1::SelectedCallableV1;
    let ShadowResolvedOwnerV0 { function, lambdas } = match brand_catalog {
        Some(catalog) => resolve_owner_shadow_view_with_profile_and_brand_catalog_v1(
            view,
            BTreeSet::new(),
            profile,
            catalog,
        )?,
        None => resolve_owner_shadow_view_with_profile_v0(view, BTreeSet::new(), profile)?,
    };
    construct_owner_tree_v1(function, lambdas, &BTreeSet::new(), profile, brand_catalog)
}

pub(super) fn construct_script_owner_tree_v1<'ast>(
    view: ScriptSyntaxViewV1<'ast>,
    window: &'ast VerifiedScriptRootDemandWindowV1,
    record_schemas: &dyn RecordSchemaDemandV1,
    enum_variants: &dyn EnumVariantDemandV1,
    enum_matches: &dyn EnumMatchDemandV1,
    brand_catalog: &VerifiedBrandProgramDeclarationCatalogV1,
) -> Result<ShadowOwnerConstructionTreeV1<'ast>, ShadowResolveErrorV0> {
    let ShadowResolvedOwnerV0 { function, lambdas } =
        resolve_script_owner_shadow_view_with_brand_catalog_v1(
            view,
            window,
            record_schemas,
            enum_variants,
            enum_matches,
            brand_catalog,
        )?;
    construct_owner_tree_v1(
        function,
        lambdas,
        &BTreeSet::new(),
        ShadowTraversalProfileV1::ScriptLambdaLeafV1,
        Some(brand_catalog),
    )
}

fn construct_owner_tree_v1<'ast>(
    function: ShadowResolvedFunctionV0,
    lambdas: Box<[ShadowLambdaSyntaxV0<'ast>]>,
    ancestor_names: &BTreeSet<Box<str>>,
    child_profile: ShadowTraversalProfileV1,
    brand_catalog: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
) -> Result<ShadowOwnerConstructionTreeV1<'ast>, ShadowResolveErrorV0> {
    let children = lambdas
        .into_vec()
        .into_iter()
        .map(|lambda| {
            let mut child_ancestor_names = ancestor_names.clone();
            child_ancestor_names.extend(lambda.visible_bindings.keys().cloned());
            let ShadowResolvedOwnerV0 { function, lambdas } = match brand_catalog {
                Some(catalog) => resolve_owner_shadow_view_with_profile_and_brand_catalog_v1(
                    lambda.syntax_view(),
                    child_ancestor_names.clone(),
                    child_profile,
                    catalog,
                )?,
                None => resolve_owner_shadow_view_with_profile_v0(
                    lambda.syntax_view(),
                    child_ancestor_names.clone(),
                    child_profile,
                )?,
            };
            let tree = construct_owner_tree_v1(
                function,
                lambdas,
                &child_ancestor_names,
                child_profile,
                brand_catalog,
            )?;
            Ok(ShadowOwnerConstructionChildV1 {
                lambda,
                tree: Box::new(tree),
            })
        })
        .collect::<Result<Vec<_>, ShadowResolveErrorV0>>()?;
    Ok(ShadowOwnerConstructionTreeV1 { function, children })
}
