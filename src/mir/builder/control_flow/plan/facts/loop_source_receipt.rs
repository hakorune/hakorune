//! Loop-relative source order captured before facts normalize the body.
//!
//! This is an identity receipt, not an AST view. It deliberately retains no
//! node reference and grants no lowering or route-selection authority.

use crate::ast::ASTNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopSourceSlotV1 {
    Condition,
    BodyStatement(usize),
}

/// Original loop-source coordinates observed before `ScopeBox` flattening.
///
/// A receipt is unavailable for synthetic `LoopFacts` assembled by tests or
/// transforms. Future logical-demand issuance must reject that state instead
/// of inferring coordinates from normalized facts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::mir::builder) struct LoopSourceReceiptV1 {
    raw_body_statement_count: Option<usize>,
    source_frame: Option<LoopSourceFrameStampV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoopSourceFrameStampV1 {
    condition_address: usize,
    body_address: usize,
    body_len: usize,
}

impl LoopSourceReceiptV1 {
    pub(in crate::mir::builder) fn from_raw_loop(condition: &ASTNode, body: &[ASTNode]) -> Self {
        Self {
            raw_body_statement_count: Some(body.len()),
            source_frame: Some(LoopSourceFrameStampV1 {
                condition_address: condition as *const ASTNode as usize,
                body_address: body.as_ptr() as usize,
                body_len: body.len(),
            }),
        }
    }

    /// Test-only receipt for source-order assertions without a borrowing frame.
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_raw_loop_body(body: &[ASTNode]) -> Self {
        Self {
            raw_body_statement_count: Some(body.len()),
            source_frame: None,
        }
    }

    pub(in crate::mir::builder) fn is_available(&self) -> bool {
        self.raw_body_statement_count.is_some()
    }

    pub(in crate::mir::builder) fn raw_body_statement_count(&self) -> Option<usize> {
        self.raw_body_statement_count
    }

    pub(in crate::mir::builder) fn matches_source_frame(
        &self,
        condition: &ASTNode,
        body: &[ASTNode],
    ) -> bool {
        self.source_frame.is_some_and(|stamp| {
            stamp.condition_address == condition as *const ASTNode as usize
                && stamp.body_address == body.as_ptr() as usize
                && stamp.body_len == body.len()
        })
    }

    /// Returns the stable ordinal in the original loop source.
    ///
    /// The condition is ordinal zero; original body statements follow in their
    /// unflattened order. This never describes a flattened analysis body.
    pub(in crate::mir::builder) fn original_ordinal(
        &self,
        slot: LoopSourceSlotV1,
    ) -> Option<usize> {
        let raw_body_statement_count = self.raw_body_statement_count?;
        match slot {
            LoopSourceSlotV1::Condition => Some(0),
            LoopSourceSlotV1::BodyStatement(index) if index < raw_body_statement_count => {
                Some(index + 1)
            }
            LoopSourceSlotV1::BodyStatement(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LoopSourceReceiptV1, LoopSourceSlotV1};
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
        assert_eq!(
            receipt.original_ordinal(LoopSourceSlotV1::Condition),
            Some(0)
        );
        assert_eq!(
            receipt.original_ordinal(LoopSourceSlotV1::BodyStatement(0)),
            Some(1)
        );
        assert_eq!(
            receipt.original_ordinal(LoopSourceSlotV1::BodyStatement(1)),
            None
        );
        assert_eq!(flattened.len(), 2);
    }
}
