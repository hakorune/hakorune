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
            expected: "[brand/declaration] brand name".to_string(),
            line: p.current_token().line,
        });
    };

    p.consume(TokenType::COLON)?;
    let underlying_type_name =
        crate::parser::common::type_refs::parse_type_ref_text(p, "brand underlying type")?;

    Ok(ASTNode::BrandDeclaration {
        name,
        underlying_type_name,
        span: Span::unknown(),
    })
}
