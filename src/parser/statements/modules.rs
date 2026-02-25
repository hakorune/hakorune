/*!
 * Module System Statement Parsers
 *
 * Handles parsing of:
 * - import statements
 * - using statements (namespace)
 * - from statements (delegation)
 */

use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    /// Parse import statement: import "path" (as Alias)?
    pub(super) fn parse_import(&mut self) -> Result<ASTNode, ParseError> {
        self.advance(); // consume 'import'

        let path = if let TokenType::STRING(s) = &self.current_token().token_type {
            let v = s.clone();
            self.advance();
            v
        } else {
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "string literal".to_string(),
                line: self.current_token().line,
            });
        };

        // Optional: 'as' Alias
        let mut alias: Option<String> = None;
        if let TokenType::IDENTIFIER(w) = &self.current_token().token_type {
            if w == "as" {
                self.advance();
                if let TokenType::IDENTIFIER(name) = &self.current_token().token_type {
                    alias = Some(name.clone());
                    self.advance();
                } else {
                    return Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: "alias name".to_string(),
                        line: self.current_token().line,
                    });
                }
            }
        }

        Ok(ASTNode::ImportStatement {
            path,
            alias,
            span: Span::unknown(),
        })
    }

    /// Parse using statement
    /// Accepts forms:
    ///  - using "module.path" (as Alias)?
    ///  - using module.path (as Alias)?
    /// Alias (if present) is currently ignored by the core parser and handled by runner-side resolution.
    pub(super) fn parse_using(&mut self) -> Result<ASTNode, ParseError> {
        self.advance(); // consume 'using'

        // Parse target: string literal or dotted identifiers
        let namespace = match &self.current_token().token_type {
            TokenType::STRING(s) => {
                let v = s.clone();
                self.advance();
                v
            }
            TokenType::IDENTIFIER(first) => {
                let mut parts = vec![first.clone()];
                self.advance();
                while let TokenType::DOT = self.current_token().token_type {
                    // consume '.' and the following IDENTIFIER-like segment
                    self.advance();
                    match &self.current_token().token_type {
                        TokenType::IDENTIFIER(seg) => {
                            parts.push(seg.clone());
                            self.advance();
                        }
                        // Allow `box` as a namespace segment (e.g. lang.compiler.parser.box)
                        // even though it is a keyword at the statement level.
                        TokenType::BOX => {
                            parts.push("box".to_string());
                            self.advance();
                        }
                        other => {
                            return Err(ParseError::UnexpectedToken {
                                found: other.clone(),
                                expected: "identifier after '.'".to_string(),
                                line: self.current_token().line,
                            });
                        }
                    }
                }
                parts.join(".")
            }
            other => {
                return Err(ParseError::UnexpectedToken {
                    found: other.clone(),
                    expected: "string or identifier".to_string(),
                    line: self.current_token().line,
                })
            }
        };

        // Optional: 'as' Alias — runner handles alias; parser skips if present
        if let TokenType::IDENTIFIER(w) = &self.current_token().token_type {
            if w == "as" {
                self.advance();
                // consume alias identifier (single segment)
                if let TokenType::IDENTIFIER(_alias) = &self.current_token().token_type {
                    self.advance();
                } else {
                    return Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: "alias name".to_string(),
                        line: self.current_token().line,
                    });
                }
            }
        }

        Ok(ASTNode::UsingStatement {
            namespace_name: namespace,
            span: Span::unknown(),
        })
    }

    /// Parse from statement: from Parent.method(args)
    /// Delegates to the existing parse_from_call() expression parser
    pub(super) fn parse_from_call_statement(&mut self) -> Result<ASTNode, ParseError> {
        // Use existing parse_from_call() to create FromCall AST node
        let from_call_expr = self.parse_from_call()?;

        // FromCall can be used as both expression and statement
        // Example: from Animal.constructor() (return value unused)
        Ok(from_call_expr)
    }
}
