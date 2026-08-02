//! Source-bound loop demand views.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::loop_source_receipt::{
    LoopSourceReceiptV1, LoopSourceSlotV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopSourceViewErrorV1 {
    ReceiptUnavailable,
    ReceiptBodyLengthMismatch,
    SourceFrameMismatch,
    SlotOutOfBounds,
}

/// A borrowed view of the original loop source, sealed by the facts receipt.
#[derive(Debug, Clone, Copy)]
pub(crate) struct LoopSourceViewV1<'src> {
    condition: &'src ASTNode,
    body: &'src [ASTNode],
    receipt: &'src LoopSourceReceiptV1,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct LoopSourceDemandV1<'src> {
    slot: LoopSourceSlotV1,
    node: &'src ASTNode,
}

impl<'src> LoopSourceViewV1<'src> {
    pub(crate) fn try_new(
        condition: &'src ASTNode,
        body: &'src [ASTNode],
        receipt: &'src LoopSourceReceiptV1,
    ) -> Result<Self, LoopSourceViewErrorV1> {
        match receipt.raw_body_statement_count() {
            None => Err(LoopSourceViewErrorV1::ReceiptUnavailable),
            Some(count) if count != body.len() => {
                Err(LoopSourceViewErrorV1::ReceiptBodyLengthMismatch)
            }
            Some(_) if !receipt.matches_source_frame(condition, body) => {
                Err(LoopSourceViewErrorV1::SourceFrameMismatch)
            }
            Some(_) => Ok(Self {
                condition,
                body,
                receipt,
            }),
        }
    }

    pub(crate) fn demand(
        self,
        slot: LoopSourceSlotV1,
    ) -> Result<LoopSourceDemandV1<'src>, LoopSourceViewErrorV1> {
        let node = match slot {
            LoopSourceSlotV1::Condition => self.condition,
            LoopSourceSlotV1::BodyStatement(index) => self
                .body
                .get(index)
                .ok_or(LoopSourceViewErrorV1::SlotOutOfBounds)?,
        };
        self.receipt
            .original_ordinal(slot)
            .ok_or(LoopSourceViewErrorV1::SlotOutOfBounds)?;
        Ok(LoopSourceDemandV1 { slot, node })
    }
}

impl<'src> LoopSourceDemandV1<'src> {
    pub(crate) fn slot(self) -> LoopSourceSlotV1 {
        self.slot
    }

    pub(crate) fn node(self) -> &'src ASTNode {
        self.node
    }
}

#[cfg(test)]
mod tests {
    use super::{LoopSourceViewErrorV1, LoopSourceViewV1};
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::facts::loop_source_receipt::{
        LoopSourceReceiptV1, LoopSourceSlotV1,
    };

    fn literal(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn view_preserves_receipt_order_without_rebuilding_nodes() {
        let condition = literal(0);
        let body = vec![literal(1), literal(2)];
        let receipt = LoopSourceReceiptV1::from_raw_loop(&condition, &body);
        let view = LoopSourceViewV1::try_new(&condition, &body, &receipt).expect("view");

        assert_eq!(
            view.demand(LoopSourceSlotV1::Condition).unwrap().slot(),
            LoopSourceSlotV1::Condition
        );
        assert_eq!(
            view.demand(LoopSourceSlotV1::BodyStatement(1))
                .unwrap()
                .node(),
            &body[1]
        );
    }

    #[test]
    fn view_rejects_unavailable_or_out_of_range_receipts() {
        let condition = literal(0);
        let body = vec![literal(1)];
        assert!(matches!(
            LoopSourceViewV1::try_new(&condition, &body, &LoopSourceReceiptV1::default()),
            Err(LoopSourceViewErrorV1::ReceiptUnavailable)
        ));

        let receipt = LoopSourceReceiptV1::from_raw_loop(&condition, &body);
        let view = LoopSourceViewV1::try_new(&condition, &body, &receipt).expect("view");
        assert!(matches!(
            view.demand(LoopSourceSlotV1::BodyStatement(1)),
            Err(LoopSourceViewErrorV1::SlotOutOfBounds)
        ));
    }

    #[test]
    fn view_rejects_same_length_foreign_source_frame() {
        let condition = literal(0);
        let body = vec![literal(1)];
        let receipt = LoopSourceReceiptV1::from_raw_loop(&condition, &body);
        let foreign_condition = literal(0);
        let foreign_body = vec![literal(1)];

        assert!(matches!(
            LoopSourceViewV1::try_new(&foreign_condition, &foreign_body, &receipt),
            Err(LoopSourceViewErrorV1::SourceFrameMismatch)
        ));
    }
}
