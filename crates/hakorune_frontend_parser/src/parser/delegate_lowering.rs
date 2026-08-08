//! DEL-003 delegate exposes lowering.
//!
//! This pass runs after parsing the whole Program so it can resolve typed
//! delegate fields against sibling box declarations. It deliberately stays
//! narrow: explicit `delegate field exposes { method [as alias] }` only.

use crate::ast::{
    ASTNode, BoxMethodGeneratedProvenanceV1, BoxMethodInventoryV1, DelegateDecl, FieldDecl,
    ParamDecl, PreparedGeneratedBoxMethodBatchV1, PreparedGeneratedBoxMethodV1, Span,
};
use crate::parser::ParseError;
use std::collections::HashMap;

#[derive(Clone)]
struct MethodSig {
    source_name: String,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
}

#[derive(Clone)]
struct BoxInfo {
    methods: BoxMethodInventoryV1,
}

fn delegate_error(message: impl Into<String>) -> ParseError {
    ParseError::DelegateLowering {
        message: message.into(),
        line: 0,
    }
}

pub fn lower_delegate_exposes(ast: ASTNode) -> Result<ASTNode, ParseError> {
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
        methods,
        constructors,
        init_fields,
        weak_fields,
        delegates,
        invariants,
        transitions,
        is_interface,
        is_record,
        extends,
        implements,
        type_parameters,
        is_sync,
        is_static,
        static_init,
        attrs,
        span,
    } = statement
    else {
        return Ok(statement);
    };

    let mut methods = methods;
    if !is_record && !delegates.is_empty() {
        let batch = prepare_delegates_for_box(&name, &field_decls, &delegates, boxes)?;
        methods.try_commit_generated_batch(batch).map_err(|error| {
            delegate_error(format!(
                "delegate method batch conflicts in box '{name}': {error}"
            ))
        })?;
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
        transitions,
        is_interface,
        is_record,
        extends,
        implements,
        type_parameters,
        is_sync,
        is_static,
        static_init,
        attrs,
        span,
    })
}

fn prepare_delegates_for_box(
    box_name: &str,
    field_decls: &[FieldDecl],
    delegates: &[DelegateDecl],
    boxes: &HashMap<String, BoxInfo>,
) -> Result<PreparedGeneratedBoxMethodBatchV1, ParseError> {
    let mut rows = Vec::new();

    for delegate in delegates {
        let selection = delegate
            .explicit_source_selection()
            .cloned()
            .ok_or_else(|| {
                delegate_error(format!(
                    "delegate field '{}' in box '{}' has compatibility-only provenance and cannot issue generated methods",
                    delegate.field_name, box_name
                ))
            })?;
        let target_type = delegate_field_type(box_name, field_decls, &delegate.field_name)?;
        let target = boxes.get(&target_type).ok_or_else(|| {
            delegate_error(format!(
                "delegate field '{}' in box '{}' refers to unknown target box '{}'",
                delegate.field_name, box_name, target_type
            ))
        })?;

        for expose in &delegate.exposes {
            let sig = resolve_unique_method(&target_type, target, &expose.source_name)?;
            let declaration =
                build_forwarding_method(&delegate.field_name, &expose.exposed_name, sig);
            rows.push(
                PreparedGeneratedBoxMethodV1::new(
                    expose.exposed_name.clone(),
                    declaration,
                    BoxMethodGeneratedProvenanceV1::Delegate {
                        field_name: delegate.field_name.clone().into_boxed_str(),
                        exposed_name: expose.exposed_name.clone().into_boxed_str(),
                        selection: selection.clone(),
                    },
                    Span::unknown(),
                )
                .map_err(|error| {
                    delegate_error(format!(
                        "invalid delegate method '{}' in box '{}': {error}",
                        expose.exposed_name, box_name
                    ))
                })?,
            );
        }
    }

    PreparedGeneratedBoxMethodBatchV1::try_new(rows).map_err(|error| {
        delegate_error(format!(
            "invalid delegate method batch in box '{box_name}': {error}"
        ))
    })
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
    let Some(method) = target.methods.get_declaration(method_name) else {
        return Err(delegate_error(format!(
            "delegate target '{}' has no method '{}'",
            target_type, method_name
        )));
    };
    let ASTNode::FunctionDeclaration {
        name,
        params,
        param_decls,
        return_type_name,
        ..
    } = method
    else {
        return Err(delegate_error(format!(
            "delegate target '{}' method '{}' is not a function declaration",
            target_type, method_name
        )));
    };
    Ok(MethodSig {
        source_name: name.clone(),
        params: params.clone(),
        param_decls: param_decls.clone(),
        return_type_name: return_type_name.clone(),
    })
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
        uses: vec![],

        contracts: vec![],
        is_static: false,
        is_override: false,
        attrs: crate::ast::DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}
