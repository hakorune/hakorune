//! Resolver-owned semantic rows for parser-issued instance constructors.

use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, ReceiverPolicyV1,
    ResolveSourceBoundSelectedCallableForestsWithBodyShapesOutcomeV1,
    SelectedCallableResolverDeferredBatchV1, SelectedCallableResolverInputV1,
    SemanticOwnerRootProfileV1, SourceBoundSelectedCallableResolverRejectV1,
    VerifiedSemanticOwnerForestV1,
};
use crate::parser::{ConstructorSourceIdV1, VerifiedFinalCallableProgramSourceV1};

#[derive(Debug)]
pub(crate) enum InstanceConstructorSemanticBatchIssueV1 {
    ParserSyntax,
    SourceCoverage,
    Resolver(SourceBoundSelectedCallableResolverRejectV1),
    ResolverDeferred(SelectedCallableResolverDeferredBatchV1),
    MissingRoot,
    RootProfileMismatch,
    SourceProjection(String),
}

#[derive(Debug)]
pub(crate) struct VerifiedInstanceConstructorSemanticRowV1 {
    source_id: ConstructorSourceIdV1,
    final_box_ordinal: u32,
    box_name: Box<str>,
    key: Box<str>,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
}

#[derive(Debug)]
pub(crate) struct VerifiedInstanceConstructorSemanticBatchV1 {
    rows: Box<[VerifiedInstanceConstructorSemanticRowV1]>,
}

impl VerifiedInstanceConstructorSemanticBatchV1 {
    pub(crate) fn rows(&self) -> &[VerifiedInstanceConstructorSemanticRowV1] {
        &self.rows
    }
}

impl VerifiedInstanceConstructorSemanticRowV1 {
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

    pub(crate) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
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
                .owner_root(source, *root)
                .map_err(|error| error.to_string())?,
            function,
        ) {
            return Err(
                "[freeze:contract][mir/instance-constructor-semantic/root-identity]".to_owned(),
            );
        }
        Ok(input)
    }
}

pub(crate) fn issue_instance_constructor_semantic_batch_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: &VerifiedFinalCallableProgramSourceV1,
    brand_catalog: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
) -> Result<VerifiedInstanceConstructorSemanticBatchV1, InstanceConstructorSemanticBatchIssueV1> {
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
                    syntax.final_box_ordinal(),
                    Box::<str>::from(syntax.box_name()),
                    Box::<str>::from(syntax.key()),
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
            let forests = match resolver
                .resolve_source_bound_selected_callable_forests_with_body_shapes_and_brand_catalog(
                    &resolver_inputs,
                    brand_catalog,
                )
                .map_err(InstanceConstructorSemanticBatchIssueV1::Resolver)?
            {
                ResolveSourceBoundSelectedCallableForestsWithBodyShapesOutcomeV1::Complete {
                    forests,
                    ..
                } => forests,
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
            for ((source_id, final_box_ordinal, box_name, key, declaration, view), forest) in
                candidates.into_iter().zip(forests)
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
                    InstanceConstructorSemanticBatchIssueV1::SourceProjection(error.to_string())
                })?;
                rows.push(VerifiedInstanceConstructorSemanticRowV1 {
                    source_id,
                    final_box_ordinal,
                    box_name,
                    key,
                    forest,
                    projection,
                });
            }
            Ok(VerifiedInstanceConstructorSemanticBatchV1 {
                rows: rows.into_boxed_slice(),
            })
        })
        .map_err(|_| InstanceConstructorSemanticBatchIssueV1::ParserSyntax)?
}
