//! Source-only root disposition for the shared raw module lifecycle.
//!
//! The selected invocation port is parity-safe only for expression trees whose
//! complete recursive surface is Literal, Variable, Me, Unary, Binary, Await,
//! Check, Array, Map, GroupedAssignment, Index, or an empty-prelude BlockExpr,
//! plus Print, Nowait, and annotation-free Local roots whose values are such
//! trees, exact variable assignments and compound assignments whose right-hand
//! sides are such trees, root-only Return with no value or such a tree, and
//! TaskScope bodies recursively composed from the non-terminal safe statements.
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
    LocalRoot(PortNeutralLocalRootV1),
    VariableAssignmentRoot(PortNeutralVariableAssignmentRootV1),
    VariableCompoundAssignmentRoot(PortNeutralVariableCompoundAssignmentRootV1),
    ReturnRoot(PortNeutralReturnRootV1),
    TaskScopeRoot(PortNeutralTaskScopeRootV1),
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

struct PortNeutralLocalRootV1 {
    node: ASTNode,
}

struct PortNeutralVariableAssignmentRootV1 {
    node: ASTNode,
}

struct PortNeutralVariableCompoundAssignmentRootV1 {
    node: ASTNode,
}

struct PortNeutralReturnRootV1 {
    node: ASTNode,
}

struct PortNeutralTaskScopeRootV1 {
    node: ASTNode,
}

impl SelectedRawNonProgramRootV1 {
    fn into_node(self) -> ASTNode {
        match self {
            Self::ExprTree(tree) => tree.node,
            Self::PrintRoot(root) => root.node,
            Self::NowaitRoot(root) => root.node,
            Self::LocalRoot(root) => root.node,
            Self::VariableAssignmentRoot(root) => root.node,
            Self::VariableCompoundAssignmentRoot(root) => root.node,
            Self::ReturnRoot(root) => root.node,
            Self::TaskScopeRoot(root) => root.node,
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
            node @ ASTNode::Index { .. } if is_port_neutral_expr_tree(&node) => {
                Self::selected_expr_tree(node)
            }
            node @ ASTNode::Index { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::BlockExpr { .. } if is_port_neutral_expr_tree(&node) => {
                Self::selected_expr_tree(node)
            }
            node @ ASTNode::BlockExpr { .. } => Self::compatibility(
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
            node @ ASTNode::Local { .. } if is_port_neutral_local_root(&node) => {
                Self::selected_local_root(node)
            }
            node @ ASTNode::Local { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::Assignment { .. }
                if is_port_neutral_variable_assignment_root(&node) =>
            {
                Self::selected_variable_assignment_root(node)
            }
            node @ ASTNode::Assignment { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::CompoundAssignment { .. }
                if is_port_neutral_variable_compound_assignment_root(&node) =>
            {
                Self::selected_variable_compound_assignment_root(node)
            }
            node @ ASTNode::CompoundAssignment { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::Return { .. } if is_port_neutral_return_root(&node) => {
                Self::selected_return_root(node)
            }
            node @ ASTNode::Return { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ ASTNode::TaskScope { .. } if is_port_neutral_task_scope_root(&node) => {
                Self::selected_task_scope_root(node)
            }
            node @ ASTNode::TaskScope { .. } => Self::compatibility(
                node,
                RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
            ),
            node @ (ASTNode::BoxDeclaration { .. } | ASTNode::Loop { .. }) => {
                Self::compatibility(node, RawNonProgramRootCompatibilityClassV1::ExplicitRoot)
            }
            node @ (ASTNode::If { .. }
            | ASTNode::QMarkPropagate { .. }
            | ASTNode::MatchExpr { .. }
            | ASTNode::EnumMatchExpr { .. }
            | ASTNode::RecordLiteral { .. }
            | ASTNode::RecordUpdate { .. }
            | ASTNode::Lambda { .. }
            | ASTNode::TryCatch { .. }
            | ASTNode::Throw { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::FieldAccess { .. }
            | ASTNode::New { .. }
            | ASTNode::FromCall { .. }
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

    fn selected_local_root(node: ASTNode) -> Self {
        Self::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::LocalRoot(PortNeutralLocalRootV1 { node }),
        ))
    }

    fn selected_task_scope_root(node: ASTNode) -> Self {
        Self::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::TaskScopeRoot(PortNeutralTaskScopeRootV1 { node }),
        ))
    }

    fn selected_variable_assignment_root(node: ASTNode) -> Self {
        Self::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::VariableAssignmentRoot(
                PortNeutralVariableAssignmentRootV1 { node },
            ),
        ))
    }

    fn selected_variable_compound_assignment_root(node: ASTNode) -> Self {
        Self::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::VariableCompoundAssignmentRoot(
                PortNeutralVariableCompoundAssignmentRootV1 { node },
            ),
        ))
    }

    fn selected_return_root(node: ASTNode) -> Self {
        Self::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::ReturnRoot(PortNeutralReturnRootV1 { node }),
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

fn is_port_neutral_local_root(node: &ASTNode) -> bool {
    let ASTNode::Local {
        initial_values,
        declared_type_names,
        ..
    } = node
    else {
        return false;
    };
    declared_type_names.iter().all(Option::is_none)
        && initial_values
            .iter()
            .all(|value| value.as_deref().is_none_or(is_port_neutral_expr_tree))
}

fn is_port_neutral_variable_assignment_root(node: &ASTNode) -> bool {
    let ASTNode::Assignment { target, value, .. } = node else {
        return false;
    };
    matches!(target.as_ref(), ASTNode::Variable { .. }) && is_port_neutral_expr_tree(value)
}

fn is_port_neutral_variable_compound_assignment_root(node: &ASTNode) -> bool {
    let ASTNode::CompoundAssignment { target, value, .. } = node else {
        return false;
    };
    matches!(target.as_ref(), ASTNode::Variable { .. }) && is_port_neutral_expr_tree(value)
}

fn is_port_neutral_return_root(node: &ASTNode) -> bool {
    let ASTNode::Return { value, .. } = node else {
        return false;
    };
    value.as_deref().map_or(true, is_port_neutral_expr_tree)
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
        ASTNode::Index { target, index, .. } => {
            is_port_neutral_expr_tree(target) && is_port_neutral_expr_tree(index)
        }
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => {
            prelude_stmts.iter().all(is_port_neutral_block_prelude_stmt)
                && is_port_neutral_expr_tree(tail_expr)
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
        | ASTNode::QMarkPropagate { .. }
        | ASTNode::MatchExpr { .. }
        | ASTNode::EnumMatchExpr { .. }
        | ASTNode::RecordLiteral { .. }
        | ASTNode::RecordUpdate { .. }
        | ASTNode::Lambda { .. }
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

fn is_port_neutral_block_prelude_stmt(node: &ASTNode) -> bool {
    is_port_neutral_expr_tree(node)
        || is_port_neutral_print_root(node)
        || is_port_neutral_nowait_root(node)
        || is_port_neutral_local_root(node)
        || is_port_neutral_variable_assignment_root(node)
        || is_port_neutral_variable_compound_assignment_root(node)
        || is_port_neutral_task_scope_root(node)
}

fn is_port_neutral_task_scope_root(node: &ASTNode) -> bool {
    let ASTNode::TaskScope { body, .. } = node else {
        return false;
    };
    body.iter().all(is_port_neutral_block_prelude_stmt)
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
