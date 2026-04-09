use super::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

pub(crate) fn parse_type_ref_text(
    p: &mut NyashParser,
    context: &str,
) -> Result<String, ParseError> {
    let mut text = parse_type_ref_path(p, context)?;

    if p.match_token(&TokenType::LESS) {
        p.advance();
        let mut args = Vec::new();
        while !p.match_token(&TokenType::GREATER) && !p.is_at_end() {
            args.push(parse_type_ref_text(p, context)?);
            if p.match_token(&TokenType::COMMA) {
                p.advance();
                continue;
            }
            break;
        }
        p.consume(TokenType::GREATER)?;
        text.push('<');
        text.push_str(&args.join(", "));
        text.push('>');
    }

    while p.match_token(&TokenType::LBRACK) {
        p.advance();
        p.consume(TokenType::RBRACK)?;
        text.push_str("[]");
    }

    Ok(text)
}

fn parse_type_ref_path(p: &mut NyashParser, context: &str) -> Result<String, ParseError> {
    let mut parts = vec![parse_type_ident(p, context)?];
    while p.match_token(&TokenType::DOT) {
        p.advance();
        parts.push(parse_type_ident(p, context)?);
    }
    Ok(parts.join("."))
}

fn parse_type_ident(p: &mut NyashParser, context: &str) -> Result<String, ParseError> {
    if let TokenType::IDENTIFIER(name) = &p.current_token().token_type {
        let name = name.clone();
        p.advance();
        Ok(name)
    } else {
        Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: format!("type name in {}", context),
            line: p.current_token().line,
        })
    }
}
