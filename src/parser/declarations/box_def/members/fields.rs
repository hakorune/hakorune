//! Fields parsing (header-first: `name: Type` + unified members gates)
use crate::ast::{ASTNode, FieldDecl, Span};
use crate::parser::common::ParserUtils;
use crate::parser::declarations::box_def::members::property_emit;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::HashMap;

fn parse_optional_declared_type_name(p: &mut NyashParser) -> Option<String> {
    if let TokenType::IDENTIFIER(ty) = &p.current_token().token_type {
        let ty = Some(ty.clone());
        p.advance();
        ty
    } else {
        None
    }
}

fn try_parse_computed_body(
    p: &mut NyashParser,
    fname: String,
    methods: &mut HashMap<String, ASTNode>,
) -> Result<bool, ParseError> {
    // name: Type => expr  → computed property (getter method with return expr)
    if p.match_token(&TokenType::FatArrow) {
        p.advance();
        let expr = p.parse_expression()?;
        let body = vec![ASTNode::Return {
            value: Some(Box::new(expr)),
            span: Span::unknown(),
        }];
        property_emit::insert_computed_getter(methods, fname, body);
        return Ok(true);
    }
    // name: Type { ... } [postfix]
    if p.match_token(&TokenType::LBRACE) {
        let body = p.parse_block_statements()?;
        let body =
            crate::parser::declarations::box_def::members::postfix::wrap_with_optional_postfix(
                p, body,
            )?;
        property_emit::insert_computed_getter(methods, fname, body);
        return Ok(true);
    }
    Ok(false)
}

/// Parse canonical computed property syntax after contextual `get`.
///
/// `get` remains a normal identifier outside Box member head position, and
/// `get: Type` still parses as a stored field named `get`.
pub(crate) fn try_parse_get_computed_property(
    p: &mut NyashParser,
    get_line: usize,
    methods: &mut HashMap<String, ASTNode>,
) -> Result<Option<String>, ParseError> {
    if !crate::config::env::unified_members() {
        return Ok(None);
    }
    let TokenType::IDENTIFIER(fname) = &p.current_token().token_type else {
        return Ok(None);
    };
    if p.current_token().line != get_line {
        return Ok(None);
    }
    if p.peek_token() != &TokenType::COLON {
        return Err(ParseError::UnexpectedToken {
            expected: "':' after get property name".to_string(),
            found: p.peek_token().clone(),
            line: p.current_token().line,
        });
    }

    let fname = fname.clone();
    p.advance(); // consume property name
    p.consume(TokenType::COLON)?;
    let _declared_type_name = parse_optional_declared_type_name(p);

    if try_parse_computed_body(p, fname.clone(), methods)? {
        return Ok(Some(fname));
    }

    Err(ParseError::UnexpectedToken {
        expected: "'=>' expression or block for get property".to_string(),
        found: p.current_token().token_type.clone(),
        line: p.current_token().line,
    })
}

/// Parse a header-first field or property that starts with an already parsed identifier `fname`.
/// Handles:
/// - `name: Type`                      → field
/// - `get name: Type => expr`          → canonical computed property (handled before this function)
/// - `name: Type = expr`               → field with initializer (initializer is parsed then discarded at P0)
/// - `name: Type => expr`              → computed property (getter function generated)
/// - `name: Type { ... } [catch|cleanup]` → computed property block with optional postfix handlers
/// Note: weak field parsing is handled at the top level in parse_box_declaration (Phase 285A1.2)
/// Returns Ok(true) when this function consumed and handled the construct; Ok(false) if not applicable.
pub(crate) fn try_parse_header_first_field_or_property(
    p: &mut NyashParser,
    fname: String,
    methods: &mut HashMap<String, ASTNode>,
    fields: &mut Vec<String>,
    field_decls: &mut Vec<FieldDecl>,
    _weak_fields: &mut Vec<String>,
    is_weak: bool,
) -> Result<bool, ParseError> {
    // Expect ':' Type after name
    if !p.match_token(&TokenType::COLON) {
        // No type annotation: treat as bare stored field
        fields.push(fname.clone());
        field_decls.push(FieldDecl {
            name: fname,
            declared_type_name: None,
            is_weak,
        });
        return Ok(true);
    }
    p.advance(); // consume ':'
                 // Optional type name (identifier). Keep it as declared field metadata.
    let declared_type_name = parse_optional_declared_type_name(p);

    // Unified members gate behavior
    if crate::config::env::unified_members() {
        // name: Type = expr  → field with initializer (store as field, initializer discarded at P0)
        if p.match_token(&TokenType::ASSIGN) {
            p.advance();
            let _init_expr = p.parse_expression()?; // P0: parse and discard
            fields.push(fname.clone());
            field_decls.push(FieldDecl {
                name: fname,
                declared_type_name,
                is_weak,
            });
            return Ok(true);
        }
        if try_parse_computed_body(p, fname.clone(), methods)? {
            return Ok(true);
        }
    }

    // Default: treat as a plain field when unified-members gate didn't match any special form
    fields.push(fname.clone());
    field_decls.push(FieldDecl {
        name: fname,
        declared_type_name,
        is_weak,
    });
    Ok(true)
}

/// Parse a visibility block or a single header-first property with visibility prefix.
/// Handles:
/// - `public { a, b, c }`  → pushes to fields/public_fields
/// - `private { ... }`      → pushes to fields/private_fields
/// - `public name: Type ...` (delegates to header-first field/property)
/// Returns Ok(true) if consumed, Ok(false) if visibility keyword not matched.
pub(crate) fn try_parse_visibility_block_or_single(
    p: &mut NyashParser,
    visibility: &str,
    methods: &mut HashMap<String, ASTNode>,
    fields: &mut Vec<String>,
    field_decls: &mut Vec<FieldDecl>,
    public_fields: &mut Vec<String>,
    private_fields: &mut Vec<String>,
    last_method_name: &mut Option<String>,
    weak_fields: &mut Vec<String>,
) -> Result<bool, ParseError> {
    if visibility != "public" && visibility != "private" {
        return Ok(false);
    }
    if p.match_token(&TokenType::LBRACE) {
        p.advance();
        while !p.match_token(&TokenType::RBRACE) && !p.is_at_end() {
            // Phase 285A1.3: Check for weak modifier in visibility block
            let is_weak = if p.match_token(&TokenType::WEAK) {
                p.advance();
                true
            } else {
                false
            };

            if let TokenType::IDENTIFIER(fname) = &p.current_token().token_type {
                let fname = fname.clone();
                if visibility == "public" {
                    public_fields.push(fname.clone());
                } else {
                    private_fields.push(fname.clone());
                }
                if is_weak {
                    weak_fields.push(fname.clone());
                }
                fields.push(fname.clone());
                field_decls.push(FieldDecl {
                    name: fname,
                    declared_type_name: None,
                    is_weak,
                });
                p.advance();
                if p.match_token(&TokenType::COMMA) {
                    p.advance();
                }
                continue;
            }
            return Err(ParseError::UnexpectedToken {
                expected: if is_weak {
                    "field name after 'weak' in visibility block"
                } else {
                    "identifier in visibility block"
                }
                .to_string(),
                found: p.current_token().token_type.clone(),
                line: p.current_token().line,
            });
        }
        p.consume(TokenType::RBRACE)?;
        return Ok(true);
    }
    // Phase 285A1.4: Sugar syntax - public weak parent, private weak parent
    if p.match_token(&TokenType::WEAK) {
        p.advance(); // consume WEAK only

        // Read field name (reuse existing pattern)
        if let TokenType::IDENTIFIER(fname) = &p.current_token().token_type {
            let fname = fname.clone();
            p.advance(); // consume IDENTIFIER

            // Delegate to existing weak field parser (handles type annotation, etc.)
            parse_weak_field(p, fname.clone(), methods, fields, field_decls, weak_fields)?;

            // Register with visibility tracking
            if visibility == "public" {
                public_fields.push(fname);
            } else {
                private_fields.push(fname);
            }

            *last_method_name = None; // Reset method context (Phase 285A1.4)
            return Ok(true);
        } else {
            return Err(ParseError::UnexpectedToken {
                expected: "field name after 'weak' in visibility context".to_string(),
                found: p.current_token().token_type.clone(),
                line: p.current_token().line,
            });
        }
    }
    if let TokenType::IDENTIFIER(n) = &p.current_token().token_type {
        let n = n.clone();
        if crate::config::env::unified_members() && n == "get" {
            let get_line = p.current_token().line;
            p.advance();
            if let Some(property_name) = try_parse_get_computed_property(p, get_line, methods)? {
                if visibility == "public" {
                    public_fields.push(property_name);
                } else {
                    private_fields.push(property_name);
                }
                *last_method_name = None;
                return Ok(true);
            }
            let fname = "get".to_string();
            if try_parse_header_first_field_or_property(
                p,
                fname.clone(),
                methods,
                fields,
                field_decls,
                weak_fields,
                false,
            )? {
                if visibility == "public" {
                    public_fields.push(fname.clone());
                } else {
                    private_fields.push(fname.clone());
                }
                *last_method_name = None;
                return Ok(true);
            }
        }

        let fname = n;
        p.advance();
        if try_parse_header_first_field_or_property(
            p,
            fname.clone(),
            methods,
            fields,
            field_decls,
            weak_fields,
            false,
        )? {
            if visibility == "public" {
                public_fields.push(fname.clone());
            } else {
                private_fields.push(fname.clone());
            }
            *last_method_name = None;
            return Ok(true);
        } else {
            if visibility == "public" {
                public_fields.push(fname.clone());
            } else {
                private_fields.push(fname.clone());
            }
            fields.push(fname);
            return Ok(true);
        }
    }
    Ok(false)
}

/// Parse a weak field after WEAK token has been consumed.
/// Handles both bare `weak parent` and `weak parent: Type` syntax.
/// Returns Ok(()) on success.
/// Phase 285A1.3: Unified weak field parsing logic.
pub(crate) fn parse_weak_field(
    p: &mut NyashParser,
    field_name: String,
    methods: &mut HashMap<String, ASTNode>,
    fields: &mut Vec<String>,
    field_decls: &mut Vec<FieldDecl>,
    weak_fields: &mut Vec<String>,
) -> Result<(), ParseError> {
    // Parse optional type annotation or property syntax via header-first parser
    try_parse_header_first_field_or_property(
        p,
        field_name.clone(),
        methods,
        fields,
        field_decls,
        weak_fields,
        true,
    )?;
    // Add to weak_fields vector (unified location for all weak field tracking)
    weak_fields.push(field_name);
    Ok(())
}

/// Parse `init { ... }` non-call block to collect initializable fields and weak flags.
/// Returns Ok(true) if consumed; Ok(false) if no `init {` at current position.
pub(crate) fn parse_init_block_if_any(
    p: &mut NyashParser,
    init_fields: &mut Vec<String>,
    weak_fields: &mut Vec<String>,
) -> Result<bool, ParseError> {
    if !(p.match_token(&TokenType::INIT) && p.peek_token() != &TokenType::LPAREN) {
        return Ok(false);
    }
    p.advance(); // consume 'init'
    p.consume(TokenType::LBRACE)?;
    while !p.match_token(&TokenType::RBRACE) && !p.is_at_end() {
        if p.match_token(&TokenType::RBRACE) {
            break;
        }
        let is_weak = if p.match_token(&TokenType::WEAK) {
            p.advance();
            true
        } else {
            false
        };
        if let TokenType::IDENTIFIER(field_name) = &p.current_token().token_type {
            init_fields.push(field_name.clone());
            if is_weak {
                weak_fields.push(field_name.clone());
            }
            p.advance();
            if p.match_token(&TokenType::COMMA) {
                p.advance();
            }
        } else {
            return Err(ParseError::UnexpectedToken {
                expected: if is_weak {
                    "field name after 'weak'"
                } else {
                    "field name"
                }
                .to_string(),
                found: p.current_token().token_type.clone(),
                line: p.current_token().line,
            });
        }
    }
    p.consume(TokenType::RBRACE)?;
    Ok(true)
}
