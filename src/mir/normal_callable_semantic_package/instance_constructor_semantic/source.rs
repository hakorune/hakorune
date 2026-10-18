//! Original constructor source capture, without construction/Home disposition.
use super::*;

pub(in crate::mir::normal_callable_semantic_package) fn capture_instance_constructor_sources_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: &VerifiedFinalCallableProgramSourceV1,
    brand_catalog: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
) -> Result<CapturedInstanceConstructorSourcesV1, InstanceConstructorSemanticBatchIssueV1> {
    let mut object_sources = Vec::new();
    let mut declarations = Vec::new();
    for (index, parent) in source.ordinary_box_coverage().rows().iter().enumerate() {
        let definition = source
            .with_ordinary_box_syntax(parent, object_definition::capture_declaration)
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
        declarations.push(definition);
    }
    source
        .with_constructor_semantic_syntax(|loan| {
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
                        return Err(
                            InstanceConstructorSemanticBatchIssueV1::BodyShapeOwnerMismatch,
                        );
                    }
                    constructor_shapes.insert(owner, shape);
                }
                let birth_completion = if kind == ConstructorSourceKindV1::Birth {
                    let shape = constructor_shapes
                        .get(root)
                        .ok_or(InstanceConstructorSemanticBatchIssueV1::BodyShapeMissing)?;
                    super::super::instance_constructor_non_escape::verify_birth_receiver_non_escape_v1(
                        function, shape, &forest,
                    )
                    .map_err(|_issue| {
                        InstanceConstructorSemanticBatchIssueV1::ReceiverNonEscape { _issue }
                    })?;
                    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
                        declaration,
                        &forest,
                        &projection,
                    )
                    .map_err(|error| {
                        InstanceConstructorSemanticBatchIssueV1::SourceProjection {
                            _error: format!("{error:?}"),
                        }
                    })?;
                    Some(verify_function_completion_v1(input).map_err(|_issue| {
                        InstanceConstructorSemanticBatchIssueV1::Completion { _issue }
                    })?)
                } else {
                    None
                };
                let object = object_sources
                    .iter()
                    .find(|(own, _)| own.same_source_as(&box_source))
                    .map(|(_, id)| *id)
                    .ok_or(InstanceConstructorSemanticBatchIssueV1::SourceCoverage)?;
                rows.push(InstanceConstructorSourceDataV1 {
                    // Only the exact unannotated source contract is selected.
                    // Explicit constructor contracts need their own admission;
                    // never turn them into the implicit opaque default.
                    birth_effect: match declaration {
                        ASTNode::FunctionDeclaration {
                            attrs, contracts, ..
                        } if kind == ConstructorSourceKindV1::Birth
                            && attrs.is_empty()
                            && contracts.is_empty() =>
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
                    object,
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
            Ok(CapturedInstanceConstructorSourcesV1 {
                rows: rows.into_boxed_slice(),
                box_sources: source.ordinary_box_coverage().clone(),
                object_sources: object_sources.into_boxed_slice(),
                declarations: declarations.into_boxed_slice(),
            })
        })
        .map_err(|_| InstanceConstructorSemanticBatchIssueV1::ParserSyntax)?
}
