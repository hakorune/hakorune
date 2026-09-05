//! Resolver-owned semantic rows for parser-issued instance constructors.

use std::collections::BTreeMap;
use hakorune_mir_defs::CanonicalObjectIdV1;
use crate::mir::function::CanonicalObjectDefinitionV1;
mod object_definition;
use super::instance_construction::{issue_construction_plan, ConstructionEligibilityV1};

use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_control_flow::{
    verify_function_completion_v1, FunctionCompletionVerificationErrorV1,
    VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, ReceiverPolicyV1,
    ResolveSourceBoundSelectedCallableForestsWithBodyShapesOutcomeV1,
    SelectedCallableResolverDeferredBatchV1, SelectedCallableResolverInputV1,
    SemanticOwnerRootProfileV1, SourceBoundSelectedCallableResolverRejectV1,
    FunctionOwnerIdV1, VerifiedResolvedBodyShapeInventoryV1, VerifiedSemanticOwnerForestV1,
    DeclaredInstanceCallSemanticEffectV1,
};
use crate::parser::{
    ConstructorSourceIdV1, ConstructorSourceKindV1, VerifiedFinalCallableProgramSourceV1,
    ParserOrdinaryBoxSourceCoverageV1, ParserOrdinaryBoxSourceRowV1,
};

#[derive(Debug)]
pub(crate) enum InstanceConstructorSemanticBatchIssueV1 {
    ParserSyntax,
    SourceCoverage,
    Resolver(SourceBoundSelectedCallableResolverRejectV1),
    ResolverDeferred(SelectedCallableResolverDeferredBatchV1),
    MissingRoot,
    RootProfileMismatch,
    BodyShapeMissing,
    BodyShapeOwnerMismatch,
    BodyShapeResidual,
    Completion { _issue: FunctionCompletionVerificationErrorV1 },
    SourceProjection { _error: String },
    ReceiverNonEscape { _issue: super::instance_constructor_non_escape::BirthReceiverNonEscapeIssueV1 },
}

#[derive(Debug)]
pub(crate) struct VerifiedInstanceConstructorSemanticRowV1 {
    source_id: ConstructorSourceIdV1,
    box_source: ParserOrdinaryBoxSourceRowV1,
    published_birth_key: Option<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1>,
    final_box_ordinal: u32,
    box_name: Box<str>,
    key: Box<str>,
    kind: ConstructorSourceKindV1,
    source_arity: u32,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    body_shapes: BTreeMap<FunctionOwnerIdV1, VerifiedResolvedBodyShapeInventoryV1>,
    birth_completion: Option<VerifiedFunctionCompletionV1>,
    birth_effect: Option<DeclaredInstanceCallSemanticEffectV1>,
    construction: ConstructionEligibilityV1,
}

#[derive(Debug)]
pub(crate) struct VerifiedInstanceConstructorSemanticBatchV1 {
    rows: Box<[VerifiedInstanceConstructorSemanticRowV1]>,
    box_sources: ParserOrdinaryBoxSourceCoverageV1,
    object_sources: Box<[(ParserOrdinaryBoxSourceRowV1, CanonicalObjectIdV1)]>,
    object_definitions: std::cell::RefCell<Option<Box<[CanonicalObjectDefinitionV1]>>>,
    no_birth_construction: Vec<(ParserOrdinaryBoxSourceRowV1, ConstructionEligibilityV1)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InstanceConstructorBirthLookupErrorV1 {
    SourceArityOverflow,
    DuplicateBirth,
    ParentSourceMismatch,
    BirthArityMismatch,
}

impl VerifiedInstanceConstructorSemanticBatchV1 {
    pub(super) fn has_pending_object_definitions(&self) -> bool {
        self.object_definitions.borrow().is_some()
    }

    pub(super) fn take_object_definitions(&self) -> Option<Box<[CanonicalObjectDefinitionV1]>> {
        self.object_definitions.borrow_mut().take()
    }

    pub(crate) fn object_for(
        &self,
        source: &ParserOrdinaryBoxSourceRowV1,
    ) -> Result<CanonicalObjectIdV1, InstanceConstructorBirthLookupErrorV1> {
        self.object_sources.iter()
            .find(|(own, _)| own.same_source_as(source))
            .map(|(_, id)| *id)
            .ok_or(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)
    }

    pub(crate) fn rows(&self) -> &[VerifiedInstanceConstructorSemanticRowV1] {
        &self.rows
    }

    pub(crate) fn birth_for(
        &self,
        box_source: &ParserOrdinaryBoxSourceRowV1,
        source_arity: usize,
    ) -> Result<
        Option<&VerifiedInstanceConstructorSemanticRowV1>,
        InstanceConstructorBirthLookupErrorV1,
    > {
        let source_arity = u32::try_from(source_arity)
            .map_err(|_| InstanceConstructorBirthLookupErrorV1::SourceArityOverflow)?;
        if !matches!(self.box_sources.row_for(box_source.name()),
            Ok(Some(own)) if own.same_source_as(box_source)) {
            return Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch);
        }
        if self.rows.iter().any(|row| {
            row.final_box_ordinal as usize == box_source.final_box_ordinal()
                && !row.box_source.same_source_as(box_source)
        }) {
            return Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch);
        }
        let mut matches = self.rows.iter().filter(|row| {
            row.box_source.same_source_as(box_source)
                && row.kind == ConstructorSourceKindV1::Birth
                && row.source_arity == source_arity
        });
        let Some(row) = matches.next() else {
            if self.rows.iter().any(|row| row.box_source.same_source_as(box_source)
                && row.kind == ConstructorSourceKindV1::Birth) {
                return Err(InstanceConstructorBirthLookupErrorV1::BirthArityMismatch);
            }
            return Ok(None);
        };
        if matches.next().is_some() {
            return Err(InstanceConstructorBirthLookupErrorV1::DuplicateBirth);
        }
        Ok(Some(row))
    }

    pub(crate) fn construction_for(
        &self, parent: &ParserOrdinaryBoxSourceRowV1, arity: usize,
    ) -> Result<&ConstructionEligibilityV1, InstanceConstructorBirthLookupErrorV1> {
        if let Some(row) = self.birth_for(parent, arity)? {
            return Ok(&row.construction);
        }
        self.no_birth_construction.iter()
            .find(|(own, _)| own.same_source_as(parent))
            .map(|(_, plan)| plan)
            .ok_or(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)
    }
}

impl VerifiedInstanceConstructorSemanticRowV1 {
    pub(crate) fn published_birth_key(
        &self,
    ) -> Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1> {
        self.published_birth_key.as_ref()
    }

    pub(crate) fn source_id(&self) -> &ConstructorSourceIdV1 {
        &self.source_id
    }

    pub(crate) const fn final_box_ordinal(&self) -> u32 {
        self.final_box_ordinal
    }

    pub(crate) fn box_name(&self) -> &str {
        &self.box_name
    }

    pub(crate) fn key(&self) -> &str {
        &self.key
    }

    pub(crate) const fn kind(&self) -> ConstructorSourceKindV1 {
        self.kind
    }

    pub(crate) const fn source_arity(&self) -> u32 {
        self.source_arity
    }

    pub(crate) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }

    pub(crate) fn birth_completion(&self) -> Option<&VerifiedFunctionCompletionV1> {
        self.birth_completion.as_ref()
    }

    pub(crate) const fn birth_effect(&self) -> Option<DeclaredInstanceCallSemanticEffectV1> {
        self.birth_effect
    }

    pub(crate) fn lowering_input<'a>(
        &'a self,
        source: &'a ASTNode,
    ) -> Result<ResolvedFunctionLoweringInputV1<'a>, String> {
        let ASTNode::Program { statements, .. } = source else {
            return Err("[freeze:contract][mir/instance-constructor-semantic/program]".to_owned());
        };
        let Some(ASTNode::BoxDeclaration {
            name, constructors, ..
        }) = statements.get(self.final_box_ordinal as usize)
        else {
            return Err("[freeze:contract][mir/instance-constructor-semantic/box]".to_owned());
        };
        if name != self.box_name.as_ref() {
            return Err("[freeze:contract][mir/instance-constructor-semantic/box-name]".to_owned());
        }
        let Some(function) = constructors.get(self.key.as_ref()) else {
            return Err("[freeze:contract][mir/instance-constructor-semantic/key]".to_owned());
        };
        let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
            function,
            &self.forest,
            &self.projection,
        )
        .map_err(|error| {
            format!("[freeze:contract][mir/instance-constructor-semantic/input] {error:?}")
        })?;
        let [root] = self.forest.roots() else {
            return Err("[freeze:contract][mir/instance-constructor-semantic/root]".to_owned());
        };
        if !std::ptr::eq(
            self.projection
                .owner_root(function, *root)
                .map_err(|error| error.to_string())?,
            function,
        ) {
            return Err(
                "[freeze:contract][mir/instance-constructor-semantic/root-identity]".to_owned(),
            );
        }
        let shape = self.body_shapes.get(root).ok_or_else(|| {
            "[freeze:contract][mir/instance-constructor-semantic/body-shape]".to_owned()
        })?;
        if shape.owner() != *root
            || shape.body_root() != &input.function().root_profile().body_root()
        {
            return Err(
                "[freeze:contract][mir/instance-constructor-semantic/body-shape-owner]".to_owned(),
            );
        }
        if self.kind == ConstructorSourceKindV1::Birth
            && self.birth_completion().map(|completion| completion.owner()) != Some(*root)
        {
            return Err(
                "[freeze:contract][mir/instance-constructor-semantic/completion-owner]".to_owned(),
            );
        }
        Ok(input.with_body_shape(shape))
    }
}

pub(crate) fn issue_instance_constructor_semantic_batch_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: &VerifiedFinalCallableProgramSourceV1,
    brand_catalog: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
) -> Result<VerifiedInstanceConstructorSemanticBatchV1, InstanceConstructorSemanticBatchIssueV1> {
    let mut object_sources = Vec::new();
    let mut object_definitions = Vec::new();
    for (index, parent) in source.ordinary_box_coverage().rows().iter().enumerate() {
        let definition = source.with_ordinary_box_syntax(parent, object_definition::issue)
            .map_err(|_| InstanceConstructorSemanticBatchIssueV1::SourceCoverage)??;
        if object_sources.iter().any(|(own, _): &(ParserOrdinaryBoxSourceRowV1, CanonicalObjectIdV1)|
            own.same_source_as(parent)) {
            return Err(InstanceConstructorSemanticBatchIssueV1::SourceCoverage);
        }
        let id = CanonicalObjectIdV1::from_declaration_index(index)
            .ok_or(InstanceConstructorSemanticBatchIssueV1::SourceCoverage)?;
        object_sources.push((parent.clone(), id));
        object_definitions.push(definition);
    }
    source
        .with_constructor_semantic_syntax(|loan| {
            let mut no_birth_construction = Vec::new();
            for (parent, object_id) in &object_sources {
                if loan.rows().iter().any(|row| row.kind() == ConstructorSourceKindV1::Birth
                    && row.box_source().same_source_as(parent)) {
                    continue;
                }
                let plan = source.with_ordinary_box_syntax(parent, |declaration| {
                    issue_construction_plan(*object_id, parent, declaration, None)
                }).map_err(|_| InstanceConstructorSemanticBatchIssueV1::SourceCoverage)?;
                no_birth_construction.push((parent.clone(), plan));
            }
            let mut candidates = Vec::with_capacity(loan.rows().len());
            let mut resolver_inputs = Vec::with_capacity(loan.rows().len());
            for syntax in loan.rows() {
                let ASTNode::FunctionDeclaration { params, body, .. } = syntax.declaration() else {
                    return Err(InstanceConstructorSemanticBatchIssueV1::SourceCoverage);
                };
                let view = FunctionSyntaxViewV1::from_borrowed_function_parts(
                    params,
                    body,
                    ReceiverPolicyV1::DeclaredInstance,
                );
                candidates.push((
                    syntax.source_id().clone(),
                    syntax.box_source().clone(),
                    syntax.final_box_ordinal(),
                    Box::<str>::from(syntax.box_name()),
                    Box::<str>::from(syntax.key()),
                    syntax.kind(),
                    syntax.source_arity(),
                    syntax.declaration(),
                    view,
                ));
                resolver_inputs.push(SelectedCallableResolverInputV1::constructor(
                    syntax.source_id().clone(),
                    syntax.box_name(),
                    syntax.key(),
                    view,
                ));
            }
            let (forests, mut body_shapes) = match resolver
                .resolve_source_bound_selected_callable_forests_with_body_shapes_and_brand_catalog(
                    &resolver_inputs,
                    brand_catalog,
                )
                .map_err(InstanceConstructorSemanticBatchIssueV1::Resolver)?
            {
                ResolveSourceBoundSelectedCallableForestsWithBodyShapesOutcomeV1::Complete {
                    forests,
                    body_shapes,
                } => (forests, body_shapes),
                ResolveSourceBoundSelectedCallableForestsWithBodyShapesOutcomeV1::Deferred(
                    deferred,
                ) => {
                    return Err(InstanceConstructorSemanticBatchIssueV1::ResolverDeferred(
                        deferred,
                    ))
                }
            };
            if forests.len() != candidates.len() {
                return Err(InstanceConstructorSemanticBatchIssueV1::SourceCoverage);
            }
            let mut rows = Vec::with_capacity(forests.len());
            for (
                (
                    source_id,
                    box_source,
                    final_box_ordinal,
                    box_name,
                    key,
                    kind,
                    source_arity,
                    declaration,
                    view,
                ),
                forest,
            ) in candidates.into_iter().zip(forests)
            {
                let [root] = forest.roots() else {
                    return Err(InstanceConstructorSemanticBatchIssueV1::MissingRoot);
                };
                let function = forest
                    .owner(*root)
                    .ok_or(InstanceConstructorSemanticBatchIssueV1::MissingRoot)?;
                if function.root_profile() != view.root_profile()
                    || !matches!(
                        function.root_profile(),
                        SemanticOwnerRootProfileV1::DeclaredFunction {
                            receiver_policy: ReceiverPolicyV1::DeclaredInstance
                        }
                    )
                {
                    return Err(InstanceConstructorSemanticBatchIssueV1::RootProfileMismatch);
                }
                let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
                    declaration,
                    &forest,
                    view.root_profile(),
                )
                .map_err(|error| {
                    InstanceConstructorSemanticBatchIssueV1::SourceProjection {
                        _error: error.to_string(),
                    }
                })?;
                let mut constructor_shapes = BTreeMap::new();
                for (owner, product) in forest.owners() {
                    let shape = body_shapes
                        .remove(&owner)
                        .ok_or(InstanceConstructorSemanticBatchIssueV1::BodyShapeMissing)?;
                    if shape.owner() != owner
                        || shape.body_root() != &product.root_profile().body_root()
                    {
                        return Err(InstanceConstructorSemanticBatchIssueV1::BodyShapeOwnerMismatch);
                    }
                    constructor_shapes.insert(owner, shape);
                }
                let birth_completion = if kind == ConstructorSourceKindV1::Birth {
                    let shape = constructor_shapes.get(root)
                        .ok_or(InstanceConstructorSemanticBatchIssueV1::BodyShapeMissing)?;
                    super::instance_constructor_non_escape::verify_birth_receiver_non_escape_v1(
                        function, shape, &forest,
                    ).map_err(|_issue| InstanceConstructorSemanticBatchIssueV1::ReceiverNonEscape {
                        _issue,
                    })?;
                    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
                        declaration,
                        &forest,
                        &projection,
                    )
                    .map_err(|error| InstanceConstructorSemanticBatchIssueV1::SourceProjection {
                        _error: format!("{error:?}"),
                    })?;
                    Some(verify_function_completion_v1(input).map_err(|_issue| {
                        InstanceConstructorSemanticBatchIssueV1::Completion { _issue }
                    })?)
                } else {
                    None
                };
                let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
                    declaration, &forest, &projection,
                ).map_err(|error| InstanceConstructorSemanticBatchIssueV1::SourceProjection {
                    _error: format!("{error:?}"),
                })?.with_body_shape(constructor_shapes.get(root)
                    .ok_or(InstanceConstructorSemanticBatchIssueV1::BodyShapeMissing)?);
                let construction = if kind == ConstructorSourceKindV1::Birth {
                    let object_id = object_sources.iter()
                        .find(|(own, _)| own.same_source_as(&box_source))
                        .map(|(_, id)| *id)
                        .ok_or(InstanceConstructorSemanticBatchIssueV1::SourceCoverage)?;
                    source.with_ordinary_box_syntax(&box_source, |parent| {
                        issue_construction_plan(object_id, &box_source, parent, Some((&source_id, input)))
                    }).map_err(|_| InstanceConstructorSemanticBatchIssueV1::SourceCoverage)?
                } else {
                    Err(super::instance_construction::ConstructionUnavailableV1::BodyCoverageUnsupported)
                };
                rows.push(VerifiedInstanceConstructorSemanticRowV1 {
                    construction,
                    // Only the exact unannotated source contract is selected.
                    // Explicit constructor contracts need their own admission;
                    // never turn them into the implicit opaque default.
                    birth_effect: match declaration {
                        ASTNode::FunctionDeclaration { attrs, contracts, .. }
                            if kind == ConstructorSourceKindV1::Birth
                                && attrs.is_empty() && contracts.is_empty() =>
                        {
                            Some(DeclaredInstanceCallSemanticEffectV1::OpaqueObservable)
                        }
                        _ => None,
                    },
                    published_birth_key: (kind == ConstructorSourceKindV1::Birth).then(|| {
                        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor(
                            &box_name,
                            source_arity,
                        )
                    }),
                    source_id,
                    box_source,
                    final_box_ordinal,
                    box_name,
                    key,
                    kind,
                    source_arity,
                    forest,
                    projection,
                    body_shapes: constructor_shapes,
                    birth_completion,
                });
            }
            if !body_shapes.is_empty() {
                return Err(InstanceConstructorSemanticBatchIssueV1::BodyShapeResidual);
            }
            Ok(VerifiedInstanceConstructorSemanticBatchV1 {
                rows: rows.into_boxed_slice(),
                box_sources: source.ordinary_box_coverage().clone(),
                object_sources: object_sources.into_boxed_slice(),
                object_definitions: std::cell::RefCell::new(Some(object_definitions.into_boxed_slice())),
                no_birth_construction,
            })
        })
        .map_err(|_| InstanceConstructorSemanticBatchIssueV1::ParserSyntax)?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_transfer_rejects_foreign_context_and_missing_empty_payload() {
        use crate::mir::builder::CompilationContext;
        use super::super::NormalCallableSemanticPackageInstallIssueV1;
        let issue = || super::super::brand_catalog_tests::issue_with_brand_catalog("box Empty {}").unwrap();
        let mut own_context = CompilationContext::new();
        let own = issue().prepare_install(&mut own_context).unwrap().commit();
        let mut foreign_context = CompilationContext::new();
        let _foreign = issue().prepare_install(&mut foreign_context).unwrap().commit();
        let mut port = own.begin_lowering(&own_context).unwrap();
        assert!(port.take_object_definitions(&foreign_context).unwrap_err()
            .contains("object-definitions/foreign-package"));
        assert!(own.instance_constructors().has_pending_object_definitions());
        let payload = port.take_object_definitions(&own_context).unwrap();
        assert_eq!(payload.len(), 1);
        assert!(payload[0].fields().is_empty());
        assert!(port.take_object_definitions(&own_context).is_err());
        port.complete().unwrap();

        let mut pending_context = CompilationContext::new();
        let pending = issue().prepare_install(&mut pending_context).unwrap().commit();
        assert_eq!(pending.begin_lowering(&pending_context).unwrap().complete(),
            Err(NormalCallableSemanticPackageInstallIssueV1::ObjectDefinitionsNotConsumed));
    }

    #[test]
    fn object_identity_covers_distinct_boxes_and_rejects_foreign_same_index() {
        let text = "box First { value: i64\nbirth(x) { me.value = x } }\nbox Second { value: i64\nbirth(x) { me.value = x } }\nbox Empty {}";
        let own = super::super::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
        let foreign = super::super::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
        let batch = &own.instance_constructors;
        let mut ids = std::collections::BTreeSet::new();
        for (index, name) in ["First", "Second", "Empty"].into_iter().enumerate() {
            let source = batch.box_sources.row_for(name).unwrap().unwrap();
            let id = batch.object_for(source).unwrap();
            assert_eq!(id.declaration_index() as usize, index);
            assert!(ids.insert(id));
            let arity = if name == "Empty" { 0 } else { 1 };
            let plan = batch.construction_for(source, arity).unwrap().as_ref().unwrap();
            assert_eq!(plan.object(), id);
            assert!(plan.stores().iter().all(|(_, field)| field.object() == id));
            let foreign_source = foreign.instance_constructors.box_sources.row_for(name).unwrap().unwrap();
            assert_eq!(foreign.instance_constructors.object_for(foreign_source).unwrap(), id,
                "the raw number is module-local, not a membership proof");
            assert_eq!(batch.object_for(foreign_source),
                Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch));
        }
        assert_eq!(ids.len(), 3);
        let definitions = batch.take_object_definitions().unwrap();
        assert_eq!(definitions.len(), 3);
        assert_eq!(definitions[0].diagnostic_name(), "First");
        assert_eq!(definitions[1].fields()[0].declared_type_name.as_deref(), Some("i64"));
        assert!(definitions[2].fields().is_empty());
        assert!(batch.take_object_definitions().is_none());
        assert_eq!(batch.object_sources.len(), 3, "taking payload preserves exact claim linkage");
    }

    #[test]
    fn construction_plan_retains_declaration_order_and_source_store_cutpoints() {
        for (fields, body, expected) in [
            ("left: i64\nright: i64", "me.left = value\nme.right = 2", vec![0, 1]),
            ("east: i64\nwest: i64", "me.west = 2\nme.east = value", vec![1, 0]),
        ] {
            let source = format!("box Page {{ {fields}\nbirth(value) {{ {body} }} }}");
            let package = super::super::brand_catalog_tests::issue_with_brand_catalog(&source).unwrap();
            let batch = &package.instance_constructors;
            let parent = batch.box_sources.row_for("Page").unwrap().unwrap();
            let plan = batch.construction_for(parent, 1).unwrap().as_ref().unwrap();
            assert_eq!(plan.field_demands(), &[crate::mir::resolved_semantics::HomeDemandV1::Trivial; 2]);
            assert_eq!(plan.stores().iter().map(|(_, field)| field.declaration_ordinal()).collect::<Vec<_>>(), expected);
            assert_eq!(plan.object(), batch.object_for(parent).unwrap());
            assert!(plan.stores().iter().all(|(_, field)| field.object() == plan.object()));
            assert_ne!(plan.stores()[0].0.statement_site(), plan.stores()[1].0.statement_site());
            assert!(plan.reclaims_unpublished_outer_storage());
            let row = batch.birth_for(parent, 1).unwrap().unwrap();
            assert_eq!(plan.constructor(), Some(&(row.source_id().clone(), row.forest.roots()[0])));
        }
        let package = super::super::brand_catalog_tests::issue_with_brand_catalog("box Empty {}").unwrap();
        let batch = &package.instance_constructors;
        let parent = batch.box_sources.row_for("Empty").unwrap().unwrap();
        let plan = batch.construction_for(parent, 0).unwrap().as_ref().unwrap();
        assert!(plan.field_demands().is_empty() && plan.stores().is_empty());
        assert!(plan.constructor().is_none());
        assert!(plan.reclaims_unpublished_outer_storage(), "no fields is not no construction cleanup");
    }

    #[test]
    fn construction_plan_keeps_unavailable_dependencies_out_of_empty_cleanup() {
        use super::super::instance_construction::ConstructionUnavailableV1 as U;
        let mut failures = Vec::new();
        for (source, expected) in [
            ("box Page { value: i64 }", U::InitializationContractMissing),
            ("box Page { value: i64\nbirth() {} }", U::InitializationContractMissing),
            ("box Page { value: i64\nbirth() { return } }", U::InitializationContractMissing),
            ("box Page { value\nbirth() { me.value = 1 } }", U::FieldContractUnsupported),
            ("box Page { value: i64 = 1\nbirth() {} }", U::FieldContractUnsupported),
            ("box Page { value: i64 = 1 }", U::FieldContractUnsupported),
            ("box Page { value: i64\nbirth() { me.value = [1] } }", U::BodyCoverageUnsupported),
            ("box Page { value: i64\nbirth(other) { other.value = 1 } }", U::BodyCoverageUnsupported),
            ("box Page { value: i64\nbirth() { me.value = new Page() } }", U::BodyCoverageUnsupported),
            ("box Page { value: i64\nbirth() { me.value = fn() { return 1 } } }", U::BodyCoverageUnsupported),
            ("box Page { value: i64\nbirth() { me.value = 1 + 2 } }", U::BodyCoverageUnsupported),
            ("box Page { value: i64\nbirth() { me.value += 1 } }", U::BodyCoverageUnsupported),
            ("box Page { value: i64\nbirth() { me.value = 1\nme.value = 2 } }", U::BodyCoverageUnsupported),
            ("box Page { value: i64\nbirth() { local x = 1\nme.value = x } }", U::BodyCoverageUnsupported),
            ("box Page { value: i64\nbirth() { me.other = 1 } }", U::SourceRelationMissing),
        ] {
            let package = match super::super::brand_catalog_tests::issue_with_brand_catalog(source) {
                Ok(package) => package,
                Err(error) => { failures.push(format!("source {source}: {error:?}")); continue; }
            };
            let batch = &package.instance_constructors;
            let parent = batch.box_sources.row_for("Page").unwrap().unwrap();
            let arity = batch.rows.iter().find(|row| row.kind() == ConstructorSourceKindV1::Birth)
                .map_or(0, |row| row.source_arity() as usize);
            let actual = batch.construction_for(parent, arity).unwrap();
            if actual != &Err(expected) {
                failures.push(format!("{source}: expected {expected:?}, got {actual:?}"));
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn construction_plan_rejects_foreign_parent_and_birth_arity_not_as_no_birth() {
        let source = "box Page { value: i64\nbirth(value) { me.value = value } }";
        let own = super::super::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
        let foreign = super::super::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
        let batch = &own.instance_constructors;
        let foreign_parent = foreign.instance_constructors.box_sources.row_for("Page").unwrap().unwrap();
        assert_eq!(batch.construction_for(foreign_parent, 1),
            Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch));
        let parent = batch.box_sources.row_for("Page").unwrap().unwrap();
        assert_eq!(batch.construction_for(parent, 0),
            Err(InstanceConstructorBirthLookupErrorV1::BirthArityMismatch));
    }

    #[test]
    fn constructor_lookup_rejects_foreign_or_mismatched_parent_not_as_no_birth() {
        for source in ["box Page { birth() {} }", "box Page {}"] {
            let own = super::super::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
            let foreign = super::super::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
            let row = foreign.batch().ordinary_box_coverage().row_for("Page").unwrap().unwrap();
            assert!(matches!(own.instance_constructors.birth_for(row, 0),
                Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)));
        }
        let mut package = super::super::brand_catalog_tests::issue_with_brand_catalog(
            "box Page { birth() {} } box Other { birth() {} }",
        ).unwrap();
        let batch = &mut package.instance_constructors;
        let page = batch.box_sources.row_for("Page").unwrap().unwrap().clone();
        let other = batch.box_sources.row_for("Other").unwrap().unwrap().clone();
        batch.rows[0].box_source = other;
        assert!(matches!(batch.birth_for(&page, 0),
            Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)));
    }

    #[test]
    fn ordinary_new_constructor_lookup_reports_missing_nonzero_birth() {
        let package = super::super::brand_catalog_tests::issue_with_brand_catalog("box Page {}").unwrap();
        let batch = &package.instance_constructors;
        let parent = batch.box_sources.row_for("Page").unwrap().unwrap();
        assert!(matches!(batch.birth_for(parent, 1), Ok(None)));
    }

    #[test]
    fn ordinary_new_constructor_lookup_rejects_source_arity_overflow() {
        let package = super::super::brand_catalog_tests::issue_with_brand_catalog("box Page {}").unwrap();
        let batch = &package.instance_constructors;
        let parent = batch.box_sources.row_for("Page").unwrap().unwrap();
        assert!(matches!(
            batch.birth_for(parent, usize::MAX),
            Err(InstanceConstructorBirthLookupErrorV1::SourceArityOverflow)
        ));
    }

    #[test]
    fn constructor_loan_rejects_lost_shape_and_completion() {
        for fault in ["missing-shape", "foreign-shape", "missing-completion", "foreign-completion"] {
            let mut package = super::super::brand_catalog_tests::issue_with_brand_catalog(
                "box Page { birth(value) { local saved = value } }
                 box Other { birth(value) { return 1 } }",
            )
            .unwrap();
            let (first, rest) = package.instance_constructors.rows.split_at_mut(1);
            let row = &mut first[0];
            let foreign = &mut rest[0];
            assert!(!row.birth_completion().unwrap().returns_value());
            assert!(foreign.birth_completion().unwrap().returns_value());
            let expected = match fault {
                "missing-shape" => {
                    row.body_shapes.clear();
                    "body-shape"
                }
                "foreign-shape" => {
                    let shape = foreign.body_shapes.remove(&foreign.forest.roots()[0]).unwrap();
                    row.body_shapes.insert(row.forest.roots()[0], shape);
                    "body-shape-owner"
                }
                "missing-completion" => {
                    row.birth_completion = None;
                    "completion-owner"
                }
                "foreign-completion" => {
                    row.birth_completion = foreign.birth_completion.take();
                    "completion-owner"
                }
                _ => unreachable!(),
            };
            package.with_normal_program_source_loan(|loan| {
                let error = package.instance_constructors().rows()[0]
                    .lowering_input(loan.program()).unwrap_err();
                assert_eq!(error, format!(
                    "[freeze:contract][mir/instance-constructor-semantic/{expected}]"
                ));
            }).unwrap();
        }
    }
}
