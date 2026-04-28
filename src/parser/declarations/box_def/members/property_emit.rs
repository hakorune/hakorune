//! Synthetic method emission for unified member properties.
//!
//! Parsers decide which property shape was written. This module owns the AST
//! method bodies used to represent computed/once/birth_once properties.

use crate::ast::{ASTNode, Span};
use std::collections::HashMap;

fn function_decl(name: String, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name,
        params: vec![],
        body,
        is_static: false,
        is_override: false,
        attrs: crate::ast::DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn me() -> ASTNode {
    ASTNode::Me {
        span: Span::unknown(),
    }
}

fn string_lit(value: String) -> ASTNode {
    ASTNode::Literal {
        value: crate::ast::LiteralValue::String(value),
        span: Span::unknown(),
    }
}

fn null_lit() -> ASTNode {
    ASTNode::Literal {
        value: crate::ast::LiteralValue::Null,
        span: Span::unknown(),
    }
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn me_call(method: String, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(me()),
        method,
        arguments,
        span: Span::unknown(),
    }
}

fn birth_once_compute_method_name(name: &str) -> String {
    format!("__compute_birth_{}", name)
}

fn birth_once_getter_method_name(name: &str) -> String {
    format!("__get_birth_{}", name)
}

fn birth_once_storage_key(name: &str) -> String {
    format!("__birth_{}", name)
}

fn local_with_init(name: &str, init: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![Some(Box::new(init))],
        span: Span::unknown(),
    }
}

fn return_expr(expr: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(expr)),
        span: Span::unknown(),
    }
}

fn not_null(local_name: &str) -> ASTNode {
    ASTNode::BinaryOp {
        operator: crate::ast::BinaryOperator::NotEqual,
        left: Box::new(var(local_name)),
        right: Box::new(null_lit()),
        span: Span::unknown(),
    }
}

fn if_then(condition: ASTNode, then_body: Vec<ASTNode>) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body: None,
        span: Span::unknown(),
    }
}

pub(crate) fn insert_computed_getter(
    methods: &mut HashMap<String, ASTNode>,
    name: String,
    body: Vec<ASTNode>,
) {
    let getter_name = format!("__get_{}", name);
    methods.insert(getter_name.clone(), function_decl(getter_name, body));
}

pub(crate) fn insert_once_methods(
    methods: &mut HashMap<String, ASTNode>,
    name: String,
    compute_body: Vec<ASTNode>,
) {
    let compute_name = format!("__compute_once_{}", name);
    methods.insert(
        compute_name.clone(),
        function_decl(compute_name.clone(), compute_body),
    );

    let key = format!("__once_{}", name);
    let poison_key = format!("__once_poison_{}", name);
    let cached_local = format!("__ny_cached_{}", name);
    let poison_local = format!("__ny_poison_{}", name);
    let val_local = format!("__ny_val_{}", name);

    let local_cached = local_with_init(
        &cached_local,
        me_call("getField".to_string(), vec![string_lit(key.clone())]),
    );
    let if_cached = if_then(
        not_null(&cached_local),
        vec![return_expr(var(&cached_local))],
    );

    let local_poison = local_with_init(
        &poison_local,
        me_call("getField".to_string(), vec![string_lit(poison_key)]),
    );
    let if_poison = if_then(
        not_null(&poison_local),
        vec![ASTNode::Throw {
            expression: Box::new(string_lit(format!("once '{}' previously failed", name))),
            span: Span::unknown(),
        }],
    );

    let local_val = local_with_init(&val_local, me_call(compute_name, vec![]));
    let set_call = me_call(
        "setField".to_string(),
        vec![string_lit(key), var(&val_local)],
    );
    let getter_body = vec![
        local_cached,
        if_cached,
        local_poison,
        if_poison,
        local_val,
        set_call,
        return_expr(var(&val_local)),
    ];

    let getter_name = format!("__get_once_{}", name);
    methods.insert(getter_name.clone(), function_decl(getter_name, getter_body));
}

pub(crate) fn insert_birth_once_methods(
    methods: &mut HashMap<String, ASTNode>,
    name: String,
    compute_body: Vec<ASTNode>,
) {
    let compute_name = birth_once_compute_method_name(&name);
    methods.insert(
        compute_name.clone(),
        function_decl(compute_name, compute_body),
    );

    let getter_body = vec![return_expr(me_call(
        "getField".to_string(),
        vec![string_lit(birth_once_storage_key(&name))],
    ))];
    let getter_name = birth_once_getter_method_name(&name);
    methods.insert(getter_name.clone(), function_decl(getter_name, getter_body));
}

fn birth_once_initializer_pair(name: &str) -> Vec<ASTNode> {
    let tmp = format!("__ny_birth_{}", name);
    vec![
        local_with_init(&tmp, me_call(birth_once_compute_method_name(name), vec![])),
        me_call(
            "setField".to_string(),
            vec![string_lit(birth_once_storage_key(name)), var(&tmp)],
        ),
    ]
}

pub(crate) fn prepend_birth_once_initializers(
    birth_once_props: &[String],
    mut user_body: Vec<ASTNode>,
) -> Vec<ASTNode> {
    if birth_once_props.is_empty() {
        return user_body;
    }

    let mut body = Vec::with_capacity(birth_once_props.len() * 2 + user_body.len());
    for prop in birth_once_props {
        body.extend(birth_once_initializer_pair(prop));
    }
    body.append(&mut user_body);
    body
}

fn empty_birth_constructor(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "birth".to_string(),
        params: vec![],
        body,
        is_static: false,
        is_override: false,
        attrs: crate::ast::DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

pub(crate) fn apply_birth_once_constructor_prologues(
    constructors: &mut HashMap<String, ASTNode>,
    birth_once_props: &[String],
) {
    if birth_once_props.is_empty() {
        return;
    }

    let birth_keys: Vec<String> = constructors
        .keys()
        .filter(|key| key.starts_with("birth/"))
        .cloned()
        .collect();

    if birth_keys.is_empty() {
        let body = prepend_birth_once_initializers(birth_once_props, vec![]);
        constructors.insert("birth/0".to_string(), empty_birth_constructor(body));
        return;
    }

    for key in birth_keys {
        if let Some(ASTNode::FunctionDeclaration { body, .. }) = constructors.get_mut(&key) {
            let user_body = std::mem::take(body);
            *body = prepend_birth_once_initializers(birth_once_props, user_body);
        }
    }
}
