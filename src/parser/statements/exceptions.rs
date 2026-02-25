/*!
 * Exception Handling Statement Parsers
 *
 * Handles parsing of:
 * - try-catch statements
 * - throw statements
 * - cleanup (finally) blocks
 */

use crate::ast::{ASTNode, CatchClause, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

const FINI_MARKER_EXCEPTION_TYPE: &str = "\u{0}drop_scope_fini_marker";

pub(super) fn make_fini_registration_marker(
    prelude_stmts: Vec<ASTNode>,
    fini_body: Vec<ASTNode>,
) -> ASTNode {
    ASTNode::TryCatch {
        try_body: prelude_stmts,
        catch_clauses: vec![CatchClause {
            exception_type: Some(FINI_MARKER_EXCEPTION_TYPE.to_string()),
            variable_name: None,
            body: fini_body,
            span: Span::unknown(),
        }],
        finally_body: None,
        span: Span::unknown(),
    }
}

pub(super) fn extract_fini_registration_marker(
    node: &ASTNode,
) -> Option<(Vec<ASTNode>, Vec<ASTNode>)> {
    match node {
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body: None,
            ..
        } if catch_clauses.len() == 1
            && catch_clauses[0].exception_type.as_deref() == Some(FINI_MARKER_EXCEPTION_TYPE) =>
        {
            Some((try_body.clone(), catch_clauses[0].body.clone()))
        }
        _ => None,
    }
}

impl NyashParser {
    /// Parse exception statement dispatch
    pub(super) fn parse_exception_statement(&mut self) -> Result<ASTNode, ParseError> {
        match &self.current_token().token_type {
            TokenType::TRY => self.parse_try_catch(),
            TokenType::FINI => self.parse_fini_stmt(),
            TokenType::THROW => self.parse_throw(),
            _ => Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "exception statement".to_string(),
                line: self.current_token().line,
            }),
        }
    }

    /// Parse try-catch statement
    pub(super) fn parse_try_catch(&mut self) -> Result<ASTNode, ParseError> {
        if !crate::config::env::parser_try_compat_enabled() {
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "[freeze:contract][parser/try_reserved] 'try' is legacy/prohibited; use postfix catch/cleanup or enable compatibility only during migration".to_string(),
                line: self.current_token().line,
            });
        }
        self.advance(); // consume 'try'
        let try_body = self.parse_block_statements()?;

        let mut catch_clauses = Vec::new();

        // Parse catch clauses
        while self.match_token(&TokenType::CATCH) {
            self.advance(); // consume 'catch'
            self.consume(TokenType::LPAREN)?;
            let (exception_type, exception_var) = self.parse_catch_param()?;
            self.consume(TokenType::RPAREN)?;
            let catch_body = self.parse_block_statements()?;

            catch_clauses.push(CatchClause {
                exception_type,
                variable_name: exception_var,
                body: catch_body,
                span: Span::unknown(),
            });
        }

        // Parse optional cleanup (finally) clause
        let finally_body = if self.match_token(&TokenType::CLEANUP) {
            self.advance(); // consume 'cleanup'
            Some(self.parse_block_statements()?)
        } else {
            None
        };

        Ok(ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            span: Span::unknown(),
        })
    }

    /// Parse throw statement
    pub(super) fn parse_throw(&mut self) -> Result<ASTNode, ParseError> {
        Err(ParseError::UnexpectedToken {
            found: self.current_token().token_type.clone(),
            expected:
                "[freeze:contract][parser/throw_reserved] 'throw' is reserved/prohibited in surface language".to_string(),
            line: self.current_token().line,
        })
    }

    /// Parse DropScope registration statement: fini { ... }
    pub(super) fn parse_fini_stmt(&mut self) -> Result<ASTNode, ParseError> {
        let fini_body = self.parse_fini_block()?;
        Ok(make_fini_registration_marker(Vec::new(), fini_body))
    }

    /// Parse `fini { ... }` block body and validate fini-specific restrictions.
    pub(super) fn parse_fini_block(&mut self) -> Result<Vec<ASTNode>, ParseError> {
        let fini_line = self.current_token().line;
        self.consume(TokenType::FINI)?;
        let body = self.parse_block_statements()?;
        Self::validate_fini_body(&body, fini_line)?;
        Ok(body)
    }

    fn validate_fini_body(body: &[ASTNode], line: usize) -> Result<(), ParseError> {
        if body.iter().any(ASTNode::contains_non_local_exit) {
            return Err(ParseError::UnexpectedToken {
                found: TokenType::FINI,
                expected:
                    "fini block must not contain return/break/continue/throw (non-local exits)"
                        .to_string(),
                line,
            });
        }
        Ok(())
    }

    /// Parse catch parameter: (ExceptionType varName) or (varName) or ()
    pub(crate) fn parse_catch_param(
        &mut self,
    ) -> Result<(Option<String>, Option<String>), ParseError> {
        match &self.current_token().token_type {
            TokenType::IDENTIFIER(first) => {
                let first_str = first.clone();
                let two_idents = matches!(self.peek_token(), TokenType::IDENTIFIER(_));
                if two_idents {
                    self.advance(); // consume type identifier
                    if let TokenType::IDENTIFIER(var_name) = &self.current_token().token_type {
                        let var = var_name.clone();
                        self.advance();
                        Ok((Some(first_str), Some(var)))
                    } else {
                        Err(ParseError::UnexpectedToken {
                            found: self.current_token().token_type.clone(),
                            expected: "exception variable name".to_string(),
                            line: self.current_token().line,
                        })
                    }
                } else {
                    self.advance();
                    Ok((None, Some(first_str)))
                }
            }
            _ => {
                if self.match_token(&TokenType::RPAREN) {
                    Ok((None, None))
                } else {
                    Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: ") or identifier".to_string(),
                        line: self.current_token().line,
                    })
                }
            }
        }
    }

    /// Parse postfix catch/cleanup error handler
    pub(super) fn parse_postfix_catch_cleanup_error(&mut self) -> Result<ASTNode, ParseError> {
        Err(ParseError::UnexpectedToken {
            found: self.current_token().token_type.clone(),
            expected: "catch/cleanup must follow a try block or standalone block".to_string(),
            line: self.current_token().line,
        })
    }
}
