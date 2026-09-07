//! Pure default-derive selection and syntax construction.
//!
//! The caller owns configuration and invocation order. This owner neither reads
//! environment/registries nor issues source identity, parameter contracts or ABI.
//! Generated callable source transactions must attach their own exact authority.

use crate::ast::{ASTNode, BinaryOperator, BoxMethodInventoryV1, LiteralValue, Span};

#[derive(Clone, Copy)]
pub(crate) struct DefaultDeriveSelectionV1 {
    pub(crate) equals: bool,
    pub(crate) to_string: bool,
}

pub(crate) fn select(
    is_static: bool,
    methods: &BoxMethodInventoryV1,
    derive_all: bool,
    derive_set: &str,
) -> DefaultDeriveSelectionV1 {
    let receiver_based = !is_static;
    DefaultDeriveSelectionV1 {
        equals: receiver_based
            && (derive_all || derive_set.contains("Equals"))
            && methods.get_declaration("equals").is_none(),
        to_string: receiver_based
            && (derive_all || derive_set.contains("ToString"))
            && methods.get_declaration("toString").is_none(),
    }
}

fn me_field(name: &str) -> ASTNode {
    ASTNode::FieldAccess {
        object: Box::new(ASTNode::Me {
            span: Span::unknown(),
        }),
        field: name.to_string(),
        span: Span::unknown(),
    }
}

fn var_field(var: &str, field: &str) -> ASTNode {
    ASTNode::FieldAccess {
        object: Box::new(ASTNode::Variable {
            name: var.to_string(),
            span: Span::unknown(),
        }),
        field: field.to_string(),
        span: Span::unknown(),
    }
}

fn bin_add(lhs: ASTNode, rhs: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(lhs),
        right: Box::new(rhs),
        span: Span::unknown(),
    }
}

fn bin_and(lhs: ASTNode, rhs: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::And,
        left: Box::new(lhs),
        right: Box::new(rhs),
        span: Span::unknown(),
    }
}

fn bin_eq(lhs: ASTNode, rhs: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(lhs),
        right: Box::new(rhs),
        span: Span::unknown(),
    }
}

fn lit_str(s: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(s.to_string()),
        span: Span::unknown(),
    }
}

pub(crate) fn build_equals_method(_box_name: &str, fields: &Vec<String>) -> ASTNode {
    // equals(other) { return me.f1 == other.f1 && ...; }
    let cond = if fields.is_empty() {
        ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        }
    } else {
        let mut it = fields.iter();
        let first = it.next().unwrap();
        let mut expr = bin_eq(me_field(first), var_field("__ny_other", first));
        for f in it {
            expr = bin_and(expr, bin_eq(me_field(f), var_field("__ny_other", f)));
        }
        expr
    };
    // Hygiene: use gensym-like param to avoid collisions
    let param_name = "__ny_other".to_string();
    ASTNode::FunctionDeclaration {
        name: "equals".to_string(),
        params: vec![param_name.clone()],
        param_decls: vec![crate::ast::ParamDecl {
            name: param_name.clone(),
            declared_type_name: None,
        }],
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: Some(Box::new(cond)),
            span: Span::unknown(),
        }],
        is_static: false,
        is_override: false,
        attrs: crate::ast::DeclarationAttrs::default(),
        uses: vec![],
        contracts: vec![],
        span: Span::unknown(),
    }
}

pub(crate) fn build_tostring_method(box_name: &str, fields: &Vec<String>) -> ASTNode {
    // toString() { return "Name(" + me.f1 + "," + me.f2 + ")" }
    let mut expr = lit_str(&format!("{}(", box_name));
    let mut first = true;
    for f in fields {
        if !first {
            expr = bin_add(expr, lit_str(","));
        }
        first = false;
        expr = bin_add(expr, me_field(f));
    }
    expr = bin_add(expr, lit_str(")"));
    ASTNode::FunctionDeclaration {
        name: "toString".to_string(),
        params: vec![],
        param_decls: vec![],
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: Some(Box::new(expr)),
            span: Span::unknown(),
        }],
        is_static: false,
        is_override: false,
        attrs: crate::ast::DeclarationAttrs::default(),
        uses: vec![],
        contracts: vec![],
        span: Span::unknown(),
    }
}
