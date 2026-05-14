use super::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

pub(crate) fn parse_type_ref_text(
    p: &mut NyashParser,
    context: &str,
) -> Result<String, ParseError> {
    let (text, pending_gt) = parse_type_ref_text_inner(p, context)?;
    if pending_gt {
        return Err(ParseError::UnexpectedToken {
            found: TokenType::ShiftRight,
            expected: format!("balanced generic type reference in {}", context),
            line: p.current_token().line,
        });
    }
    Ok(text)
}

fn parse_type_ref_text_inner(
    p: &mut NyashParser,
    context: &str,
) -> Result<(String, bool), ParseError> {
    let mut text = parse_type_ref_path(p, context)?;
    let mut pending_gt = false;

    if p.match_token(&TokenType::LESS) {
        p.advance();
        let mut args = Vec::new();
        loop {
            if p.match_token(&TokenType::GREATER) {
                p.advance();
                break;
            }
            if p.match_token(&TokenType::ShiftRight) {
                p.advance();
                pending_gt = true;
                break;
            }
            let (arg, child_pending_gt) = parse_type_ref_text_inner(p, context)?;
            args.push(arg);
            if child_pending_gt {
                break;
            }
            if p.match_token(&TokenType::COMMA) {
                p.advance();
                continue;
            }
            if p.match_token(&TokenType::GREATER) {
                p.advance();
                break;
            }
            if p.match_token(&TokenType::ShiftRight) {
                p.advance();
                pending_gt = true;
                break;
            }
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: format!("',' or '>' in {}", context),
                line: p.current_token().line,
            });
        }
        if args.is_empty() {
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: format!("generic type argument in {}", context),
                line: p.current_token().line,
            });
        }
        text.push('<');
        text.push_str(&args.join(", "));
        text.push('>');
    }

    while p.match_token(&TokenType::LBRACK) {
        p.advance();
        p.consume(TokenType::RBRACK)?;
        text.push_str("[]");
    }

    Ok((text, pending_gt))
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
