use crate::ast::ASTNode;
use crate::mir::control_tree::step_tree_contract_box::StepTreeContractBox;

use super::fact_extractor::extract_facts_from_tree;
use super::signature::collect_node_kinds;
use super::summary::{ast_kind_name, summarize_ast};
use super::types::{AstNodeHandle, StepNode, StepStmtKind, StepTree, StepTreeFeatures, StepTreeSignature};

pub struct StepTreeBuilderBox;

impl StepTreeBuilderBox {
    pub fn build_from_ast(ast: &ASTNode) -> StepTree {
        match ast {
            ASTNode::Program { statements, .. } => Self::build_from_block(statements),
            ASTNode::ScopeBox { body, .. } => Self::build_from_block(body),
            _ => {
                let (node, features) = Self::build_node(ast, 0, 0);
                build_step_tree(node, features)
            }
        }
    }

    pub fn build_from_block(stmts: &[ASTNode]) -> StepTree {
        let mut nodes = Vec::with_capacity(stmts.len());
        let mut features = StepTreeFeatures::default();
        for stmt in stmts {
            let (node, node_features) = Self::build_node(stmt, 0, 0);
            nodes.push(node);
            features = merge_features(features, node_features);
        }
        build_step_tree(StepNode::Block(nodes), features)
    }

    fn build_node(ast: &ASTNode, if_depth: u32, loop_depth: u32) -> (StepNode, StepTreeFeatures) {
        match ast {
            ASTNode::Program { statements, span } => {
                let (node, features) = Self::build_block_node(statements, if_depth, loop_depth);
                (node.with_span(span.clone()), features)
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                span,
            } => {
                let cond = summarize_ast(condition);
                let cond_ast = AstNodeHandle(condition.clone());
                let (then_node, then_features) =
                    Self::build_block_node(then_body, if_depth + 1, loop_depth);
                let (else_node, else_features) = match else_body {
                    Some(else_body) => {
                        let (node, f) =
                            Self::build_block_node(else_body, if_depth + 1, loop_depth);
                        (Some(Box::new(node)), f)
                    }
                    None => (None, StepTreeFeatures::default()),
                };
                let mut features = StepTreeFeatures {
                    has_if: true,
                    max_if_depth: (if_depth + 1).max(then_features.max_if_depth),
                    ..StepTreeFeatures::default()
                };
                features = merge_features(features, then_features);
                features = merge_features(features, else_features);

                (
                    StepNode::If {
                        cond,
                        cond_ast,
                        then_branch: Box::new(then_node),
                        else_branch: else_node,
                        span: span.clone(),
                    },
                    features,
                )
            }
            ASTNode::Loop {
                condition, body, span, ..
            } => {
                let cond = summarize_ast(condition);
                let cond_ast = AstNodeHandle(condition.clone());
                let (body_node, body_features) =
                    Self::build_block_node(body, if_depth, loop_depth + 1);
                let mut features = StepTreeFeatures {
                    has_loop: true,
                    max_loop_depth: (loop_depth + 1).max(body_features.max_loop_depth),
                    ..StepTreeFeatures::default()
                };
                features = merge_features(features, body_features);
                (
                    StepNode::Loop {
                        cond,
                        cond_ast,
                        body: Box::new(body_node),
                        span: span.clone(),
                    },
                    features,
                )
            }
            ASTNode::ScopeBox { body, span } => {
                let (node, features) = Self::build_block_node(body, if_depth, loop_depth);
                (node.with_span(span.clone()), features)
            }
            ASTNode::Return { value, span } => (
                StepNode::Stmt {
                    kind: StepStmtKind::Return {
                        value_ast: value.as_ref().map(|v| AstNodeHandle(v.clone())),
                    },
                    span: span.clone(),
                },
                StepTreeFeatures {
                    has_return: true,
                    ..StepTreeFeatures::default()
                },
            ),
            ASTNode::Break { span } => (
                StepNode::Stmt {
                    kind: StepStmtKind::Break,
                    span: span.clone(),
                },
                StepTreeFeatures {
                    has_break: true,
                    ..StepTreeFeatures::default()
                },
            ),
            ASTNode::Continue { span } => (
                StepNode::Stmt {
                    kind: StepStmtKind::Continue,
                    span: span.clone(),
                },
                StepTreeFeatures {
                    has_continue: true,
                    ..StepTreeFeatures::default()
                },
            ),
            ASTNode::Local { variables, span, .. } => (
                StepNode::Stmt {
                    kind: StepStmtKind::LocalDecl {
                        vars: variables.clone(),
                    },
                    span: span.clone(),
                },
                StepTreeFeatures::default(),
            ),
            ASTNode::Assignment { span, value, .. } => (
                StepNode::Stmt {
                    kind: StepStmtKind::Assign {
                        target: match ast {
                            ASTNode::Assignment { target, .. } => match target.as_ref() {
                                ASTNode::Variable { name, .. } => Some(name.clone()),
                                _ => None,
                            },
                            _ => None,
                        },
                        // Phase 128: Store value AST for Normalized lowering
                        value_ast: Some(AstNodeHandle(value.clone())),
                    },
                    span: span.clone(),
                },
                StepTreeFeatures::default(),
            ),
            ASTNode::Print { span, .. } => (
                StepNode::Stmt {
                    kind: StepStmtKind::Print,
                    span: span.clone(),
                },
                StepTreeFeatures::default(),
            ),
            other => (
                StepNode::Stmt {
                    kind: StepStmtKind::Other(ast_kind_name(other)),
                    span: other.span(),
                },
                StepTreeFeatures::default(),
            ),
        }
    }

    fn build_block_node(
        stmts: &[ASTNode],
        if_depth: u32,
        loop_depth: u32,
    ) -> (StepNode, StepTreeFeatures) {
        let mut nodes = Vec::with_capacity(stmts.len());
        let mut features = StepTreeFeatures::default();
        for stmt in stmts {
            let (node, node_features) = Self::build_node(stmt, if_depth, loop_depth);
            nodes.push(node);
            features = merge_features(features, node_features);
        }
        (StepNode::Block(nodes), features)
    }
}

fn build_step_tree(root: StepNode, features: StepTreeFeatures) -> StepTree {
    // Phase 120: Facts → Contract → Signature (separated concerns)
    let facts = extract_facts_from_tree(&root, &features);
    let contract = StepTreeContractBox::from_facts(&facts);
    let mut kinds = Vec::new();
    collect_node_kinds(&root, &mut kinds);
    let kinds = kinds.join(",");
    let basis = contract.signature_basis_string(&kinds);
    let signature = StepTreeSignature::from_basis_string(&basis);

    StepTree {
        root,
        features,
        contract,
        signature,
    }
}

fn merge_features(mut a: StepTreeFeatures, b: StepTreeFeatures) -> StepTreeFeatures {
    a.has_if |= b.has_if;
    a.has_loop |= b.has_loop;
    a.has_break |= b.has_break;
    a.has_continue |= b.has_continue;
    a.has_return |= b.has_return;
    a.max_if_depth = a.max_if_depth.max(b.max_if_depth);
    a.max_loop_depth = a.max_loop_depth.max(b.max_loop_depth);
    a
}
