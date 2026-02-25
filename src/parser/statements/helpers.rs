/*!
 * Statement Parser Helper Functions
 *
 * Common utility functions used across statement parsers
 */

use crate::ast::ASTNode;
use crate::parser::common::ParserUtils;
use crate::parser::cursor::TokenCursor;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

/// Check if token cursor is enabled
pub(super) fn cursor_enabled() -> bool {
    std::env::var("NYASH_PARSER_TOKEN_CURSOR").ok().as_deref() == Some("1")
}

impl NyashParser {
    /// Parse optimization annotation directive and drop it as no-op.
    ///
    /// Accepted forms (feature-gated in tokenizer):
    /// - @hint(inline|noinline|hot|cold)
    /// - @contract(pure|readonly|no_alloc|no_safepoint)
    /// - @intrinsic_candidate("symbol")
    ///
    /// Returns true when an annotation was consumed.
    pub(crate) fn maybe_parse_opt_annotation_noop(&mut self) -> Result<bool, ParseError> {
        if !self.match_token(&TokenType::AT) {
            return Ok(false);
        }

        self.advance(); // consume '@'

        let anno_name = if let TokenType::IDENTIFIER(name) = &self.current_token().token_type {
            let n = name.clone();
            self.advance();
            n
        } else {
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "[freeze:contract][parser/annotation] annotation name after '@'"
                    .to_string(),
                line: self.current_token().line,
            });
        };

        self.consume(TokenType::LPAREN)?;

        match anno_name.as_str() {
            "hint" => {
                let v = if let TokenType::IDENTIFIER(name) = &self.current_token().token_type {
                    name.clone()
                } else {
                    return Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: "[freeze:contract][parser/annotation] @hint(<ident>)".to_string(),
                        line: self.current_token().line,
                    });
                };
                let ok = matches!(v.as_str(), "inline" | "noinline" | "hot" | "cold");
                if !ok {
                    return Err(ParseError::UnexpectedToken {
                        found: TokenType::IDENTIFIER(v),
                        expected:
                            "[freeze:contract][parser/annotation] @hint(inline|noinline|hot|cold)"
                                .to_string(),
                        line: self.current_token().line,
                    });
                }
                self.advance();
            }
            "contract" => {
                let v = if let TokenType::IDENTIFIER(name) = &self.current_token().token_type {
                    name.clone()
                } else {
                    return Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: "[freeze:contract][parser/annotation] @contract(<ident>)"
                            .to_string(),
                        line: self.current_token().line,
                    });
                };
                let ok = matches!(
                    v.as_str(),
                    "pure" | "readonly" | "no_alloc" | "no_safepoint"
                );
                if !ok {
                    return Err(ParseError::UnexpectedToken {
                        found: TokenType::IDENTIFIER(v),
                        expected: "[freeze:contract][parser/annotation] @contract(pure|readonly|no_alloc|no_safepoint)".to_string(),
                        line: self.current_token().line,
                    });
                }
                self.advance();
            }
            "intrinsic_candidate" => {
                if let TokenType::STRING(s) = &self.current_token().token_type {
                    if s.is_empty() {
                        return Err(ParseError::UnexpectedToken {
                            found: TokenType::STRING(s.clone()),
                            expected: "[freeze:contract][parser/annotation] @intrinsic_candidate(\"symbol\") with non-empty symbol".to_string(),
                            line: self.current_token().line,
                        });
                    }
                    self.advance();
                } else {
                    return Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected:
                            "[freeze:contract][parser/annotation] @intrinsic_candidate(\"symbol\")"
                                .to_string(),
                        line: self.current_token().line,
                    });
                }
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    found: TokenType::IDENTIFIER(anno_name),
                    expected:
                        "[freeze:contract][parser/annotation] supported: hint|contract|intrinsic_candidate"
                            .to_string(),
                    line: self.current_token().line,
                });
            }
        }

        self.consume(TokenType::RPAREN)?;
        if self.match_token(&TokenType::SEMICOLON) {
            self.advance();
        }
        Ok(true)
    }

    /// Thin adapter: when Cursor route is enabled, align statement start position
    /// by letting TokenCursor apply its statement-mode newline policy
    pub(super) fn with_stmt_cursor<F>(&mut self, f: F) -> Result<ASTNode, ParseError>
    where
        F: FnOnce(&mut Self) -> Result<ASTNode, ParseError>,
    {
        if cursor_enabled() {
            let mut cursor = TokenCursor::new(&self.tokens);
            cursor.set_position(self.current);
            cursor.with_stmt_mode(|c| {
                // Allow cursor to collapse any leading NEWLINEs in stmt mode
                c.skip_newlines();
            });
            self.current = cursor.position();
        }
        f(self)
    }

    /// Map a starting token into a grammar keyword string used by GRAMMAR_DIFF tracing
    pub(super) fn grammar_keyword_for(start: &TokenType) -> Option<&'static str> {
        match start {
            TokenType::BOX => Some("box"),
            TokenType::GLOBAL => Some("global"),
            TokenType::FUNCTION => Some("function"),
            TokenType::STATIC => Some("static"),
            TokenType::IF => Some("if"),
            TokenType::LOOP => Some("loop"),
            TokenType::BREAK => Some("break"),
            TokenType::RETURN => Some("return"),
            TokenType::PRINT => Some("print"),
            TokenType::NOWAIT => Some("nowait"),
            TokenType::LOCAL => Some("local"),
            TokenType::OUTBOX => Some("outbox"),
            TokenType::TRY => Some("try"),
            TokenType::FINI => Some("fini"),
            TokenType::THROW => Some("throw"),
            TokenType::USING => Some("using"),
            TokenType::FROM => Some("from"),
            _ => None,
        }
    }

    /// Small helper: build UnexpectedToken with current token and line
    #[allow(dead_code)]
    pub(super) fn err_unexpected<S: Into<String>>(&self, expected: S) -> ParseError {
        ParseError::UnexpectedToken {
            found: self.current_token().token_type.clone(),
            expected: expected.into(),
            line: self.current_token().line,
        }
    }

    /// Expect an identifier and advance. Returns its string or an UnexpectedToken error
    #[allow(dead_code)]
    pub(super) fn expect_identifier(&mut self, what: &str) -> Result<String, ParseError> {
        if let TokenType::IDENTIFIER(name) = &self.current_token().token_type {
            let out = name.clone();
            self.advance();
            Ok(out)
        } else {
            Err(self.err_unexpected(what))
        }
    }
}
