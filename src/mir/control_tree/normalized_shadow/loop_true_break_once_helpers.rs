use crate::mir::control_tree::step_tree::{StepNode, StepStmtKind};

pub(super) fn extract_loop_true_pattern(
    root: &StepNode,
) -> Option<(&[StepNode], &StepNode, &[StepNode])> {
    match root {
        StepNode::Loop { .. } => {
            // Loop is the only statement
            Some((&[][..], root, &[][..]))
        }
        StepNode::Block(nodes) => {
            // Find loop in block
            for (i, node) in nodes.iter().enumerate() {
                if matches!(node, StepNode::Loop { .. }) {
                    let prefix = &nodes[..i];
                    let post = &nodes[i + 1..];
                    return Some((prefix, node, post));
                }
            }
            None
        }
        _ => None,
    }
}

pub(super) fn body_ends_with_break(stmts: &[StepNode]) -> bool {
    if let Some(last) = stmts.last() {
        matches!(
            last,
            StepNode::Stmt {
                kind: StepStmtKind::Break,
                ..
            }
        )
    } else {
        false
    }
}

pub(super) fn has_unsupported_exits(stmts: &[StepNode]) -> bool {
    for stmt in stmts {
        match stmt {
            StepNode::Stmt { kind, .. } => match kind {
                StepStmtKind::Return { .. } | StepStmtKind::Continue => return true,
                _ => {}
            },
            StepNode::Block(inner) => {
                if has_unsupported_exits(inner) {
                    return true;
                }
            }
            StepNode::If {
                then_branch,
                else_branch,
                ..
            } => {
                if has_unsupported_exits_in_node(then_branch) {
                    return true;
                }
                if let Some(else_node) = else_branch {
                    if has_unsupported_exits_in_node(else_node) {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}

fn has_unsupported_exits_in_node(node: &StepNode) -> bool {
    match node {
        StepNode::Stmt { kind, .. } => {
            matches!(kind, StepStmtKind::Return { .. } | StepStmtKind::Continue)
        }
        StepNode::Block(stmts) => has_unsupported_exits(stmts),
        StepNode::If {
            then_branch,
            else_branch,
            ..
        } => {
            has_unsupported_exits_in_node(then_branch)
                || else_branch
                    .as_ref()
                    .map_or(false, |n| has_unsupported_exits_in_node(n))
        }
        _ => false,
    }
}

pub(super) fn log_post_nodes_debug(post_nodes: &[StepNode]) {
    if crate::config::env::joinir_dev_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[phase133/debug] post_nodes.len() = {}",
            post_nodes.len()
        ));
        for (i, node) in post_nodes.iter().enumerate() {
            match node {
                StepNode::Stmt { kind, .. } => ring0.log.debug(&format!(
                    "[phase133/debug] post_nodes[{}] = Stmt({:?})",
                    i, kind
                )),
                _ => ring0
                    .log
                    .debug(&format!("[phase133/debug] post_nodes[{}] = {:?}", i, node)),
            }
        }
    }
}

pub(super) fn log_post_computation_debug(has_post_computation: bool) {
    if crate::config::env::joinir_dev_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[phase133/debug] has_post_computation = {}",
            has_post_computation
        ));
    }
}
