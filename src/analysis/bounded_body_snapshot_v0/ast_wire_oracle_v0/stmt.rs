use crate::ast::{ASTNode, LiteralValue, Span};

use super::{
    AstWireOracleErrorV0, AstWireOracleV0, AtomKeyV0, AtomValueV0, ChildRoleV0, PathV0,
    SnapshotNodeIndexV0, WireNodeKindV0, WireStmtKindV0,
};

impl AstWireOracleV0 {
    pub(super) fn emit_body(
        &mut self,
        statements: &[ASTNode],
        body_path: PathV0,
        depth: usize,
        roots: bool,
    ) -> Result<Vec<SnapshotNodeIndexV0>, AstWireOracleErrorV0> {
        let mut indices = Vec::new();
        for statement in statements {
            self.emit_stmt_many(statement, &body_path, depth, roots, &mut indices)?;
        }
        Ok(indices)
    }

    fn emit_stmt_many(
        &mut self,
        statement: &ASTNode,
        body_path: &PathV0,
        depth: usize,
        roots: bool,
        out: &mut Vec<SnapshotNodeIndexV0>,
    ) -> Result<(), AstWireOracleErrorV0> {
        match statement {
            ASTNode::Program { statements, .. }
            | ASTNode::ScopeBox {
                body: statements, ..
            } => {
                for nested in statements {
                    self.emit_stmt_many(nested, body_path, depth, roots, out)?;
                }
            }
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                for (binding, name) in variables.iter().enumerate() {
                    let path = body_path.index(out.len());
                    let value = initial_values
                        .get(binding)
                        .and_then(|value| value.as_deref());
                    let index = self.emit_local(name, value, &path, depth)?;
                    self.publish_stmt(index, roots, out)?;
                }
            }
            _ => {
                let path = body_path.index(out.len());
                let index = self.emit_stmt(statement, &path, depth)?;
                self.publish_stmt(index, roots, out)?;
            }
        }
        Ok(())
    }

    fn publish_stmt(
        &mut self,
        index: SnapshotNodeIndexV0,
        roots: bool,
        out: &mut Vec<SnapshotNodeIndexV0>,
    ) -> Result<(), AstWireOracleErrorV0> {
        if roots {
            self.builder
                .add_root(index)
                .map_err(AstWireOracleErrorV0::Snapshot)?;
        }
        out.push(index);
        Ok(())
    }

    fn emit_stmt(
        &mut self,
        statement: &ASTNode,
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        match statement {
            ASTNode::Assignment { target, value, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return Self::unsupported(
                        path,
                        "Assignment",
                        "unsupported.assignment_target_projection",
                    );
                };
                self.emit_local(name, Some(value), path, depth)
            }
            ASTNode::Print { expression, .. } => self.emit_print(expression, path, depth),
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => self.emit_if(condition, then_body, else_body.as_deref(), path, depth),
            ASTNode::Loop {
                condition, body, ..
            } => self.emit_loop(condition, body, path, depth),
            ASTNode::LoopRange {
                var_name,
                start,
                end,
                body,
                ..
            } => self.emit_loop_range(var_name, start, end, body, path, depth),
            ASTNode::Return { value, .. } => self.emit_return(value.as_deref(), path, depth),
            ASTNode::Break { .. } => self.emit_leaf_stmt(WireStmtKindV0::Break, path, depth),
            ASTNode::Continue { .. } => self.emit_leaf_stmt(WireStmtKindV0::Continue, path, depth),
            ASTNode::Literal { .. }
            | ASTNode::Variable { .. }
            | ASTNode::UnaryOp { .. }
            | ASTNode::BinaryOp { .. }
            | ASTNode::This { .. }
            | ASTNode::Me { .. }
            | ASTNode::FunctionCall { .. } => self.emit_expr_stmt(statement, path, depth),
            ASTNode::Program { .. }
            | ASTNode::CompoundAssignment { .. }
            | ASTNode::Release { .. }
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
            | ASTNode::Call { .. } => Self::unsupported(
                path,
                statement.node_type(),
                "unsupported.source_wire_projection",
            ),
        }
    }

    fn emit_local(
        &mut self,
        name: &str,
        value: Option<&ASTNode>,
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        let index = self.reserve(path, WireNodeKindV0::Stmt(WireStmtKindV0::Local), depth)?;
        let null = ASTNode::Literal {
            value: LiteralValue::Null,
            span: Span::unknown(),
        };
        let child = self.emit_expr(
            value.unwrap_or(&null),
            &path.field(ChildRoleV0::Expr.path_field()),
            depth + 1,
        )?;
        self.seal(
            index,
            vec![(AtomKeyV0::Name, AtomValueV0::Text(name.to_string()))],
            vec![(ChildRoleV0::Expr, child)],
        )?;
        Ok(index)
    }

    fn emit_print(
        &mut self,
        expression: &ASTNode,
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        let index = self.reserve(path, WireNodeKindV0::Stmt(WireStmtKindV0::Expr), depth)?;
        let child = self.emit_call(
            "env.console.log",
            std::slice::from_ref(expression),
            &path.field(ChildRoleV0::Expr.path_field()),
            depth + 1,
        )?;
        self.seal(index, vec![], vec![(ChildRoleV0::Expr, child)])?;
        Ok(index)
    }

    fn emit_expr_stmt(
        &mut self,
        expression: &ASTNode,
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        let index = self.reserve(path, WireNodeKindV0::Stmt(WireStmtKindV0::Expr), depth)?;
        let child = self.emit_expr(
            expression,
            &path.field(ChildRoleV0::Expr.path_field()),
            depth + 1,
        )?;
        self.seal(index, vec![], vec![(ChildRoleV0::Expr, child)])?;
        Ok(index)
    }

    fn emit_if(
        &mut self,
        condition: &ASTNode,
        then_body: &[ASTNode],
        else_body: Option<&[ASTNode]>,
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        let index = self.reserve(path, WireNodeKindV0::Stmt(WireStmtKindV0::If), depth)?;
        let cond = self.emit_expr(
            condition,
            &path.field(ChildRoleV0::Cond.path_field()),
            depth + 1,
        )?;
        let then_nodes = self.emit_body(
            then_body,
            path.field(ChildRoleV0::Then.path_field()),
            depth + 1,
            false,
        )?;
        let else_nodes = match else_body {
            Some(body) => self.emit_body(
                body,
                path.field(ChildRoleV0::Else.path_field()),
                depth + 1,
                false,
            )?,
            None => vec![],
        };
        let mut children = vec![(ChildRoleV0::Cond, cond)];
        children.extend(then_nodes.into_iter().map(|node| (ChildRoleV0::Then, node)));
        children.extend(else_nodes.into_iter().map(|node| (ChildRoleV0::Else, node)));
        self.seal(index, vec![], children)?;
        Ok(index)
    }

    fn emit_loop(
        &mut self,
        condition: &ASTNode,
        body: &[ASTNode],
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        let index = self.reserve(path, WireNodeKindV0::Stmt(WireStmtKindV0::Loop), depth)?;
        let cond = self.emit_expr(
            condition,
            &path.field(ChildRoleV0::Cond.path_field()),
            depth + 1,
        )?;
        let body_nodes = self.emit_body(
            body,
            path.field(ChildRoleV0::Body.path_field()),
            depth + 1,
            false,
        )?;
        let mut children = vec![(ChildRoleV0::Cond, cond)];
        children.extend(body_nodes.into_iter().map(|node| (ChildRoleV0::Body, node)));
        self.seal(index, vec![], children)?;
        Ok(index)
    }

    fn emit_loop_range(
        &mut self,
        var_name: &str,
        start: &ASTNode,
        end: &ASTNode,
        body: &[ASTNode],
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        let index = self.reserve(path, WireNodeKindV0::Stmt(WireStmtKindV0::LoopRange), depth)?;
        let start = self.emit_expr(
            start,
            &path.field(ChildRoleV0::Start.path_field()),
            depth + 1,
        )?;
        let end = self.emit_expr(end, &path.field(ChildRoleV0::End.path_field()), depth + 1)?;
        let body_nodes = self.emit_body(
            body,
            path.field(ChildRoleV0::Body.path_field()),
            depth + 1,
            false,
        )?;
        let mut children = vec![(ChildRoleV0::Start, start), (ChildRoleV0::End, end)];
        children.extend(body_nodes.into_iter().map(|node| (ChildRoleV0::Body, node)));
        self.seal(
            index,
            vec![(AtomKeyV0::VarName, AtomValueV0::Text(var_name.to_string()))],
            children,
        )?;
        Ok(index)
    }

    fn emit_return(
        &mut self,
        value: Option<&ASTNode>,
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        let index = self.reserve(path, WireNodeKindV0::Stmt(WireStmtKindV0::Return), depth)?;
        let zero = ASTNode::Literal {
            value: LiteralValue::Integer(0),
            span: Span::unknown(),
        };
        let child = self.emit_expr(
            value.unwrap_or(&zero),
            &path.field(ChildRoleV0::Expr.path_field()),
            depth + 1,
        )?;
        self.seal(index, vec![], vec![(ChildRoleV0::Expr, child)])?;
        Ok(index)
    }

    fn emit_leaf_stmt(
        &mut self,
        kind: WireStmtKindV0,
        path: &PathV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        let index = self.reserve(path, WireNodeKindV0::Stmt(kind), depth)?;
        self.seal(index, vec![], vec![])?;
        Ok(index)
    }
}
