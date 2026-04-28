//! Properties parsing (once/birth_once, header-first)
use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::declarations::box_def::members::property_emit;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PropertyMemberKind {
    Computed,
    Once,
    BirthOnce,
}

impl PropertyMemberKind {
    fn from_keyword(keyword: &str) -> Option<Self> {
        match keyword {
            "once" => Some(Self::Once),
            "birth_once" => Some(Self::BirthOnce),
            _ => None,
        }
    }

    fn emit(
        self,
        methods: &mut HashMap<String, ASTNode>,
        birth_once_props: &mut Vec<String>,
        name: String,
        body: Vec<ASTNode>,
    ) {
        match self {
            Self::Computed => property_emit::insert_computed_getter(methods, name, body),
            Self::Once => property_emit::insert_once_methods(methods, name, body),
            Self::BirthOnce => {
                birth_once_props.push(name.clone());
                property_emit::insert_birth_once_methods(methods, name, body);
            }
        }
    }
}

/// Try to parse a unified member property: `once name: Type ...` or `birth_once name: Type ...`
/// Returns Ok(true) if consumed and handled; otherwise Ok(false).
pub(crate) fn try_parse_unified_property(
    p: &mut NyashParser,
    kind_kw: &str,
    methods: &mut HashMap<String, ASTNode>,
    birth_once_props: &mut Vec<String>,
) -> Result<bool, ParseError> {
    let Some(kind) = PropertyMemberKind::from_keyword(kind_kw) else {
        return Ok(false);
    };

    // Name
    let name = if let TokenType::IDENTIFIER(n) = &p.current_token().token_type {
        let n2 = n.clone();
        p.advance();
        n2
    } else {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "identifier after once/birth_once".to_string(),
            line: p.current_token().line,
        });
    };
    // ':' TYPE (type is accepted and ignored for now)
    if p.match_token(&TokenType::COLON) {
        p.advance();
        if let TokenType::IDENTIFIER(_ty) = &p.current_token().token_type {
            p.advance();
        } else {
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "type name".to_string(),
                line: p.current_token().line,
            });
        }
    } else {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: ": type".to_string(),
            line: p.current_token().line,
        });
    }
    // Body: either fat arrow expr or block
    let orig_body: Vec<ASTNode> = if p.match_token(&TokenType::FatArrow) {
        p.advance(); // consume '=>'
        let expr = p.parse_expression()?;
        vec![ASTNode::Return {
            value: Some(Box::new(expr)),
            span: Span::unknown(),
        }]
    } else {
        p.parse_block_statements()?
    };
    // Optional postfix handlers (Stage-3) directly after body
    let final_body =
        crate::parser::declarations::box_def::members::postfix::wrap_with_optional_postfix(
            p, orig_body,
        )?;
    kind.emit(methods, birth_once_props, name, final_body);
    Ok(true)
}

/// Try to parse a block-first unified member: `{ body } as [once|birth_once]? name : Type [postfix]`
/// Returns Ok(true) if a member was parsed and emitted into `methods`.
pub(crate) fn try_parse_block_first_property(
    p: &mut NyashParser,
    methods: &mut HashMap<String, ASTNode>,
    birth_once_props: &mut Vec<String>,
) -> Result<bool, ParseError> {
    if !(crate::config::env::unified_members() && p.match_token(&TokenType::LBRACE)) {
        return Ok(false);
    }
    // 1) Parse block body first
    let mut final_body = p.parse_block_statements()?;

    // 2) Expect 'as'
    if let TokenType::IDENTIFIER(kw) = &p.current_token().token_type {
        if kw != "as" {
            let line = p.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "'as' after block for block-first member".to_string(),
                line,
            });
        }
    } else {
        let line = p.current_token().line;
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "'as' after block for block-first member".to_string(),
            line,
        });
    }
    p.advance(); // consume 'as'

    // 3) Optional kind keyword: once | birth_once
    let mut kind = PropertyMemberKind::Computed;
    if let TokenType::IDENTIFIER(k) = &p.current_token().token_type {
        if let Some(parsed_kind) = PropertyMemberKind::from_keyword(k) {
            kind = parsed_kind;
            p.advance();
        }
    }

    // 4) Name : Type
    let name = if let TokenType::IDENTIFIER(n) = &p.current_token().token_type {
        let s = n.clone();
        p.advance();
        s
    } else {
        let line = p.current_token().line;
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "identifier for member name".to_string(),
            line,
        });
    };
    if p.match_token(&TokenType::COLON) {
        p.advance();
        if let TokenType::IDENTIFIER(_ty) = &p.current_token().token_type {
            p.advance();
        } else {
            let line = p.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "type name after ':'".to_string(),
                line,
            });
        }
    } else {
        let line = p.current_token().line;
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: ": type".to_string(),
            line,
        });
    }

    // 5) Optional postfix handlers (Stage‑3) directly after block (shared helper)
    final_body =
        crate::parser::declarations::box_def::members::postfix::wrap_with_optional_postfix(
            p, final_body,
        )?;

    kind.emit(methods, birth_once_props, name, final_body);
    Ok(true)
}
