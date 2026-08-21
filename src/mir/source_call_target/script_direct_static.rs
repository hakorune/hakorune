//! Source-owned Script direct-static call inventory.
//!
//! This is an observation product only.  It gives a later Script Recipe
//! producer an exact caller/site and canonical callee, but it does not issue a
//! Recipe, a result publication row, or a physical call.

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
    observe_script_method_calls_shadow_view_v0, project_source_node_v1, ProjectedSourceNodeV1,
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

        let observed = observe_script_method_calls_shadow_view_v0(view, window)
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

#[cfg(test)]
impl VerifiedScriptDirectStaticCallLookupV1 {
    pub(crate) fn from_test_inventory(
        inventory: &VerifiedScriptDirectStaticCallTargetInventoryV1,
        results: &VerifiedSameModuleCallableResultCatalogV1<'_, '_>,
    ) -> Self {
        let rows: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticCallLookupRowV1> = inventory
            .target_rows()
            .map(|(site, target)| {
                let observation = inventory
                    .site(site)
                    .expect("test inventory target has its source site");
                let disposition = results
                    .disposition(target.target())
                    .expect("test target has a result disposition");
                let representation = disposition
                    .representation()
                    .expect("test target has a result representation");
                let required_callee_i64_arguments =
                    disposition.required_i64_arguments().map_or_else(
                        || Vec::<u32>::new().into_boxed_slice(),
                        |values| values.to_vec().into_boxed_slice(),
                    );
                (
                    site.clone(),
                    VerifiedScriptDirectStaticCallLookupRowV1 {
                        site: site.clone(),
                        receiver_site: observation.receiver_site().clone(),
                        argument_sites: observation.argument_sites().to_vec().into_boxed_slice(),
                        target: target.target().clone(),
                        representation,
                        required_callee_i64_arguments,
                    },
                )
            })
            .collect();
        Self {
            invocation: crate::parser::ParserInvocationWitnessV1::for_test(),
            coverage: VerifiedScriptCallCoverageV1::from_rows(
                crate::parser::ParserInvocationWitnessV1::for_test(),
                rows.iter()
                    .map(|(site, row)| {
                        (
                            site.clone(),
                            VerifiedScriptCallCoverageRowV1::new(
                                site.clone(),
                                row.receiver_site().clone(),
                                row.argument_sites().to_vec().into_boxed_slice(),
                                site.clone(),
                                VerifiedScriptCallCoverageDispositionV1::QualifiedUnboundOrdinary,
                            ),
                        )
                    })
                    .collect(),
            ),
            rows,
        }
    }

    pub(crate) fn empty_for_test() -> Self {
        Self {
            invocation: crate::parser::ParserInvocationWitnessV1::for_test(),
            coverage: VerifiedScriptCallCoverageV1::CompleteEmpty {
                invocation: crate::parser::ParserInvocationWitnessV1::for_test(),
            },
            rows: BTreeMap::new(),
        }
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

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ScriptStaticCallSourceOwnerIdV1(u32);

#[cfg(test)]
impl ScriptStaticCallSourceOwnerIdV1 {
    const ROOT: Self = Self(0);
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedScriptDirectStaticCallSiteV1 {
    owner: ScriptStaticCallSourceOwnerIdV1,
    site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    argument_sites: Box<[SourceExprSiteV1]>,
}

#[cfg(test)]
impl VerifiedScriptDirectStaticCallSiteV1 {
    pub(crate) const fn owner(&self) -> ScriptStaticCallSourceOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedScriptDirectStaticCallTargetV1 {
    site: SourceExprSiteV1,
    target: crate::mir::builder::CanonicalSameModuleCallableKeyV1,
}

#[cfg(test)]
impl VerifiedScriptDirectStaticCallTargetV1 {
    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn target(&self) -> &crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
        &self.target
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ScriptDirectStaticCallTargetErrorV1 {
    ProgramRequired,
    WindowSourceCardinalityMismatch {
        window: usize,
        program: usize,
    },
    MethodObservation(ShadowResolveErrorV0),
    SiteProjectionMismatch {
        site: SourceExprSiteV1,
    },
    ReceiverSiteMismatch {
        site: SourceExprSiteV1,
    },
    DuplicateSite {
        site: SourceExprSiteV1,
    },
    ReceiverNameRequired {
        site: SourceExprSiteV1,
    },
    TargetOutsideCatalog {
        site: SourceExprSiteV1,
        owner: Box<str>,
        method: Box<str>,
        arity: u32,
    },
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedScriptDirectStaticCallTargetInventoryV1 {
    owner: ScriptStaticCallSourceOwnerIdV1,
    source_identity: usize,
    window_identity: usize,
    declarations_identity: usize,
    imports_identity: usize,
    sites: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticCallSiteV1>,
    targets: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticCallTargetV1>,
    noncandidate_count: usize,
}

#[cfg(test)]
impl VerifiedScriptDirectStaticCallTargetInventoryV1 {
    pub(crate) fn issue(
        source_ast: &ASTNode,
        window: &VerifiedScriptRootDemandWindowV1,
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
        imports: &VerifiedStaticImportAliasViewV1<'_>,
    ) -> Result<Self, ScriptDirectStaticCallTargetErrorV1> {
        if !imports.is_branded_by(declarations) {
            return Err(ScriptDirectStaticCallTargetErrorV1::ProgramRequired);
        }
        let view = ScriptSyntaxViewV1::from_program(source_ast)
            .ok_or(ScriptDirectStaticCallTargetErrorV1::ProgramRequired)?;
        if view.body().len() != window.entries().len() {
            return Err(
                ScriptDirectStaticCallTargetErrorV1::WindowSourceCardinalityMismatch {
                    window: window.entries().len(),
                    program: view.body().len(),
                },
            );
        }

        let observed = observe_script_method_calls_shadow_view_v0(view, window)
            .map_err(ScriptDirectStaticCallTargetErrorV1::MethodObservation)?;
        let owner = ScriptStaticCallSourceOwnerIdV1::ROOT;
        let mut sites = BTreeMap::new();
        let mut targets = BTreeMap::new();
        let mut noncandidate_count = 0usize;

        for (site, observation) in observed {
            let Some(ProjectedSourceNodeV1::Node(ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            })) = project_source_node_v1(source_ast, site.node())
            else {
                return Err(ScriptDirectStaticCallTargetErrorV1::SiteProjectionMismatch { site });
            };
            let expected_receiver_site = SourcePathV1::from_node(site.node())
                .child(SourcePathSegmentV1::Receiver)
                .expr();
            if observation.receiver_site() != &expected_receiver_site {
                return Err(ScriptDirectStaticCallTargetErrorV1::ReceiverSiteMismatch { site });
            }
            if sites.contains_key(&site) {
                return Err(ScriptDirectStaticCallTargetErrorV1::DuplicateSite { site });
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
            sites.insert(
                site.clone(),
                VerifiedScriptDirectStaticCallSiteV1 {
                    owner,
                    site: site.clone(),
                    receiver_site: expected_receiver_site,
                    argument_sites,
                },
            );

            let ShadowMethodCallReceiverV0::Qualified(
                ShadowQualifiedReceiverDispositionV0::ProvenUnbound,
            ) = observation.receiver()
            else {
                noncandidate_count += 1;
                continue;
            };
            let ASTNode::Variable { name, .. } = object.as_ref() else {
                return Err(ScriptDirectStaticCallTargetErrorV1::ReceiverNameRequired { site });
            };
            if !matches!(
                classify_source_method_typeop_route_v1(method, arguments),
                SourceMethodTypeOpDispositionV1::Ordinary
            ) {
                noncandidate_count += 1;
                continue;
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
                noncandidate_count += 1;
                continue;
            }
            let canonical_owner = imports.canonical_owner(name).unwrap_or(name);
            let Some(declaration) = declarations.declaration_for(
                SameModuleCallableNamespaceV1::StaticBoxMethod,
                canonical_owner,
                method,
                arguments.len(),
            ) else {
                return Err(ScriptDirectStaticCallTargetErrorV1::TargetOutsideCatalog {
                    site,
                    owner: canonical_owner.into(),
                    method: method.clone().into(),
                    arity: arguments.len() as u32,
                });
            };
            targets.insert(
                site.clone(),
                VerifiedScriptDirectStaticCallTargetV1 {
                    site,
                    target: declaration.key().clone(),
                },
            );
        }

        Ok(Self {
            owner,
            source_identity: source_ast as *const _ as usize,
            window_identity: window as *const _ as usize,
            declarations_identity: declarations as *const _ as usize,
            imports_identity: imports as *const _ as usize,
            sites,
            targets,
            noncandidate_count,
        })
    }

    pub(crate) const fn owner(&self) -> ScriptStaticCallSourceOwnerIdV1 {
        self.owner
    }

    pub(crate) fn is_branded_by(
        &self,
        source_ast: &ASTNode,
        window: &VerifiedScriptRootDemandWindowV1,
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
        imports: &VerifiedStaticImportAliasViewV1<'_>,
    ) -> bool {
        self.source_identity == source_ast as *const _ as usize
            && self.window_identity == window as *const _ as usize
            && self.declarations_identity == declarations as *const _ as usize
            && self.imports_identity == imports as *const _ as usize
    }

    pub(crate) fn site(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedScriptDirectStaticCallSiteV1> {
        self.sites.get(site)
    }

    pub(crate) fn target(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedScriptDirectStaticCallTargetV1> {
        self.targets.get(site)
    }

    pub(crate) fn target_rows(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &VerifiedScriptDirectStaticCallTargetV1)> {
        self.targets.iter()
    }

    pub(crate) fn observed_len(&self) -> usize {
        self.sites.len()
    }

    pub(crate) fn target_len(&self) -> usize {
        self.targets.len()
    }

    pub(crate) const fn noncandidate_len(&self) -> usize {
        self.noncandidate_count
    }
}
