//! Source-owned Script direct-static call inventory.
//!
//! This is an observation product only.  It gives a later Script Recipe
//! producer an exact caller/site and canonical callee, but it does not issue a
//! Recipe, a result publication row, or a physical call.

use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use std::collections::BTreeMap;

use crate::ast::ASTNode;
use crate::mir::builder::{
    SameModuleCallableNamespaceV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::callable_result_representation::{
    VerifiedCallableResultRepresentationV1, VerifiedSameModuleCallableResultCatalogV1,
};
use crate::mir::policies::source_method_reserved_route::{
    classify_source_method_reserved_route_v1, SourceMethodReservedRouteContextV1,
    SourceMethodReservedRouteDecisionV1,
};
use crate::mir::policies::source_method_typeop_route::{
    classify_source_method_typeop_route_v1, SourceMethodTypeOpDispositionV1,
};
use crate::mir::resolved_semantics::{
    observe_script_method_calls_shadow_view_v0, project_source_node_v1, FileCallScopeV1, ProjectedSourceNodeV1,
    ScriptSyntaxViewV1, ShadowMethodCallReceiverV0, ShadowQualifiedReceiverDispositionV0,
    ShadowResolveErrorV0, SourceExprSiteV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedScriptRootDemandWindowV1,
};
use crate::parser::{ParserNormalProgramSourceLoanRejectV1, ParserNormalProgramSourceLoanV1};

use super::{
    ScriptDirectStaticCallCoverageIssueV1, VerifiedScriptCallCoverageDispositionV1,
    VerifiedScriptCallCoverageRowV1, VerifiedScriptCallCoverageV1,
    VerifiedScriptNonDirectCallReasonV1, VerifiedStaticImportAliasViewV1,
};

/// One owned Script direct-static lookup row.
///
/// The row is emitted only after the parser loan, source window, declaration
/// catalog, import relation, and result contract have been checked together.
/// It deliberately owns only AST-free source sites and canonical result facts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedScriptDirectStaticCallLookupRowV1 {
    site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    argument_sites: Box<[SourceExprSiteV1]>,
    target: crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    representation: VerifiedCallableResultRepresentationV1,
    required_callee_i64_arguments: Box<[u32]>,
}

impl VerifiedScriptDirectStaticCallLookupRowV1 {
    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }

    pub(crate) const fn target(&self) -> &crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
        &self.target
    }

    pub(crate) const fn representation(&self) -> &VerifiedCallableResultRepresentationV1 {
        &self.representation
    }

    pub(crate) fn required_callee_i64_arguments(&self) -> &[u32] {
        &self.required_callee_i64_arguments
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SourceExprSiteV1,
        SourceExprSiteV1,
        Box<[SourceExprSiteV1]>,
        crate::mir::builder::CanonicalSameModuleCallableKeyV1,
        VerifiedCallableResultRepresentationV1,
        Box<[u32]>,
    ) {
        (
            self.site,
            self.receiver_site,
            self.argument_sites,
            self.target,
            self.representation,
            self.required_callee_i64_arguments,
        )
    }
}

/// The sole owned selected-Script target/result lookup product.
///
/// This type has no AST or catalog lifetime.  The parser invocation witness
/// is retained as provenance, while all source rows and result facts are
/// copied into the one-shot product during the HRTB loan.
#[derive(Debug)]
pub(crate) struct VerifiedScriptDirectStaticCallLookupV1 {
    invocation: crate::parser::ParserInvocationWitnessV1,
    coverage: VerifiedScriptCallCoverageV1,
    rows: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticCallLookupRowV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ScriptDirectStaticCallLookupErrorV1 {
    ProgramRequired,
    CatalogRelationMismatch,
    WindowSourceCardinalityMismatch {
        window: usize,
        program: usize,
    },
    MethodObservation(ShadowResolveErrorV0),
    Coverage(ScriptDirectStaticCallCoverageIssueV1),
    TargetOutsideCatalog {
        site: SourceExprSiteV1,
        owner: Box<str>,
        method: Box<str>,
        arity: u32,
    },
    ResultUnavailable {
        site: SourceExprSiteV1,
    },
    SourceLoan(ParserNormalProgramSourceLoanRejectV1),
}

impl VerifiedScriptDirectStaticCallLookupV1 {
    /// Issue one owned relation from the parser loan.  The caller is the
    /// source-package facade; this lower-level function never receives an AST
    /// or returns a borrowed catalog.
    pub(crate) fn issue_from_program_loan(
        loan: &ParserNormalProgramSourceLoanV1<'_>,
        window: &VerifiedScriptRootDemandWindowV1,
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
        imports: &VerifiedStaticImportAliasViewV1<'_>,
        targets: &crate::mir::source_call_target::VerifiedSourceStaticCallTargetCatalogV1<'_>,
        results: &VerifiedSameModuleCallableResultCatalogV1<'_, '_>,
        file_brands: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
    ) -> Result<Self, ScriptDirectStaticCallLookupErrorV1> {
        if !imports.is_branded_by(declarations) {
            return Err(ScriptDirectStaticCallLookupErrorV1::CatalogRelationMismatch);
        }
        if !results.is_branded_by(declarations, targets) {
            return Err(ScriptDirectStaticCallLookupErrorV1::CatalogRelationMismatch);
        }
        let invocation = loan.invocation_witness().clone();
        let view = ScriptSyntaxViewV1::from_program(loan.program())
            .ok_or(ScriptDirectStaticCallLookupErrorV1::ProgramRequired)?;
        if view.body().len() != window.entries().len() {
            return Err(
                ScriptDirectStaticCallLookupErrorV1::WindowSourceCardinalityMismatch {
                    window: window.entries().len(),
                    program: view.body().len(),
                },
            );
        }

        let file_calls = file_brands.map(|brands| FileCallScopeV1 {
            declarations,
            static_owner: None,
            brands,
        });
        let observed = observe_script_method_calls_shadow_view_v0(view, window, file_calls)
            .map_err(ScriptDirectStaticCallLookupErrorV1::MethodObservation)?;
        let mut coverage_rows = BTreeMap::new();
        let mut rows = BTreeMap::new();
        for (site, observation) in observed {
            let Some(ProjectedSourceNodeV1::Node(ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            })) = project_source_node_v1(loan.program(), site.node())
            else {
                return Err(ScriptDirectStaticCallLookupErrorV1::Coverage(
                    ScriptDirectStaticCallCoverageIssueV1::MissingSite { site },
                ));
            };
            let expected_receiver_site = SourcePathV1::from_node(site.node())
                .child(SourcePathSegmentV1::Receiver)
                .expr();
            if observation.receiver_site() != &expected_receiver_site {
                return Err(ScriptDirectStaticCallLookupErrorV1::Coverage(
                    ScriptDirectStaticCallCoverageIssueV1::ReceiverSiteMismatch { site },
                ));
            }
            let argument_sites = arguments
                .iter()
                .enumerate()
                .map(|(index, _)| {
                    SourcePathV1::from_node(site.node())
                        .child(SourcePathSegmentV1::Argument(index as u32))
                        .expr()
                })
                .collect::<Vec<_>>()
                .into_boxed_slice();
            let (disposition, receiver_name) = classify_script_call_source_route_v1(
                observation.receiver(),
                object,
                method,
                arguments,
            );
            if coverage_rows
                .insert(
                    site.clone(),
                    VerifiedScriptCallCoverageRowV1::new(
                        site.clone(),
                        expected_receiver_site.clone(),
                        argument_sites.clone(),
                        site.clone(),
                        disposition,
                    ),
                )
                .is_some()
            {
                return Err(ScriptDirectStaticCallLookupErrorV1::Coverage(
                    ScriptDirectStaticCallCoverageIssueV1::DuplicateSite { site },
                ));
            }
            let Some(name) = receiver_name else {
                continue;
            };
            let canonical_owner = imports.canonical_owner(name).unwrap_or(name);
            let Some(declaration) = declarations.declaration_for(
                SameModuleCallableNamespaceV1::StaticBoxMethod,
                canonical_owner,
                method,
                arguments.len(),
            ) else {
                return Err(ScriptDirectStaticCallLookupErrorV1::TargetOutsideCatalog {
                    site,
                    owner: canonical_owner.into(),
                    method: method.clone().into(),
                    arity: arguments.len() as u32,
                });
            };
            let target = declaration.key().clone();
            let Some(disposition) = results.disposition(&target) else {
                return Err(ScriptDirectStaticCallLookupErrorV1::ResultUnavailable { site });
            };
            let Some(representation) = disposition.representation() else {
                return Err(ScriptDirectStaticCallLookupErrorV1::ResultUnavailable { site });
            };
            let required_callee_i64_arguments = disposition.required_i64_arguments().map_or_else(
                || Vec::<u32>::new().into_boxed_slice(),
                |values| values.to_vec().into_boxed_slice(),
            );
            rows.insert(
                site.clone(),
                VerifiedScriptDirectStaticCallLookupRowV1 {
                    site: site.clone(),
                    receiver_site: expected_receiver_site,
                    argument_sites,
                    target,
                    representation,
                    required_callee_i64_arguments,
                },
            );
        }

        Ok(Self {
            invocation: invocation.clone(),
            coverage: VerifiedScriptCallCoverageV1::from_rows(invocation, coverage_rows),
            rows,
        })
    }

    pub(crate) fn is_from_invocation(
        &self,
        witness: &crate::parser::ParserInvocationWitnessV1,
    ) -> bool {
        self.invocation.same_as(witness)
    }

    pub(crate) fn row(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedScriptDirectStaticCallLookupRowV1> {
        self.rows.get(site)
    }

    pub(crate) fn source_coverage(&self) -> &VerifiedScriptCallCoverageV1 {
        &self.coverage
    }

    pub(crate) fn rows(
        &self,
    ) -> impl Iterator<
        Item = (
            &SourceExprSiteV1,
            &VerifiedScriptDirectStaticCallLookupRowV1,
        ),
    > {
        self.rows.iter()
    }

    pub(crate) fn into_rows(
        self,
    ) -> BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticCallLookupRowV1> {
        self.rows
    }
}


fn classify_script_call_source_route_v1<'a>(
    receiver: ShadowMethodCallReceiverV0,
    object: &'a ASTNode,
    method: &str,
    arguments: &[ASTNode],
) -> (VerifiedScriptCallCoverageDispositionV1, Option<&'a str>) {
    match receiver {
        ShadowMethodCallReceiverV0::CurrentOwner => (
            VerifiedScriptCallCoverageDispositionV1::NonDirect(
                VerifiedScriptNonDirectCallReasonV1::CurrentOwner,
            ),
            None,
        ),
        ShadowMethodCallReceiverV0::Qualified(ShadowQualifiedReceiverDispositionV0::Bound) => (
            VerifiedScriptCallCoverageDispositionV1::NonDirect(
                VerifiedScriptNonDirectCallReasonV1::QualifiedReceiverBound,
            ),
            None,
        ),
        ShadowMethodCallReceiverV0::Dynamic => (
            VerifiedScriptCallCoverageDispositionV1::NonDirect(
                VerifiedScriptNonDirectCallReasonV1::DynamicReceiver,
            ),
            None,
        ),
        ShadowMethodCallReceiverV0::Qualified(
            ShadowQualifiedReceiverDispositionV0::ProvenUnbound,
        ) => {
            let ASTNode::Variable { name, .. } = object else {
                return (
                    VerifiedScriptCallCoverageDispositionV1::NonDirect(
                        VerifiedScriptNonDirectCallReasonV1::ReceiverShapeUnsupported,
                    ),
                    None,
                );
            };
            if !matches!(
                classify_source_method_typeop_route_v1(method, arguments),
                SourceMethodTypeOpDispositionV1::Ordinary
            ) {
                return (
                    VerifiedScriptCallCoverageDispositionV1::NonDirect(
                        VerifiedScriptNonDirectCallReasonV1::TypeOperation,
                    ),
                    None,
                );
            }
            if !matches!(
                classify_source_method_reserved_route_v1(
                    SourceMethodReservedRouteContextV1::Ordinary,
                    object,
                    method,
                    arguments,
                ),
                SourceMethodReservedRouteDecisionV1::Ordinary
            ) {
                return (
                    VerifiedScriptCallCoverageDispositionV1::NonDirect(
                        VerifiedScriptNonDirectCallReasonV1::ReservedRoute,
                    ),
                    None,
                );
            }
            (
                VerifiedScriptCallCoverageDispositionV1::QualifiedUnboundOrdinary,
                Some(name.as_str()),
            )
        }
    }
}
