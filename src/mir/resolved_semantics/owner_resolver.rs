//! Recursive owner-forest construction for non-capturing nested functions.

use std::collections::{BTreeMap, BTreeSet};

use super::callable_header_view::CallableFunctionSyntaxViewV1;
use super::callable_index::{CallableIndexSealErrorV1, VerifiedCallableIndexV1};
use super::function_view::FunctionSyntaxViewV1;
use super::ids::{BindingRefV1, FunctionOwnerIdV1, ScopeId};
use super::owner_forest::{
    OwnerParentEdgeV1, SemanticOwnerForestDraftV1, SemanticOwnerForestVerificationErrorV1,
    VerifiedSemanticOwnerForestV1,
};
use super::resolved_callable_forest::{
    ResolvedCallableForestVerificationErrorV1, VerifiedResolvedCallableForestV1,
};
use super::resolver::{
    AncestorBindingV1, FunctionSemanticResolverSessionV1, ResolveFunctionErrorV1,
    SealedOwnerConstructionV1,
};
use super::shadow::{
    resolve_function_shadow_view_v0, resolve_owner_shadow_view_v0, ShadowLambdaSyntaxV0,
    ShadowResolvedOwnerV0,
};
use super::OwnedExprSiteV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolveOwnerForestErrorV1 {
    Function(ResolveFunctionErrorV1),
    Verification(SemanticOwnerForestVerificationErrorV1),
    CallableIndex(CallableIndexSealErrorV1),
    CallableForest(ResolvedCallableForestVerificationErrorV1),
}

#[derive(Debug, Clone)]
struct PendingParentV1 {
    parent_owner: FunctionOwnerIdV1,
    definition_site: OwnedExprSiteV1,
    parent_scope: ScopeId,
}

impl FunctionSemanticResolverSessionV1 {
    pub(crate) fn resolve_forest_with_root_callable(
        &mut self,
        views: CallableFunctionSyntaxViewV1<'_>,
    ) -> Result<VerifiedResolvedCallableForestV1, ResolveOwnerForestErrorV1> {
        let (origin, owner) = self
            .issue_owner()
            .map_err(ResolveOwnerForestErrorV1::Function)?;
        let callable_index = VerifiedCallableIndexV1::seal_one(owner, views.header())
            .map_err(ResolveOwnerForestErrorV1::CallableIndex)?;
        let shadow = resolve_function_shadow_view_v0(origin, views.function())
            .map_err(ResolveFunctionErrorV1::Syntax)
            .map_err(ResolveOwnerForestErrorV1::Function)?;
        let product = self
            .seal_owner_with_callable_index(owner, shadow, &callable_index)
            .map_err(ResolveOwnerForestErrorV1::Function)?
            .product;
        let mut draft = SemanticOwnerForestDraftV1::new();
        draft
            .insert_owner(owner, product)
            .map_err(ResolveOwnerForestErrorV1::Verification)?;
        let forest = draft
            .seal()
            .map_err(ResolveOwnerForestErrorV1::Verification)?;
        VerifiedResolvedCallableForestV1::seal(forest, callable_index)
            .map_err(ResolveOwnerForestErrorV1::CallableForest)
    }

    pub(crate) fn resolve_forest(
        &mut self,
        root: FunctionSyntaxViewV1<'_>,
    ) -> Result<VerifiedSemanticOwnerForestV1, ResolveOwnerForestErrorV1> {
        let mut draft = SemanticOwnerForestDraftV1::new();
        self.resolve_owner_recursive(root, &BTreeMap::new(), None, &mut draft)?;
        draft
            .seal()
            .map_err(ResolveOwnerForestErrorV1::Verification)
    }

    fn resolve_owner_recursive<'ast>(
        &mut self,
        view: FunctionSyntaxViewV1<'ast>,
        ancestor_bindings: &BTreeMap<Box<str>, AncestorBindingV1>,
        parent: Option<PendingParentV1>,
        draft: &mut SemanticOwnerForestDraftV1,
    ) -> Result<FunctionOwnerIdV1, ResolveOwnerForestErrorV1> {
        let (origin, owner) = self
            .issue_owner()
            .map_err(ResolveOwnerForestErrorV1::Function)?;
        let ancestor_names = ancestor_bindings.keys().cloned().collect::<BTreeSet<_>>();
        let shadow = resolve_owner_shadow_view_v0(origin, view, ancestor_names)
            .map_err(ResolveFunctionErrorV1::Syntax)
            .map_err(ResolveOwnerForestErrorV1::Function)?;
        let ShadowResolvedOwnerV0 { function, lambdas } = shadow;
        let SealedOwnerConstructionV1 {
            product,
            binding_refs,
            scope_ids,
        } = self
            .seal_owner_with_ancestors(owner, function, ancestor_bindings)
            .map_err(ResolveOwnerForestErrorV1::Function)?;

        let children = lambdas
            .into_vec()
            .into_iter()
            .map(|lambda| {
                let child_bindings =
                    visible_bindings_for_child(ancestor_bindings, &binding_refs, &lambda);
                let child_parent = PendingParentV1 {
                    parent_owner: owner,
                    definition_site: OwnedExprSiteV1::new(owner, lambda.definition_site.clone()),
                    parent_scope: scope_ids[&lambda.parent_scope],
                };
                (lambda, child_bindings, child_parent)
            })
            .collect::<Vec<_>>();

        if let Some(parent) = parent {
            draft
                .insert_parent(
                    owner,
                    OwnerParentEdgeV1::new(
                        parent.parent_owner,
                        parent.definition_site,
                        parent.parent_scope,
                    ),
                )
                .map_err(ResolveOwnerForestErrorV1::Verification)?;
        }
        draft
            .insert_owner(owner, product)
            .map_err(ResolveOwnerForestErrorV1::Verification)?;

        for (lambda, child_bindings, child_parent) in children {
            self.resolve_owner_recursive(
                lambda.syntax_view(),
                &child_bindings,
                Some(child_parent),
                draft,
            )?;
        }
        Ok(owner)
    }
}

fn visible_bindings_for_child(
    ancestors: &BTreeMap<Box<str>, AncestorBindingV1>,
    binding_refs: &BTreeMap<super::shadow::ShadowBindingOrdinalV0, BindingRefV1>,
    lambda: &ShadowLambdaSyntaxV0<'_>,
) -> BTreeMap<Box<str>, AncestorBindingV1> {
    let mut visible = ancestors.clone();
    for (name, binding) in &lambda.visible_bindings {
        let reference = binding_refs[binding];
        visible.insert(name.clone(), AncestorBindingV1 { reference });
    }
    visible
}
