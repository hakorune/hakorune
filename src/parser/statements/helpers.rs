/*!
 * Statement Parser Helper Functions
 *
 * Common utility functions used across statement parsers
 */

use crate::ast::ASTNode;
use crate::parser::common::ParserUtils;
use crate::parser::cursor::TokenCursor;
use crate::parser::{NyashParser, ParseError, RuneAttr};
use crate::tokenizer::TokenType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AnnotationSite {
    TopLevel,
    Member,
    Statement,
}

/// Check if token cursor is enabled
pub(super) fn cursor_enabled() -> bool {
    crate::config::env::parser_token_cursor_enabled()
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
    pub(crate) fn maybe_parse_opt_annotation_noop(
        &mut self,
        site: AnnotationSite,
    ) -> Result<bool, ParseError> {
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

        if anno_name == "rune" {
            return self.parse_rune_annotation();
        }

        let rune = self.parse_legacy_annotation_as_rune(anno_name)?;
        if self.legacy_annotation_should_normalize(site) {
            self.push_pending_rune(rune);
        }
        Ok(true)
    }

    fn parse_legacy_annotation_as_rune(
        &mut self,
        anno_name: String,
    ) -> Result<RuneAttr, ParseError> {
        self.consume(TokenType::LPAREN)?;

        let (name, arg) = match anno_name.as_str() {
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
                ("Hint".to_string(), v)
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
                ("Contract".to_string(), v)
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
                    let symbol = s.clone();
                    self.advance();
                    ("IntrinsicCandidate".to_string(), symbol)
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
        };

        self.consume(TokenType::RPAREN)?;
        if self.match_token(&TokenType::SEMICOLON) {
            self.advance();
        }

        Ok(RuneAttr {
            name,
            args: vec![arg],
        })
    }

    fn legacy_annotation_should_normalize(&self, site: AnnotationSite) -> bool {
        if matches!(site, AnnotationSite::Statement) {
            return false;
        }

        let mut idx = self.skip_annotation_trivia(self.current);
        while matches!(
            self.tokens.get(idx).map(|token| &token.token_type),
            Some(TokenType::AT)
        ) {
            let Some(next_idx) = self.scan_annotation_end(idx) else {
                return false;
            };
            idx = self.skip_annotation_trivia(next_idx);
        }

        match site {
            AnnotationSite::TopLevel => self.top_level_callable_starts_at(idx),
            AnnotationSite::Member => self.member_callable_starts_at(idx),
            AnnotationSite::Statement => false,
        }
    }

    fn skip_annotation_trivia(&self, mut idx: usize) -> usize {
        while let Some(token) = self.tokens.get(idx) {
            match token.token_type {
                TokenType::NEWLINE | TokenType::SEMICOLON => idx += 1,
                _ => break,
            }
        }
        idx
    }

    fn scan_annotation_end(&self, start: usize) -> Option<usize> {
        if !matches!(
            self.tokens.get(start).map(|token| &token.token_type),
            Some(TokenType::AT)
        ) {
            return Some(start);
        }

        let mut idx = start + 1;
        let name = match self.tokens.get(idx).map(|token| &token.token_type) {
            Some(TokenType::IDENTIFIER(name)) => {
                idx += 1;
                name.as_str()
            }
            _ => return None,
        };

        idx = self.skip_annotation_trivia(idx);
        match name {
            "rune" => {
                if matches!(
                    self.tokens.get(idx).map(|token| &token.token_type),
                    Some(TokenType::IDENTIFIER(_)) | Some(TokenType::STRING(_))
                ) {
                    idx += 1;
                }
                idx = self.skip_annotation_trivia(idx);
                if matches!(
                    self.tokens.get(idx).map(|token| &token.token_type),
                    Some(TokenType::LPAREN)
                ) {
                    idx += 1;
                    while !matches!(
                        self.tokens.get(idx).map(|token| &token.token_type),
                        Some(TokenType::RPAREN) | Some(TokenType::EOF) | None
                    ) {
                        idx += 1;
                    }
                    if !matches!(
                        self.tokens.get(idx).map(|token| &token.token_type),
                        Some(TokenType::RPAREN)
                    ) {
                        return None;
                    }
                    idx += 1;
                }
            }
            "hint" | "contract" | "intrinsic_candidate" => {
                if !matches!(
                    self.tokens.get(idx).map(|token| &token.token_type),
                    Some(TokenType::LPAREN)
                ) {
                    return None;
                }
                idx += 1;
                while !matches!(
                    self.tokens.get(idx).map(|token| &token.token_type),
                    Some(TokenType::RPAREN) | Some(TokenType::EOF) | None
                ) {
                    idx += 1;
                }
                if !matches!(
                    self.tokens.get(idx).map(|token| &token.token_type),
                    Some(TokenType::RPAREN)
                ) {
                    return None;
                }
                idx += 1;
            }
            _ => return None,
        }

        idx = self.skip_annotation_trivia(idx);
        if matches!(
            self.tokens.get(idx).map(|token| &token.token_type),
            Some(TokenType::SEMICOLON)
        ) {
            idx += 1;
        }
        Some(idx)
    }

    fn top_level_callable_starts_at(&self, idx: usize) -> bool {
        match self.tokens.get(idx).map(|token| &token.token_type) {
            Some(TokenType::FUNCTION) => true,
            Some(TokenType::STATIC) => {
                let next = self.skip_annotation_trivia(idx + 1);
                matches!(
                    self.tokens.get(next).map(|token| &token.token_type),
                    Some(TokenType::FUNCTION)
                )
            }
            _ => false,
        }
    }

    fn member_callable_starts_at(&self, idx: usize) -> bool {
        let mut next = idx;
        if matches!(
            self.tokens.get(next).map(|token| &token.token_type),
            Some(TokenType::OVERRIDE)
        ) {
            next = self.skip_annotation_trivia(next + 1);
        }

        match self.tokens.get(next).map(|token| &token.token_type) {
            Some(TokenType::IDENTIFIER(_))
            | Some(TokenType::INIT)
            | Some(TokenType::PACK)
            | Some(TokenType::BIRTH) => {
                let after_name = self.skip_annotation_trivia(next + 1);
                matches!(
                    self.tokens.get(after_name).map(|token| &token.token_type),
                    Some(TokenType::LPAREN)
                )
            }
            _ => false,
        }
    }

    fn parse_rune_annotation(&mut self) -> Result<bool, ParseError> {
        fn parse_rune_name(this: &mut NyashParser) -> Result<RuneAttr, ParseError> {
            let rune_name = if let TokenType::IDENTIFIER(name) = &this.current_token().token_type {
                let n = name.clone();
                this.advance();
                n
            } else if let TokenType::STRING(name) = &this.current_token().token_type {
                let n = name.clone();
                this.advance();
                n
            } else {
                return Err(ParseError::UnexpectedToken {
                    found: this.current_token().token_type.clone(),
                    expected: "[freeze:contract][parser/rune] rune name after @rune".to_string(),
                    line: this.current_token().line,
                });
            };

            if !RuneAttr::supported_name(&rune_name) {
                return Err(ParseError::UnexpectedToken {
                    found: TokenType::IDENTIFIER(rune_name),
                    expected: format!(
                        "[freeze:contract][parser/rune] supported: {}",
                        RuneAttr::supported_names_msg()
                    ),
                    line: this.current_token().line,
                });
            }

            let mut args = Vec::new();
            if this.match_token(&TokenType::LPAREN) {
                this.advance();
                while !this.match_token(&TokenType::RPAREN) {
                    let arg = if let TokenType::IDENTIFIER(name) = &this.current_token().token_type
                    {
                        let n = name.clone();
                        this.advance();
                        n
                    } else if let TokenType::STRING(name) = &this.current_token().token_type {
                        let n = name.clone();
                        this.advance();
                        n
                    } else {
                        return Err(ParseError::UnexpectedToken {
                            found: this.current_token().token_type.clone(),
                            expected: "[freeze:contract][parser/rune] rune argument".to_string(),
                            line: this.current_token().line,
                        });
                    };
                    args.push(arg);
                    if this.match_token(&TokenType::COMMA) {
                        this.advance();
                        continue;
                    }
                    break;
                }
                this.consume(TokenType::RPAREN)?;
            } else if RuneAttr::noarg_name(&rune_name) {
                // no-arg bare form
            } else {
                let arg = if let TokenType::IDENTIFIER(name) = &this.current_token().token_type {
                    let n = name.clone();
                    this.advance();
                    n
                } else if let TokenType::STRING(name) = &this.current_token().token_type {
                    let n = name.clone();
                    this.advance();
                    n
                } else {
                    return Err(ParseError::UnexpectedToken {
                        found: this.current_token().token_type.clone(),
                        expected:
                            "[freeze:contract][parser/rune] rune argument after bare rune name"
                                .to_string(),
                        line: this.current_token().line,
                    });
                };
                args.push(arg);
            }

            Ok(RuneAttr {
                name: rune_name,
                args,
            })
        }

        let rune = parse_rune_name(self)?;
        if !RuneAttr::supported_name(&rune.name) {
            return Err(ParseError::UnexpectedToken {
                found: TokenType::IDENTIFIER(rune.name),
                expected: format!(
                    "[freeze:contract][parser/rune] supported: {}",
                    RuneAttr::supported_names_msg()
                ),
                line: self.current_token().line,
            });
        }
        if RuneAttr::noarg_name(&rune.name) && !rune.args.is_empty() {
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: format!(
                    "[freeze:contract][parser/rune] @rune({}) takes no args",
                    rune.name
                ),
                line: self.current_token().line,
            });
        }
        if RuneAttr::single_arg_name(&rune.name) && rune.args.len() != 1 {
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: format!(
                    "[freeze:contract][parser/rune] @rune({})(<ident|string>)",
                    rune.name
                ),
                line: self.current_token().line,
            });
        }
        if let Some(expected) = RuneAttr::value_contract_error(&rune.name, &rune.args) {
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected,
                line: self.current_token().line,
            });
        }
        self.push_pending_rune(rune);

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
