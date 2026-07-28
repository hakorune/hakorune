//! Source-only root disposition for the shared raw module lifecycle.
//!
//! The selected invocation port is parity-safe only for expression trees whose
//! complete recursive surface is Literal, Variable, Me, Unary, or Binary.
//! Every other non-Program root keeps the existing raw compatibility terminal
//! until its own production responsibility cell removes that residual.

use crate::ast::ASTNode;

use super::recursive_child_lowering::{
    drive_legacy_expression_v1, drive_raw_legacy_expression_v1, RawAstChildLoweringPortV1,
};
use super::{MirBuilder, ValueId};

pub(super) enum PreparedRawRootPartitionV1 {
    Program { statements: Vec<ASTNode> },
    NonProgram(PreparedRawNonProgramRootV1),
}

pub(super) enum PreparedRawNonProgramRootV1 {
    SelectedPortParity(PortNeutralExprTreeV1),
    Compatibility {
        node: ASTNode,
        class: RawNonProgramRootCompatibilityClassV1,
    },
}

struct PortNeutralExprTreeV1 {
    node: ASTNode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RawNonProgramRootCompatibilityClassV1 {
    ExplicitRoot,
    SeparateDesignStop,
    OutsideNormalFileIngress,
}

impl PreparedRawRootPartitionV1 {
    pub(super) fn classify(node: ASTNode) -> Self {
        match node {
            ASTNode::Program { statements, .. } => Self::Program { statements },
            node @ (ASTNode::Literal { .. } | ASTNode::Variable { .. } | ASTNode::Me { .. }) => {
                Self::selected(node)
            }
            node @ ASTNode::UnaryOp { .. } if is_port_neutral_expr_tree(&node) => {
                Self::selected(node)
            }
            node @ ASTNode::UnaryOp { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::BinaryOp { .. } if is_port_neutral_expr_tree(&node) => {
                Self::selected(node)
            }
            node @ ASTNode::BinaryOp { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ (ASTNode::BoxDeclaration { .. } | ASTNode::Loop { .. }) => {
                Self::compatibility(node, RawNonProgramRootCompatibilityClassV1::ExplicitRoot)
            }
            node @ (ASTNode::Assignment { .. }
            | ASTNode::CompoundAssignment { .. }
            | ASTNode::Print { .. }
            | ASTNode::If { .. }
            | ASTNode::Return { .. }
            | ASTNode::Nowait { .. }
            | ASTNode::TaskScope { .. }
            | ASTNode::AwaitExpression { .. }
            | ASTNode::QMarkPropagate { .. }
            | ASTNode::MatchExpr { .. }
            | ASTNode::EnumMatchExpr { .. }
            | ASTNode::ArrayLiteral { .. }
            | ASTNode::MapLiteral { .. }
            | ASTNode::RecordLiteral { .. }
            | ASTNode::RecordUpdate { .. }
            | ASTNode::Lambda { .. }
            | ASTNode::BlockExpr { .. }
            | ASTNode::TryCatch { .. }
            | ASTNode::Throw { .. }
            | ASTNode::CheckExpr { .. }
            | ASTNode::GroupedAssignmentExpr { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::FieldAccess { .. }
            | ASTNode::Index { .. }
            | ASTNode::New { .. }
            | ASTNode::FromCall { .. }
            | ASTNode::Local { .. }
            | ASTNode::ScopeBox { .. }
            | ASTNode::FunctionCall { .. }
            | ASTNode::Call { .. }) => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ (ASTNode::LoopRange { .. }
            | ASTNode::Break { .. }
            | ASTNode::Continue { .. }
            | ASTNode::UsingStatement { .. }
            | ASTNode::ImportStatement { .. }
            | ASTNode::BuildGate { .. }
            | ASTNode::ContextScope { .. }
            | ASTNode::FastMemRegion { .. }
            | ASTNode::Arrow { .. }
            | ASTNode::FunctionDeclaration { .. }
            | ASTNode::EnumDeclaration { .. }
            | ASTNode::BrandDeclaration { .. }
            | ASTNode::TypeAliasDeclaration { .. }
            | ASTNode::GlobalVar { .. }
            | ASTNode::StaticConstTable { .. }
            | ASTNode::This { .. }
            | ASTNode::ThisField { .. }
            | ASTNode::MeField { .. }
            | ASTNode::Outbox { .. }) => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::OutsideNormalFileIngress,
            ),
        }
    }

    fn selected(node: ASTNode) -> Self {
        Self::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            PortNeutralExprTreeV1 { node },
        ))
    }

    fn compatibility(node: ASTNode, class: RawNonProgramRootCompatibilityClassV1) -> Self {
        Self::NonProgram(PreparedRawNonProgramRootV1::Compatibility { node, class })
    }
}

fn is_port_neutral_expr_tree(node: &ASTNode) -> bool {
    match node {
        ASTNode::Literal { .. } | ASTNode::Variable { .. } | ASTNode::Me { .. } => true,
        ASTNode::UnaryOp { operand, .. } => is_port_neutral_expr_tree(operand),
        ASTNode::BinaryOp { left, right, .. } => {
            is_port_neutral_expr_tree(left) && is_port_neutral_expr_tree(right)
        }
        ASTNode::Program { .. }
        | ASTNode::Assignment { .. }
        | ASTNode::CompoundAssignment { .. }
        | ASTNode::Print { .. }
        | ASTNode::If { .. }
        | ASTNode::Loop { .. }
        | ASTNode::LoopRange { .. }
        | ASTNode::Return { .. }
        | ASTNode::Break { .. }
        | ASTNode::Continue { .. }
        | ASTNode::UsingStatement { .. }
        | ASTNode::ImportStatement { .. }
        | ASTNode::BuildGate { .. }
        | ASTNode::Nowait { .. }
        | ASTNode::TaskScope { .. }
        | ASTNode::ContextScope { .. }
        | ASTNode::FastMemRegion { .. }
        | ASTNode::AwaitExpression { .. }
        | ASTNode::QMarkPropagate { .. }
        | ASTNode::MatchExpr { .. }
        | ASTNode::EnumMatchExpr { .. }
        | ASTNode::ArrayLiteral { .. }
        | ASTNode::MapLiteral { .. }
        | ASTNode::RecordLiteral { .. }
        | ASTNode::RecordUpdate { .. }
        | ASTNode::Lambda { .. }
        | ASTNode::BlockExpr { .. }
        | ASTNode::Arrow { .. }
        | ASTNode::TryCatch { .. }
        | ASTNode::Throw { .. }
        | ASTNode::BoxDeclaration { .. }
        | ASTNode::FunctionDeclaration { .. }
        | ASTNode::EnumDeclaration { .. }
        | ASTNode::BrandDeclaration { .. }
        | ASTNode::TypeAliasDeclaration { .. }
        | ASTNode::GlobalVar { .. }
        | ASTNode::StaticConstTable { .. }
        | ASTNode::CheckExpr { .. }
        | ASTNode::GroupedAssignmentExpr { .. }
        | ASTNode::MethodCall { .. }
        | ASTNode::FieldAccess { .. }
        | ASTNode::Index { .. }
        | ASTNode::New { .. }
        | ASTNode::This { .. }
        | ASTNode::FromCall { .. }
        | ASTNode::ThisField { .. }
        | ASTNode::MeField { .. }
        | ASTNode::Local { .. }
        | ASTNode::ScopeBox { .. }
        | ASTNode::Outbox { .. }
        | ASTNode::FunctionCall { .. }
        | ASTNode::Call { .. } => false,
    }
}

pub(super) fn lower_raw_nonprogram_root_with_port_v1<Port>(
    builder: &mut MirBuilder,
    selected_port: &mut Port,
    prepared: PreparedRawNonProgramRootV1,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1,
{
    match prepared {
        PreparedRawNonProgramRootV1::SelectedPortParity(tree) => {
            drive_legacy_expression_v1(builder, selected_port, tree.node)
        }
        PreparedRawNonProgramRootV1::Compatibility { node, class } => {
            ExistingRawNonProgramRootCompatibilityV1::lower(builder, node, class)
        }
    }
}

struct ExistingRawNonProgramRootCompatibilityV1;

impl ExistingRawNonProgramRootCompatibilityV1 {
    fn lower(
        builder: &mut MirBuilder,
        node: ASTNode,
        _class: RawNonProgramRootCompatibilityClassV1,
    ) -> Result<ValueId, String> {
        drive_raw_legacy_expression_v1(builder, node)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
    use crate::parser::NyashParser;

    use super::{
        PreparedRawNonProgramRootV1, PreparedRawRootPartitionV1,
        RawNonProgramRootCompatibilityClassV1,
    };

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_owned(),
            span: Span::unknown(),
        }
    }

    fn assert_selected(node: ASTNode) {
        assert!(matches!(
            PreparedRawRootPartitionV1::classify(node),
            PreparedRawRootPartitionV1::NonProgram(
                PreparedRawNonProgramRootV1::SelectedPortParity(_)
            )
        ));
    }

    fn assert_compatibility(node: ASTNode, expected: RawNonProgramRootCompatibilityClassV1) {
        match PreparedRawRootPartitionV1::classify(node) {
            PreparedRawRootPartitionV1::NonProgram(
                PreparedRawNonProgramRootV1::Compatibility { class, .. },
            ) => assert_eq!(class, expected),
            _ => panic!("root must remain on the compatibility route"),
        }
    }

    fn first_statement(source: &str) -> ASTNode {
        match NyashParser::parse_from_string(source).expect("root source") {
            ASTNode::Program { mut statements, .. } => statements.remove(0),
            _ => panic!("parser must return Program"),
        }
    }

    #[test]
    fn port_neutral_partition_is_recursive_and_disjoint() {
        assert_selected(integer(1));
        assert_selected(variable("x"));
        assert_selected(ASTNode::Me {
            span: Span::unknown(),
        });
        assert_selected(ASTNode::UnaryOp {
            operator: UnaryOperator::Minus,
            operand: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(integer(2)),
                right: Box::new(variable("x")),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        });

        assert_compatibility(
            ASTNode::UnaryOp {
                operator: UnaryOperator::Minus,
                operand: Box::new(ASTNode::New {
                    class: "Page".to_owned(),
                    arguments: Vec::new(),
                    field_initializers: Vec::new(),
                    type_arguments: Vec::new(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
        );
        assert_compatibility(
            ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(integer(3)),
                right: Box::new(ASTNode::FieldAccess {
                    object: Box::new(variable("page")),
                    field: "value".to_owned(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
        );
    }

    #[test]
    fn program_box_and_loop_keep_their_existing_root_owners() {
        assert!(matches!(
            PreparedRawRootPartitionV1::classify(ASTNode::Program {
                statements: vec![integer(1)],
                span: Span::unknown(),
            }),
            PreparedRawRootPartitionV1::Program { .. }
        ));
        assert_compatibility(
            first_statement("box Page {}"),
            RawNonProgramRootCompatibilityClassV1::ExplicitRoot,
        );
        assert_compatibility(
            first_statement("loop(true) { break }"),
            RawNonProgramRootCompatibilityClassV1::ExplicitRoot,
        );
    }
}
