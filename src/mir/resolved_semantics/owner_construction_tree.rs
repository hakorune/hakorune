//! Fallible recursive shadow construction before canonical owner issuance.
//!
//! This private tree gives every nested Lambda one shared shadow traversal
//! before the owner forest assigns any canonical ID.

use std::collections::BTreeSet;

use super::function_view::FunctionSyntaxViewV1;
use super::script_view::ScriptSyntaxViewV1;
use super::shadow::ShadowTraversalProfileV1;
use super::shadow::{
    resolve_owner_shadow_view_v0, resolve_owner_shadow_view_with_profile_v0,
    resolve_script_owner_shadow_view_v0, ShadowLambdaSyntaxV0, ShadowResolveErrorV0,
    ShadowResolvedFunctionV0, ShadowResolvedOwnerV0, VerifiedScriptRootDemandWindowV1,
};
use super::{EnumVariantDemandV1, RecordSchemaDemandV1};

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
    )
}

pub(super) fn construct_script_owner_tree_v1<'ast>(
    view: ScriptSyntaxViewV1<'ast>,
    window: &'ast VerifiedScriptRootDemandWindowV1,
    record_schemas: &dyn RecordSchemaDemandV1,
    enum_variants: &dyn EnumVariantDemandV1,
) -> Result<ShadowOwnerConstructionTreeV1<'ast>, ShadowResolveErrorV0> {
    let ShadowResolvedOwnerV0 { function, lambdas } =
        resolve_script_owner_shadow_view_v0(view, window, record_schemas, enum_variants)?;
    construct_owner_tree_v1(
        function,
        lambdas,
        &BTreeSet::new(),
        ShadowTraversalProfileV1::ScriptLambdaLeafV1,
    )
}

fn construct_owner_tree_v1<'ast>(
    function: ShadowResolvedFunctionV0,
    lambdas: Box<[ShadowLambdaSyntaxV0<'ast>]>,
    ancestor_names: &BTreeSet<Box<str>>,
    child_profile: ShadowTraversalProfileV1,
) -> Result<ShadowOwnerConstructionTreeV1<'ast>, ShadowResolveErrorV0> {
    let children = lambdas
        .into_vec()
        .into_iter()
        .map(|lambda| {
            let mut child_ancestor_names = ancestor_names.clone();
            child_ancestor_names.extend(lambda.visible_bindings.keys().cloned());
            let ShadowResolvedOwnerV0 { function, lambdas } =
                resolve_owner_shadow_view_with_profile_v0(
                    lambda.syntax_view(),
                    child_ancestor_names.clone(),
                    child_profile,
                )?;
            let tree =
                construct_owner_tree_v1(function, lambdas, &child_ancestor_names, child_profile)?;
            Ok(ShadowOwnerConstructionChildV1 {
                lambda,
                tree: Box::new(tree),
            })
        })
        .collect::<Result<Vec<_>, ShadowResolveErrorV0>>()?;
    Ok(ShadowOwnerConstructionTreeV1 { function, children })
}
