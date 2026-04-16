//! Utility functions for body access and binding synchronization.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::features::body_view::BodyView;
use crate::mir::builder::control_flow::recipes::refs::{StmtRef, StmtSpan};
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

/// Get a single statement from the body by reference.
pub(super) fn get_body_stmt<'a>(
    body: &BodyView<'a>,
    stmt_ref: StmtRef,
    error_prefix: &str,
) -> Result<&'a ASTNode, String> {
    body.get_stmt(stmt_ref).ok_or_else(|| {
        format!(
            "{error_prefix}: recipe stmt idx out of range (idx={})",
            stmt_ref.index()
        )
    })
}

/// Get a span of statements from the body.
pub(super) fn get_body_span<'a>(
    body: &BodyView<'a>,
    span: StmtSpan,
    error_prefix: &str,
    label: &str,
) -> Result<&'a [ASTNode], String> {
    body.get_span(span).ok_or_else(|| {
        format!(
            "{error_prefix}: {label} span out of range (start={}, end={})",
            span.start_index(),
            span.end_index()
        )
    })
}

/// Check if a map mutation affects existing variables.
pub(super) fn map_mutates_existing_vars(
    pre: &BTreeMap<String, crate::mir::ValueId>,
    post: &BTreeMap<String, crate::mir::ValueId>,
) -> bool {
    for (name, pre_id) in pre {
        if let Some(post_id) = post.get(name) {
            if post_id != pre_id {
                return true;
            }
        }
    }
    false
}

/// Phase 29bq: Sync carrier bindings from variable_ctx after nested loop lowering.
pub(super) fn sync_carrier_bindings(
    builder: &MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
) {
    for (name, _) in carrier_phis {
        if let Some(value_id) = builder.variable_ctx.variable_map.get(name) {
            current_bindings.insert(name.clone(), *value_id);
        }
    }
}
