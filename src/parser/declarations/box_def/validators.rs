//! Validators and light analysis for box members
use crate::ast::ASTNode;
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::{HashMap, HashSet};

/// Forbid user-defined methods named exactly as the box (constructor-like names).
/// Nyash constructors are explicit: init/pack/birth. A method with the same name
/// as the box is likely a mistaken constructor attempt; reject for clarity.
pub(crate) fn validate_no_ctor_like_name(
    p: &mut NyashParser,
    box_name: &str,
    methods: &HashMap<String, ASTNode>,
) -> Result<(), ParseError> {
    if methods.contains_key(box_name) {
        let line = p.current_token().line;
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: format!(
                "method name must not match box name '{}'; use init/pack/birth for constructors",
                box_name
            ),
            line,
        });
    }
    Ok(())
}

/// Validate that birth_once properties do not have cyclic dependencies via me.<prop> references
pub(crate) fn validate_birth_once_cycles(
    p: &mut NyashParser,
    methods: &HashMap<String, ASTNode>,
) -> Result<(), ParseError> {
    if !crate::config::env::unified_members() {
        return Ok(());
    }
    // Collect birth_once compute bodies
    let mut birth_bodies: HashMap<String, Vec<ASTNode>> = HashMap::new();
    for (mname, mast) in methods {
        if let Some(prop) = mname.strip_prefix("__compute_birth_") {
            if let ASTNode::FunctionDeclaration { body, .. } = mast {
                birth_bodies.insert(prop.to_string(), body.clone());
            }
        }
    }
    if birth_bodies.is_empty() {
        return Ok(());
    }
    // Build dependency graph: A -> {B | me.B used inside A}
    let mut deps: HashMap<String, HashSet<String>> = HashMap::new();
    let props: HashSet<String> = birth_bodies.keys().cloned().collect();
    for (pname, body) in &birth_bodies {
        let used = ast_collect_me_fields(body);
        let mut set = HashSet::new();
        for u in used {
            if props.contains(&u) && u != *pname {
                set.insert(u);
            }
        }
        deps.insert(pname.clone(), set);
    }
    // Detect cycle via DFS
    fn has_cycle(
        node: &str,
        deps: &HashMap<String, HashSet<String>>,
        temp: &mut HashSet<String>,
        perm: &mut HashSet<String>,
    ) -> bool {
        if perm.contains(node) {
            return false;
        }
        if !temp.insert(node.to_string()) {
            return true;
        } // back-edge
        if let Some(ns) = deps.get(node) {
            for n in ns {
                if has_cycle(n, deps, temp, perm) {
                    return true;
                }
            }
        }
        temp.remove(node);
        perm.insert(node.to_string());
        false
    }
    let mut perm = HashSet::new();
    let mut temp = HashSet::new();
    for pname in deps.keys() {
        if has_cycle(pname, &deps, &mut temp, &mut perm) {
            let line = p.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "birth_once declarations must not have cyclic dependencies".to_string(),
                line,
            });
        }
    }
    Ok(())
}

/// Forbid constructor call with the same name as the box; enforce `birth()` usage.
pub(crate) fn forbid_box_named_constructor(
    p: &mut NyashParser,
    box_name: &str,
) -> Result<(), ParseError> {
    if let TokenType::IDENTIFIER(id) = &p.current_token().token_type {
        if id == box_name && p.peek_token() == &TokenType::LPAREN {
            return Err(ParseError::UnexpectedToken {
                expected: format!(
                    "birth() constructor instead of {}(). Nyash uses birth() for unified constructor syntax.",
                    box_name
                ),
                found: TokenType::IDENTIFIER(box_name.to_string()),
                line: p.current_token().line,
            });
        }
    }
    Ok(())
}

/// Collect all `me.<field>` accessed in nodes (flat set)
fn ast_collect_me_fields(nodes: &[ASTNode]) -> std::collections::HashSet<String> {
    use std::collections::HashSet;

    fn scan_optional_node(node: &Option<Box<ASTNode>>, out: &mut HashSet<String>) {
        if let Some(node) = node {
            scan_node(node, out);
        }
    }

    fn scan_optional_body(body: &Option<Vec<ASTNode>>, out: &mut HashSet<String>) {
        if let Some(body) = body {
            scan_body(body, out);
        }
    }

    fn scan_body(nodes: &[ASTNode], out: &mut HashSet<String>) {
        for n in nodes {
            scan_node(n, out);
        }
    }

    fn scan_node(node: &ASTNode, out: &mut HashSet<String>) {
        match node {
            ASTNode::FieldAccess { object, field, .. } => {
                if matches!(object.as_ref(), ASTNode::Me { .. }) {
                    out.insert(field.clone());
                } else {
                    scan_node(object, out);
                }
            }
            ASTNode::MeField { field, .. } => {
                out.insert(field.clone());
            }
            ASTNode::Program { statements, .. }
            | ASTNode::ScopeBox {
                body: statements, ..
            } => scan_body(statements, out),
            ASTNode::Assignment { target, value, .. } => {
                scan_node(target, out);
                scan_node(value, out);
            }
            ASTNode::Print { expression, .. }
            | ASTNode::Nowait { expression, .. }
            | ASTNode::AwaitExpression { expression, .. }
            | ASTNode::QMarkPropagate { expression, .. }
            | ASTNode::Throw { expression, .. } => scan_node(expression, out),
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                scan_node(condition, out);
                scan_body(then_body, out);
                scan_optional_body(else_body, out);
            }
            ASTNode::Loop {
                condition, body, ..
            }
            | ASTNode::While {
                condition, body, ..
            } => {
                scan_node(condition, out);
                scan_body(body, out);
            }
            ASTNode::ForRange {
                start, end, body, ..
            } => {
                scan_node(start, out);
                scan_node(end, out);
                scan_body(body, out);
            }
            ASTNode::Return { value, .. } => scan_optional_node(value, out),
            ASTNode::GlobalVar { value, .. } => scan_node(value, out),
            ASTNode::UnaryOp { operand, .. } => scan_node(operand, out),
            ASTNode::BinaryOp { left, right, .. } => {
                scan_node(left, out);
                scan_node(right, out);
            }
            ASTNode::GroupedAssignmentExpr { rhs, .. } => scan_node(rhs, out),
            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                scan_node(object, out);
                for arg in arguments {
                    scan_node(arg, out);
                }
            }
            ASTNode::Index { target, index, .. } => {
                scan_node(target, out);
                scan_node(index, out);
            }
            ASTNode::New { arguments, .. }
            | ASTNode::FromCall { arguments, .. }
            | ASTNode::FunctionCall { arguments, .. } => {
                for arg in arguments {
                    scan_node(arg, out);
                }
            }
            ASTNode::Call {
                callee, arguments, ..
            } => {
                scan_node(callee, out);
                for arg in arguments {
                    scan_node(arg, out);
                }
            }
            ASTNode::MatchExpr {
                scrutinee,
                arms,
                else_expr,
                ..
            } => {
                scan_node(scrutinee, out);
                for (_, arm_expr) in arms {
                    scan_node(arm_expr, out);
                }
                scan_node(else_expr, out);
            }
            ASTNode::EnumMatchExpr {
                scrutinee,
                arms,
                else_expr,
                ..
            } => {
                scan_node(scrutinee, out);
                for arm in arms {
                    scan_node(&arm.body, out);
                }
                scan_optional_node(else_expr, out);
            }
            ASTNode::ArrayLiteral { elements, .. } => {
                for element in elements {
                    scan_node(element, out);
                }
            }
            ASTNode::MapLiteral { entries, .. } => {
                for (_, value) in entries {
                    scan_node(value, out);
                }
            }
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } => {
                scan_body(prelude_stmts, out);
                scan_node(tail_expr, out);
            }
            ASTNode::Arrow {
                sender, receiver, ..
            } => {
                scan_node(sender, out);
                scan_node(receiver, out);
            }
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                ..
            } => {
                scan_body(try_body, out);
                for clause in catch_clauses {
                    scan_body(&clause.body, out);
                }
                scan_optional_body(finally_body, out);
            }
            ASTNode::Local { initial_values, .. } | ASTNode::Outbox { initial_values, .. } => {
                for value in initial_values {
                    scan_optional_node(value, out);
                }
            }
            ASTNode::Literal { .. }
            | ASTNode::Variable { .. }
            | ASTNode::This { .. }
            | ASTNode::Me { .. }
            | ASTNode::ThisField { .. }
            | ASTNode::Break { .. }
            | ASTNode::Continue { .. }
            | ASTNode::UsingStatement { .. }
            | ASTNode::ImportStatement { .. }
            | ASTNode::FunctionDeclaration { .. }
            | ASTNode::EnumDeclaration { .. }
            | ASTNode::BoxDeclaration { .. }
            | ASTNode::Lambda { .. } => {}
        }
    }

    let mut hs = HashSet::new();
    scan_body(nodes, &mut hs);
    hs
}
