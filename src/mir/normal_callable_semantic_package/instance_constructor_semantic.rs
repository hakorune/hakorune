//! Resolver-owned semantic rows for parser-issued instance constructors.

use crate::mir::function::CanonicalObjectDefinitionV1;
use hakorune_mir_defs::CanonicalObjectIdV1;
use std::collections::BTreeMap;
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
    DeclaredInstanceCallSemanticEffectV1, FunctionOwnerIdV1, FunctionSemanticResolverSessionV1,
    FunctionSyntaxViewV1, ReceiverPolicyV1,
    ResolveSourceBoundSelectedCallableForestsWithBodyShapesOutcomeV1,
    SelectedCallableResolverDeferredBatchV1, SelectedCallableResolverInputV1,
    SemanticOwnerRootProfileV1, SourceBoundSelectedCallableResolverRejectV1,
    VerifiedResolvedBodyShapeInventoryV1, VerifiedSemanticOwnerForestV1,
};
use crate::parser::{
    ConstructorSourceIdV1, ConstructorSourceKindV1, ParserOrdinaryBoxSourceCoverageV1,
    ParserOrdinaryBoxSourceRowV1, VerifiedFinalCallableProgramSourceV1,
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
    Completion {
        _issue: FunctionCompletionVerificationErrorV1,
    },
    SourceProjection {
        _error: String,
    },
    ReceiverNonEscape {
        _issue: super::instance_constructor_non_escape::BirthReceiverNonEscapeIssueV1,
    },
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
    ObjectDefinitionsTransferred,
    ObjectDefinitionMissing,
}

impl VerifiedInstanceConstructorSemanticBatchV1 {
    /// One-way projection while co-sealing New claims, before collector transfer.
    /// `object_for` deliberately remains usable after the payload has moved.
    pub(super) fn destruction_for(
        &self,
        source: &ParserOrdinaryBoxSourceRowV1,
    ) -> Result<
        (
            CanonicalObjectIdV1,
            crate::mir::function::ObjectDestructionDispositionV1,
        ),
        InstanceConstructorBirthLookupErrorV1,
    > {
        self.with_source_object_definition(source, |object, definition| {
            (object, definition.destruction_disposition())
        })
    }

    pub(super) fn with_source_object_definition<R>(
        &self,
        source: &ParserOrdinaryBoxSourceRowV1,
        consume: impl FnOnce(CanonicalObjectIdV1, &CanonicalObjectDefinitionV1) -> R,
    ) -> Result<R, InstanceConstructorBirthLookupErrorV1> {
        let object = self.object_for(source)?;
        let definitions = self.object_definitions.borrow();
        let definitions = definitions
            .as_ref()
            .ok_or(InstanceConstructorBirthLookupErrorV1::ObjectDefinitionsTransferred)?;
        let definition = definitions
            .get(object.declaration_index() as usize)
            .ok_or(InstanceConstructorBirthLookupErrorV1::ObjectDefinitionMissing)?;
        Ok(consume(object, definition))
    }

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
        self.object_sources
            .iter()
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
            Ok(Some(own)) if own.same_source_as(box_source))
        {
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
            if self.rows.iter().any(|row| {
                row.box_source.same_source_as(box_source)
                    && row.kind == ConstructorSourceKindV1::Birth
            }) {
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
        &self,
        parent: &ParserOrdinaryBoxSourceRowV1,
        arity: usize,
    ) -> Result<&ConstructionEligibilityV1, InstanceConstructorBirthLookupErrorV1> {
        if let Some(row) = self.birth_for(parent, arity)? {
            return Ok(&row.construction);
        }
        self.no_birth_construction
            .iter()
            .find(|(own, _)| own.same_source_as(parent))
            .map(|(_, plan)| plan)
            .ok_or(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)
    }
}

impl VerifiedInstanceConstructorSemanticRowV1 {
    pub(crate) fn construction(&self) -> &ConstructionEligibilityV1 {
        &self.construction
    }

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
        let definition = source
            .with_ordinary_box_syntax(parent, object_definition::issue)
            .map_err(|_| InstanceConstructorSemanticBatchIssueV1::SourceCoverage)??;
        if object_sources.iter().any(
            |(own, _): &(ParserOrdinaryBoxSourceRowV1, CanonicalObjectIdV1)| {
                own.same_source_as(parent)
            },
        ) {
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
mod tests;
