//! REPL AST Rewriter - Phase 288.1
//!
//! Transforms AST to bridge session variables via __repl.get/set
//!
//! Box-First Design: REPL-only logic, completely isolated from file mode
//!
//! Key responsibilities:
//! - Collect declared names (local, function params, loop vars)
//! - Rewrite undeclared Variable → __repl.get("name")
//! - Rewrite undeclared Assignment → __repl.set("name", value)
//! - Skip nested scopes (function/method bodies)
//! - Exclude reserved words (me, true, false, null, _)

use crate::ast::{ASTNode, Span};
use std::collections::HashSet;

/// Reserved names that should NOT be rewritten
/// Phase 288.1: "_" is NOT reserved - it should be rewritten like any other session variable
const RESERVED_NAMES: &[&str] = &["me", "true", "false", "null"];

/// REPL AST Rewriter
pub struct ReplAstRewriter {
    /// Names declared in the current input (local, params, loop vars)
    declared_names: HashSet<String>,
    /// Are we inside a nested scope (function/method body)?
    in_nested_scope: bool,
}

impl ReplAstRewriter {
    /// Create new rewriter
    fn new() -> Self {
        Self {
            declared_names: HashSet::new(),
            in_nested_scope: false,
        }
    }

    /// Main entry point: rewrite AST for REPL session variable bridge
    pub fn rewrite(ast: ASTNode) -> ASTNode {
        let mut rewriter = Self::new();
        rewriter.collect_declared_names(&ast);
        rewriter.rewrite_node(ast)
    }

    /// Phase 1: Collect declared names in current input
    fn collect_declared_names(&mut self, ast: &ASTNode) {
        match ast {
            ASTNode::Program { statements, .. } => {
                for stmt in statements {
                    self.collect_from_node(stmt);
                }
            }
            _ => self.collect_from_node(ast),
        }
    }

    fn collect_from_node(&mut self, node: &ASTNode) {
        match node {
            // Local declarations (variables: Vec<String>)
            ASTNode::Local { variables, .. } => {
                for var in variables {
                    self.declared_names.insert(var.clone());
                }
            }

            // Function declarations (params are declared)
            ASTNode::FunctionDeclaration { params, .. } => {
                for param in params {
                    self.declared_names.insert(param.clone());
                }
            }

            // Box declarations (methods: HashMap<String, ASTNode>)
            ASTNode::BoxDeclaration {
                methods,
                static_init,
                ..
            } => {
                for (_method_name, method_node) in methods {
                    self.collect_from_node(method_node);
                }
                if let Some(init) = static_init {
                    for stmt in init {
                        self.collect_from_node(stmt);
                    }
                }
            }

            // Recurse into compound nodes
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                for stmt in then_body {
                    self.collect_from_node(stmt);
                }
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        self.collect_from_node(stmt);
                    }
                }
            }

            ASTNode::Loop { body, .. } => {
                for stmt in body {
                    self.collect_from_node(stmt);
                }
            }

            _ => {} // Other nodes don't declare names
        }
    }

    /// Phase 2: Rewrite AST nodes
    fn rewrite_node(&mut self, node: ASTNode) -> ASTNode {
        match node {
            // Program: recurse into statements
            ASTNode::Program { statements, span } => {
                let rewritten = statements
                    .into_iter()
                    .map(|s| self.rewrite_node(s))
                    .collect();
                ASTNode::Program {
                    statements: rewritten,
                    span,
                }
            }

            // Variable read: rewrite if undeclared and not in nested scope
            ASTNode::Variable { name, span } => {
                if self.should_rewrite_variable(&name) {
                    self.make_repl_get(name, span)
                } else {
                    ASTNode::Variable { name, span }
                }
            }

            // Assignment: rewrite target if undeclared Variable
            ASTNode::Assignment {
                target,
                value,
                span,
            } => {
                let rewritten_value = Box::new(self.rewrite_node(*value));

                // Check if target is a simple Variable
                if let ASTNode::Variable {
                    name,
                    span: var_span,
                } = *target
                {
                    if self.should_rewrite_variable(&name) {
                        // Rewrite to __repl.set("name", value)
                        return self.make_repl_set(name, *rewritten_value, span);
                    } else {
                        // Keep as Assignment
                        return ASTNode::Assignment {
                            target: Box::new(ASTNode::Variable {
                                name,
                                span: var_span,
                            }),
                            value: rewritten_value,
                            span,
                        };
                    }
                } else {
                    // Complex assignment target (e.g., field access)
                    ASTNode::Assignment {
                        target: Box::new(self.rewrite_node(*target)),
                        value: rewritten_value,
                        span,
                    }
                }
            }

            // Function/Method declarations: enter nested scope
            // Phase 288.1: Special case - don't enter nested scope for "main" function (REPL wrapper)
            ASTNode::FunctionDeclaration {
                name,
                params,
                body,
                is_static,
                is_override,
                attrs,
                span,
            } => {
                let prev_nested = self.in_nested_scope;
                // Don't enter nested scope for "main" function (REPL wrapper)
                let is_repl_main = name == "main";
                if !is_repl_main {
                    self.in_nested_scope = true;
                }

                let rewritten_body = body.into_iter().map(|s| self.rewrite_node(s)).collect();

                self.in_nested_scope = prev_nested;

                ASTNode::FunctionDeclaration {
                    name,
                    params,
                    body: rewritten_body,
                    is_static,
                    is_override,
                    attrs,
                    span,
                }
            }

            // Box declarations: enter nested scope for methods (methods: HashMap)
            // Phase 288.1: Special case - don't enter nested scope for static box "Main" (REPL wrapper)
            ASTNode::BoxDeclaration {
                name,
                fields,
                field_decls,
                public_fields,
                private_fields,
                methods,
                constructors,
                init_fields,
                weak_fields,
                is_interface,
                extends,
                implements,
                type_parameters,
                is_static,
                static_init,
                attrs,
                span,
            } => {
                let prev_nested = self.in_nested_scope;
                // Don't enter nested scope for static box "Main" (REPL wrapper)
                let is_repl_wrapper = is_static && name == "Main";
                if !is_repl_wrapper {
                    self.in_nested_scope = true;
                }

                let rewritten_methods = methods
                    .into_iter()
                    .map(|(k, v)| (k, self.rewrite_node(v)))
                    .collect();
                let rewritten_constructors = constructors
                    .into_iter()
                    .map(|(k, v)| (k, self.rewrite_node(v)))
                    .collect();
                let rewritten_static_init = static_init
                    .map(|stmts| stmts.into_iter().map(|s| self.rewrite_node(s)).collect());

                self.in_nested_scope = prev_nested;

                ASTNode::BoxDeclaration {
                    name,
                    fields,
                    field_decls,
                    public_fields,
                    private_fields,
                    methods: rewritten_methods,
                    constructors: rewritten_constructors,
                    init_fields,
                    weak_fields,
                    is_interface,
                    extends,
                    implements,
                    type_parameters,
                    is_static,
                    static_init: rewritten_static_init,
                    attrs,
                    span,
                }
            }

            // If: recurse into branches
            ASTNode::If {
                condition,
                then_body,
                else_body,
                span,
            } => {
                let rewritten_cond = Box::new(self.rewrite_node(*condition));
                let rewritten_then = then_body
                    .into_iter()
                    .map(|s| self.rewrite_node(s))
                    .collect();
                let rewritten_else = else_body
                    .map(|stmts| stmts.into_iter().map(|s| self.rewrite_node(s)).collect());

                ASTNode::If {
                    condition: rewritten_cond,
                    then_body: rewritten_then,
                    else_body: rewritten_else,
                    span,
                }
            }

            // Loop: recurse into body
            ASTNode::Loop {
                condition,
                body,
                span,
            } => {
                let rewritten_cond = Box::new(self.rewrite_node(*condition));
                let rewritten_body = body.into_iter().map(|s| self.rewrite_node(s)).collect();

                ASTNode::Loop {
                    condition: rewritten_cond,
                    body: rewritten_body,
                    span,
                }
            }

            // Binary operation: recurse into operands
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                span,
            } => ASTNode::BinaryOp {
                operator,
                left: Box::new(self.rewrite_node(*left)),
                right: Box::new(self.rewrite_node(*right)),
                span,
            },

            // Unary operation: recurse into operand
            ASTNode::UnaryOp {
                operator,
                operand,
                span,
            } => ASTNode::UnaryOp {
                operator,
                operand: Box::new(self.rewrite_node(*operand)),
                span,
            },

            // Method call: recurse into object and arguments
            ASTNode::MethodCall {
                object,
                method,
                arguments,
                span,
            } => ASTNode::MethodCall {
                object: Box::new(self.rewrite_node(*object)),
                method,
                arguments: arguments
                    .into_iter()
                    .map(|a| self.rewrite_node(a))
                    .collect(),
                span,
            },

            // Function call: recurse into arguments
            ASTNode::FunctionCall {
                name,
                arguments,
                span,
            } => ASTNode::FunctionCall {
                name,
                arguments: arguments
                    .into_iter()
                    .map(|a| self.rewrite_node(a))
                    .collect(),
                span,
            },

            // Field access: recurse into object
            ASTNode::FieldAccess {
                object,
                field,
                span,
            } => ASTNode::FieldAccess {
                object: Box::new(self.rewrite_node(*object)),
                field,
                span,
            },

            // Return: recurse into value
            ASTNode::Return { value, span } => ASTNode::Return {
                value: value.map(|v| Box::new(self.rewrite_node(*v))),
                span,
            },

            // Print: recurse into expression
            ASTNode::Print { expression, span } => ASTNode::Print {
                expression: Box::new(self.rewrite_node(*expression)),
                span,
            },

            // New: recurse into arguments
            ASTNode::New {
                class,
                arguments,
                type_arguments,
                span,
            } => ASTNode::New {
                class,
                arguments: arguments
                    .into_iter()
                    .map(|a| self.rewrite_node(a))
                    .collect(),
                type_arguments,
                span,
            },

            // Match: recurse into scrutinee and arms
            ASTNode::MatchExpr {
                scrutinee,
                arms,
                else_expr,
                span,
            } => ASTNode::MatchExpr {
                scrutinee: Box::new(self.rewrite_node(*scrutinee)),
                arms: arms
                    .into_iter()
                    .map(|(pattern, body)| (pattern, self.rewrite_node(body)))
                    .collect(),
                else_expr: Box::new(self.rewrite_node(*else_expr)),
                span,
            },
            ASTNode::EnumMatchExpr {
                enum_name,
                scrutinee,
                arms,
                else_expr,
                span,
            } => ASTNode::EnumMatchExpr {
                enum_name,
                scrutinee: Box::new(self.rewrite_node(*scrutinee)),
                arms: arms
                    .into_iter()
                    .map(|arm| crate::ast::EnumMatchArm {
                        variant_name: arm.variant_name,
                        binding_name: arm.binding_name,
                        body: self.rewrite_node(arm.body),
                    })
                    .collect(),
                else_expr: else_expr.map(|expr| Box::new(self.rewrite_node(*expr))),
                span,
            },

            // All other nodes pass through unchanged
            other => other,
        }
    }

    /// Should this variable be rewritten?
    fn should_rewrite_variable(&self, name: &str) -> bool {
        !self.declared_names.contains(name)
            && !self.in_nested_scope
            && !RESERVED_NAMES.contains(&name)
    }

    /// Create __repl.get("name") call
    fn make_repl_get(&self, name: String, span: Span) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: "__repl".to_string(),
                span: span.clone(),
            }),
            method: "get".to_string(),
            arguments: vec![ASTNode::Literal {
                value: crate::ast::LiteralValue::String(name),
                span: span.clone(),
            }],
            span,
        }
    }

    /// Create __repl.set("name", value) call
    fn make_repl_set(&self, name: String, value: ASTNode, span: Span) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: "__repl".to_string(),
                span: span.clone(),
            }),
            method: "set".to_string(),
            arguments: vec![
                ASTNode::Literal {
                    value: crate::ast::LiteralValue::String(name),
                    span: span.clone(),
                },
                value,
            ],
            span,
        }
    }

    /// Check if wrapper AST represents a pure expression (for auto-display)
    pub fn is_pure_expression(ast: &ASTNode) -> bool {
        match ast {
            ASTNode::Program { statements, .. } => {
                // Look for static box Main { ... }
                for stmt in statements {
                    if let ASTNode::BoxDeclaration { name, methods, .. } = stmt {
                        if name == "Main" {
                            // Check if main() method has a single expression in its body
                            if let Some(main_fn) = methods.get("main") {
                                return Self::check_main_function_is_expression(main_fn);
                            }
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }

    fn check_main_function_is_expression(func: &ASTNode) -> bool {
        // Check if main() function body has exactly 1 expression node
        if let ASTNode::FunctionDeclaration { body, .. } = func {
            return body.len() == 1 && Self::is_expression_node(&body[0]);
        }
        false
    }

    fn is_expression_node(node: &ASTNode) -> bool {
        match node {
            // Expressions: display target
            ASTNode::Literal { .. }
            | ASTNode::Variable { .. }
            | ASTNode::BinaryOp { .. }
            | ASTNode::UnaryOp { .. }
            | ASTNode::FieldAccess { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::FunctionCall { .. }
            | ASTNode::New { .. }
            | ASTNode::MatchExpr { .. }
            | ASTNode::EnumMatchExpr { .. } => true,

            // Statements: don't display
            ASTNode::Assignment { .. }
            | ASTNode::Local { .. }
            | ASTNode::Return { .. }
            | ASTNode::Print { .. }
            | ASTNode::If { .. }
            | ASTNode::Loop { .. } => false,

            _ => false,
        }
    }
}
