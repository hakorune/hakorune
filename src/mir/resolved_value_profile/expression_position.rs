//! The structural position of an expression site, decided once from its source
//! path. Every `SourcePathSegmentV1` is matched here, so a new kind of statement
//! or expression position is a compile error instead of silently counting as an
//! expression value. This owner grants no execution; who realizes a Call at a
//! position is decided elsewhere.
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourcePathSegmentV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExpressionPositionV1 {
    /// An item of a statement sequence: its value ends with the statement.
    Statement,
    /// The whole initializer of a `local`.
    LocalInitializer,
    /// An actual of a Call/New, or the callee of a call.
    Argument,
    /// The receiver of a method call.
    Receiver,
    /// A binary or unary operand.
    Operand,
    /// An `if` or loop condition.
    Condition,
    /// The value of an Assignment, print or return statement. The plan root
    /// tells which; the path alone cannot.
    StatementValue,
    /// An element, entry, field, update, base, check item, target, binding,
    /// match, `?` or block-expression tail inside a larger expression.
    Component,
    /// A scope or body container, never itself a value.
    Container,
}

impl ExpressionPositionV1 {
    pub(crate) fn of(site: &SourceExprSiteV1) -> Option<Self> {
        site.node().segments().last().map(Self::of_segment)
    }

    pub(crate) fn of_segment(segment: &SourcePathSegmentV1) -> Self {
        use SourcePathSegmentV1 as S;
        match segment {
            S::Body(_) | S::ProgramBody(_) | S::ScopeBody(_) | S::TaskScopeBody(_)
            | S::FastMemBody(_) | S::IfThen(_) | S::IfElse(_) | S::LoopBody(_)
            | S::LambdaBody(_) | S::BlockExprPrelude(_) | S::TryBody(_) | S::CatchBody(_)
            | S::CleanupBody(_) => Self::Statement,
            S::Initializer(_) => Self::LocalInitializer,
            S::Argument(_) | S::Callee => Self::Argument,
            S::Receiver => Self::Receiver,
            S::Lhs | S::Rhs | S::Operand => Self::Operand,
            S::IfCondition | S::LoopCondition => Self::Condition,
            S::Value => Self::StatementValue,
            S::Element(_) | S::EntryValue(_) | S::FieldValue(_) | S::UpdateValue(_) | S::Base
            | S::CheckItem(_) | S::Target | S::Binding(_) | S::QMarkOperand | S::MatchScrutinee
            | S::MatchArm(_) | S::MatchElse | S::EnumMatchScrutinee | S::EnumMatchArm(_)
            | S::EnumMatchElse | S::BlockExprTail | S::CatchClause(_) => Self::Component,
            S::FunctionBody | S::ProgramBodyRoot | S::ScopeBodyRoot | S::TaskScopeBodyRoot
            | S::FastMemBodyRoot | S::IfThenBody | S::IfElseBody | S::LoopBodyRoot
            | S::LambdaBodyRoot | S::BlockExprPreludeRoot | S::TryBodyRoot | S::CatchBodyRoot
            | S::CleanupBodyRoot => Self::Container,
        }
    }

    /// A Call here is an expression value (argument, operand, condition, printed,
    /// returned or assigned value, ...), except as the whole initializer of a
    /// Local or as a statement of its own, which keep their statement-level
    /// writers.
    pub(crate) fn stands_as_expression_value(self) -> bool {
        match self {
            Self::Statement | Self::LocalInitializer => false,
            Self::Argument | Self::Receiver | Self::Operand | Self::Condition
            | Self::StatementValue | Self::Component | Self::Container => true,
        }
    }
}

/// Whether a Call at `site` is an expression value. An empty path is the
/// function root, which is not a statement and is not a Local initializer.
pub(crate) fn site_stands_as_expression_value_v1(site: &SourceExprSiteV1) -> bool {
    ExpressionPositionV1::of(site).map_or(true, ExpressionPositionV1::stands_as_expression_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use SourcePathSegmentV1 as S;

    #[test]
    fn statement_sequence_items_are_statements_not_values() {
        for segment in [
            S::Body(0), S::ProgramBody(0), S::ScopeBody(1), S::TaskScopeBody(0), S::FastMemBody(0),
            S::IfThen(0), S::IfElse(0), S::LoopBody(0), S::LambdaBody(0),
            S::BlockExprPrelude(0), S::TryBody(0), S::CatchBody(0), S::CleanupBody(0),
        ] {
            let position = ExpressionPositionV1::of_segment(&segment);
            assert_eq!(position, ExpressionPositionV1::Statement, "{segment:?}");
            assert!(!position.stands_as_expression_value(), "{segment:?}");
        }
    }

    #[test]
    fn only_statements_and_whole_local_initializers_keep_a_statement_writer() {
        assert!(!ExpressionPositionV1::of_segment(&S::Initializer(0)).stands_as_expression_value());
        for segment in [
            S::Argument(0), S::Callee, S::Receiver, S::Lhs, S::Rhs, S::Operand, S::IfCondition,
            S::LoopCondition, S::Value, S::Element(0), S::BlockExprTail, S::MatchArm(0),
        ] {
            assert!(ExpressionPositionV1::of_segment(&segment).stands_as_expression_value(), "{segment:?}");
        }
    }
}
