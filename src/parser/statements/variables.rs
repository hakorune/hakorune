/*!
 * Variable Declaration and Assignment Parsers
 *
 * Handles parsing of:
 * - local variable declarations
 * - outbox variable declarations
 * - assignment statements
 */

use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::cursor::TokenCursor;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::BTreeSet;

impl NyashParser {
    /// Parse variable declaration statement dispatch
    pub(super) fn parse_variable_declaration_statement(&mut self) -> Result<ASTNode, ParseError> {
        match &self.current_token().token_type {
            TokenType::LOCAL => self.parse_local(),
            TokenType::OUTBOX => self.parse_outbox(),
            _ => Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "variable declaration".to_string(),
                line: self.current_token().line,
            }),
        }
    }

    /// Parse local variable declaration: local var1, var2, var3 or local x = 10
    pub(super) fn parse_local(&mut self) -> Result<ASTNode, ParseError> {
        let debug_parse_local =
            std::env::var("NYASH_DEBUG_PARSE_LOCAL").ok().as_deref() == Some("1");
        if debug_parse_local {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[parse_local] entry: current_token={:?} at line {}",
                self.current_token().token_type,
                self.current_token().line
            ));
        }

        // Always skip leading NEWLINEs before consuming 'local' keyword
        while self.match_token(&TokenType::NEWLINE) {
            self.advance();
        }

        if super::helpers::cursor_enabled() {
            let mut cursor = TokenCursor::new(&self.tokens);
            cursor.set_position(self.current);
            cursor.with_stmt_mode(|c| c.skip_newlines());
            self.current = cursor.position();
        }
        self.advance(); // consume 'local'

        // Skip any NEWLINE tokens after 'local' keyword
        while self.match_token(&TokenType::NEWLINE) {
            self.advance();
        }

        if debug_parse_local {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[parse_local] after advance: current_token={:?} at line {}",
                self.current_token().token_type,
                self.current_token().line
            ));
        }

        let mut names = Vec::new();
        let mut initial_values = Vec::new();

        // Get first variable name
        if let TokenType::IDENTIFIER(name) = &self.current_token().token_type {
            names.push(name.clone());
            self.advance();

            // Check for initialization
            if self.match_token(&TokenType::ASSIGN) {
                self.advance(); // consume '='
                initial_values.push(Some(Box::new(self.parse_expression()?)));

                // With initialization, only single variable allowed
                let local_node = ASTNode::Local {
                    variables: names,
                    initial_values,
                    span: Span::unknown(),
                };
                self.maybe_attach_local_fini(local_node, 1)
            } else {
                // Without initialization, comma-separated variables allowed
                initial_values.push(None);

                // Parse additional comma-separated variables
                while self.match_token(&TokenType::COMMA) {
                    self.advance(); // consume ','

                    if let TokenType::IDENTIFIER(name) = &self.current_token().token_type {
                        names.push(name.clone());
                        initial_values.push(None);
                        self.advance();
                    } else {
                        return Err(ParseError::UnexpectedToken {
                            found: self.current_token().token_type.clone(),
                            expected: "identifier".to_string(),
                            line: self.current_token().line,
                        });
                    }
                }

                let local_node = ASTNode::Local {
                    variables: names,
                    initial_values,
                    span: Span::unknown(),
                };
                let local_count = match &local_node {
                    ASTNode::Local { variables, .. } => variables.len(),
                    _ => 0,
                };
                self.maybe_attach_local_fini(local_node, local_count)
            }
        } else {
            // Enhanced error message for debugging
            if debug_parse_local {
                crate::runtime::get_global_ring0().log.error(&format!(
                    "[parse_local] ERROR: Expected IDENTIFIER, found {:?} at line {}",
                    self.current_token().token_type,
                    self.current_token().line
                ));
                crate::runtime::get_global_ring0()
                    .log
                    .error("[parse_local] ERROR: Previous 3 tokens:");
                for i in 1..=3 {
                    if self.current >= i {
                        let idx = self.current - i;
                        if idx < self.tokens.len() {
                            crate::runtime::get_global_ring0()
                                .log
                                .error(&format!("  [-{}] {:?}", i, self.tokens[idx].token_type));
                        }
                    }
                }
            }
            Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "identifier".to_string(),
                line: self.current_token().line,
            })
        }
    }

    fn maybe_attach_local_fini(
        &mut self,
        local_node: ASTNode,
        local_count: usize,
    ) -> Result<ASTNode, ParseError> {
        if !self.match_token(&TokenType::FINI) {
            return Ok(local_node);
        }
        if local_count != 1 {
            return Err(ParseError::UnexpectedToken {
                found: TokenType::FINI,
                expected: "local ... fini requires exactly one local binding".to_string(),
                line: self.current_token().line,
            });
        }
        let fini_body = self.parse_fini_block()?;
        Ok(super::exceptions::make_fini_registration_marker(
            vec![local_node],
            fini_body,
        ))
    }

    /// Parse outbox variable declaration: outbox var1, var2, var3
    pub(super) fn parse_outbox(&mut self) -> Result<ASTNode, ParseError> {
        self.advance(); // consume 'outbox'

        let mut names = Vec::new();
        let mut seen = BTreeSet::new();

        // Get first variable name
        if let TokenType::IDENTIFIER(name) = &self.current_token().token_type {
            if !seen.insert(name.clone()) {
                return Err(ParseError::UnexpectedToken {
                    found: TokenType::OUTBOX,
                    expected: format!(
                        "[freeze:contract][moved/outbox_duplicate] duplicate outbox binding '{}'",
                        name
                    ),
                    line: self.current_token().line,
                });
            }
            names.push(name.clone());
            self.advance();

            // Parse additional comma-separated variables
            while self.match_token(&TokenType::COMMA) {
                self.advance(); // consume ','

                if let TokenType::IDENTIFIER(name) = &self.current_token().token_type {
                    if !seen.insert(name.clone()) {
                        return Err(ParseError::UnexpectedToken {
                            found: TokenType::OUTBOX,
                            expected: format!(
                                "[freeze:contract][moved/outbox_duplicate] duplicate outbox binding '{}'",
                                name
                            ),
                            line: self.current_token().line,
                        });
                    }
                    names.push(name.clone());
                    self.advance();
                } else {
                    return Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: "identifier".to_string(),
                        line: self.current_token().line,
                    });
                }
            }

            let len = names.len();
            Ok(ASTNode::Outbox {
                variables: names,
                initial_values: vec![None; len],
                span: Span::unknown(),
            })
        } else {
            Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "identifier".to_string(),
                line: self.current_token().line,
            })
        }
    }
}
