//! Current-loop assignment scanning for LoopUpdateSummary.
//!
//! This module only collects statement-level carrier assignments from the
//! current loop body and current-loop if branches. Nested loop bodies and
//! assignment-value expressions are not update proof for the current loop.

use crate::ast::ASTNode;

/// Collect every current-loop RHS expression assigning to `var_name`.
pub(super) fn collect_assignment_rhses<'a>(
    var_name: &str,
    loop_body: &'a [ASTNode],
) -> Vec<&'a ASTNode> {
    fn visit_node<'a>(var_name: &str, node: &'a ASTNode, rhses: &mut Vec<&'a ASTNode>) {
        match node {
            ASTNode::Assignment { target, value, .. } => {
                if let ASTNode::Variable { name, .. } = target.as_ref() {
                    if name == var_name {
                        rhses.push(value.as_ref());
                    }
                }
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                for stmt in then_body {
                    visit_node(var_name, stmt, rhses);
                }
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        visit_node(var_name, stmt, rhses);
                    }
                }
            }
            ASTNode::Loop { .. } => {}
            _ => {}
        }
    }

    let mut rhses = Vec::new();
    for stmt in loop_body {
        visit_node(var_name, stmt, &mut rhses);
    }
    rhses
}
