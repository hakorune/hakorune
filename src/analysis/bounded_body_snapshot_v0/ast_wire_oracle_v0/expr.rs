use crate::ast::{ASTNode, BinaryOperator, LiteralValue, UnaryOperator};

use super::{
    AstWireOracleErrorV0, AstWireOracleV0, AtomKeyV0, AtomValueV0, ChildRoleV0, PathV0,
    SnapshotNodeIndexV0, WireExprKindV0, WireNodeKindV0,
};

impl AstWireOracleV0 {
    pub(super) fn emit_expr(
        &mut self,
        expression: &ASTNode,
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        match expression {
            ASTNode::Literal { value, .. } => self.emit_literal(value, path, depth),
            ASTNode::Variable { name, .. } => self.emit_leaf(
                path,
                depth,
                WireExprKindV0::Var,
                vec![(AtomKeyV0::Name, AtomValueV0::Text(name.clone()))],
            ),
            ASTNode::This { .. } => self.emit_named_var("this", path, depth),
            ASTNode::Me { .. } => self.emit_named_var("me", path, depth),
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => self.emit_binary(operator, left, right, path, depth),
            ASTNode::UnaryOp {
                operator: UnaryOperator::Minus,
                operand,
                ..
            } => self.emit_negative_integer(operand, path, depth),
            ASTNode::FunctionCall {
                name, arguments, ..
            } if name == "env.console.log" => self.emit_call(name, arguments, path, depth),
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
            | ASTNode::FromCall { .. }
            | ASTNode::ThisField { .. }
            | ASTNode::MeField { .. }
            | ASTNode::Local { .. }
            | ASTNode::ScopeBox { .. }
            | ASTNode::Outbox { .. }
            | ASTNode::Call { .. }
            | ASTNode::FunctionCall { .. }
            | ASTNode::UnaryOp { .. } => Self::unsupported(
                path,
                expression.node_type(),
                "unsupported.source_wire_projection",
            ),
        }
    }

    pub(super) fn emit_call(
        &mut self,
        name: &str,
        arguments: &[ASTNode],
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        let index = self.reserve(path, WireNodeKindV0::Expr(WireExprKindV0::Call), depth)?;
        let mut children = Vec::with_capacity(arguments.len());
        for (ordinal, argument) in arguments.iter().enumerate() {
            let child = self.emit_expr(
                argument,
                &path.field(ChildRoleV0::Args.path_field()).index(ordinal),
                depth + 1,
            )?;
            children.push((ChildRoleV0::Args, child));
        }
        self.seal(
            index,
            vec![(AtomKeyV0::Name, AtomValueV0::Text(name.to_string()))],
            children,
        )?;
        Ok(index)
    }

    fn emit_literal(
        &mut self,
        value: &LiteralValue,
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        match value {
            LiteralValue::Integer(value) | LiteralValue::TypedInteger { value, .. } => self
                .emit_leaf(
                    path,
                    depth,
                    WireExprKindV0::Int,
                    vec![(AtomKeyV0::Value, AtomValueV0::I64(*value))],
                ),
            LiteralValue::String(value) => self.emit_leaf(
                path,
                depth,
                WireExprKindV0::Str,
                vec![(AtomKeyV0::Value, AtomValueV0::Text(value.clone()))],
            ),
            LiteralValue::Bool(value) => self.emit_leaf(
                path,
                depth,
                WireExprKindV0::Bool,
                vec![(AtomKeyV0::Value, AtomValueV0::Bool(*value))],
            ),
            LiteralValue::Null | LiteralValue::Void => self.emit_leaf(
                path,
                depth,
                WireExprKindV0::Null,
                vec![(AtomKeyV0::Value, AtomValueV0::Null)],
            ),
            LiteralValue::Float(_) => {
                Self::unsupported(path, "Literal", "unsupported.source_float_projection")
            }
        }
    }

    fn emit_negative_integer(
        &mut self,
        operand: &ASTNode,
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        let value = match operand {
            ASTNode::Literal {
                value: LiteralValue::Integer(value),
                ..
            }
            | ASTNode::Literal {
                value: LiteralValue::TypedInteger { value, .. },
                ..
            } => value.checked_neg(),
            _ => None,
        };
        let Some(value) = value else {
            return Self::unsupported(path, "UnaryOp", "unsupported.source_unary_projection");
        };
        self.emit_leaf(
            path,
            depth,
            WireExprKindV0::Int,
            vec![(AtomKeyV0::Value, AtomValueV0::I64(value))],
        )
    }

    fn emit_binary(
        &mut self,
        operator: &BinaryOperator,
        left: &ASTNode,
        right: &ASTNode,
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        let kind = match operator {
            BinaryOperator::Add
            | BinaryOperator::Subtract
            | BinaryOperator::Multiply
            | BinaryOperator::Divide
            | BinaryOperator::Modulo
            | BinaryOperator::BitAnd
            | BinaryOperator::BitOr
            | BinaryOperator::BitXor
            | BinaryOperator::Shl
            | BinaryOperator::Shr => WireExprKindV0::Binary,
            BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::Less
            | BinaryOperator::Greater
            | BinaryOperator::LessEqual
            | BinaryOperator::GreaterEqual => WireExprKindV0::Compare,
            BinaryOperator::And | BinaryOperator::Or => WireExprKindV0::Logical,
        };
        let index = self.reserve(path, WireNodeKindV0::Expr(kind), depth)?;
        let lhs = self.emit_expr(left, &path.field(ChildRoleV0::Lhs.path_field()), depth + 1)?;
        let rhs = self.emit_expr(right, &path.field(ChildRoleV0::Rhs.path_field()), depth + 1)?;
        self.seal(
            index,
            vec![(AtomKeyV0::Op, AtomValueV0::Text(operator.to_string()))],
            vec![(ChildRoleV0::Lhs, lhs), (ChildRoleV0::Rhs, rhs)],
        )?;
        Ok(index)
    }

    fn emit_named_var(
        &mut self,
        name: &str,
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        self.emit_leaf(
            path,
            depth,
            WireExprKindV0::Var,
            vec![(AtomKeyV0::Name, AtomValueV0::Text(name.to_string()))],
        )
    }

    fn emit_leaf(
        &mut self,
        path: &PathV0,
        depth: usize,
        kind: WireExprKindV0,
        atoms: Vec<(AtomKeyV0, AtomValueV0)>,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        let index = self.reserve(path, WireNodeKindV0::Expr(kind), depth)?;
        self.seal(index, atoms, vec![])?;
        Ok(index)
    }
}
