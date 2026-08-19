//! Resolver-owned semantic rows for parser-issued instance constructors.

use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, ReceiverPolicyV1,
    ResolveOwnerForestErrorV1, ResolveSelectedCallableForestsWithBodyShapesOutcomeV1,
    SemanticOwnerRootProfileV1, VerifiedSemanticOwnerForestV1,
};
use crate::parser::{ConstructorSourceIdV1, VerifiedFinalCallableProgramSourceV1};

#[derive(Debug)]
pub(crate) enum InstanceConstructorSemanticBatchIssueV1 {
    ParserSyntax,
    SourceCoverage,
    Resolver(ResolveOwnerForestErrorV1),
    ResolverDeferred,
    MissingRoot,
    RootProfileMismatch,
}

#[derive(Debug)]
pub(crate) struct VerifiedInstanceConstructorSemanticRowV1 {
    source_id: ConstructorSourceIdV1,
    box_name: Box<str>,
    key: Box<str>,
    forest: VerifiedSemanticOwnerForestV1,
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

    pub(crate) fn box_name(&self) -> &str {
        &self.box_name
    }

    pub(crate) fn key(&self) -> &str {
        &self.key
    }

    pub(crate) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
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
            let mut views = Vec::with_capacity(loan.rows().len());
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
                    Box::<str>::from(syntax.box_name()),
                    Box::<str>::from(syntax.key()),
                    view,
                ));
                views.push(view);
            }
            let forests = match resolver
                .resolve_selected_callable_forests_with_body_shapes_and_brand_catalog(
                    &views,
                    brand_catalog,
                )
                .map_err(InstanceConstructorSemanticBatchIssueV1::Resolver)?
            {
                ResolveSelectedCallableForestsWithBodyShapesOutcomeV1::Complete {
                    forests, ..
                } => forests,
                ResolveSelectedCallableForestsWithBodyShapesOutcomeV1::Deferred => {
                    return Err(InstanceConstructorSemanticBatchIssueV1::ResolverDeferred)
                }
            };
            if forests.len() != candidates.len() {
                return Err(InstanceConstructorSemanticBatchIssueV1::SourceCoverage);
            }
            let mut rows = Vec::with_capacity(forests.len());
            for ((source_id, box_name, key, view), forest) in candidates.into_iter().zip(forests) {
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
                rows.push(VerifiedInstanceConstructorSemanticRowV1 {
                    source_id,
                    box_name,
                    key,
                    forest,
                });
            }
            Ok(VerifiedInstanceConstructorSemanticBatchV1 {
                rows: rows.into_boxed_slice(),
            })
        })
        .map_err(|_| InstanceConstructorSemanticBatchIssueV1::ParserSyntax)?
}
