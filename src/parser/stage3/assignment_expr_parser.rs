/*!
 * Phase 152-A: Grouped Assignment Expression Parser (箱化モジュール化)
 *
 * Stage-3 構文: (x = expr) を expression として受け入れる
 * 仕様:
 * - `x = expr` は statement のまま（変更なし）
 * - `(x = expr)` のみ expression として受け入れる
 * - 値・型は右辺と同じ
 *
 * 責務:
 * 1. Stage-3 gate 確認
 * 2. `(` IDENT `=` expr `)` パターン検出
 * 3. GroupedAssignmentExpr AST ノード生成
 */

use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    /// Try to parse grouped assignment expression: (x = expr)
    ///
    /// Returns Some(ASTNode) if pattern matches and Stage-3 is enabled,
    /// None otherwise to allow normal grouped expression fallback.
    ///
    /// This function is designed for 1-line delegation from primary.rs:
    /// ```
    /// if let Some(assignment) = self.try_parse_grouped_assignment()? {
    ///     return Ok(assignment);
    /// }
    /// ```
    pub(crate) fn try_parse_grouped_assignment(&mut self) -> Result<Option<ASTNode>, ParseError> {
        // Stage-3 gate check
        if !crate::config::env::parser_stage3_enabled() {
            return Ok(None);
        }

        // Look ahead pattern check: '(' IDENT '=' ...
        if !self.is_grouped_assignment_pattern() {
            return Ok(None);
        }

        // Parse: '(' IDENT '=' expr ')'
        self.consume(TokenType::LPAREN)?;

        let ident = match &self.current_token().token_type {
            TokenType::IDENTIFIER(name) => {
                let var_name = name.clone();
                self.advance();
                var_name
            }
            _ => {
                let line = self.current_token().line;
                return Err(ParseError::UnexpectedToken {
                    found: self.current_token().token_type.clone(),
                    expected: "identifier in grouped assignment".to_string(),
                    line,
                });
            }
        };

        self.consume(TokenType::ASSIGN)?;

        let rhs = self.parse_expression()?;

        self.consume(TokenType::RPAREN)?;

        Ok(Some(ASTNode::GroupedAssignmentExpr {
            lhs: ident,
            rhs: Box::new(rhs),
            span: Span::unknown(),
        }))
    }

    /// Check if current token position matches grouped assignment pattern
    ///
    /// Pattern: '(' IDENT '=' ...
    fn is_grouped_assignment_pattern(&self) -> bool {
        // Current token should be '('
        if !self.match_token(&TokenType::LPAREN) {
            return false;
        }

        // Next token should be IDENTIFIER
        if self.current() + 1 >= self.tokens().len() {
            return false;
        }
        if !matches!(
            &self.tokens()[self.current() + 1].token_type,
            TokenType::IDENTIFIER(_)
        ) {
            return false;
        }

        // Token after that should be '='
        if self.current() + 2 >= self.tokens().len() {
            return false;
        }
        matches!(
            &self.tokens()[self.current() + 2].token_type,
            TokenType::ASSIGN
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::NyashTokenizer;

    #[test]
    fn test_grouped_assignment_simple() {
        std::env::set_var("NYASH_FEATURES", "stage3");

        let input = "local y = (x = 42)";
        let mut tokenizer = NyashTokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = NyashParser::new(tokens);

        // Skip 'local' and 'y' and '='
        parser.advance(); // local
        parser.advance(); // y
        parser.advance(); // =

        let result = parser.try_parse_grouped_assignment().unwrap();
        assert!(result.is_some());

        if let Some(ASTNode::GroupedAssignmentExpr { lhs, rhs, .. }) = result {
            assert_eq!(lhs, "x");
            // rhs should be Literal(42)
            assert!(matches!(*rhs, ASTNode::Literal { .. }));
        } else {
            panic!("Expected GroupedAssignmentExpr");
        }
    }

    // TODO: These tests need to be updated to use the new tokenizer API
    // #[test]
    // fn test_grouped_assignment_pattern_detection() {
    //     std::env::set_var("NYASH_FEATURES", "stage3");
    //
    //     // Positive case: (x = expr)
    //     let tokens = Tokenizer::tokenize("(x = 42)").unwrap();
    //     let parser = NyashParser::new(tokens);
    //     assert!(parser.is_grouped_assignment_pattern());
    //
    //     // Negative case: (42) - not an identifier
    //     let tokens = Tokenizer::tokenize("(42)").unwrap();
    //     let parser = NyashParser::new(tokens);
    //     assert!(!parser.is_grouped_assignment_pattern());
    //
    //     // Negative case: x = 42 - no parenthesis
    //     let tokens = Tokenizer::tokenize("x = 42").unwrap();
    //     let parser = NyashParser::new(tokens);
    //     assert!(!parser.is_grouped_assignment_pattern());
    // }
    //
    // #[test]
    // fn test_stage3_gate_off() {
    //     std::env::remove_var("NYASH_FEATURES");
    //     std::env::remove_var("NYASH_PARSER_STAGE3");
    //     std::env::remove_var("HAKO_PARSER_STAGE3");
    //
    //     let input = "(x = 42)";
    //     let tokens = Tokenizer::tokenize(input).unwrap();
    //     let mut parser = NyashParser::new(tokens);
    //
    //     let result = parser.try_parse_grouped_assignment().unwrap();
    //     assert!(result.is_none()); // Should return None when Stage-3 is off
    // }
}
