//! Header parsing: `Name<T...> from Parent1, Parent2`
//! Assumes the caller already consumed the leading `box` token.
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

fn at_implements_keyword(p: &NyashParser) -> bool {
    match &p.current_token().token_type {
        TokenType::INTERFACE => true,
        TokenType::IDENTIFIER(name) if name == "implements" => true,
        _ => false,
    }
}

/// Parse the leading header of a box declaration and return
/// (name, type_params, extends, implements). Does not consume the opening '{'.
pub(crate) fn parse_header(
    p: &mut NyashParser,
) -> Result<(String, Vec<String>, Vec<String>, Vec<String>), ParseError> {
    // Name
    let name = if let TokenType::IDENTIFIER(name) = &p.current_token().token_type {
        let name = name.clone();
        p.advance();
        name
    } else {
        let line = p.current_token().line;
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "identifier".to_string(),
            line,
        });
    };

    // Generic type parameters: <T, U>
    let type_parameters = if p.match_token(&TokenType::LESS) {
        p.advance(); // consume '<'
        let mut params = Vec::new();
        while !p.match_token(&TokenType::GREATER) && !p.is_at_end() {
            crate::must_advance!(p, _unused, "generic type parameter parsing");
            if let TokenType::IDENTIFIER(param) = &p.current_token().token_type {
                params.push(param.clone());
                p.advance();
                if p.match_token(&TokenType::COMMA) {
                    p.advance();
                }
            } else {
                return Err(ParseError::UnexpectedToken {
                    found: p.current_token().token_type.clone(),
                    expected: "type parameter name".to_string(),
                    line: p.current_token().line,
                });
            }
        }
        p.consume(TokenType::GREATER)?; // consume '>'
        params
    } else {
        Vec::new()
    };

    // extends: from Parent1, Parent2
    let extends = if p.match_token(&TokenType::FROM) {
        p.advance(); // consume 'from'
        let mut parents = Vec::new();
        if let TokenType::IDENTIFIER(parent) = &p.current_token().token_type {
            parents.push(parent.clone());
            p.advance();
        } else {
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "parent box name after 'from'".to_string(),
                line: p.current_token().line,
            });
        }
        while p.match_token(&TokenType::COMMA) {
            p.advance(); // consume ','
            if let TokenType::IDENTIFIER(parent) = &p.current_token().token_type {
                parents.push(parent.clone());
                p.advance();
            } else {
                return Err(ParseError::UnexpectedToken {
                    found: p.current_token().token_type.clone(),
                    expected: "parent box name after comma".to_string(),
                    line: p.current_token().line,
                });
            }
        }
        parents
    } else {
        Vec::new()
    };

    // implements/interface list: `interface A, B` or `implements A, B`
    let implements = if at_implements_keyword(p) {
        p.advance(); // consume INTERFACE or IDENTIFIER("implements")
        let mut list = Vec::new();
        if let TokenType::IDENTIFIER(interface_name) = &p.current_token().token_type {
            list.push(interface_name.clone());
            p.advance();
        } else {
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "interface name after implements/interface clause".to_string(),
                line: p.current_token().line,
            });
        }

        while p.match_token(&TokenType::COMMA) {
            p.advance(); // consume ','
            if let TokenType::IDENTIFIER(interface_name) = &p.current_token().token_type {
                list.push(interface_name.clone());
                p.advance();
            } else {
                return Err(ParseError::UnexpectedToken {
                    found: p.current_token().token_type.clone(),
                    expected: "interface name after comma".to_string(),
                    line: p.current_token().line,
                });
            }
        }
        list
    } else {
        Vec::new()
    };

    Ok((name, type_parameters, extends, implements))
}
