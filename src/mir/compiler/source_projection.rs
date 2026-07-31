//! Sealed structural projection from semantic source sites to canonical syntax.

use std::collections::BTreeMap;
use std::fmt;
use std::num::NonZeroU32;

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    project_source_node_v1, BindingOriginV1, FunctionOwnerIdV1, ProjectedSourceNodeV1,
    RegionOriginV1, ResolvedExitOriginV1, ResolvedExitSiteV1, ScopeOriginV1,
    SemanticOwnerRootProfileV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
    VerifiedResolvedFunctionV1, VerifiedSemanticOwnerForestV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceNavigationErrorV1 {
    UnknownOwner(FunctionOwnerIdV1),
    ForeignOwner {
        expected: FunctionOwnerIdV1,
        actual: FunctionOwnerIdV1,
    },
    SignatureMismatch {
        owner: FunctionOwnerIdV1,
        expected_parameters: u32,
        actual_parameters: u32,
        expected_receiver: bool,
        actual_receiver: bool,
    },
    InvalidOwnerRoot {
        owner: FunctionOwnerIdV1,
        expected: &'static str,
        actual: &'static str,
    },
    InvalidSite {
        owner: FunctionOwnerIdV1,
        site: SourceNodeSiteV1,
        reason: &'static str,
    },
    BodyIndexOutOfBounds {
        owner: FunctionOwnerIdV1,
        body: SourceNodeSiteV1,
        index: u32,
        len: u32,
    },
    SuffixStartOutOfBounds {
        owner: FunctionOwnerIdV1,
        body: SourceNodeSiteV1,
        start: u32,
        len: u32,
    },
    SourceIndexOverflow {
        owner: FunctionOwnerIdV1,
        body: SourceNodeSiteV1,
        value: usize,
        role: &'static str,
    },
    EmptyBodySuffix {
        owner: FunctionOwnerIdV1,
        body: SourceNodeSiteV1,
        start: u32,
    },
    ConsumedRangeEndOverflow {
        owner: FunctionOwnerIdV1,
        body: SourceNodeSiteV1,
        start: u32,
        count: NonZeroU32,
    },
    ConsumedRangeOutOfBounds {
        owner: FunctionOwnerIdV1,
        body: SourceNodeSiteV1,
        start: u32,
        count: NonZeroU32,
        len: u32,
    },
    ConsumedRangeBodyMismatch {
        owner: FunctionOwnerIdV1,
        expected_body: SourceNodeSiteV1,
        actual_body: SourceNodeSiteV1,
    },
    ConsumedRangeStartMismatch {
        owner: FunctionOwnerIdV1,
        body: SourceNodeSiteV1,
        expected: u32,
        actual: u32,
    },
}

impl fmt::Display for SourceNavigationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownOwner(owner) => write!(
                formatter,
                "[freeze:contract][canonical_source/unknown_owner] owner={owner:?}"
            ),
            Self::ForeignOwner { expected, actual } => write!(
                formatter,
                "[freeze:contract][canonical_source/foreign_owner] expected={expected:?} actual={actual:?}"
            ),
            Self::SignatureMismatch {
                owner,
                expected_parameters,
                actual_parameters,
                expected_receiver,
                actual_receiver,
            } => write!(
                formatter,
                "[freeze:contract][canonical_source/signature_mismatch] owner={owner:?} expected_parameters={expected_parameters} actual_parameters={actual_parameters} expected_receiver={expected_receiver} actual_receiver={actual_receiver}"
            ),
            Self::InvalidOwnerRoot {
                owner,
                expected,
                actual,
            } => write!(
                formatter,
                "[freeze:contract][canonical_source/invalid_owner_root] owner={owner:?} expected={expected} actual={actual}"
            ),
            Self::InvalidSite {
                owner,
                site,
                reason,
            } => write!(
                formatter,
                "[freeze:contract][canonical_source/invalid_site] owner={owner:?} site={site:?} reason={reason}"
            ),
            Self::BodyIndexOutOfBounds {
                owner,
                body,
                index,
                len,
            } => write!(
                formatter,
                "[freeze:contract][canonical_source/body_index_out_of_bounds] owner={owner:?} body={body:?} index={index} len={len}"
            ),
            Self::SuffixStartOutOfBounds {
                owner,
                body,
                start,
                len,
            } => write!(
                formatter,
                "[freeze:contract][canonical_source/suffix_start_out_of_bounds] owner={owner:?} body={body:?} start={start} len={len}"
            ),
            Self::SourceIndexOverflow {
                owner,
                body,
                value,
                role,
            } => write!(
                formatter,
                "[freeze:contract][canonical_source/source_index_overflow] owner={owner:?} body={body:?} value={value} role={role}"
            ),
            Self::EmptyBodySuffix { owner, body, start } => write!(
                formatter,
                "[freeze:contract][canonical_source/empty_body_suffix] owner={owner:?} body={body:?} start={start}"
            ),
            Self::ConsumedRangeEndOverflow {
                owner,
                body,
                start,
                count,
            } => write!(
                formatter,
                "[freeze:contract][canonical_source/consumed_range_end_overflow] owner={owner:?} body={body:?} start={start} count={count}"
            ),
            Self::ConsumedRangeOutOfBounds {
                owner,
                body,
                start,
                count,
                len,
            } => write!(
                formatter,
                "[freeze:contract][canonical_source/consumed_range_out_of_bounds] owner={owner:?} body={body:?} start={start} count={count} len={len}"
            ),
            Self::ConsumedRangeBodyMismatch {
                owner,
                expected_body,
                actual_body,
            } => write!(
                formatter,
                "[freeze:contract][canonical_source/consumed_range_body_mismatch] owner={owner:?} expected_body={expected_body:?} actual_body={actual_body:?}"
            ),
            Self::ConsumedRangeStartMismatch {
                owner,
                body,
                expected,
                actual,
            } => write!(
                formatter,
                "[freeze:contract][canonical_source/consumed_range_start_mismatch] owner={owner:?} body={body:?} expected={expected} actual={actual}"
            ),
        }
    }
}

impl std::error::Error for SourceNavigationErrorV1 {}

#[derive(Debug)]
pub(crate) struct VerifiedSourceProjectionV1 {
    definition_chains: BTreeMap<FunctionOwnerIdV1, Box<[SourceExprSiteV1]>>,
}

pub(super) type ProjectedSourceV1<'source> = ProjectedSourceNodeV1<'source>;

impl VerifiedSourceProjectionV1 {
    pub(super) fn seal(
        syntax_root: &ASTNode,
        forest: &VerifiedSemanticOwnerForestV1,
    ) -> Result<Self, SourceNavigationErrorV1> {
        let root_owner = forest.roots()[0];
        let profile = forest
            .owner(root_owner)
            .ok_or(SourceNavigationErrorV1::UnknownOwner(root_owner))?
            .root_profile();
        Self::seal_with_root_profile(syntax_root, forest, profile)
    }

    pub(super) fn seal_with_root_profile(
        syntax_root: &ASTNode,
        forest: &VerifiedSemanticOwnerForestV1,
        root_profile: SemanticOwnerRootProfileV1,
    ) -> Result<Self, SourceNavigationErrorV1> {
        let root_owner = forest.roots()[0];
        if !root_matches_profile(syntax_root, root_profile) {
            return Err(SourceNavigationErrorV1::InvalidOwnerRoot {
                owner: root_owner,
                expected: expected_root_name(root_profile),
                actual: syntax_root.node_type(),
            });
        }

        let mut definition_chains = BTreeMap::new();
        for (owner, product) in forest.owners() {
            let chain = definition_chain(forest, root_owner, owner)?;
            let owner_root = locate_owner_root(syntax_root, owner, &chain)?;
            verify_owner_root(owner, owner == root_owner, root_profile, owner_root)?;
            verify_semantic_sites(owner, owner_root, product)?;
            definition_chains.insert(owner, chain.into_boxed_slice());
        }
        if definition_chains.len() != forest.owner_count() {
            return Err(SourceNavigationErrorV1::InvalidSite {
                owner: root_owner,
                site: SourceNodeSiteV1::from_segments(Vec::new()),
                reason: "owner_projection_count_mismatch",
            });
        }
        Ok(Self { definition_chains })
    }

    pub(super) fn owner_root<'a>(
        &self,
        syntax_root: &'a ASTNode,
        owner: FunctionOwnerIdV1,
    ) -> Result<&'a ASTNode, SourceNavigationErrorV1> {
        let chain = self
            .definition_chains
            .get(&owner)
            .ok_or(SourceNavigationErrorV1::UnknownOwner(owner))?;
        locate_owner_root(syntax_root, owner, chain)
    }

    pub(super) fn project<'a>(
        &self,
        owner_root: &'a ASTNode,
        owner: FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<ProjectedSourceV1<'a>, SourceNavigationErrorV1> {
        project_site(owner_root, owner, site)
    }
}

fn definition_chain(
    forest: &VerifiedSemanticOwnerForestV1,
    root: FunctionOwnerIdV1,
    mut owner: FunctionOwnerIdV1,
) -> Result<Vec<SourceExprSiteV1>, SourceNavigationErrorV1> {
    let requested = owner;
    let mut reverse = Vec::new();
    while owner != root {
        let edge = forest
            .parent(owner)
            .ok_or(SourceNavigationErrorV1::UnknownOwner(requested))?;
        reverse.push(edge.definition_site().site().clone());
        owner = edge.parent_owner();
    }
    reverse.reverse();
    Ok(reverse)
}

fn locate_owner_root<'a>(
    syntax_root: &'a ASTNode,
    owner: FunctionOwnerIdV1,
    chain: &[SourceExprSiteV1],
) -> Result<&'a ASTNode, SourceNavigationErrorV1> {
    let mut current = syntax_root;
    for definition_site in chain {
        current = match project_site(current, owner, definition_site.node())? {
            ProjectedSourceV1::Node(node @ ASTNode::Lambda { .. }) => node,
            ProjectedSourceV1::Node(node) => {
                return Err(SourceNavigationErrorV1::InvalidOwnerRoot {
                    owner,
                    expected: "Lambda",
                    actual: node.node_type(),
                });
            }
            _ => {
                return Err(SourceNavigationErrorV1::InvalidSite {
                    owner,
                    site: definition_site.node().clone(),
                    reason: "lambda_definition_is_not_node",
                });
            }
        };
    }
    Ok(current)
}

fn verify_owner_root(
    owner: FunctionOwnerIdV1,
    is_root: bool,
    root_profile: SemanticOwnerRootProfileV1,
    syntax: &ASTNode,
) -> Result<(), SourceNavigationErrorV1> {
    let valid = if is_root {
        root_matches_profile(syntax, root_profile)
    } else {
        matches!(syntax, ASTNode::Lambda { .. })
    };
    if valid {
        Ok(())
    } else {
        Err(SourceNavigationErrorV1::InvalidOwnerRoot {
            owner,
            expected: if is_root {
                expected_root_name(root_profile)
            } else {
                "Lambda"
            },
            actual: syntax.node_type(),
        })
    }
}

fn root_matches_profile(syntax: &ASTNode, profile: SemanticOwnerRootProfileV1) -> bool {
    matches!(
        (profile, syntax),
        (
            SemanticOwnerRootProfileV1::DeclaredFunction { .. },
            ASTNode::FunctionDeclaration { .. }
        ) | (SemanticOwnerRootProfileV1::Script, ASTNode::Program { .. })
            | (SemanticOwnerRootProfileV1::Lambda, ASTNode::Lambda { .. })
    )
}

fn expected_root_name(profile: SemanticOwnerRootProfileV1) -> &'static str {
    match profile {
        SemanticOwnerRootProfileV1::DeclaredFunction { .. } => "FunctionDeclaration",
        SemanticOwnerRootProfileV1::Script => "Program",
        SemanticOwnerRootProfileV1::Lambda => "Lambda",
    }
}

fn verify_semantic_sites(
    owner: FunctionOwnerIdV1,
    syntax: &ASTNode,
    product: &VerifiedResolvedFunctionV1,
) -> Result<(), SourceNavigationErrorV1> {
    verify_signature_sites(owner, syntax, product)?;
    for site in product.declaration_sites() {
        verify_declaration_site(owner, syntax, site)?;
    }
    for (site, _) in product.variable_refs() {
        match project_site(syntax, owner, site.node())? {
            ProjectedSourceV1::Node(ASTNode::Variable { .. } | ASTNode::Me { .. }) => {}
            _ => return invalid_site(owner, site.node(), "variable_use_kind_mismatch"),
        }
    }
    for (site, _) in product.assignment_targets() {
        match project_site(syntax, owner, site.node())? {
            ProjectedSourceV1::Node(
                ASTNode::Variable { .. } | ASTNode::FieldAccess { .. } | ASTNode::Index { .. },
            )
            | ProjectedSourceV1::SyntheticName => {}
            _ => return invalid_site(owner, site.node(), "assignment_target_kind_mismatch"),
        }
    }
    for (site, record) in product.resolved_exits() {
        let ResolvedExitSiteV1::Statement(statement) = site else {
            return invalid_site(owner, site.node(), "expression_exit_not_activated");
        };
        let valid = matches!(
            (
                record.origin(),
                project_site(syntax, owner, statement.node())?
            ),
            (
                ResolvedExitOriginV1::ExplicitContinue,
                ProjectedSourceV1::Node(ASTNode::Continue { .. })
            ) | (
                ResolvedExitOriginV1::ExplicitBreak,
                ProjectedSourceV1::Node(ASTNode::Break { .. })
            ) | (
                ResolvedExitOriginV1::ExplicitReturn,
                ProjectedSourceV1::Node(ASTNode::Return { .. })
            )
        );
        if !valid {
            return invalid_site(owner, statement.node(), "exit_kind_mismatch");
        }
    }
    for (_, binding) in product.bindings() {
        if let BindingOriginV1::Synthetic { owner: site, .. } = binding.origin() {
            project_site(syntax, owner, site)?;
        }
    }
    for (_, scope) in product.scopes() {
        if let ScopeOriginV1::Source(site) = scope.origin() {
            project_site(syntax, owner, site)?;
        }
    }
    for (_, region) in product.regions() {
        if let RegionOriginV1::Source(site) = region.origin() {
            project_site(syntax, owner, site)?;
        }
    }
    Ok(())
}

fn verify_signature_sites(
    owner: FunctionOwnerIdV1,
    syntax: &ASTNode,
    product: &VerifiedResolvedFunctionV1,
) -> Result<(), SourceNavigationErrorV1> {
    let (expected_parameters, expected_receiver) = match syntax {
        ASTNode::FunctionDeclaration {
            params, is_static, ..
        } => (params.len() as u32, !*is_static),
        ASTNode::Lambda { params, .. } => (params.len() as u32, false),
        _ => (0, false),
    };
    let actual_parameters = product
        .declaration_sites()
        .filter(|site| matches!(site, SourceBindingSiteV1::Parameter { .. }))
        .count() as u32;
    let actual_receiver = product
        .declaration_binding(&SourceBindingSiteV1::Receiver)
        .is_some();
    if (expected_parameters, expected_receiver) == (actual_parameters, actual_receiver) {
        Ok(())
    } else {
        Err(SourceNavigationErrorV1::SignatureMismatch {
            owner,
            expected_parameters,
            actual_parameters,
            expected_receiver,
            actual_receiver,
        })
    }
}

fn verify_declaration_site(
    owner: FunctionOwnerIdV1,
    syntax: &ASTNode,
    site: &SourceBindingSiteV1,
) -> Result<(), SourceNavigationErrorV1> {
    match site {
        SourceBindingSiteV1::Receiver => match syntax {
            ASTNode::FunctionDeclaration {
                is_static: false, ..
            } => Ok(()),
            _ => invalid_site(
                owner,
                &SourceNodeSiteV1::from_segments(Vec::new()),
                "receiver_declaration_mismatch",
            ),
        },
        SourceBindingSiteV1::Parameter { index } => {
            let parameter_count = match syntax {
                ASTNode::FunctionDeclaration { params, .. } | ASTNode::Lambda { params, .. } => {
                    params.len()
                }
                _ => 0,
            };
            if (*index as usize) < parameter_count {
                Ok(())
            } else {
                invalid_site(
                    owner,
                    &SourceNodeSiteV1::from_segments(Vec::new()),
                    "parameter_declaration_mismatch",
                )
            }
        }
        SourceBindingSiteV1::Local { statement, ordinal } => {
            match project_site(syntax, owner, statement.node())? {
                ProjectedSourceV1::Node(ASTNode::Local { variables, .. })
                    if (*ordinal as usize) < variables.len() =>
                {
                    Ok(())
                }
                _ => invalid_site(owner, statement.node(), "local_declaration_mismatch"),
            }
        }
        SourceBindingSiteV1::Outbox { statement, ordinal } => {
            match project_site(syntax, owner, statement.node())? {
                ProjectedSourceV1::Node(ASTNode::Outbox { variables, .. })
                    if (*ordinal as usize) < variables.len() =>
                {
                    Ok(())
                }
                _ => invalid_site(owner, statement.node(), "outbox_declaration_mismatch"),
            }
        }
        SourceBindingSiteV1::Nowait { statement } => {
            match project_site(syntax, owner, statement.node())? {
                ProjectedSourceV1::Node(ASTNode::Nowait { .. }) => Ok(()),
                _ => invalid_site(owner, statement.node(), "nowait_declaration_mismatch"),
            }
        }
        SourceBindingSiteV1::LoopBinder { loop_site } => invalid_site(
            owner,
            loop_site.node(),
            "loop_binder_projection_not_activated",
        ),
        SourceBindingSiteV1::CatchBinder { node, .. }
        | SourceBindingSiteV1::PatternBinder { node, .. } => {
            invalid_site(owner, node, "binder_projection_not_activated")
        }
    }
}

fn invalid_site<T>(
    owner: FunctionOwnerIdV1,
    site: &SourceNodeSiteV1,
    reason: &'static str,
) -> Result<T, SourceNavigationErrorV1> {
    Err(SourceNavigationErrorV1::InvalidSite {
        owner,
        site: site.clone(),
        reason,
    })
}

fn project_site<'a>(
    root: &'a ASTNode,
    owner: FunctionOwnerIdV1,
    site: &SourceNodeSiteV1,
) -> Result<ProjectedSourceV1<'a>, SourceNavigationErrorV1> {
    project_source_node_v1(root, site).ok_or_else(|| SourceNavigationErrorV1::InvalidSite {
        owner,
        site: site.clone(),
        reason: "segment_does_not_match_syntax",
    })
}
