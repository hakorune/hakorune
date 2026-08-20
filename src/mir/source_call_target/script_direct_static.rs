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
use crate::mir::policies::source_method_reserved_route::{
    classify_source_method_reserved_route_v1, SourceMethodReservedRouteContextV1,
    SourceMethodReservedRouteDecisionV1,
};
use crate::mir::resolved_semantics::{
    observe_script_method_calls_shadow_view_v0, project_source_node_v1, ProjectedSourceNodeV1,
    ScriptSyntaxViewV1, ShadowMethodCallReceiverV0, ShadowQualifiedReceiverDispositionV0,
    ShadowResolveErrorV0, SourceExprSiteV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedScriptRootDemandWindowV1,
};

use super::VerifiedStaticImportAliasViewV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ScriptStaticCallSourceOwnerIdV1(u32);

impl ScriptStaticCallSourceOwnerIdV1 {
    const ROOT: Self = Self(0);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedScriptDirectStaticCallSiteV1 {
    owner: ScriptStaticCallSourceOwnerIdV1,
    site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    argument_sites: Box<[SourceExprSiteV1]>,
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedScriptDirectStaticCallTargetV1 {
    site: SourceExprSiteV1,
    target: crate::mir::builder::CanonicalSameModuleCallableKeyV1,
}

impl VerifiedScriptDirectStaticCallTargetV1 {
    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn target(
        &self,
    ) -> &crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
        &self.target
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ScriptDirectStaticCallTargetErrorV1 {
    ProgramRequired,
    WindowSourceCardinalityMismatch { window: usize, program: usize },
    MethodObservation(ShadowResolveErrorV0),
    SiteProjectionMismatch { site: SourceExprSiteV1 },
    ReceiverSiteMismatch { site: SourceExprSiteV1 },
    DuplicateSite { site: SourceExprSiteV1 },
    ReceiverNameRequired { site: SourceExprSiteV1 },
    TargetOutsideCatalog {
        site: SourceExprSiteV1,
        owner: Box<str>,
        method: Box<str>,
        arity: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedScriptDirectStaticCallTargetInventoryV1 {
    owner: ScriptStaticCallSourceOwnerIdV1,
    sites: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticCallSiteV1>,
    targets: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticCallTargetV1>,
    noncandidate_count: usize,
}

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
                return Err(ScriptDirectStaticCallTargetErrorV1::SiteProjectionMismatch {
                    site,
                });
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
            sites,
            targets,
            noncandidate_count,
        })
    }

    pub(crate) const fn owner(&self) -> ScriptStaticCallSourceOwnerIdV1 {
        self.owner
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
