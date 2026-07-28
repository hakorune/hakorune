//! Source-only root disposition for the shared raw module lifecycle.
//!
//! The selected invocation port is parity-safe only for expression trees whose
//! complete recursive surface is Literal, Variable, Me, Unary, Binary, Await,
//! Check, Array, Map, or GroupedAssignment, plus Print and Nowait roots whose
//! value is one such tree.
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
    SelectedPortParity(SelectedRawNonProgramRootV1),
    Compatibility {
        node: ASTNode,
        class: RawNonProgramRootCompatibilityClassV1,
    },
}

enum SelectedRawNonProgramRootV1 {
    ExprTree(PortNeutralExprTreeV1),
    PrintRoot(PortNeutralPrintRootV1),
    NowaitRoot(PortNeutralNowaitRootV1),
}

struct PortNeutralExprTreeV1 {
    node: ASTNode,
}

struct PortNeutralPrintRootV1 {
    node: ASTNode,
}

struct PortNeutralNowaitRootV1 {
    node: ASTNode,
}

impl SelectedRawNonProgramRootV1 {
    fn into_node(self) -> ASTNode {
        match self {
            Self::ExprTree(tree) => tree.node,
            Self::PrintRoot(root) => root.node,
            Self::NowaitRoot(root) => root.node,
        }
    }
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
                Self::selected_expr_tree(node)
            }
            node @ ASTNode::UnaryOp { .. } if is_port_neutral_expr_tree(&node) => {
                Self::selected_expr_tree(node)
            }
            node @ ASTNode::UnaryOp { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::BinaryOp { .. } if is_port_neutral_expr_tree(&node) => {
                Self::selected_expr_tree(node)
            }
            node @ ASTNode::BinaryOp { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::AwaitExpression { .. } if is_port_neutral_expr_tree(&node) => {
                Self::selected_expr_tree(node)
            }
            node @ ASTNode::AwaitExpression { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::CheckExpr { .. } if is_port_neutral_expr_tree(&node) => {
                Self::selected_expr_tree(node)
            }
            node @ ASTNode::CheckExpr { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::ArrayLiteral { .. } if is_port_neutral_expr_tree(&node) => {
                Self::selected_expr_tree(node)
            }
            node @ ASTNode::ArrayLiteral { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::MapLiteral { .. } if is_port_neutral_expr_tree(&node) => {
                Self::selected_expr_tree(node)
            }
            node @ ASTNode::MapLiteral { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::GroupedAssignmentExpr { .. } if is_port_neutral_expr_tree(&node) => {
                Self::selected_expr_tree(node)
            }
            node @ ASTNode::GroupedAssignmentExpr { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::Print { .. } if is_port_neutral_print_root(&node) => {
                Self::selected_print_root(node)
            }
            node @ ASTNode::Print { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::Nowait { .. } if is_port_neutral_nowait_root(&node) => {
                Self::selected_nowait_root(node)
            }
            node @ ASTNode::Nowait { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ (ASTNode::BoxDeclaration { .. } | ASTNode::Loop { .. }) => {
                Self::compatibility(node, RawNonProgramRootCompatibilityClassV1::ExplicitRoot)
            }
            node @ (ASTNode::Assignment { .. }
            | ASTNode::CompoundAssignment { .. }
            | ASTNode::If { .. }
            | ASTNode::Return { .. }
            | ASTNode::TaskScope { .. }
            | ASTNode::QMarkPropagate { .. }
            | ASTNode::MatchExpr { .. }
            | ASTNode::EnumMatchExpr { .. }
            | ASTNode::RecordLiteral { .. }
            | ASTNode::RecordUpdate { .. }
            | ASTNode::Lambda { .. }
            | ASTNode::BlockExpr { .. }
            | ASTNode::TryCatch { .. }
            | ASTNode::Throw { .. }
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

    fn selected_expr_tree(node: ASTNode) -> Self {
        Self::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::ExprTree(PortNeutralExprTreeV1 { node }),
        ))
    }

    fn selected_print_root(node: ASTNode) -> Self {
        Self::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::PrintRoot(PortNeutralPrintRootV1 { node }),
        ))
    }

    fn selected_nowait_root(node: ASTNode) -> Self {
        Self::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::NowaitRoot(PortNeutralNowaitRootV1 { node }),
        ))
    }

    fn compatibility(node: ASTNode, class: RawNonProgramRootCompatibilityClassV1) -> Self {
        Self::NonProgram(PreparedRawNonProgramRootV1::Compatibility { node, class })
    }
}

fn is_port_neutral_print_root(node: &ASTNode) -> bool {
    let ASTNode::Print { expression, .. } = node else {
        return false;
    };
    is_port_neutral_expr_tree(expression)
}

fn is_port_neutral_nowait_root(node: &ASTNode) -> bool {
    let ASTNode::Nowait { expression, .. } = node else {
        return false;
    };
    is_port_neutral_expr_tree(expression)
}

fn is_port_neutral_expr_tree(node: &ASTNode) -> bool {
    match node {
        ASTNode::Literal { .. } | ASTNode::Variable { .. } | ASTNode::Me { .. } => true,
        ASTNode::UnaryOp { operand, .. } => is_port_neutral_expr_tree(operand),
        ASTNode::BinaryOp { left, right, .. } => {
            is_port_neutral_expr_tree(left) && is_port_neutral_expr_tree(right)
        }
        ASTNode::AwaitExpression { expression, .. } => is_port_neutral_expr_tree(expression),
        ASTNode::CheckExpr { items, .. } => items
            .iter()
            .all(|item| is_port_neutral_expr_tree(&item.expression)),
        ASTNode::ArrayLiteral { elements, .. } => elements.iter().all(is_port_neutral_expr_tree),
        ASTNode::MapLiteral { entries, .. } => entries
            .iter()
            .all(|(_, value)| is_port_neutral_expr_tree(value)),
        ASTNode::GroupedAssignmentExpr { rhs, .. } => is_port_neutral_expr_tree(rhs),
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
        | ASTNode::QMarkPropagate { .. }
        | ASTNode::MatchExpr { .. }
        | ASTNode::EnumMatchExpr { .. }
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
        PreparedRawNonProgramRootV1::SelectedPortParity(root) => {
            drive_legacy_expression_v1(builder, selected_port, root.into_node())
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
#[path = "raw_nonprogram_root_descent_tests.rs"]
mod tests;
