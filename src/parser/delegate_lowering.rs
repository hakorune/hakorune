//! DEL-003 delegate exposes lowering.
//!
//! This pass runs after parsing the whole Program so it can resolve typed
//! delegate fields against sibling box declarations. It deliberately stays
//! narrow: explicit `delegate field exposes { method [as alias] }` only.

use crate::ast::{ASTNode, DelegateDecl, FieldDecl, ParamDecl, Span};
use crate::parser::ParseError;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct MethodSig {
    source_name: String,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
}

#[derive(Clone)]
struct BoxInfo {
    methods: HashMap<String, ASTNode>,
}

fn delegate_error(message: impl Into<String>) -> ParseError {
    ParseError::DelegateLowering {
        message: message.into(),
        line: 0,
    }
}

pub(super) fn lower_delegate_exposes(ast: ASTNode) -> Result<ASTNode, ParseError> {
    let ASTNode::Program { statements, span } = ast else {
        return Ok(ast);
    };

    let boxes = collect_box_info(&statements);
    let lowered = statements
        .into_iter()
        .map(|statement| lower_statement(statement, &boxes))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ASTNode::Program {
        statements: lowered,
        span,
    })
}

fn collect_box_info(statements: &[ASTNode]) -> HashMap<String, BoxInfo> {
    statements
        .iter()
        .filter_map(|statement| {
            let ASTNode::BoxDeclaration {
                name,
                field_decls: _,
                methods,
                is_record,
                ..
            } = statement
            else {
                return None;
            };
            if *is_record {
                return None;
            }
            Some((
                name.clone(),
                BoxInfo {
                    methods: methods.clone(),
                },
            ))
        })
        .collect()
}

fn lower_statement(
    statement: ASTNode,
    boxes: &HashMap<String, BoxInfo>,
) -> Result<ASTNode, ParseError> {
    let ASTNode::BoxDeclaration {
        name,
        fields,
        field_decls,
        public_fields,
        private_fields,
        mut methods,
        constructors,
        init_fields,
        weak_fields,
        delegates,
        invariants,
        is_interface,
        is_record,
        extends,
        implements,
        type_parameters,
        is_static,
        static_init,
        attrs,
        span,
    } = statement
    else {
        return Ok(statement);
    };

    if !is_record && !delegates.is_empty() {
        lower_delegates_for_box(&name, &field_decls, &delegates, &mut methods, boxes)?;
    }

    Ok(ASTNode::BoxDeclaration {
        name,
        fields,
        field_decls,
        public_fields,
        private_fields,
        methods,
        constructors,
        init_fields,
        weak_fields,
        delegates,
        invariants,
        is_interface,
        is_record,
        extends,
        implements,
        type_parameters,
        is_static,
        static_init,
        attrs,
        span,
    })
}

fn lower_delegates_for_box(
    box_name: &str,
    field_decls: &[FieldDecl],
    delegates: &[DelegateDecl],
    methods: &mut HashMap<String, ASTNode>,
    boxes: &HashMap<String, BoxInfo>,
) -> Result<(), ParseError> {
    let mut exposed_names = HashSet::new();

    for delegate in delegates {
        let target_type = delegate_field_type(box_name, field_decls, &delegate.field_name)?;
        let target = boxes.get(&target_type).ok_or_else(|| {
            delegate_error(format!(
                "delegate field '{}' in box '{}' refers to unknown target box '{}'",
                delegate.field_name, box_name, target_type
            ))
        })?;

        for expose in &delegate.exposes {
            if !exposed_names.insert(expose.exposed_name.clone()) {
                return Err(delegate_error(format!(
                    "delegate exposed method '{}' is duplicated in box '{}'",
                    expose.exposed_name, box_name
                )));
            }
            if methods.contains_key(&expose.exposed_name) {
                return Err(delegate_error(format!(
                    "delegate exposed method '{}' conflicts with local method in box '{}'",
                    expose.exposed_name, box_name
                )));
            }

            let sig = resolve_unique_method(&target_type, target, &expose.source_name)?;
            methods.insert(
                expose.exposed_name.clone(),
                build_forwarding_method(&delegate.field_name, &expose.exposed_name, sig),
            );
        }
    }

    Ok(())
}

fn delegate_field_type(
    box_name: &str,
    field_decls: &[FieldDecl],
    field_name: &str,
) -> Result<String, ParseError> {
    let field = field_decls
        .iter()
        .find(|decl| decl.name == field_name)
        .ok_or_else(|| {
            delegate_error(format!(
                "delegate field '{}' is not declared in box '{}'",
                field_name, box_name
            ))
        })?;
    field.declared_type_name.clone().ok_or_else(|| {
        delegate_error(format!(
            "delegate field '{}' in box '{}' must have a declared type",
            field_name, box_name
        ))
    })
}

fn resolve_unique_method(
    target_type: &str,
    target: &BoxInfo,
    method_name: &str,
) -> Result<MethodSig, ParseError> {
    let matches = target
        .methods
        .values()
        .filter_map(|method| {
            let ASTNode::FunctionDeclaration {
                name,
                params,
                param_decls,
                return_type_name,
                ..
            } = method
            else {
                return None;
            };
            (name == method_name).then(|| MethodSig {
                source_name: name.clone(),
                params: params.clone(),
                param_decls: param_decls.clone(),
                return_type_name: return_type_name.clone(),
            })
        })
        .collect::<Vec<_>>();

    match matches.len() {
        1 => Ok(matches[0].clone()),
        0 => Err(delegate_error(format!(
            "delegate target '{}' has no method '{}'",
            target_type, method_name
        ))),
        _ => Err(delegate_error(format!(
            "delegate target '{}' has ambiguous method '{}'",
            target_type, method_name
        ))),
    }
}

fn build_forwarding_method(field_name: &str, exposed_name: &str, sig: MethodSig) -> ASTNode {
    let arguments = sig
        .params
        .iter()
        .map(|name| ASTNode::Variable {
            name: name.clone(),
            span: Span::unknown(),
        })
        .collect::<Vec<_>>();

    let call = ASTNode::MethodCall {
        object: Box::new(ASTNode::FieldAccess {
            object: Box::new(ASTNode::Me {
                span: Span::unknown(),
            }),
            field: field_name.to_string(),
            span: Span::unknown(),
        }),
        method: sig.source_name,
        arguments,
        span: Span::unknown(),
    };

    ASTNode::FunctionDeclaration {
        name: exposed_name.to_string(),
        params: sig.params,
        param_decls: sig.param_decls,
        return_type_name: sig.return_type_name,
        body: vec![ASTNode::Return {
            value: Some(Box::new(call)),
            span: Span::unknown(),
        }],
        contracts: vec![],
        is_static: false,
        is_override: false,
        attrs: crate::ast::DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}
