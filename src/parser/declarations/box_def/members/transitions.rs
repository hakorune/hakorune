use crate::ast::TransitionDecl;
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

pub(crate) fn try_parse_transition_decl(
    p: &mut NyashParser,
) -> Result<Option<TransitionDecl>, ParseError> {
    if !matches!(&p.current_token().token_type, TokenType::IDENTIFIER(name) if name == "transition") {
        return Ok(None);
    }

    p.advance(); // transition
    let from_state = parse_state_ref(p, "transition source state")?;
    p.consume(TokenType::MINUS)?;
    p.consume(TokenType::GREATER)?;
    let to_state = parse_state_ref(p, "transition target state")?;
    consume_context_word(p, "by", "transition by clause")?;
    let method_name = parse_ident(p, "transition method name")?;
    if p.match_token(&TokenType::SEMICOLON) {
        p.advance();
    }

    Ok(Some(TransitionDecl {
        from_state,
        to_state,
        method_name,
    }))
}

fn parse_state_ref(p: &mut NyashParser, context: &str) -> Result<String, ParseError> {
    let mut out = parse_ident(p, context)?;
    while p.match_token(&TokenType::DOT) {
        p.advance();
        out.push('.');
        out.push_str(&parse_ident(p, context)?);
    }
    Ok(out)
}

fn parse_ident(p: &mut NyashParser, context: &str) -> Result<String, ParseError> {
    if let TokenType::IDENTIFIER(name) = &p.current_token().token_type {
        let name = name.clone();
        p.advance();
        Ok(name)
    } else {
        Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: context.to_string(),
            line: p.current_token().line,
        })
    }
}

fn consume_context_word(
    p: &mut NyashParser,
    word: &str,
    context: &str,
) -> Result<(), ParseError> {
    if matches!(&p.current_token().token_type, TokenType::IDENTIFIER(name) if name == word) {
        p.advance();
        Ok(())
    } else {
        Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: context.to_string(),
            line: p.current_token().line,
        })
    }
}
