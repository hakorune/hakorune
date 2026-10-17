//! Exact source receipts and raw syntax child topology; no lowering.
use super::{ASTNode, RawInvocationSourceContextV1, RawLoopChildEntryDispositionV1,
    SourceBodyKindV1, SourcePathSegmentV1};

pub(super) fn verify_exact_loop_child_receipts(
    condition: &RawInvocationSourceContextV1,
    body: &RawInvocationSourceContextV1,
) -> Result<(), String> {
    let condition_is_exact = condition
        .site()
        .is_some_and(|site| site.segments().last() == Some(&SourcePathSegmentV1::LoopCondition));
    let body_is_exact = matches!(
        body,
        RawInvocationSourceContextV1::Located {
            site,
            body_kind: Some(SourceBodyKindV1::Loop),
            ..
        } if site.segments().last() == Some(&SourcePathSegmentV1::LoopBodyRoot)
    );
    if condition_is_exact && body_is_exact {
        Ok(())
    } else {
        Err("[freeze:contract][raw-loop-child-entry/exact-child-receipts]".to_owned())
    }
}

/// Classify the original condition and body of one raw `ASTNode::Loop`.
///
/// The AST traversal API is the generic child-topology SSOT. Lambda and nested
/// function declaration bodies are deferred ownership surfaces: neither is
/// executed by the surrounding raw Loop lowering, so this classifier does not
/// descend into them. A `BoxDeclaration` itself is executable on the raw
/// dispatcher path and is therefore a direct child-entry boundary.
pub(in crate::mir::builder) fn classify_raw_loop_child_entry_v1(
    condition: &ASTNode,
    body: &[ASTNode],
) -> RawLoopChildEntryDispositionV1 {
    let has_child_entry = contains_reachable_box_declaration(condition)
        || body.iter().any(contains_reachable_box_declaration);

    if has_child_entry {
        RawLoopChildEntryDispositionV1::ReachableBoxDeclaration
    } else {
        RawLoopChildEntryDispositionV1::NoChildFunctionEntry
    }
}

fn contains_reachable_box_declaration(node: &ASTNode) -> bool {
    match node {
        ASTNode::BoxDeclaration { .. } => true,
        ASTNode::Lambda { .. } | ASTNode::FunctionDeclaration { .. } => false,
        _ => node.any_child(contains_reachable_box_declaration),
    }
}
