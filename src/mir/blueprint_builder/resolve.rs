//! Source rejection, explicit signatures, types and evaluation order live here.
use super::declaration_table::*;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1 as Key;

fn reject(code: &'static str, detail: impl Into<String>, span: Span) -> Refusal {
    Refusal {
        code,
        detail: detail.into(),
        span,
    }
}

fn declared_type(name: Option<&str>, span: Span) -> Result<Type, Refusal> {
    match name {
        Some("i64") => Ok(Type::I64),
        Some("Bool") => Ok(Type::Bool),
        None => Err(reject(
            "MissingTypeAnnotation",
            "an explicit i64/Bool signature is required",
            span,
        )),
        Some(name) => Err(reject("UnsupportedType", name, span)),
    }
}

pub(super) fn module(ast: ASTNode) -> Result<ResolvedModule, Refusal> {
    let ASTNode::Program { statements, span } = ast else {
        return Err(reject(
            "ExpectedModule",
            "expected a parsed module",
            ast.span(),
        ));
    };
    let mut declarations = DeclarationTable {
        entries: Vec::new(),
    };
    let mut bodies = Vec::new();
    for statement in statements {
        match statement {
            ASTNode::BoxDeclaration {
                name,
                is_static: true,
                methods,
                fields,
                field_decls,
                constructors,
                static_init,
                span,
                ..
            } if fields.is_empty()
                && field_decls.is_empty()
                && constructors.is_empty()
                && static_init.is_none() =>
            {
                for method in methods.iter_selected_declaration_order() {
                    declare(
                        &mut declarations,
                        &mut bodies,
                        Some(&name),
                        method.declaration().clone(),
                    )?;
                }
                let _ = span;
            }
            declaration @ ASTNode::FunctionDeclaration { .. } => {
                declare(&mut declarations, &mut bodies, None, declaration)?
            }
            other => {
                return Err(reject(
                    "UnsupportedDeclaration",
                    "only scalar static methods and free functions are selected",
                    other.span(),
                ))
            }
        }
    }
    if declarations.entries.is_empty() {
        return Err(reject("MissingEntry", "module has no functions", span));
    }
    let mut functions = Vec::new();
    for (index, body) in bodies.into_iter().enumerate() {
        let mut resolver = FunctionResolver {
            next_node: 0,
            declaration: &declarations.entries[index],
        };
        let body = body
            .iter()
            .map(|node| resolver.statement(node))
            .collect::<Result<Vec<_>, _>>()?;
        if !matches!(body.last().map(|s| &s.kind), Some(StatementKind::Return(_))) {
            return Err(reject(
                "MissingReturn",
                "typed function must return a value",
                declarations.entries[index].span,
            ));
        }
        functions.push(ResolvedFunction {
            declaration: DeclarationId(index),
            body,
        });
    }
    Ok(ResolvedModule {
        declarations,
        functions,
    })
}

fn declare(
    table: &mut DeclarationTable,
    bodies: &mut Vec<Vec<ASTNode>>,
    owner: Option<&str>,
    node: ASTNode,
) -> Result<(), Refusal> {
    let ASTNode::FunctionDeclaration {
        name,
        params,
        param_decls,
        return_type_name,
        body,
        uses,
        contracts,
        span,
        ..
    } = node
    else {
        return Err(reject(
            "UnsupportedDeclaration",
            "expected a method",
            node.span(),
        ));
    };
    if !uses.is_empty() || !contracts.is_empty() {
        return Err(reject(
            "UnsupportedContract",
            "function uses/contracts are not selected",
            span,
        ));
    }
    if params.len() != param_decls.len() {
        return Err(reject(
            "MissingTypeAnnotation",
            "every formal requires its declaration",
            span,
        ));
    }
    let mut parameters = Vec::new();
    for (name, param) in params.iter().zip(&param_decls) {
        if name != &param.name || parameters.iter().any(|(old, _)| old == name) {
            return Err(reject("DuplicateBinding", name, span));
        }
        parameters.push((
            name.clone(),
            declared_type(param.declared_type_name.as_deref(), span)?,
        ));
    }
    let result = declared_type(return_type_name.as_deref(), span)?;
    let key = match owner {
        Some(owner) => Key::static_box_method(owner, &name, params.len() as u32),
        None => Key::free_function(&name, params.len() as u32),
    };
    if table.entries.iter().any(|old| old.key == key) {
        return Err(reject("DuplicateDeclaration", name, span));
    }
    table.entries.push(Declaration {
        id: DeclarationId(table.entries.len()),
        key,
        owner: owner.map(str::to_owned),
        parameters,
        result,
        span,
    });
    bodies.push(body);
    Ok(())
}

struct FunctionResolver<'a> {
    next_node: usize,
    declaration: &'a Declaration,
}
impl FunctionResolver<'_> {
    fn node(&mut self) -> NodeId {
        let id = NodeId(self.next_node);
        self.next_node += 1;
        id
    }
    fn statement(&mut self, node: &ASTNode) -> Result<Statement, Refusal> {
        let id = self.node();
        let kind = match node {
            ASTNode::Return {
                value: Some(value), ..
            } => {
                let value = self.expression(value)?;
                if value.ty != self.declaration.result {
                    return Err(reject(
                        "TypeMismatch",
                        "return type differs from signature",
                        node.span(),
                    ));
                }
                StatementKind::Return(value)
            }
            _ => {
                return Err(reject(
                    "UnsupportedStatement",
                    "statement not implemented in the vertical slice",
                    node.span(),
                ))
            }
        };
        Ok(Statement {
            id,
            span: node.span(),
            kind,
        })
    }
    fn expression(&mut self, node: &ASTNode) -> Result<Expression, Refusal> {
        let id = self.node();
        let (ty, kind) = match node {
            ASTNode::Literal {
                value: LiteralValue::Integer(value),
                ..
            } => (Type::I64, ExpressionKind::Integer(*value)),
            ASTNode::Literal {
                value: LiteralValue::Bool(value),
                ..
            } => (Type::Bool, ExpressionKind::Bool(*value)),
            ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left,
                right,
                ..
            } => {
                let left = self.expression(left)?;
                let right = self.expression(right)?;
                if left.ty != Type::I64 || right.ty != Type::I64 {
                    return Err(reject(
                        "TypeMismatch",
                        "i64 arithmetic requires two i64 operands",
                        node.span(),
                    ));
                }
                (
                    Type::I64,
                    ExpressionKind::Binary {
                        op: Binary::Add,
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                )
            }
            _ => {
                return Err(reject(
                    "UnsupportedExpression",
                    "expression not implemented in the vertical slice",
                    node.span(),
                ))
            }
        };
        Ok(Expression {
            id,
            span: node.span(),
            ty,
            kind,
        })
    }
}
