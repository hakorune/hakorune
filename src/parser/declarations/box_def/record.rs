use crate::ast::{ASTNode, FieldDecl, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::HashMap;

fn is_supported_record_default_expr(expr: &ASTNode) -> bool {
    match expr {
        ASTNode::Literal { value, .. } => matches!(
            value,
            crate::ast::LiteralValue::Integer(_)
                | crate::ast::LiteralValue::TypedInteger { .. }
                | crate::ast::LiteralValue::Bool(_)
                | crate::ast::LiteralValue::String(_)
                | crate::ast::LiteralValue::Null
        ),
        ASTNode::UnaryOp {
            operator, operand, ..
        } => {
            matches!(operator, crate::ast::UnaryOperator::Minus)
                && matches!(
                    operand.as_ref(),
                    ASTNode::Literal {
                        value: crate::ast::LiteralValue::Integer(_)
                            | crate::ast::LiteralValue::TypedInteger { .. },
                        ..
                    }
                )
        }
        _ => false,
    }
}

pub(crate) fn parse_record_declaration(p: &mut NyashParser) -> Result<ASTNode, ParseError> {
    if !p.match_token(&TokenType::RECORD) {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "'record'".to_string(),
            line: p.current_token().line,
        });
    }
    p.advance(); // consume RECORD
    let attrs = p.take_pending_runes_for_box()?;
    let (name, type_parameters, extends, implements) =
        crate::parser::declarations::box_def::header::parse_header(p)?;
    if !extends.is_empty() || !implements.is_empty() {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "record declaration without from/implements clauses".to_string(),
            line: p.current_token().line,
        });
    }

    p.consume(TokenType::LBRACE)?;

    let mut fields = Vec::new();
    let mut field_decls = Vec::new();
    let mut invariants = Vec::new();
    while !p.match_token(&TokenType::RBRACE) && !p.is_at_end() {
        while p.match_token(&TokenType::NEWLINE) {
            p.advance();
        }
        if p.match_token(&TokenType::RBRACE) {
            break;
        }
        if let Some(invariant) = p.try_parse_invariant_clause()? {
            invariants.push(invariant);
            continue;
        }
        if p.match_token(&TokenType::WEAK) {
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "record field name; weak fields are not part of C202".to_string(),
                line: p.current_token().line,
            });
        }
        let TokenType::IDENTIFIER(field_name) = &p.current_token().token_type else {
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "record field name".to_string(),
                line: p.current_token().line,
            });
        };
        let field_name = field_name.clone();
        p.advance();
        p.consume(TokenType::COLON)?;
        let declared_type_name =
            crate::parser::common::type_refs::parse_type_ref_text(p, "record field type")?;

        let default_value = if p.match_token(&TokenType::ASSIGN) {
            p.advance();
            let default_expr = p.parse_expression()?;
            if !is_supported_record_default_expr(&default_expr) {
                return Err(ParseError::UnexpectedToken {
                    found: p.current_token().token_type.clone(),
                    expected: "record scalar literal default expression".to_string(),
                    line: p.current_token().line,
                });
            }
            Some(Box::new(default_expr))
        } else {
            None
        };

        fields.push(field_name.clone());
        field_decls.push(FieldDecl {
            name: field_name,
            declared_type_name: Some(declared_type_name),
            is_weak: false,
            default_value,
        });

        if p.match_token(&TokenType::COMMA) {
            p.advance();
        }
    }

    if field_decls.is_empty() {
        return Err(ParseError::InvalidStatement {
            line: p.current_token().line,
        });
    }

    p.consume(TokenType::RBRACE)?;

    let node = ASTNode::BoxDeclaration {
        name,
        fields,
        field_decls,
        public_fields: vec![],
        private_fields: vec![],
        methods: HashMap::new(),
        constructors: HashMap::new(),
        init_fields: vec![],
        weak_fields: vec![],
        delegates: vec![],
        invariants,
        transitions: vec![],
        is_interface: false,
        is_record: true,
        extends: vec![],
        implements: vec![],
        type_parameters,
        is_sync: false,
        is_static: false,
        static_init: None,
        attrs,
        span: Span::unknown(),
    };

    p.wrap_with_pending_build_gate(node)
}
