//! StepTree fact extraction
//!
//! This file is the structure-only collector for StepTree facts.
//! It must stay separate from JoinIR lowering and from contract execution.

use crate::ast::ASTNode;
use crate::mir::control_tree::step_tree_facts::StepTreeFacts;

use super::types::{ExitKind, StepCapability, StepNode, StepStmtKind, StepTreeFeatures};

/// Extract raw facts from StepNode tree (Phase 120)
pub(super) fn extract_facts_from_tree(
    root: &StepNode,
    features: &StepTreeFeatures,
) -> StepTreeFacts {
    let mut facts = StepTreeFacts::new();

    // Required caps from features (structural only)
    if features.has_if {
        facts.add_capability(StepCapability::If);
    }
    if features.max_if_depth > 1 {
        facts.add_capability(StepCapability::NestedIf);
    }
    if features.has_loop {
        facts.add_capability(StepCapability::Loop);
    }
    if features.max_loop_depth > 1 {
        facts.add_capability(StepCapability::NestedLoop);
    }
    if features.has_return {
        facts.add_capability(StepCapability::Return);
    }
    if features.has_break {
        facts.add_capability(StepCapability::Break);
    }
    if features.has_continue {
        facts.add_capability(StepCapability::Continue);
    }

    walk_for_facts(root, &mut facts);
    facts
}

/// Walk StepNode tree to collect facts (Phase 120, Phase 124)
fn walk_for_facts(node: &StepNode, facts: &mut StepTreeFacts) {
    match node {
        StepNode::Block(nodes) => {
            for n in nodes {
                walk_for_facts(n, facts);
            }
        }
        StepNode::If {
            cond,
            cond_ast,
            then_branch,
            else_branch,
            ..
        } => {
            facts.add_cond_sig(cond.to_compact_string());
            // Phase 124: Extract reads from condition AST
            extract_variables_from_ast(&cond_ast.0, facts);
            walk_for_facts(then_branch, facts);
            if let Some(else_branch) = else_branch {
                walk_for_facts(else_branch, facts);
            }
        }
        StepNode::Loop {
            cond,
            cond_ast,
            body,
            ..
        } => {
            facts.add_cond_sig(cond.to_compact_string());
            // Phase 124: Extract reads from condition AST
            extract_variables_from_ast(&cond_ast.0, facts);
            walk_for_facts(body, facts);
        }
        StepNode::Stmt { kind, .. } => match kind {
            StepStmtKind::LocalDecl { vars } => {
                for v in vars {
                    facts.add_write(v.clone());
                }
            }
            StepStmtKind::Assign { target, value_ast } => {
                if let Some(name) = target.as_ref() {
                    facts.add_write(name.clone());
                }
                // Phase 128: Extract reads from assignment value AST
                if let Some(ast) = value_ast {
                    extract_variables_from_ast(&ast.0, facts);
                }
            }
            StepStmtKind::Print => {}
            StepStmtKind::Return { value_ast } => {
                facts.add_exit(ExitKind::Return);
                // Phase 124: Extract reads from return value AST
                if let Some(ast) = value_ast {
                    extract_variables_from_ast(&ast.0, facts);
                }
            }
            StepStmtKind::Break => {
                facts.add_exit(ExitKind::Break);
            }
            StepStmtKind::Continue => {
                facts.add_exit(ExitKind::Continue);
            }
            StepStmtKind::Other(name) => match *name {
                "TryCatch" => {
                    facts.add_capability(StepCapability::TryCatch);
                }
                "Throw" => {
                    facts.add_capability(StepCapability::Throw);
                }
                "Lambda" => {
                    facts.add_capability(StepCapability::Lambda);
                }
                "While" => {
                    facts.add_capability(StepCapability::While);
                }
                "ForRange" => {
                    facts.add_capability(StepCapability::ForRange);
                }
                "MatchExpr" => {
                    facts.add_capability(StepCapability::Match);
                }
                "Arrow" => {
                    facts.add_capability(StepCapability::Arrow);
                }
                _ => {}
            },
        },
    }
}

/// Extract Variable names from AST (Phase 124: reads collection)
///
/// SSOT for reads extraction:
/// - Recursively walk AST tree
/// - Add Variable { name } to facts.reads
/// - Ignore other node types
fn extract_variables_from_ast(ast: &ASTNode, facts: &mut StepTreeFacts) {
    match ast {
        ASTNode::Variable { name, .. } => {
            facts.add_read(name.clone());
        }
        // Recursively walk binary/unary operations
        ASTNode::BinaryOp { left, right, .. } => {
            extract_variables_from_ast(left, facts);
            extract_variables_from_ast(right, facts);
        }
        ASTNode::UnaryOp { operand, .. } => {
            extract_variables_from_ast(operand, facts);
        }
        // Function calls
        ASTNode::FunctionCall { arguments, .. } => {
            for arg in arguments {
                extract_variables_from_ast(arg, facts);
            }
        }
        // Method calls
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            extract_variables_from_ast(object, facts);
            for arg in arguments {
                extract_variables_from_ast(arg, facts);
            }
        }
        // Field access
        ASTNode::FieldAccess { object, .. } => {
            extract_variables_from_ast(object, facts);
        }
        // Array/Index access
        ASTNode::Index { target, index, .. } => {
            extract_variables_from_ast(target, facts);
            extract_variables_from_ast(index, facts);
        }
        // Assignment (RHS only)
        ASTNode::Assignment { value, .. } => {
            extract_variables_from_ast(value, facts);
        }
        // Print
        ASTNode::Print { expression, .. } => {
            extract_variables_from_ast(expression, facts);
        }
        // Ignore literals, keywords, and other non-variable nodes
        _ => {}
    }
}
