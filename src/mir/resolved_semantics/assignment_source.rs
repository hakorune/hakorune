//! Resolver-owned source form for one assignment statement.
//!
//! Target/value paths are intentionally not enough to identify the semantic
//! operation: plain and compound assignments share those paths.  This row is
//! emitted during the same shadow traversal as the body-shape inventory.

use super::body_shape::ShadowBodyShapeDraftV0;
use super::source_site::{SourceExprSiteV1, SourceStmtSiteV1};
use super::source_site::{SourcePathSegmentV1, SourcePathV1};
use crate::ast::ASTNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedAssignmentFormV1 {
    Plain,
    Compound,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedAssignmentSourceV1 {
    statement_site: SourceStmtSiteV1,
    target_site: SourceExprSiteV1,
    value_site: SourceExprSiteV1,
    form: ResolvedAssignmentFormV1,
}

impl ResolvedAssignmentSourceV1 {
    pub(crate) const fn statement_site(&self) -> &SourceStmtSiteV1 {
        &self.statement_site
    }

    pub(crate) const fn target_site(&self) -> &SourceExprSiteV1 {
        &self.target_site
    }

    pub(crate) const fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }

    pub(crate) const fn form(&self) -> ResolvedAssignmentFormV1 {
        self.form
    }
}

fn issue_assignment_source_v1(
    statement_site: SourceStmtSiteV1,
    target_site: SourceExprSiteV1,
    value_site: SourceExprSiteV1,
    form: ResolvedAssignmentFormV1,
) -> ResolvedAssignmentSourceV1 {
    ResolvedAssignmentSourceV1 {
        statement_site,
        target_site,
        value_site,
        form,
    }
}

pub(super) fn record_shadow_assignment_source(
    body_shape: &mut ShadowBodyShapeDraftV0,
    statement: &ASTNode,
    site: SourceStmtSiteV1,
) {
    let Some((target_segment, value_segment, form)) = (match statement {
        ASTNode::Assignment { .. } => Some((
            SourcePathSegmentV1::Target,
            SourcePathSegmentV1::Value,
            ResolvedAssignmentFormV1::Plain,
        )),
        ASTNode::CompoundAssignment { .. } => Some((
            SourcePathSegmentV1::Target,
            SourcePathSegmentV1::Value,
            ResolvedAssignmentFormV1::Compound,
        )),
        _ => None,
    }) else {
        return;
    };
    let source = SourcePathV1::from_node(site.node());
    body_shape.assignment_sources.insert(
        site.clone(),
        issue_assignment_source_v1(
            site,
            source.child(target_segment).expr(),
            source.child(value_segment).expr(),
            form,
        ),
    );
}
