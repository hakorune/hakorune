use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, SourceBodyKindV1, SourcePathSegmentV1, SourcePathV1,
};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

mod nested_control;
mod root_control_lambda;
mod scalar_local;
