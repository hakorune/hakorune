use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::{BTreeMap, BTreeSet};

impl NyashParser {
    pub(crate) fn parse_known_enum_record_ctor_arguments(
        &mut self,
        enum_name: &str,
        variant_name: &str,
    ) -> Result<Vec<ASTNode>, ParseError> {
        let line = self.current_token().line;
        let variant_decl = self
            .known_enums
            .get(enum_name)
            .and_then(|variants| variants.iter().find(|variant| variant.name == variant_name))
            .cloned()
            .ok_or_else(|| ParseError::UnexpectedToken {
                found: TokenType::LBRACE,
                expected: format!("known enum variant `{}` for `{}`", variant_name, enum_name),
                line,
            })?;
        if !variant_decl.is_record_payload() {
            return Err(ParseError::UnexpectedToken {
                found: TokenType::LBRACE,
                expected: format!("tuple constructor `{}::{}(...)`", enum_name, variant_name),
                line,
            });
        }

        self.consume(TokenType::LBRACE)?;
        let mut values = BTreeMap::new();
        let mut seen = BTreeSet::new();
        while !self.match_token(&TokenType::RBRACE) && !self.is_at_end() {
            if self.match_token(&TokenType::COMMA) || self.match_token(&TokenType::NEWLINE) {
                self.advance();
                continue;
            }

            let field_name = match &self.current_token().token_type {
                TokenType::IDENTIFIER(name) => {
                    let name = name.clone();
                    self.advance();
                    name
                }
                other => {
                    return Err(ParseError::UnexpectedToken {
                        found: other.clone(),
                        expected: "record field name".to_string(),
                        line: self.current_token().line,
                    });
                }
            };
            if !seen.insert(field_name.clone()) {
                return Err(ParseError::InvalidMatchPattern {
                    detail: format!(
                        "duplicate field `{}` in record enum constructor {}::{}",
                        field_name, enum_name, variant_name
                    ),
                    line: self.current_token().line,
                });
            }
            if !variant_decl
                .record_field_decls
                .iter()
                .any(|field| field.name == field_name)
            {
                return Err(ParseError::InvalidMatchPattern {
                    detail: format!(
                        "unknown field `{}` in record enum constructor {}::{}",
                        field_name, enum_name, variant_name
                    ),
                    line: self.current_token().line,
                });
            }

            self.consume(TokenType::COLON)?;
            let value = self.parse_expression()?;
            values.insert(field_name, value);

            if self.match_token(&TokenType::COMMA) {
                self.advance();
            }
        }
        self.consume(TokenType::RBRACE)?;

        let mut ordered = Vec::with_capacity(variant_decl.record_field_decls.len());
        let mut missing = Vec::new();
        for field in &variant_decl.record_field_decls {
            if let Some(value) = values.remove(&field.name) {
                ordered.push(value);
            } else {
                missing.push(field.name.clone());
            }
        }
        if !missing.is_empty() {
            return Err(ParseError::InvalidMatchPattern {
                detail: format!(
                    "record enum constructor {}::{} is missing field(s): {}",
                    enum_name,
                    variant_name,
                    missing.join(", ")
                ),
                line,
            });
        }
        Ok(ordered)
    }

    pub(crate) fn parse_record_literal(
        &mut self,
        record_type_name: String,
    ) -> Result<ASTNode, ParseError> {
        self.consume(TokenType::LBRACE)?;
        let mut fields = Vec::new();
        let mut seen = BTreeSet::new();

        while !self.match_token(&TokenType::RBRACE) && !self.is_at_end() {
            if self.match_token(&TokenType::COMMA) || self.match_token(&TokenType::NEWLINE) {
                self.advance();
                continue;
            }

            let field_name = match &self.current_token().token_type {
                TokenType::IDENTIFIER(name) => {
                    let name = name.clone();
                    self.advance();
                    name
                }
                other => {
                    return Err(ParseError::UnexpectedToken {
                        found: other.clone(),
                        expected: "[record-literal] field name".to_string(),
                        line: self.current_token().line,
                    });
                }
            };
            if !seen.insert(field_name.clone()) {
                return Err(ParseError::UnexpectedToken {
                    found: TokenType::IDENTIFIER(field_name),
                    expected: "[record-literal] unique field name".to_string(),
                    line: self.current_token().line,
                });
            }
            let value = if self.match_token(&TokenType::COLON) {
                self.advance();
                self.parse_expression()?
            } else {
                ASTNode::Variable {
                    name: field_name.clone(),
                    span: Span::unknown(),
                }
            };
            fields.push((field_name, value));

            if self.match_token(&TokenType::COMMA) {
                self.advance();
            }
        }

        self.consume(TokenType::RBRACE)?;
        Ok(ASTNode::RecordLiteral {
            record_type_name,
            fields,
            span: Span::unknown(),
        })
    }

    pub(crate) fn current_lbrace_starts_record_literal(&self) -> bool {
        if !self.match_token(&TokenType::LBRACE) {
            return false;
        }

        let mut offset = 1;
        while matches!(
            self.peek_nth_token(offset),
            TokenType::NEWLINE | TokenType::COMMA
        ) {
            offset += 1;
        }
        if matches!(self.peek_nth_token(offset), TokenType::RBRACE) {
            return true;
        }
        if !matches!(self.peek_nth_token(offset), TokenType::IDENTIFIER(_)) {
            return false;
        }
        offset += 1;
        while matches!(self.peek_nth_token(offset), TokenType::NEWLINE) {
            offset += 1;
        }
        matches!(
            self.peek_nth_token(offset),
            TokenType::COLON | TokenType::COMMA | TokenType::RBRACE | TokenType::NEWLINE
        )
    }

    pub(crate) fn peek_is_legacy_map_literal(&self) -> bool {
        let next = self.peek_nth_token(1);
        let after = self.peek_nth_token(2);
        matches!(next, TokenType::STRING(_)) && matches!(after, TokenType::COLON)
    }
}
