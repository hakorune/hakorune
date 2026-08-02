//! Loop-relative source order captured before facts normalize the body.
//!
//! This is an arity receipt, not an AST view. It deliberately retains no node
//! reference, address stamp, lowering, or route-selection authority.

use crate::ast::ASTNode;

/// Original loop-source coordinates observed before `ScopeBox` flattening.
///
/// A receipt is unavailable for synthetic `LoopFacts` assembled by tests or
/// transforms. Future logical-demand issuance must reject that state instead
/// of inferring coordinates from normalized facts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::mir::builder) struct LoopSourceReceiptV1 {
    raw_body_statement_count: Option<usize>,
}

impl LoopSourceReceiptV1 {
    pub(in crate::mir::builder) fn from_raw_loop(_condition: &ASTNode, body: &[ASTNode]) -> Self {
        Self {
            raw_body_statement_count: Some(body.len()),
        }
    }

    /// Test-only receipt for source-order assertions without a borrowing frame.
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_raw_loop_body(body: &[ASTNode]) -> Self {
        Self {
            raw_body_statement_count: Some(body.len()),
        }
    }

    pub(in crate::mir::builder) fn is_available(&self) -> bool {
        self.raw_body_statement_count.is_some()
    }

    pub(in crate::mir::builder) fn raw_body_statement_count(&self) -> Option<usize> {
        self.raw_body_statement_count
    }
}

#[cfg(test)]
mod tests {
    use super::LoopSourceReceiptV1;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::facts::stmt_view::flatten_scope_boxes;

    #[test]
    fn scope_box_keeps_original_body_ordinal_outside_flattened_analysis_body() {
        let body = vec![ASTNode::ScopeBox {
            body: vec![
                ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                },
                ASTNode::Break {
                    span: Span::unknown(),
                },
            ],
            span: Span::unknown(),
        }];

        let condition = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let receipt = LoopSourceReceiptV1::from_raw_loop(&condition, &body);
        let flattened = flatten_scope_boxes(&body);

        assert!(receipt.is_available());
        assert_eq!(receipt.raw_body_statement_count(), Some(1));
        assert_eq!(flattened.len(), 2);
    }
}
