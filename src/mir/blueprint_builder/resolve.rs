//! Source admission, lexical binding, types and structured control contracts.
use super::declaration_table::*;
use crate::ast::{ASTNode, LiteralValue, Span};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1 as Key;
use std::collections::{BTreeMap, BTreeSet};
#[path = "resolve_declarations.rs"]
mod declarations;

pub(super) fn reject(code: &'static str, detail: impl Into<String>, span: Span) -> Refusal {
    Refusal {
        code,
        detail: detail.into(),
        span,
    }
}
pub(super) fn declared_type(name: Option<&str>, span: Span) -> Result<Type, Refusal> {
    match name {
        Some("i64") => Ok(Type::I64),
        Some("Bool") => Ok(Type::Bool),
        None => Err(reject(
            "MissingTypeAnnotation",
            "explicit i64/Bool signature required",
            span,
        )),
        Some(name) => Err(reject("UnsupportedType", name, span)),
    }
}
fn require(actual: Type, expected: Type, span: Span) -> Result<(), Refusal> {
    if actual == expected {
        Ok(())
    } else {
        Err(reject(
            "TypeMismatch",
            format!("expected {expected:?}, found {actual:?}"),
            span,
        ))
    }
}

pub(super) fn module(ast: ASTNode) -> Result<ResolvedModule, Refusal> {
    let (mut table, bodies, span) = declarations::collect(ast)?;
    let entries: Vec<_> = table
        .entries
        .iter()
        .filter(|d| {
            d.key.name() == "main" && (d.owner.is_none() || d.owner.as_deref() == Some("Main"))
        })
        .map(|d| d.id)
        .collect();
    if entries.len() != 1 {
        return Err(reject(
            "MissingOrAmbiguousEntry",
            "one Main.main or free main is required",
            span,
        ));
    }
    let entry = entries[0];
    if !table.entries[entry.0].parameters.is_empty() || table.entries[entry.0].result != Type::I64 {
        return Err(reject(
            "UnsupportedEntrySignature",
            "native entry requires () : i64",
            span,
        ));
    }
    let mut functions = Vec::new();
    let mut effects = Vec::new();
    for (index, body) in bodies.into_iter().enumerate() {
        let declaration = &table.entries[index];
        let mut resolver = Resolver {
            table: &table,
            declaration,
            next_node: 0,
            bindings: Vec::new(),
            scopes: vec![BTreeMap::new()],
            loops: Vec::new(),
            calls: BTreeSet::new(),
            io: false,
        };
        for (name, ty) in &declaration.parameters {
            resolver.bind(name, *ty, false, declaration.span)?;
        }
        let (body, flow) = resolver.block(&body, false)?;
        if flow.normal {
            return Err(reject(
                "MissingReturn",
                "a normal function exit needs a return value",
                declaration.span,
            ));
        }
        effects.push((resolver.io, resolver.calls));
        functions.push(ResolvedFunction {
            declaration: declaration.id,
            body,
            bindings: resolver.bindings,
        });
    }
    loop {
        let previous: Vec<_> = table.entries.iter().map(|d| d.io).collect();
        for (index, (direct, calls)) in effects.iter().enumerate() {
            table.entries[index].io = *direct || calls.iter().any(|id| previous[id.0]);
        }
        if table.entries.iter().map(|d| d.io).eq(previous) {
            break;
        }
    }
    Ok(ResolvedModule {
        declarations: table,
        functions,
        entry,
    })
}

#[derive(Default)]
struct Flow {
    normal: bool,
    breaks: BTreeSet<NodeId>,
}
struct Resolver<'a> {
    table: &'a DeclarationTable,
    declaration: &'a Declaration,
    next_node: usize,
    bindings: Vec<Binding>,
    scopes: Vec<BTreeMap<String, BindingId>>,
    loops: Vec<NodeId>,
    calls: BTreeSet<DeclarationId>,
    io: bool,
}
impl Resolver<'_> {
    fn node(&mut self) -> NodeId {
        let id = NodeId(self.next_node);
        self.next_node += 1;
        id
    }
    fn bind(
        &mut self,
        name: &str,
        ty: Type,
        readonly: bool,
        span: Span,
    ) -> Result<BindingId, Refusal> {
        let scope = self.scopes.last_mut().unwrap();
        if scope.contains_key(name) {
            return Err(reject("DuplicateBinding", name, span));
        }
        let id = BindingId(self.bindings.len());
        self.bindings.push(Binding { ty, readonly });
        scope.insert(name.into(), id);
        Ok(id)
    }
    fn binding(&self, name: &str, span: Span) -> Result<BindingId, Refusal> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
            .copied()
            .ok_or_else(|| reject("UnknownName", name, span))
    }
    fn visible(&self) -> Vec<BindingId> {
        // Shadowing affects name lookup, not the lifetime of outer storage.
        // Every binding in a live lexical scope must survive inner merges.
        self.scopes
            .iter()
            .flat_map(|scope| scope.values().copied())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    fn block(&mut self, nodes: &[ASTNode], scope: bool) -> Result<(Vec<Statement>, Flow), Refusal> {
        if scope {
            self.scopes.push(BTreeMap::new());
        }
        let mut statements = Vec::new();
        let mut flow = Flow {
            normal: true,
            ..Flow::default()
        };
        for node in nodes {
            let (mut row, next) = self.statement(node)?;
            statements.append(&mut row);
            if flow.normal {
                flow.normal = next.normal;
                flow.breaks.extend(next.breaks);
            }
        }
        if scope {
            self.scopes.pop();
        }
        Ok((statements, flow))
    }
    fn statement(&mut self, node: &ASTNode) -> Result<(Vec<Statement>, Flow), Refusal> {
        let id = self.node();
        let span = node.span();
        let mut flow = Flow {
            normal: true,
            ..Flow::default()
        };
        let kind = match node {
            ASTNode::Local {
                variables,
                initial_values,
                declared_type_names,
                ..
            } => {
                if variables.len() != initial_values.len() {
                    return Err(reject(
                        "MissingInitializer",
                        "all locals need an initializer",
                        span,
                    ));
                }
                let mut rows = Vec::new();
                for (index, (name, value)) in variables.iter().zip(initial_values).enumerate() {
                    let value = self.expression(
                        value
                            .as_ref()
                            .ok_or_else(|| reject("MissingInitializer", name, span))?,
                    )?;
                    if value.ty == Type::Unit {
                        return Err(reject("UnitValueUsed", name, span));
                    }
                    if let Some(Some(annotation)) = declared_type_names.get(index) {
                        require(value.ty, declared_type(Some(annotation), span)?, span)?;
                    }
                    let binding = self.bind(name, value.ty, false, span)?;
                    let row_id = if index == 0 { id } else { self.node() };
                    rows.push(Statement {
                        id: row_id,
                        span,
                        kind: StatementKind::Local { binding, value },
                    });
                }
                return Ok((rows, flow));
            }
            ASTNode::Assignment { target, value, .. }
            | ASTNode::CompoundAssignment { target, value, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return Err(reject(
                        "UnsupportedPlace",
                        "only local places are selected",
                        span,
                    ));
                };
                let binding = self.binding(name, span)?;
                if self.bindings[binding.0].readonly {
                    return Err(reject("ReadonlyBinding", name, span));
                }
                let value = self.expression(value)?;
                require(value.ty, self.bindings[binding.0].ty, span)?;
                if let ASTNode::CompoundAssignment { operator, .. } = node {
                    self.binary_type(
                        operator,
                        self.bindings[binding.0].ty,
                        value.ty,
                        &value,
                        span,
                    )?;
                    if !matches!(
                        operator,
                        Binary::Add | Binary::Subtract | Binary::Multiply | Binary::Divide
                    ) {
                        return Err(reject(
                            "UnsupportedCompoundOperator",
                            operator.to_string(),
                            span,
                        ));
                    }
                    StatementKind::Compound {
                        binding,
                        operator: operator.clone(),
                        value,
                    }
                } else {
                    StatementKind::Assign { binding, value }
                }
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                let carriers = self.visible();
                let condition = self.expression(condition)?;
                require(condition.ty, Type::Bool, span)?;
                let (yes, a) = self.block(then_body, true)?;
                let (no, b) = self.block(else_body.as_deref().unwrap_or(&[]), true)?;
                flow.normal = a.normal || b.normal;
                flow.breaks = a.breaks.union(&b.breaks).copied().collect();
                StatementKind::If {
                    condition,
                    yes,
                    no,
                    carriers,
                }
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                let carriers = self.visible();
                let condition = self.expression(condition)?;
                require(condition.ty, Type::Bool, span)?;
                let always = matches!(condition.kind, ExpressionKind::Bool(true));
                self.loops.push(id);
                let (body, next) = self.block(body, true)?;
                self.loops.pop();
                flow.normal = !always || next.breaks.contains(&id);
                flow.breaks = next.breaks.into_iter().filter(|old| *old != id).collect();
                StatementKind::Loop(LoopContract {
                    test: LoopTest::Condition {
                        expression: condition,
                        always,
                    },
                    body,
                    carriers,
                })
            }
            ASTNode::LoopRange {
                var_name,
                start,
                end,
                body,
                ..
            } => {
                let carriers = self.visible();
                let start = self.expression(start)?;
                let end = self.expression(end)?;
                require(start.ty, Type::I64, span)?;
                require(end.ty, Type::I64, span)?;
                self.scopes.push(BTreeMap::new());
                let index = self.bind(var_name, Type::I64, true, span)?;
                self.loops.push(id);
                let (body, next) = self.block(body, false)?;
                self.loops.pop();
                self.scopes.pop();
                flow.breaks = next.breaks.into_iter().filter(|old| *old != id).collect();
                StatementKind::Loop(LoopContract {
                    test: LoopTest::Range { index, start, end },
                    body,
                    carriers,
                })
            }
            ASTNode::Break { .. } | ASTNode::Continue { .. } => {
                let target = *self.loops.last().ok_or_else(|| {
                    reject("ControlOutsideLoop", "break/continue requires a loop", span)
                })?;
                flow.normal = false;
                if matches!(node, ASTNode::Break { .. }) {
                    flow.breaks.insert(target);
                    StatementKind::Break(target)
                } else {
                    StatementKind::Continue(target)
                }
            }
            ASTNode::Return { value, .. } => {
                let value = value
                    .as_ref()
                    .ok_or_else(|| reject("TypeMismatch", "typed return requires a value", span))?;
                let value = self.expression(value)?;
                require(value.ty, self.declaration.result, span)?;
                flow.normal = false;
                StatementKind::Return(value)
            }
            ASTNode::ScopeBox { body, .. } => {
                let (body, next) = self.block(body, true)?;
                flow = next;
                StatementKind::Scope(body)
            }
            other => StatementKind::Expression(self.expression(other)?),
        };
        Ok((vec![Statement { id, span, kind }], flow))
    }
    fn binary_type(
        &self,
        operator: &Binary,
        left: Type,
        right: Type,
        rhs: &Expression,
        span: Span,
    ) -> Result<Type, Refusal> {
        match operator {
            Binary::And | Binary::Or => {
                require(left, Type::Bool, span)?;
                require(right, Type::Bool, span)?;
                Ok(Type::Bool)
            }
            Binary::Equal | Binary::NotEqual => {
                if left == Type::Unit {
                    return Err(reject("UnitValueUsed", "comparison", span));
                }
                require(right, left, span)?;
                Ok(Type::Bool)
            }
            Binary::Less | Binary::LessEqual | Binary::Greater => {
                require(left, Type::I64, span)?;
                require(right, Type::I64, span)?;
                Ok(Type::Bool)
            }
            Binary::Add | Binary::Subtract | Binary::Multiply | Binary::Divide => {
                require(left, Type::I64, span)?;
                require(right, Type::I64, span)?;
                if matches!(operator, Binary::Divide)
                    && !matches!(rhs.kind,ExpressionKind::Integer(n) if n>0)
                {
                    return Err(reject(
                        "UnsupportedDivisionContract",
                        "current-I64 division requires an original positive literal divisor",
                        span,
                    ));
                }
                Ok(Type::I64)
            }
            _ => Err(reject("UnsupportedOperator", operator.to_string(), span)),
        }
    }
    fn expression(&mut self, node: &ASTNode) -> Result<Expression, Refusal> {
        let id = self.node();
        let span = node.span();
        let (ty, kind) = match node {
            ASTNode::Literal {
                value: LiteralValue::Integer(n),
                ..
            } => (Type::I64, ExpressionKind::Integer(*n)),
            ASTNode::Literal {
                value: LiteralValue::Bool(b),
                ..
            } => (Type::Bool, ExpressionKind::Bool(*b)),
            ASTNode::Variable { name, .. } => {
                let binding = self.binding(name, span)?;
                (self.bindings[binding.0].ty, ExpressionKind::Read(binding))
            }
            ASTNode::UnaryOp {
                operator, operand, ..
            } => {
                let operand = self.expression(operand)?;
                let ty = match operator {
                    Unary::Minus => Type::I64,
                    Unary::Not => Type::Bool,
                    _ => return Err(reject("UnsupportedOperator", operator.to_string(), span)),
                };
                require(operand.ty, ty, span)?;
                (
                    ty,
                    ExpressionKind::Unary {
                        operator: operator.clone(),
                        operand: Box::new(operand),
                    },
                )
            }
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                let left = self.expression(left)?;
                let right = self.expression(right)?;
                let ty = self.binary_type(operator, left.ty, right.ty, &right, span)?;
                (
                    ty,
                    ExpressionKind::Binary {
                        operator: operator.clone(),
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                )
            }
            ASTNode::FunctionCall {
                name, arguments, ..
            } => self.call(None, name, arguments, span)?,
            ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            } => {
                let owner = match object.as_ref() {
                    ASTNode::Me { .. } => self.declaration.owner.clone().ok_or_else(|| {
                        reject("UnknownReceiver", "me outside a static box", span)
                    })?,
                    ASTNode::Variable { name, .. }
                        if self.scopes.iter().all(|s| !s.contains_key(name)) =>
                    {
                        name.clone()
                    }
                    _ => {
                        return Err(reject(
                            "UnsupportedReceiver",
                            "only static declaration receivers are selected",
                            span,
                        ))
                    }
                };
                self.call(Some(&owner), method, arguments, span)?
            }
            ASTNode::Print { expression, .. } => self.call(
                None,
                "print",
                std::slice::from_ref(expression.as_ref()),
                span,
            )?,
            _ => {
                return Err(reject(
                    "UnsupportedExpression",
                    format!("source expression at {}:{}", span.line, span.column),
                    span,
                ))
            }
        };
        Ok(Expression { id, span, ty, kind })
    }
    fn call(
        &mut self,
        owner: Option<&str>,
        name: &str,
        arguments: &[ASTNode],
        span: Span,
    ) -> Result<(Type, ExpressionKind), Refusal> {
        let candidates: Vec<_> = self
            .table
            .entries
            .iter()
            .filter(|d| d.owner.as_deref() == owner && d.key.name() == name)
            .collect();
        let (target, result, parameters) = if owner.is_none() && name == "print" {
            (CallTarget::Print, Type::Unit, vec![Type::I64])
        } else {
            let declaration = candidates
                .iter()
                .find(|d| d.parameters.len() == arguments.len())
                .ok_or_else(|| {
                    reject(
                        if candidates.is_empty() {
                            "UnknownCall"
                        } else {
                            "ArityMismatch"
                        },
                        format!(
                            "{}{} / {}",
                            owner.map(|o| format!("{o}.")).unwrap_or_default(),
                            name,
                            arguments.len()
                        ),
                        span,
                    )
                })?;
            self.calls.insert(declaration.id);
            (
                CallTarget::Declaration(declaration.id),
                declaration.result,
                declaration.parameters.iter().map(|(_, t)| *t).collect(),
            )
        };
        if parameters.len() != arguments.len() {
            return Err(reject("ArityMismatch", name, span));
        }
        let mut values = Vec::new();
        for (argument, expected) in arguments.iter().zip(parameters) {
            let value = self.expression(argument)?;
            require(value.ty, expected, argument.span())?;
            values.push(value);
        }
        if target == CallTarget::Print {
            self.io = true;
        }
        Ok((
            result,
            ExpressionKind::Call(ResolvedCall {
                target,
                may_fault: false,
                receiver: None,
                arguments: values,
            }),
        ))
    }
}
