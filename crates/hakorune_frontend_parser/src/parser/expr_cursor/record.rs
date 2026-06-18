use super::ExprParserWithCursor;
use crate::ast::{ASTNode, Span};
use crate::parser::cursor::TokenCursor;
use crate::parser::ParseError;
use crate::tokenizer::TokenType;
use std::collections::BTreeSet;

impl ExprParserWithCursor {
    pub(crate) fn current_is_contextual_with(cursor: &TokenCursor) -> bool {
        matches!(&cursor.current().token_type, TokenType::IDENTIFIER(name) if name == "with")
    }

    pub(crate) fn current_lbrace_starts_record_literal(cursor: &TokenCursor) -> bool {
        if !cursor.match_token(&TokenType::LBRACE) {
            return false;
        }

        let mut offset = 1;
        while matches!(
            cursor.peek_nth_token(offset),
            TokenType::NEWLINE | TokenType::COMMA
        ) {
            offset += 1;
        }
        if matches!(cursor.peek_nth_token(offset), TokenType::RBRACE) {
            return true;
        }
        if !matches!(cursor.peek_nth_token(offset), TokenType::IDENTIFIER(_)) {
            return false;
        }
        offset += 1;
        while matches!(cursor.peek_nth_token(offset), TokenType::NEWLINE) {
            offset += 1;
        }
        matches!(
            cursor.peek_nth_token(offset),
            TokenType::COLON | TokenType::COMMA | TokenType::RBRACE | TokenType::NEWLINE
        )
    }

    pub(crate) fn parse_record_literal(
        cursor: &mut TokenCursor,
        record_type_name: String,
    ) -> Result<ASTNode, ParseError> {
        cursor.consume(TokenType::LBRACE)?;
        let mut fields = Vec::new();
        let mut seen = BTreeSet::new();

        while !cursor.match_token(&TokenType::RBRACE) && !cursor.is_at_end() {
            if cursor.match_token(&TokenType::COMMA) {
                cursor.advance();
                continue;
            }

            let field_name = match &cursor.current().token_type {
                TokenType::IDENTIFIER(name) => {
                    let name = name.clone();
                    cursor.advance();
                    name
                }
                other => {
                    let line = cursor.current().line;
                    return Err(ParseError::UnexpectedToken {
                        found: other.clone(),
                        expected: "[record-literal] field name".to_string(),
                        line,
                    });
                }
            };
            if !seen.insert(field_name.clone()) {
                let line = cursor.current().line;
                return Err(ParseError::UnexpectedToken {
                    found: TokenType::IDENTIFIER(field_name),
                    expected: "[record-literal] unique field name".to_string(),
                    line,
                });
            }
            let value = if cursor.match_token(&TokenType::COLON) {
                cursor.advance();
                Self::parse_expression(cursor)?
            } else {
                ASTNode::Variable {
                    name: field_name.clone(),
                    span: Span::unknown(),
                }
            };
            fields.push((field_name, value));

            if cursor.match_token(&TokenType::COMMA) {
                cursor.advance();
            }
        }

        cursor.consume(TokenType::RBRACE)?;
        Ok(ASTNode::RecordLiteral {
            record_type_name,
            fields,
            span: Span::unknown(),
        })
    }

    pub(crate) fn parse_record_update(
        cursor: &mut TokenCursor,
        base: ASTNode,
    ) -> Result<ASTNode, ParseError> {
        cursor.advance();
        cursor.consume(TokenType::LBRACE)?;
        let mut updates = Vec::new();
        let mut seen = BTreeSet::new();

        while !cursor.match_token(&TokenType::RBRACE) && !cursor.is_at_end() {
            if cursor.match_token(&TokenType::COMMA) || cursor.match_token(&TokenType::NEWLINE) {
                cursor.advance();
                continue;
            }

            let field_name = match &cursor.current().token_type {
                TokenType::IDENTIFIER(name) => {
                    let name = name.clone();
                    cursor.advance();
                    name
                }
                other => {
                    let line = cursor.current().line;
                    return Err(ParseError::UnexpectedToken {
                        found: other.clone(),
                        expected: "[record-update] field name".to_string(),
                        line,
                    });
                }
            };
            if !seen.insert(field_name.clone()) {
                let line = cursor.current().line;
                return Err(ParseError::UnexpectedToken {
                    found: TokenType::IDENTIFIER(field_name),
                    expected: "[record-update] unique field name".to_string(),
                    line,
                });
            }
            let value = if cursor.match_token(&TokenType::COLON) {
                cursor.advance();
                Self::parse_expression(cursor)?
            } else {
                ASTNode::Variable {
                    name: field_name.clone(),
                    span: Span::unknown(),
                }
            };
            updates.push((field_name, value));

            if cursor.match_token(&TokenType::COMMA) {
                cursor.advance();
            }
        }

        cursor.consume(TokenType::RBRACE)?;
        Ok(ASTNode::RecordUpdate {
            base: Box::new(base),
            updates,
            span: Span::unknown(),
        })
    }
}
