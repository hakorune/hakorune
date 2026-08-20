//! Recursive owner-forest construction for non-capturing nested functions.

use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use std::collections::{BTreeMap, BTreeSet};

use super::callable_index::VerifiedCallableIndexV1;
use super::function_view::FunctionSyntaxViewV1;
use super::ids::{BindingRefV1, FunctionOwnerIdV1, ScopeId};
use super::owner_construction_tree::{
    construct_function_owner_tree_v1, construct_script_owner_tree_v1,
    construct_selected_callable_owner_tree_v1, ShadowOwnerConstructionTreeV1,
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
use super::VerifiedResolvedBodyShapeInventoryV1;
use super::{
    EnumMatchDemandV1, EnumVariantDemandV1, FunctionOriginV1, OwnedExprSiteV1,
    RecordSchemaDemandV1, VerifiedScriptRootDemandWindowV1,
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

#[derive(Debug)]
pub(crate) enum ResolveSelectedCallableForestsOutcomeV1 {
    Complete(Box<[VerifiedSemanticOwnerForestV1]>),
    Deferred,
}

#[derive(Debug)]
pub(crate) enum ResolveSelectedCallableForestsWithBodyShapesOutcomeV1 {
    Complete {
        forests: Box<[VerifiedSemanticOwnerForestV1]>,
        body_shapes: BTreeMap<FunctionOwnerIdV1, VerifiedResolvedBodyShapeInventoryV1>,
    },
    Deferred,
}

#[derive(Debug, Clone)]
struct PendingParentV1 {
    parent_owner: FunctionOwnerIdV1,
    definition_site: OwnedExprSiteV1,
    parent_scope: ScopeId,
}

impl FunctionSemanticResolverSessionV1 {
    /// Traverses the complete batch before issuing any canonical owner.
    pub(crate) fn resolve_selected_callable_forests(
        &mut self,
        roots: &[FunctionSyntaxViewV1<'_>],
    ) -> Result<ResolveSelectedCallableForestsOutcomeV1, ResolveOwnerForestErrorV1> {
        match self.resolve_selected_callable_forests_with_body_shapes(roots)? {
            ResolveSelectedCallableForestsWithBodyShapesOutcomeV1::Complete { forests, .. } => {
                Ok(ResolveSelectedCallableForestsOutcomeV1::Complete(forests))
            }
            ResolveSelectedCallableForestsWithBodyShapesOutcomeV1::Deferred => {
                Ok(ResolveSelectedCallableForestsOutcomeV1::Deferred)
            }
        }
    }

    pub(crate) fn resolve_selected_callable_forests_with_body_shapes(
        &mut self,
        roots: &[FunctionSyntaxViewV1<'_>],
    ) -> Result<ResolveSelectedCallableForestsWithBodyShapesOutcomeV1, ResolveOwnerForestErrorV1>
    {
        self.resolve_selected_callable_forests_with_body_shapes_and_brand_catalog(roots, None)
    }

    pub(crate) fn resolve_selected_callable_forests_with_body_shapes_and_brand_catalog(
        &mut self,
        roots: &[FunctionSyntaxViewV1<'_>],
        brand_catalog: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
    ) -> Result<ResolveSelectedCallableForestsWithBodyShapesOutcomeV1, ResolveOwnerForestErrorV1>
    {
        let mut trees = Vec::with_capacity(roots.len());
        let mut deferred = false;
        for root in roots {
            match construct_selected_callable_owner_tree_v1(*root, brand_catalog) {
                Ok(tree) => trees.push(tree),
                Err(error) => deferred |= selected_callable_source_deferral(error)?,
            }
        }
        if deferred {
            return Ok(ResolveSelectedCallableForestsWithBodyShapesOutcomeV1::Deferred);
        }
        let mut forests = Vec::with_capacity(trees.len());
        let mut body_shapes = BTreeMap::new();
        for tree in trees {
            let mut draft = SemanticOwnerForestDraftV1::new();
            self.seal_owner_tree(
                tree,
                &BTreeMap::new(),
                None,
                None,
                None,
                &mut draft,
                Some(&mut body_shapes),
            )?;
            forests.push(
                draft
                    .seal()
                    .map_err(ResolveOwnerForestErrorV1::Verification)?,
            );
        }
        Ok(
            ResolveSelectedCallableForestsWithBodyShapesOutcomeV1::Complete {
                forests: forests.into_boxed_slice(),
                body_shapes,
            },
        )
    }

    pub(crate) fn resolve_script_forest_with_declaration_views(
        &mut self,
        view: ScriptSyntaxViewV1<'_>,
        window: &VerifiedScriptRootDemandWindowV1,
        record_schemas: &dyn RecordSchemaDemandV1,
        enum_variants: &dyn EnumVariantDemandV1,
        enum_matches: &dyn EnumMatchDemandV1,
        brand_catalog: &VerifiedBrandProgramDeclarationCatalogV1,
    ) -> Result<ResolveScriptForestOutcomeV1, ResolveOwnerForestErrorV1> {
        let tree = match construct_script_owner_tree_v1(
            view,
            window,
            record_schemas,
            enum_variants,
            enum_matches,
            brand_catalog,
        ) {
            Ok(tree) => tree,
            Err(error) if error.is_script_source_deferral() => {
                return Ok(ResolveScriptForestOutcomeV1::Deferred)
            }
            Err(error) => {
                return Err(ResolveOwnerForestErrorV1::Function(
                    ResolveFunctionErrorV1::ScriptInvariant(error),
                ))
            }
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
        self.seal_owner_tree(tree, &BTreeMap::new(), None, None, None, &mut draft, None)?;
        draft
            .seal()
            .map_err(ResolveOwnerForestErrorV1::Verification)
    }

    pub(in crate::mir) fn resolve_forest_with_body_shapes(
        &mut self,
        root: FunctionSyntaxViewV1<'_>,
    ) -> Result<
        (
            VerifiedSemanticOwnerForestV1,
            BTreeMap<FunctionOwnerIdV1, VerifiedResolvedBodyShapeInventoryV1>,
        ),
        ResolveOwnerForestErrorV1,
    > {
        let mut draft = SemanticOwnerForestDraftV1::new();
        let tree = construct_function_owner_tree_v1(root, &BTreeSet::new())
            .map_err(ResolveFunctionErrorV1::Syntax)
            .map_err(ResolveOwnerForestErrorV1::Function)?;
        let mut body_shapes = BTreeMap::new();
        self.seal_owner_tree(
            tree,
            &BTreeMap::new(),
            None,
            None,
            None,
            &mut draft,
            Some(&mut body_shapes),
        )?;
        let forest = draft
            .seal()
            .map_err(ResolveOwnerForestErrorV1::Verification)?;
        Ok((forest, body_shapes))
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
            None,
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
            None,
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
        mut body_shapes: Option<
            &mut BTreeMap<FunctionOwnerIdV1, VerifiedResolvedBodyShapeInventoryV1>,
        >,
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
            body_shape,
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
        if let Some(shapes) = body_shapes.as_deref_mut() {
            shapes.insert(owner, body_shape);
        }

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
                body_shapes.as_deref_mut(),
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
                None,
            )?;
        }
        Ok(())
    }
}

fn selected_callable_source_deferral(
    error: super::shadow::ShadowResolveErrorV0,
) -> Result<bool, ResolveOwnerForestErrorV1> {
    if error.is_script_source_deferral() {
        Ok(true)
    } else {
        Err(ResolveOwnerForestErrorV1::Function(
            ResolveFunctionErrorV1::Syntax(error),
        ))
    }
}

#[cfg(test)]
mod selected_callable_tests {
    use super::selected_callable_source_deferral;
    use crate::mir::resolved_semantics::shadow::ShadowResolveErrorV0;
    use crate::mir::resolved_semantics::SourcePathV1;

    #[test]
    fn later_invariant_is_not_hidden_by_an_earlier_source_deferral() {
        let deferred = ShadowResolveErrorV0::UnsupportedStatement {
            kind: "test-deferred",
            site: SourcePathV1::function_body().stmt(),
        };
        assert!(selected_callable_source_deferral(deferred).unwrap());
        let invariant = ShadowResolveErrorV0::DuplicateExitSite {
            site: SourcePathV1::function_body().stmt(),
        };
        assert!(selected_callable_source_deferral(invariant).is_err());
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
