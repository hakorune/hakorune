//! Explicit delegation member parser.
//!
//! DEL-002 owns only syntax and metadata transport:
//! `delegate <field> exposes { <method> [as <alias>], ... }`.
//! Forwarding generation, collision checks, and interface conformance belong to
//! later Stage1 rows.

use crate::ast::{DelegateDecl, DelegateExposeDecl};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

fn skip_separators(p: &mut NyashParser) {
    while p.match_token(&TokenType::NEWLINE) || p.match_token(&TokenType::COMMA) {
        p.advance();
    }
}

fn take_identifier(p: &mut NyashParser, expected: &str) -> Result<String, ParseError> {
    let TokenType::IDENTIFIER(name) = &p.current_token().token_type else {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: expected.to_string(),
            line: p.current_token().line,
        });
    };
    let name = name.clone();
    p.advance();
    Ok(name)
}

fn match_contextual_ident(p: &NyashParser, expected: &str) -> bool {
    matches!(&p.current_token().token_type, TokenType::IDENTIFIER(name) if name == expected)
}

pub(crate) fn parse_delegate_decl(p: &mut NyashParser) -> Result<DelegateDecl, ParseError> {
    p.consume(TokenType::DELEGATE)?;
    let field_name = take_identifier(p, "delegate field name")?;

    if !match_contextual_ident(p, "exposes") {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "'exposes' after delegate field name".to_string(),
            line: p.current_token().line,
        });
    }
    p.advance(); // contextual `exposes`
    p.consume(TokenType::LBRACE)?;

    let mut exposes = Vec::new();
    loop {
        skip_separators(p);
        if p.match_token(&TokenType::RBRACE) {
            break;
        }

        let source_name = take_identifier(p, "method name in delegate exposes list")?;
        let exposed_name = if match_contextual_ident(p, "as") {
            p.advance(); // contextual `as`
            take_identifier(p, "alias name after delegate expose 'as'")?
        } else {
            source_name.clone()
        };
        exposes.push(DelegateExposeDecl {
            source_name,
            exposed_name,
        });
    }

    if exposes.is_empty() {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "at least one method in delegate exposes list".to_string(),
            line: p.current_token().line,
        });
    }

    p.consume(TokenType::RBRACE)?;
    Ok(DelegateDecl {
        field_name,
        exposes,
    })
}
