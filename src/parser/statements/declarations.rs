/*!
 * Declaration Statement Parsers
 *
 * Dispatcher for declaration statements
 * Actual implementations are in other specialized modules
 */

use crate::ast::ASTNode;
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    /// Parse declaration statement dispatch
    pub(super) fn parse_declaration_statement(&mut self) -> Result<ASTNode, ParseError> {
        match &self.current_token().token_type {
            TokenType::BOX => crate::parser::declarations::box_def::parse_box_declaration(self),
            TokenType::FLOW => crate::parser::declarations::box_def::parse_box_declaration(self), // flow is syntactic sugar for static box
            TokenType::IMPORT => self.parse_import(),
            TokenType::INTERFACE => {
                crate::parser::declarations::box_def::parse_interface_box_declaration(self)
            }
            TokenType::GLOBAL => self.parse_global_var(),
            TokenType::FUNCTION => self.parse_function_declaration(),
            TokenType::STATIC => self.parse_static_declaration(),
            _ => Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "declaration statement".to_string(),
                line: self.current_token().line,
            }),
        }
    }
}
