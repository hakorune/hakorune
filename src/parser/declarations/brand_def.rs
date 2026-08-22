use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

pub fn parse_brand_declaration(p: &mut NyashParser) -> Result<ASTNode, ParseError> {
    p.consume(TokenType::BRAND)?;

    let name = if let TokenType::IDENTIFIER(name) = &p.current_token().token_type {
        let name = name.clone();
        p.advance();
        name
    } else {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "[freeze:contract][parser/brand_declaration_invalid] brand name".to_string(),
            line: p.current_token().line,
        });
    };

    if !matches!(p.current_token().token_type, TokenType::COLON) {
        return Err(invalid_brand_declaration(p, "colon"));
    }
    p.advance();
    let underlying_type_name =
        crate::parser::common::type_refs::parse_type_ref_text(p, "brand underlying type")
            .map_err(|_| invalid_brand_declaration(p, "underlying type"))?;

    let node = ASTNode::BrandDeclaration {
        name,
        underlying_type_name,
        span: Span::unknown(),
    };

    p.wrap_with_pending_build_gate(node)
}

fn invalid_brand_declaration(p: &NyashParser, expected_item: &str) -> ParseError {
    ParseError::UnexpectedToken {
        found: p.current_token().token_type.clone(),
        expected: format!("[freeze:contract][parser/brand_declaration_invalid] {expected_item}"),
        line: p.current_token().line,
    }
}
