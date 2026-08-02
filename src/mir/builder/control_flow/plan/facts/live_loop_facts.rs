//! Live raw-source ownership for future pre-effect loop qualification.
//!
//! This is not a generic AST view. Its fields stay private so only the facts
//! builder may bind the raw frame to the Facts derived from that frame.

use crate::ast::ASTNode;

use super::LoopFacts;

/// A non-Clone pair tying one live raw loop frame to its derived facts.
#[derive(Debug)]
pub(in crate::mir::builder) struct LiveLoopFactsV1<'src> {
    frame: LiveLoopSourceFrameV1<'src>,
    facts: LoopFacts,
}

#[derive(Debug)]
struct LiveLoopSourceFrameV1<'src> {
    condition: &'src ASTNode,
    body: &'src [ASTNode],
}

impl<'src> LiveLoopFactsV1<'src> {
    pub(super) fn from_facts(
        condition: &'src ASTNode,
        body: &'src [ASTNode],
        facts: LoopFacts,
    ) -> Self {
        Self {
            frame: LiveLoopSourceFrameV1 { condition, body },
            facts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LiveLoopFactsV1;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::facts::try_build_live_loop_facts;

    #[test]
    fn live_pair_retains_the_builder_input_frame() {
        let variable = |name: &str| ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        };
        let integer = |value| ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        };
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("i")),
            right: Box::new(integer(1)),
            span: Span::unknown(),
        };
        let body = vec![ASTNode::Assignment {
            target: Box::new(variable("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable("i")),
                right: Box::new(integer(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let live = try_build_live_loop_facts(&condition, &body)
            .expect("no freeze")
            .expect("facts");

        assert!(std::ptr::eq(live.frame.condition, &condition));
        assert_eq!(live.frame.body.as_ptr(), body.as_ptr());
        assert_eq!(live.frame.body.len(), body.len());
        assert!(live.facts.source_receipt().is_available());
        let _: LiveLoopFactsV1<'_> = live;
    }
}
