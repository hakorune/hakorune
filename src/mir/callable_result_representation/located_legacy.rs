//! SITE0-L0 located legacy inputs, disconnected from production lowering.

use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationV1,
};
use crate::mir::resolved_semantics::{
    is_statement_expression_surface_v1, BodyChildRoleV1, ExprChildRoleV1, ExprChildSyntaxV1,
    SourceBodyKindV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathV1, SourceStmtSiteV1,
};

use super::{CallableResultLegacyLocationErrorV1, VerifiedCallableResultActivationPlanV1};

#[derive(Debug)]
pub(crate) struct LocatedLegacyBodyV1<'plan> {
    plan_identity: usize,
    caller: &'plan CanonicalSameModuleCallableKeyV1,
    parent: Option<SourceNodeSiteV1>,
    domain_parent: Option<SourceNodeSiteV1>,
    kind: SourceBodyKindV1,
    statements: &'plan [ASTNode],
}

#[derive(Debug)]
pub(crate) struct UnlocatedLegacyBodyV1<'plan> {
    plan_identity: usize,
    caller: &'plan CanonicalSameModuleCallableKeyV1,
    statements: &'plan [ASTNode],
}

#[derive(Debug)]
pub(crate) enum LegacyBodyInputV1<'plan> {
    Located(LocatedLegacyBodyV1<'plan>),
    Unlocated(UnlocatedLegacyBodyV1<'plan>),
}

#[derive(Debug)]
pub(crate) struct LocatedLegacyBodySuffixV1<'plan> {
    plan_identity: usize,
    caller: &'plan CanonicalSameModuleCallableKeyV1,
    parent: Option<SourceNodeSiteV1>,
    domain_parent: Option<SourceNodeSiteV1>,
    kind: SourceBodyKindV1,
    start: u32,
    statements: &'plan [ASTNode],
}

impl<'plan> LegacyBodyInputV1<'plan> {
    pub(crate) const fn statements(&self) -> &'plan [ASTNode] {
        match self {
            Self::Located(located) => located.statements,
            Self::Unlocated(unlocated) => unlocated.statements,
        }
    }
}

#[derive(Debug)]
pub(crate) struct LocatedLegacyStmtV1<'plan> {
    plan_identity: usize,
    caller: &'plan CanonicalSameModuleCallableKeyV1,
    site: SourceStmtSiteV1,
    node: &'plan ASTNode,
}

#[derive(Debug)]
pub(crate) struct UnlocatedLegacyStmtV1<'plan> {
    plan_identity: usize,
    caller: &'plan CanonicalSameModuleCallableKeyV1,
    node: &'plan ASTNode,
}

#[derive(Debug)]
pub(crate) enum LegacyStmtInputV1<'plan> {
    Located(LocatedLegacyStmtV1<'plan>),
    Unlocated(UnlocatedLegacyStmtV1<'plan>),
}

impl<'plan> LegacyStmtInputV1<'plan> {
    pub(crate) const fn node(&self) -> &'plan ASTNode {
        match self {
            Self::Located(located) => located.node,
            Self::Unlocated(unlocated) => unlocated.node,
        }
    }

    pub(super) fn activation_prefix_parts(
        &self,
    ) -> Result<LegacyActivationPrefixPartsV1<'_>, CallableResultLegacyLocationErrorV1> {
        match self {
            Self::Located(located) => Ok(LegacyActivationPrefixPartsV1 {
                plan_identity: located.plan_identity,
                caller: located.caller,
                prefix: Some(located.site.node()),
            }),
            Self::Unlocated(_) => {
                Err(CallableResultLegacyLocationErrorV1::UnlocatedCannotProveInactive)
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct LocatedLegacyExprV1<'plan> {
    plan_identity: usize,
    caller: &'plan CanonicalSameModuleCallableKeyV1,
    site: SourceExprSiteV1,
    node: &'plan ASTNode,
}

#[derive(Debug)]
pub(crate) struct UnlocatedLegacyExprV1<'plan> {
    plan_identity: usize,
    caller: &'plan CanonicalSameModuleCallableKeyV1,
    node: &'plan ASTNode,
}

#[derive(Debug)]
pub(crate) enum LegacyExprInputV1<'plan> {
    Located(LocatedLegacyExprV1<'plan>),
    Unlocated(UnlocatedLegacyExprV1<'plan>),
}

impl<'plan> LegacyExprInputV1<'plan> {
    pub(crate) fn activation_site(
        &self,
    ) -> Result<
        (&'plan CanonicalSameModuleCallableKeyV1, &SourceExprSiteV1),
        CallableResultLegacyLocationErrorV1,
    > {
        match self {
            Self::Located(located) => Ok((located.caller, &located.site)),
            Self::Unlocated(_) => {
                Err(CallableResultLegacyLocationErrorV1::UnlocatedCannotClaimActivation)
            }
        }
    }

    pub(crate) const fn node(&self) -> &'plan ASTNode {
        match self {
            Self::Located(located) => located.node,
            Self::Unlocated(unlocated) => unlocated.node,
        }
    }

    pub(super) fn activation_claim_parts(
        &self,
    ) -> Result<LegacyActivationClaimPartsV1<'_>, CallableResultLegacyLocationErrorV1> {
        match self {
            Self::Located(located) => Ok(LegacyActivationClaimPartsV1 {
                plan_identity: located.plan_identity,
                caller: located.caller,
                site: &located.site,
            }),
            Self::Unlocated(_) => {
                Err(CallableResultLegacyLocationErrorV1::UnlocatedCannotClaimActivation)
            }
        }
    }

    pub(super) fn activation_prefix_parts(
        &self,
    ) -> Result<LegacyActivationPrefixPartsV1<'_>, CallableResultLegacyLocationErrorV1> {
        match self {
            Self::Located(located) => Ok(LegacyActivationPrefixPartsV1 {
                plan_identity: located.plan_identity,
                caller: located.caller,
                prefix: Some(located.site.node()),
            }),
            Self::Unlocated(_) => {
                Err(CallableResultLegacyLocationErrorV1::UnlocatedCannotProveInactive)
            }
        }
    }
}

impl LegacyBodyInputV1<'_> {
    pub(super) fn activation_body_domain_parts(
        &self,
    ) -> Result<LegacyActivationBodyDomainPartsV1<'_>, CallableResultLegacyLocationErrorV1> {
        match self {
            Self::Located(located) => Ok(LegacyActivationBodyDomainPartsV1 {
                plan_identity: located.plan_identity,
                caller: located.caller,
                parent: located.domain_parent.as_ref(),
                kind: located.kind,
            }),
            Self::Unlocated(_) => {
                Err(CallableResultLegacyLocationErrorV1::UnlocatedCannotProveInactive)
            }
        }
    }
}

impl<'plan> LocatedLegacyBodySuffixV1<'plan> {
    pub(super) fn into_activation_parts(self) -> LegacyActivationBodySuffixPartsV1<'plan> {
        LegacyActivationBodySuffixPartsV1 {
            plan_identity: self.plan_identity,
            caller: self.caller,
            parent: self.parent,
            domain_parent: self.domain_parent,
            kind: self.kind,
            start: self.start,
            statements: self.statements,
        }
    }
}

pub(super) struct LegacyActivationBodyDomainPartsV1<'a> {
    pub(super) plan_identity: usize,
    pub(super) caller: &'a CanonicalSameModuleCallableKeyV1,
    pub(super) parent: Option<&'a SourceNodeSiteV1>,
    pub(super) kind: SourceBodyKindV1,
}

pub(super) struct LegacyActivationBodySuffixPartsV1<'plan> {
    pub(super) plan_identity: usize,
    pub(super) caller: &'plan CanonicalSameModuleCallableKeyV1,
    pub(super) parent: Option<SourceNodeSiteV1>,
    pub(super) domain_parent: Option<SourceNodeSiteV1>,
    pub(super) kind: SourceBodyKindV1,
    pub(super) start: u32,
    pub(super) statements: &'plan [ASTNode],
}

pub(super) struct LegacyActivationClaimPartsV1<'a> {
    pub(super) plan_identity: usize,
    pub(super) caller: &'a CanonicalSameModuleCallableKeyV1,
    pub(super) site: &'a SourceExprSiteV1,
}

pub(super) struct LegacyActivationPrefixPartsV1<'a> {
    pub(super) plan_identity: usize,
    pub(super) caller: &'a CanonicalSameModuleCallableKeyV1,
    pub(super) prefix: Option<&'a SourceNodeSiteV1>,
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableResultLegacySourceViewV1<'plan> {
    plan_identity: usize,
    caller: &'plan CanonicalSameModuleCallableKeyV1,
    declaration: &'plan VerifiedSameModuleCallableDeclarationV1,
}

impl<'plan> VerifiedCallableResultLegacySourceViewV1<'plan> {
    pub(crate) fn verify(
        plan: &'plan VerifiedCallableResultActivationPlanV1,
        caller: &CanonicalSameModuleCallableKeyV1,
    ) -> Result<Self, CallableResultLegacyLocationErrorV1> {
        let declaration = plan
            .declaration_catalog()
            .declaration(caller)
            .ok_or_else(|| CallableResultLegacyLocationErrorV1::UnknownCaller(caller.clone()))?;
        Ok(Self {
            plan_identity: plan as *const _ as usize,
            caller: declaration.key(),
            declaration,
        })
    }

    pub(crate) fn root_body(&self) -> LegacyBodyInputV1<'plan> {
        LegacyBodyInputV1::Located(LocatedLegacyBodyV1 {
            plan_identity: self.plan_identity,
            caller: self.caller,
            parent: None,
            domain_parent: None,
            kind: SourceBodyKindV1::Function,
            statements: self.declaration.body(),
        })
    }

    pub(crate) fn body_suffix(
        &self,
        body: &LegacyBodyInputV1<'plan>,
        start: usize,
    ) -> Result<LocatedLegacyBodySuffixV1<'plan>, CallableResultLegacyLocationErrorV1> {
        let start = u32::try_from(start).map_err(|_| {
            CallableResultLegacyLocationErrorV1::BodySuffixIndexOverflow { index: start }
        })?;
        let LegacyBodyInputV1::Located(body) = body else {
            return Err(CallableResultLegacyLocationErrorV1::UnlocatedCannotProveInactive);
        };
        self.require_carrier(body.plan_identity, body.caller)?;
        let _len = u32::try_from(body.statements.len()).map_err(|_| {
            CallableResultLegacyLocationErrorV1::BodySuffixLengthOverflow {
                len: body.statements.len(),
            }
        })?;
        let Some(statements) = body.statements.get(start as usize..) else {
            return Err(
                CallableResultLegacyLocationErrorV1::BodySuffixStartOutOfBounds {
                    body: body.parent.clone(),
                    start,
                    len: body.statements.len(),
                },
            );
        };
        Ok(LocatedLegacyBodySuffixV1 {
            plan_identity: self.plan_identity,
            caller: self.caller,
            parent: body.parent.clone(),
            domain_parent: body.domain_parent.clone(),
            kind: body.kind,
            start,
            statements,
        })
    }

    pub(crate) fn unlocated_expr(&self, node: &'plan ASTNode) -> LegacyExprInputV1<'plan> {
        LegacyExprInputV1::Unlocated(UnlocatedLegacyExprV1 {
            plan_identity: self.plan_identity,
            caller: self.caller,
            node,
        })
    }

    /// Confirms that an already-created expression carrier belongs to this
    /// exact activation plan and caller. This is an identity check only; it
    /// creates no source path and claims no activation row.
    pub(crate) fn require_expr_carrier(
        &self,
        input: &LegacyExprInputV1<'plan>,
    ) -> Result<(), CallableResultLegacyLocationErrorV1> {
        let (identity, caller) = match input {
            LegacyExprInputV1::Located(input) => (input.plan_identity, input.caller),
            LegacyExprInputV1::Unlocated(input) => (input.plan_identity, input.caller),
        };
        self.require_carrier(identity, caller)
    }

    pub(crate) fn body_stmt(
        &self,
        body: &LegacyBodyInputV1<'plan>,
        index: usize,
    ) -> Result<LegacyStmtInputV1<'plan>, CallableResultLegacyLocationErrorV1> {
        let index = u32::try_from(index)
            .map_err(|_| CallableResultLegacyLocationErrorV1::BodyIndexOverflow { index })?;
        let (identity, caller, statements) = match body {
            LegacyBodyInputV1::Located(body) => (body.plan_identity, body.caller, body.statements),
            LegacyBodyInputV1::Unlocated(body) => {
                (body.plan_identity, body.caller, body.statements)
            }
        };
        self.require_carrier(identity, caller)?;
        let Some(node) = statements.get(index as usize) else {
            let root = match body {
                LegacyBodyInputV1::Located(body) => body.parent.clone(),
                LegacyBodyInputV1::Unlocated(_) => None,
            };
            return Err(CallableResultLegacyLocationErrorV1::BodyIndexOutOfBounds {
                body: root,
                index,
                len: statements.len(),
            });
        };
        Ok(match body {
            LegacyBodyInputV1::Located(body) => {
                let site = match &body.parent {
                    Some(parent) => SourcePathV1::from_node(parent)
                        .child(body.kind.item_segment(index))
                        .stmt(),
                    None => SourcePathV1::root_body(index as usize).stmt(),
                };
                LegacyStmtInputV1::Located(LocatedLegacyStmtV1 {
                    plan_identity: self.plan_identity,
                    caller: self.caller,
                    site,
                    node,
                })
            }
            LegacyBodyInputV1::Unlocated(_) => {
                LegacyStmtInputV1::Unlocated(UnlocatedLegacyStmtV1 {
                    plan_identity: self.plan_identity,
                    caller: self.caller,
                    node,
                })
            }
        })
    }

    pub(crate) fn statement_expression(
        &self,
        statement: &LegacyStmtInputV1<'plan>,
    ) -> Result<LegacyExprInputV1<'plan>, CallableResultLegacyLocationErrorV1> {
        let (identity, caller, node) = match statement {
            LegacyStmtInputV1::Located(statement) => {
                (statement.plan_identity, statement.caller, statement.node)
            }
            LegacyStmtInputV1::Unlocated(statement) => {
                (statement.plan_identity, statement.caller, statement.node)
            }
        };
        self.require_carrier(identity, caller)?;
        if !is_statement_expression_surface_v1(node) {
            let site = match statement {
                LegacyStmtInputV1::Located(statement) => statement.site.node().clone(),
                LegacyStmtInputV1::Unlocated(_) => SourcePathV1::function_body().node(),
            };
            return Err(CallableResultLegacyLocationErrorV1::StatementIsNotExpression(site));
        }
        Ok(match statement {
            LegacyStmtInputV1::Located(statement) => {
                LegacyExprInputV1::Located(LocatedLegacyExprV1 {
                    plan_identity: self.plan_identity,
                    caller: self.caller,
                    site: SourceExprSiteV1::from_node(statement.site.node().clone()),
                    node,
                })
            }
            LegacyStmtInputV1::Unlocated(_) => {
                LegacyExprInputV1::Unlocated(UnlocatedLegacyExprV1 {
                    plan_identity: self.plan_identity,
                    caller: self.caller,
                    node,
                })
            }
        })
    }

    pub(crate) fn child_expr(
        &self,
        parent: &LegacyExprInputV1<'plan>,
        role: ExprChildRoleV1,
    ) -> Result<LegacyExprInputV1<'plan>, CallableResultLegacyLocationErrorV1> {
        let (identity, caller) = match parent {
            LegacyExprInputV1::Located(parent) => (parent.plan_identity, parent.caller),
            LegacyExprInputV1::Unlocated(parent) => (parent.plan_identity, parent.caller),
        };
        self.require_carrier(identity, caller)?;
        let resolved = role.resolve(parent.node()).ok_or_else(|| {
            CallableResultLegacyLocationErrorV1::ExpressionRoleParentMismatch(parent_site_or_root(
                parent,
            ))
        })?;
        let node = match resolved.syntax() {
            ExprChildSyntaxV1::Node(node) => node,
            ExprChildSyntaxV1::SyntheticName | ExprChildSyntaxV1::Missing => {
                return Err(
                    CallableResultLegacyLocationErrorV1::ExpressionRoleHasNoSyntaxNode(
                        parent_site_or_root(parent),
                    ),
                )
            }
        };
        Ok(match parent {
            LegacyExprInputV1::Located(parent) => {
                let site = SourcePathV1::from_node(parent.site.node())
                    .child(resolved.segment())
                    .expr();
                LegacyExprInputV1::Located(LocatedLegacyExprV1 {
                    plan_identity: self.plan_identity,
                    caller: self.caller,
                    site,
                    node,
                })
            }
            LegacyExprInputV1::Unlocated(_) => {
                LegacyExprInputV1::Unlocated(UnlocatedLegacyExprV1 {
                    plan_identity: self.plan_identity,
                    caller: self.caller,
                    node,
                })
            }
        })
    }

    pub(crate) fn child_expr_from_stmt(
        &self,
        parent: &LegacyStmtInputV1<'plan>,
        role: ExprChildRoleV1,
    ) -> Result<LegacyExprInputV1<'plan>, CallableResultLegacyLocationErrorV1> {
        let (identity, caller) = match parent {
            LegacyStmtInputV1::Located(parent) => (parent.plan_identity, parent.caller),
            LegacyStmtInputV1::Unlocated(parent) => (parent.plan_identity, parent.caller),
        };
        self.require_carrier(identity, caller)?;
        let resolved = role.resolve(parent.node()).ok_or_else(|| {
            CallableResultLegacyLocationErrorV1::ExpressionRoleParentMismatch(stmt_site_or_root(
                parent,
            ))
        })?;
        let node = match resolved.syntax() {
            ExprChildSyntaxV1::Node(node) => node,
            ExprChildSyntaxV1::SyntheticName | ExprChildSyntaxV1::Missing => {
                return Err(
                    CallableResultLegacyLocationErrorV1::ExpressionRoleHasNoSyntaxNode(
                        stmt_site_or_root(parent),
                    ),
                )
            }
        };
        Ok(match parent {
            LegacyStmtInputV1::Located(parent) => LegacyExprInputV1::Located(LocatedLegacyExprV1 {
                plan_identity: self.plan_identity,
                caller: self.caller,
                site: SourcePathV1::from_node(parent.site.node())
                    .child(resolved.segment())
                    .expr(),
                node,
            }),
            LegacyStmtInputV1::Unlocated(_) => {
                LegacyExprInputV1::Unlocated(UnlocatedLegacyExprV1 {
                    plan_identity: self.plan_identity,
                    caller: self.caller,
                    node,
                })
            }
        })
    }

    pub(crate) fn child_body(
        &self,
        parent: &LegacyExprInputV1<'plan>,
        role: BodyChildRoleV1,
    ) -> Result<LegacyBodyInputV1<'plan>, CallableResultLegacyLocationErrorV1> {
        let (identity, caller) = match parent {
            LegacyExprInputV1::Located(parent) => (parent.plan_identity, parent.caller),
            LegacyExprInputV1::Unlocated(parent) => (parent.plan_identity, parent.caller),
        };
        self.require_carrier(identity, caller)?;
        let resolved = role.resolve(parent.node()).ok_or_else(|| {
            CallableResultLegacyLocationErrorV1::BodyRoleParentMismatch(parent_site_or_root(parent))
        })?;
        let Some(root_segment) = resolved.kind().root_segment() else {
            return Err(
                CallableResultLegacyLocationErrorV1::RootBodyRequestedAsChild(parent_site_or_root(
                    parent,
                )),
            );
        };
        let statements = resolved.statements().ok_or_else(|| {
            CallableResultLegacyLocationErrorV1::BodyRoleParentMismatch(parent_site_or_root(parent))
        })?;
        Ok(match parent {
            LegacyExprInputV1::Located(parent) => LegacyBodyInputV1::Located(LocatedLegacyBodyV1 {
                plan_identity: self.plan_identity,
                caller: self.caller,
                parent: Some(
                    SourcePathV1::from_node(parent.site.node())
                        .child(root_segment)
                        .node(),
                ),
                domain_parent: Some(parent.site.node().clone()),
                kind: resolved.kind(),
                statements,
            }),
            LegacyExprInputV1::Unlocated(_) => {
                LegacyBodyInputV1::Unlocated(UnlocatedLegacyBodyV1 {
                    plan_identity: self.plan_identity,
                    caller: self.caller,
                    statements,
                })
            }
        })
    }

    pub(crate) fn child_body_from_stmt(
        &self,
        parent: &LegacyStmtInputV1<'plan>,
        role: BodyChildRoleV1,
    ) -> Result<LegacyBodyInputV1<'plan>, CallableResultLegacyLocationErrorV1> {
        let (identity, caller) = match parent {
            LegacyStmtInputV1::Located(parent) => (parent.plan_identity, parent.caller),
            LegacyStmtInputV1::Unlocated(parent) => (parent.plan_identity, parent.caller),
        };
        self.require_carrier(identity, caller)?;
        let resolved = role.resolve(parent.node()).ok_or_else(|| {
            CallableResultLegacyLocationErrorV1::BodyRoleParentMismatch(stmt_site_or_root(parent))
        })?;
        let Some(root_segment) = resolved.kind().root_segment() else {
            return Err(
                CallableResultLegacyLocationErrorV1::RootBodyRequestedAsChild(stmt_site_or_root(
                    parent,
                )),
            );
        };
        let statements = resolved.statements().ok_or_else(|| {
            CallableResultLegacyLocationErrorV1::BodyRoleParentMismatch(stmt_site_or_root(parent))
        })?;
        Ok(match parent {
            LegacyStmtInputV1::Located(parent) => LegacyBodyInputV1::Located(LocatedLegacyBodyV1 {
                plan_identity: self.plan_identity,
                caller: self.caller,
                parent: Some(
                    SourcePathV1::from_node(parent.site.node())
                        .child(root_segment)
                        .node(),
                ),
                domain_parent: Some(parent.site.node().clone()),
                kind: resolved.kind(),
                statements,
            }),
            LegacyStmtInputV1::Unlocated(_) => {
                LegacyBodyInputV1::Unlocated(UnlocatedLegacyBodyV1 {
                    plan_identity: self.plan_identity,
                    caller: self.caller,
                    statements,
                })
            }
        })
    }

    fn require_carrier(
        &self,
        plan_identity: usize,
        actual: &CanonicalSameModuleCallableKeyV1,
    ) -> Result<(), CallableResultLegacyLocationErrorV1> {
        if plan_identity == self.plan_identity && std::ptr::eq(actual, self.caller) {
            return Ok(());
        }
        Err(CallableResultLegacyLocationErrorV1::ForeignCarrier {
            expected: self.caller.clone(),
            actual: actual.clone(),
        })
    }
}

fn parent_site_or_root(parent: &LegacyExprInputV1<'_>) -> SourceNodeSiteV1 {
    match parent {
        LegacyExprInputV1::Located(parent) => parent.site.node().clone(),
        LegacyExprInputV1::Unlocated(_) => SourcePathV1::function_body().node(),
    }
}

fn stmt_site_or_root(parent: &LegacyStmtInputV1<'_>) -> SourceNodeSiteV1 {
    match parent {
        LegacyStmtInputV1::Located(parent) => parent.site.node().clone(),
        LegacyStmtInputV1::Unlocated(_) => SourcePathV1::function_body().node(),
    }
}
