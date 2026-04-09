//! Enum declaration parser.
//!
//! Current narrow surface:
//! - `enum Name<T> { None, Some(T) }`
//! - unit variants
//! - single-payload tuple variants
//! - record variants with named fields (`Ident { name: String }`)

use crate::ast::{ASTNode, EnumVariantDecl, FieldDecl, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

pub fn parse_enum_declaration(p: &mut NyashParser) -> Result<ASTNode, ParseError> {
    p.consume(TokenType::ENUM)?;
    let attrs = p.take_pending_runes_for_box()?;
    let name = parse_enum_name(p)?;
    let type_parameters = parse_type_parameters(p)?;
    p.consume(TokenType::LBRACE)?;

    let mut variants = Vec::new();
    while !p.match_token(&TokenType::RBRACE) && !p.is_at_end() {
        if p.match_token(&TokenType::COMMA) || p.match_token(&TokenType::SEMICOLON) {
            p.advance();
            continue;
        }

        let variant_name = if let TokenType::IDENTIFIER(name) = &p.current_token().token_type {
            let name = name.clone();
            p.advance();
            name
        } else {
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "enum variant name".to_string(),
                line: p.current_token().line,
            });
        };

        let (payload_type_name, record_field_decls) = if p.match_token(&TokenType::LPAREN) {
            p.advance();
            let payload_type_name =
                crate::parser::common::type_refs::parse_type_ref_text(p, "enum variant payload")?;
            if p.match_token(&TokenType::COMMA) {
                return Err(ParseError::UnexpectedToken {
                    found: p.current_token().token_type.clone(),
                    expected: "single payload variant in the current enum surface".to_string(),
                    line: p.current_token().line,
                });
            }
            p.consume(TokenType::RPAREN)?;
            (Some(payload_type_name), Vec::new())
        } else if p.match_token(&TokenType::LBRACE) {
            (None, parse_record_variant_fields(p)?)
        } else {
            (None, Vec::new())
        };

        variants.push(EnumVariantDecl {
            name: variant_name,
            payload_type_name,
            record_field_decls,
        });

        if p.match_token(&TokenType::COMMA) || p.match_token(&TokenType::SEMICOLON) {
            p.advance();
        }
    }

    p.consume(TokenType::RBRACE)?;
    p.register_enum_declaration(&name, &variants);
    Ok(ASTNode::EnumDeclaration {
        name,
        variants,
        type_parameters,
        attrs,
        span: Span::unknown(),
    })
}

fn parse_enum_name(p: &mut NyashParser) -> Result<String, ParseError> {
    if let TokenType::IDENTIFIER(name) = &p.current_token().token_type {
        let name = name.clone();
        p.advance();
        Ok(name)
    } else {
        Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "identifier".to_string(),
            line: p.current_token().line,
        })
    }
}

fn parse_type_parameters(p: &mut NyashParser) -> Result<Vec<String>, ParseError> {
    if !p.match_token(&TokenType::LESS) {
        return Ok(Vec::new());
    }

    p.advance();
    let mut params = Vec::new();
    while !p.match_token(&TokenType::GREATER) && !p.is_at_end() {
        if let TokenType::IDENTIFIER(param) = &p.current_token().token_type {
            params.push(param.clone());
            p.advance();
        } else {
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "type parameter name".to_string(),
                line: p.current_token().line,
            });
        }

        if p.match_token(&TokenType::COMMA) {
            p.advance();
            continue;
        }
        break;
    }
    p.consume(TokenType::GREATER)?;
    Ok(params)
}

fn parse_record_variant_fields(p: &mut NyashParser) -> Result<Vec<FieldDecl>, ParseError> {
    let start_line = p.current_token().line;
    p.consume(TokenType::LBRACE)?;
    let mut fields = Vec::new();

    while !p.match_token(&TokenType::RBRACE) && !p.is_at_end() {
        if p.match_token(&TokenType::COMMA)
            || p.match_token(&TokenType::SEMICOLON)
            || p.match_token(&TokenType::NEWLINE)
        {
            p.advance();
            continue;
        }

        let field_name = if let TokenType::IDENTIFIER(name) = &p.current_token().token_type {
            let name = name.clone();
            p.advance();
            name
        } else {
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "record variant field name".to_string(),
                line: p.current_token().line,
            });
        };

        p.consume(TokenType::COLON)?;
        let declared_type_name =
            crate::parser::common::type_refs::parse_type_ref_text(p, "enum record field type")?;
        fields.push(FieldDecl {
            name: field_name,
            declared_type_name: Some(declared_type_name),
            is_weak: false,
        });

        if p.match_token(&TokenType::COMMA) || p.match_token(&TokenType::SEMICOLON) {
            p.advance();
        }
    }

    p.consume(TokenType::RBRACE)?;
    if fields.is_empty() {
        return Err(ParseError::InvalidMatchPattern {
            detail: "record enum variant requires at least one field".to_string(),
            line: start_line,
        });
    }
    Ok(fields)
}
