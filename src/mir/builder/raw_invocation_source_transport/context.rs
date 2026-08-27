use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawInvocationSourceContextV1 {
    Located {
        root: RawInvocationRootLineageV1,
        site: SourceNodeSiteV1,
        body_kind: Option<SourceBodyKindV1>,
    },
    UnlocatedCompatibility {
        reason: RawUnlocatedPortalV1,
        expected_lineage: Option<RawInvocationRootLineageV1>,
    },
}

impl RawInvocationSourceContextV1 {
    pub(in crate::mir::builder) fn from_transport<T>(
        transport: RawInvocationSourceTransportV1<T>,
    ) -> (T, Self) {
        let (node, located, unlocated) = transport.into_parts();
        let context = match (located, unlocated) {
            (Some((root, site, body_kind)), None) => Self::Located {
                root,
                site,
                body_kind: Some(body_kind),
            },
            (None, Some((reason, expected_lineage))) => Self::UnlocatedCompatibility {
                reason,
                expected_lineage,
            },
            _ => unreachable!("[freeze:contract][raw-invocation/source-transport-state]"),
        };
        (node, context)
    }

    pub(in crate::mir::builder) fn body_statement(
        &self,
        statement: ASTNode,
        index: usize,
    ) -> RawInvocationSourceTransportV1<ASTNode> {
        match self {
            Self::Located {
                root,
                site,
                body_kind,
            } => {
                if !matches!(&statement, ASTNode::BoxDeclaration { .. })
                    && !is_located_control_or_diagnostic_terminal(&statement)
                    && !is_located_scalar_statement(&statement)
                    && !is_located_zero_child_runtime_completion(&statement)
                    && !is_located_lambda_statement(&statement)
                    && !(root.allows_bare_function_call_location()
                        && is_bare_function_call_statement(&statement))
                {
                    let reason = reason_for_non_box_statement(&statement);
                    return RawInvocationSourceTransportV1::unlocated_with_expected_lineage(
                        statement,
                        reason,
                        root.clone(),
                    );
                }
                let kind = body_kind.expect("located body transport must retain its body kind");
                let child = body_item_site(kind, site, index);
                RawInvocationSourceTransportV1::Located(LocatedRawNodeV1::new(
                    statement,
                    root.clone(),
                    child,
                    kind,
                ))
            }
            Self::UnlocatedCompatibility {
                reason,
                expected_lineage,
            } => match expected_lineage {
                Some(root) => RawInvocationSourceTransportV1::unlocated_with_expected_lineage(
                    statement,
                    *reason,
                    root.clone(),
                ),
                None => RawInvocationSourceTransportV1::unlocated(statement, *reason),
            },
        }
    }

    pub(in crate::mir::builder) fn site(&self) -> Option<&SourceNodeSiteV1> {
        match self {
            Self::Located { site, .. } => Some(site),
            Self::UnlocatedCompatibility { .. } => None,
        }
    }

    pub(in crate::mir::builder) fn shares_root_lineage(
        &self,
        other: &RawInvocationSourceContextV1,
    ) -> bool {
        match (self, other) {
            (Self::Located { root: left, .. }, Self::Located { root: right, .. }) => left == right,
            _ => false,
        }
    }

    pub(in crate::mir::builder) fn is_exact_loop_condition(&self) -> bool {
        matches!(
            self,
            Self::Located {
                site,
                body_kind: None,
                ..
            } if site.segments().last() == Some(&SourcePathSegmentV1::LoopCondition)
        )
    }

    pub(in crate::mir::builder) fn is_exact_loop_body_root(&self) -> bool {
        matches!(
            self,
            Self::Located {
                site,
                body_kind: Some(SourceBodyKindV1::Loop),
                ..
            } if site.segments().last() == Some(&SourcePathSegmentV1::LoopBodyRoot)
        )
    }

    pub(super) fn child_call_argument(&self, index: usize) -> Self {
        match self {
            Self::Located { root, site, .. } => Self::Located {
                root: root.clone(),
                site: SourcePathV1::from_node(site)
                    .child(SourcePathSegmentV1::Argument(index as u32))
                    .node(),
                body_kind: None,
            },
            Self::UnlocatedCompatibility {
                reason,
                expected_lineage,
            } => Self::UnlocatedCompatibility {
                reason: *reason,
                expected_lineage: expected_lineage.clone(),
            },
        }
    }

    pub(in crate::mir::builder) fn child_expression(
        &self,
        parent: &ASTNode,
        role: ExprChildRoleV1,
    ) -> Result<Self, String> {
        let Self::Located { root, site, .. } = self else {
            return Ok(self.clone());
        };
        let resolved = role.resolve(parent).ok_or_else(|| {
            format!(
                "[freeze:contract][raw-invocation/expr-child-role] parent={} role={role:?}",
                parent.node_type()
            )
        })?;
        if !matches!(resolved.syntax(), ExprChildSyntaxV1::Node(_)) {
            return Err(format!(
                "[freeze:contract][raw-invocation/expr-child-missing] parent={} role={role:?}",
                parent.node_type()
            ));
        }
        Ok(Self::Located {
            root: root.clone(),
            site: SourcePathV1::from_node(site)
                .child(resolved.segment())
                .node(),
            body_kind: None,
        })
    }

    pub(in crate::mir::builder) fn child_body(
        &self,
        parent: &ASTNode,
        role: BodyChildRoleV1,
    ) -> Result<Self, String> {
        let Self::Located { root, site, .. } = self else {
            return Ok(self.clone());
        };
        let resolved = role.resolve(parent).ok_or_else(|| {
            format!(
                "[freeze:contract][raw-invocation/body-child-role] parent={} role={role:?}",
                parent.node_type()
            )
        })?;
        if resolved.statements().is_none() {
            return Err(format!(
                "[freeze:contract][raw-invocation/body-child-missing] parent={} role={role:?}",
                parent.node_type()
            ));
        }
        let kind = resolved.kind();
        let path = kind.append_root_path(SourcePathV1::from_node(site));
        Ok(Self::Located {
            root: root.clone(),
            site: path.node(),
            body_kind: Some(kind),
        })
    }

    pub(in crate::mir::builder) fn structured_body_statement(
        &self,
        statement: ASTNode,
        index: usize,
    ) -> Result<RawInvocationSourceTransportV1<ASTNode>, String> {
        Ok(self.body_statement(statement, index))
    }

    pub(in crate::mir::builder) fn child_statement(
        &self,
        statement: &ASTNode,
        index: usize,
    ) -> Result<Self, String> {
        let Self::Located {
            root,
            site,
            body_kind,
        } = self
        else {
            return Ok(self.clone());
        };
        let kind = body_kind.ok_or_else(|| {
            "[freeze:contract][raw-invocation/missing-parent-body-kind]".to_owned()
        })?;
        if kind != SourceBodyKindV1::Program
            && !is_located_control_or_diagnostic_terminal(statement)
            && !is_located_scalar_statement(statement)
            && !is_located_zero_child_runtime_completion(statement)
            && !is_located_lambda_statement(statement)
            && !(root.allows_bare_function_call_location()
                && is_bare_function_call_statement(statement))
        {
            return Err(format!(
                "[freeze:contract][raw-invocation/statement-source-role] kind={}",
                statement.node_type()
            ));
        }
        let child = body_item_site(kind, site, index);
        Ok(Self::Located {
            root: root.clone(),
            site: child,
            body_kind: None,
        })
    }
}
