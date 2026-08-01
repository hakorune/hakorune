//! Recursive owner-forest construction for non-capturing nested functions.

use std::collections::{BTreeMap, BTreeSet};

use super::callable_index::VerifiedCallableIndexV1;
use super::function_view::FunctionSyntaxViewV1;
use super::ids::{BindingRefV1, FunctionOwnerIdV1, ScopeId};
use super::owner_construction_tree::{
    construct_function_owner_tree_v1, construct_script_owner_tree_v1, ShadowOwnerConstructionTreeV1,
};
use super::owner_forest::{
    OwnerParentEdgeV1, SemanticOwnerForestDraftV1, SemanticOwnerForestVerificationErrorV1,
    VerifiedSemanticOwnerForestV1,
};
use super::owner_forest_payload::VerifiedSemanticOwnerProductV1;
use super::resolver::{
    AncestorBindingV1, FunctionSemanticResolverSessionV1, ResolveFunctionErrorV1,
    SealedOwnerConstructionV1, SealedScriptConstructionV1,
};
use super::script_view::ScriptSyntaxViewV1;
use super::shadow::{resolve_function_shadow_view_v0, ShadowLambdaSyntaxV0};
use super::{
    EnumVariantDemandV1, FunctionOriginV1, OwnedExprSiteV1, RecordSchemaDemandV1,
    VerifiedScriptRootDemandWindowV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolveOwnerForestErrorV1 {
    Function(ResolveFunctionErrorV1),
    Verification(SemanticOwnerForestVerificationErrorV1),
}

#[derive(Debug)]
pub(crate) enum ResolveScriptForestOutcomeV1 {
    Complete(VerifiedSemanticOwnerForestV1),
    Deferred,
}

#[derive(Debug, Clone)]
struct PendingParentV1 {
    parent_owner: FunctionOwnerIdV1,
    definition_site: OwnedExprSiteV1,
    parent_scope: ScopeId,
}

impl FunctionSemanticResolverSessionV1 {
    pub(crate) fn resolve_script_forest_with_declaration_views(
        &mut self,
        view: ScriptSyntaxViewV1<'_>,
        window: &VerifiedScriptRootDemandWindowV1,
        record_schemas: &dyn RecordSchemaDemandV1,
        enum_variants: &dyn EnumVariantDemandV1,
    ) -> Result<ResolveScriptForestOutcomeV1, ResolveOwnerForestErrorV1> {
        let tree = match construct_script_owner_tree_v1(view, window, record_schemas, enum_variants)
        {
            Ok(tree) => tree,
            Err(_) => return Ok(ResolveScriptForestOutcomeV1::Deferred),
        };
        let (origin, owner) = self
            .issue_owner()
            .map_err(ResolveOwnerForestErrorV1::Function)?;
        let mut draft = SemanticOwnerForestDraftV1::new();
        self.seal_script_owner_tree(tree, owner, origin, &mut draft)?;
        draft
            .seal()
            .map(ResolveScriptForestOutcomeV1::Complete)
            .map_err(ResolveOwnerForestErrorV1::Verification)
    }

    pub(crate) fn resolve_forest(
        &mut self,
        root: FunctionSyntaxViewV1<'_>,
    ) -> Result<VerifiedSemanticOwnerForestV1, ResolveOwnerForestErrorV1> {
        let mut draft = SemanticOwnerForestDraftV1::new();
        let tree = construct_function_owner_tree_v1(root, &BTreeSet::new())
            .map_err(ResolveFunctionErrorV1::Syntax)
            .map_err(ResolveOwnerForestErrorV1::Function)?;
        self.seal_owner_tree(tree, &BTreeMap::new(), None, None, None, &mut draft)?;
        draft
            .seal()
            .map_err(ResolveOwnerForestErrorV1::Verification)
    }

    pub(in crate::mir) fn resolve_forest_with_callable_index(
        &mut self,
        root: FunctionSyntaxViewV1<'_>,
        callable_index: &VerifiedCallableIndexV1,
    ) -> Result<VerifiedSemanticOwnerForestV1, ResolveOwnerForestErrorV1> {
        let mut draft = SemanticOwnerForestDraftV1::new();
        let tree = construct_function_owner_tree_v1(root, &BTreeSet::new())
            .map_err(ResolveFunctionErrorV1::Syntax)
            .map_err(ResolveOwnerForestErrorV1::Function)?;
        self.seal_owner_tree(
            tree,
            &BTreeMap::new(),
            None,
            None,
            Some(callable_index),
            &mut draft,
        )?;
        draft
            .seal()
            .map_err(ResolveOwnerForestErrorV1::Verification)
    }

    pub(in crate::mir) fn resolve_forest_with_reserved_root(
        &mut self,
        root: FunctionSyntaxViewV1<'_>,
        origin: FunctionOriginV1,
        owner: FunctionOwnerIdV1,
        callable_index: &VerifiedCallableIndexV1,
    ) -> Result<VerifiedSemanticOwnerForestV1, ResolveOwnerForestErrorV1> {
        let mut draft = SemanticOwnerForestDraftV1::new();
        let tree = construct_function_owner_tree_v1(root, &BTreeSet::new())
            .map_err(ResolveFunctionErrorV1::Syntax)
            .map_err(ResolveOwnerForestErrorV1::Function)?;
        self.seal_owner_tree(
            tree,
            &BTreeMap::new(),
            None,
            Some((origin, owner)),
            Some(callable_index),
            &mut draft,
        )?;
        draft
            .seal()
            .map_err(ResolveOwnerForestErrorV1::Verification)
    }

    fn seal_owner_tree<'ast>(
        &mut self,
        tree: ShadowOwnerConstructionTreeV1<'ast>,
        ancestor_bindings: &BTreeMap<Box<str>, AncestorBindingV1>,
        parent: Option<PendingParentV1>,
        reserved: Option<(FunctionOriginV1, FunctionOwnerIdV1)>,
        callable_index: Option<&VerifiedCallableIndexV1>,
        draft: &mut SemanticOwnerForestDraftV1,
    ) -> Result<FunctionOwnerIdV1, ResolveOwnerForestErrorV1> {
        let (origin, owner) = match reserved {
            Some(identity) => identity,
            None => self
                .issue_owner()
                .map_err(ResolveOwnerForestErrorV1::Function)?,
        };
        let ShadowOwnerConstructionTreeV1 { function, children } = tree;
        let SealedOwnerConstructionV1 {
            product,
            binding_refs,
            scope_ids,
            ordered_capture_demands,
        } = match callable_index {
            Some(index) => self.seal_owner_with_ancestors_and_callable_index(
                owner,
                origin,
                function,
                ancestor_bindings,
                index,
            ),
            None => self.seal_owner_with_ancestors(owner, origin, function, ancestor_bindings),
        }
        .map_err(ResolveOwnerForestErrorV1::Function)?;

        let children = children
            .into_iter()
            .map(|child| {
                let child_bindings =
                    visible_bindings_for_child(ancestor_bindings, &binding_refs, &child.lambda);
                let child_parent = PendingParentV1 {
                    parent_owner: owner,
                    definition_site: OwnedExprSiteV1::new(
                        owner,
                        child.lambda.definition_site.clone(),
                    ),
                    parent_scope: scope_ids[&child.lambda.parent_scope],
                };
                (child.tree, child_bindings, child_parent)
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
        draft
            .insert_ordered_capture_demands(owner, ordered_capture_demands)
            .map_err(ResolveOwnerForestErrorV1::Verification)?;

        for (child, child_bindings, child_parent) in children {
            self.seal_owner_tree(
                *child,
                &child_bindings,
                Some(child_parent),
                None,
                callable_index,
                draft,
            )?;
        }
        Ok(owner)
    }

    fn seal_script_owner_tree<'ast>(
        &mut self,
        tree: ShadowOwnerConstructionTreeV1<'ast>,
        owner: FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        draft: &mut SemanticOwnerForestDraftV1,
    ) -> Result<(), ResolveOwnerForestErrorV1> {
        let ShadowOwnerConstructionTreeV1 { function, children } = tree;
        let SealedScriptConstructionV1 {
            product,
            binding_refs,
            scope_ids,
            ordered_capture_demands,
        } = self
            .seal_script_owner_with_maps(owner, origin, function)
            .map_err(ResolveOwnerForestErrorV1::Function)?;
        let children = children
            .into_iter()
            .map(|child| {
                let child_bindings =
                    visible_bindings_for_child(&BTreeMap::new(), &binding_refs, &child.lambda);
                let child_parent = PendingParentV1 {
                    parent_owner: owner,
                    definition_site: OwnedExprSiteV1::new(
                        owner,
                        child.lambda.definition_site.clone(),
                    ),
                    parent_scope: scope_ids[&child.lambda.parent_scope],
                };
                (child.tree, child_bindings, child_parent)
            })
            .collect::<Vec<_>>();
        draft
            .insert_product(owner, VerifiedSemanticOwnerProductV1::Script(product))
            .map_err(ResolveOwnerForestErrorV1::Verification)?;
        draft
            .insert_ordered_capture_demands(owner, ordered_capture_demands)
            .map_err(ResolveOwnerForestErrorV1::Verification)?;
        for (child, child_bindings, child_parent) in children {
            self.seal_owner_tree(
                *child,
                &child_bindings,
                Some(child_parent),
                None,
                None,
                draft,
            )?;
        }
        Ok(())
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
