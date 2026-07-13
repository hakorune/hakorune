//! Recursive owner-forest construction for non-capturing nested functions.

use std::collections::BTreeMap;

use super::function_view::FunctionSyntaxViewV1;
use super::ids::{BindingRefV1, FunctionOwnerIdV1, ScopeId};
use super::owner_forest::{
    OwnerParentEdgeV1, SemanticOwnerForestDraftV1, SemanticOwnerForestVerificationErrorV1,
    VerifiedSemanticOwnerForestV1,
};
use super::resolver::{
    FunctionSemanticResolverSessionV1, ResolveFunctionErrorV1, SealedOwnerConstructionV1,
};
use super::shadow::{
    resolve_owner_shadow_view_v0, ShadowLambdaSyntaxV0, ShadowResolveErrorV0, ShadowResolvedOwnerV0,
};
use super::{BindingRefV1 as PublicBindingRefV1, OwnedExprSiteV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolveOwnerForestErrorV1 {
    Function(ResolveFunctionErrorV1),
    UnsupportedCapture {
        use_site: OwnedExprSiteV1,
        source: PublicBindingRefV1,
    },
    Verification(SemanticOwnerForestVerificationErrorV1),
}

#[derive(Debug, Clone)]
struct PendingParentV1 {
    parent_owner: FunctionOwnerIdV1,
    definition_site: OwnedExprSiteV1,
    parent_scope: ScopeId,
}

impl FunctionSemanticResolverSessionV1 {
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
        ancestor_bindings: &BTreeMap<Box<str>, BindingRefV1>,
        parent: Option<PendingParentV1>,
        draft: &mut SemanticOwnerForestDraftV1,
    ) -> Result<FunctionOwnerIdV1, ResolveOwnerForestErrorV1> {
        let (origin, owner) = self
            .issue_owner()
            .map_err(ResolveOwnerForestErrorV1::Function)?;
        let shadow = match resolve_owner_shadow_view_v0(origin, view) {
            Ok(shadow) => shadow,
            Err(ShadowResolveErrorV0::UnresolvedName { name, site }) => {
                if let Some(source) = ancestor_bindings.get(name.as_ref()).copied() {
                    return Err(ResolveOwnerForestErrorV1::UnsupportedCapture {
                        use_site: OwnedExprSiteV1::new(owner, site),
                        source,
                    });
                }
                return Err(ResolveOwnerForestErrorV1::Function(
                    ResolveFunctionErrorV1::Syntax(ShadowResolveErrorV0::UnresolvedName {
                        name,
                        site,
                    }),
                ));
            }
            Err(ShadowResolveErrorV0::UnsupportedExpression { kind: "Me", site }) => {
                if let Some(source) = ancestor_bindings.get("me").copied() {
                    return Err(ResolveOwnerForestErrorV1::UnsupportedCapture {
                        use_site: OwnedExprSiteV1::new(owner, site),
                        source,
                    });
                }
                return Err(ResolveOwnerForestErrorV1::Function(
                    ResolveFunctionErrorV1::Syntax(ShadowResolveErrorV0::UnsupportedExpression {
                        kind: "Me",
                        site,
                    }),
                ));
            }
            Err(error) => {
                return Err(ResolveOwnerForestErrorV1::Function(
                    ResolveFunctionErrorV1::Syntax(error),
                ));
            }
        };
        let ShadowResolvedOwnerV0 { function, lambdas } = shadow;
        let SealedOwnerConstructionV1 {
            product,
            binding_refs,
            scope_ids,
        } = self
            .seal_owner_with_maps(owner, function)
            .map_err(ResolveOwnerForestErrorV1::Function)?;

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

        for lambda in lambdas.into_vec() {
            let child_bindings =
                visible_bindings_for_child(ancestor_bindings, &binding_refs, &lambda);
            let child_parent = PendingParentV1 {
                parent_owner: owner,
                definition_site: OwnedExprSiteV1::new(owner, lambda.definition_site.clone()),
                parent_scope: scope_ids[&lambda.parent_scope],
            };
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
    ancestors: &BTreeMap<Box<str>, BindingRefV1>,
    binding_refs: &BTreeMap<super::shadow::ShadowBindingOrdinalV0, BindingRefV1>,
    lambda: &ShadowLambdaSyntaxV0<'_>,
) -> BTreeMap<Box<str>, BindingRefV1> {
    let mut visible = ancestors.clone();
    for (name, binding) in &lambda.visible_bindings {
        visible.insert(name.clone(), binding_refs[binding]);
    }
    visible
}
