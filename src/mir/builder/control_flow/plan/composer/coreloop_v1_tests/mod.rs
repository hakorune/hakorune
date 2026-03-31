//! Tests for coreloop_v1 route composers.

mod if_phi_join;
mod loop_break;
mod loop_true_early_exit;
mod split_scan;

use crate::ast::{ASTNode, LiteralValue, Span};

fn v(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn lit_int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}
