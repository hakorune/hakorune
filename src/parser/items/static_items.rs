/*!
 * Static declaration parsing
 * Handles both static functions and static boxes
 */

use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    /// 静的宣言をパース - 🔥 static function / static box 記法  
    pub fn parse_static_declaration(&mut self) -> Result<ASTNode, ParseError> {
        self.consume(TokenType::STATIC)?;

        // 次のトークンで分岐: function か box か
        match &self.current_token().token_type {
            TokenType::FUNCTION => self.parse_static_function(),
            TokenType::BOX => crate::parser::declarations::static_def::parse_static_box(self),
            TokenType::IDENTIFIER(word) if word == "const" => self.parse_static_const_table(),
            _ => {
                let line = self.current_token().line;
                Err(ParseError::UnexpectedToken {
                    found: self.current_token().token_type.clone(),
                    expected: "function, box, or const after static".to_string(),
                    line,
                })
            }
        }
    }

    /// Backend-private static readonly table declaration.
    ///
    /// Accepted M11b-decl shape:
    /// `static const NAME: u16[] = [1, 2, 3]`
    fn parse_static_const_table(&mut self) -> Result<ASTNode, ParseError> {
        self.advance(); // consume identifier `const`

        let name = if let TokenType::IDENTIFIER(name) = &self.current_token().token_type {
            let name = name.clone();
            self.advance();
            name
        } else {
            let line = self.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "[static-const/declaration] table name".to_string(),
                line,
            });
        };

        self.consume(TokenType::COLON)?;
        let type_text =
            crate::parser::common::type_refs::parse_type_ref_text(self, "static const table")?;
        if type_text != "u16[]" {
            let line = self.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "[static-const/unsupported-element] u16[]".to_string(),
                line,
            });
        }

        self.consume(TokenType::ASSIGN)?;
        self.consume(TokenType::LBRACK)?;
        let mut values = Vec::new();
        while !self.match_token(&TokenType::RBRACK) && !self.is_at_end() {
            let value = if let TokenType::NUMBER(value) = &self.current_token().token_type {
                *value
            } else {
                let line = self.current_token().line;
                return Err(ParseError::UnexpectedToken {
                    found: self.current_token().token_type.clone(),
                    expected: "[static-const/unsupported-initializer] integer literal".to_string(),
                    line,
                });
            };
            if !(0..=u16::MAX as i64).contains(&value) {
                let line = self.current_token().line;
                return Err(ParseError::UnexpectedToken {
                    found: self.current_token().token_type.clone(),
                    expected: "[static-const/value-out-of-range] 0..65535".to_string(),
                    line,
                });
            }
            values.push(value as u64);
            self.advance();

            if self.match_token(&TokenType::COMMA) {
                self.advance();
                continue;
            }
            if self.match_token(&TokenType::RBRACK) {
                break;
            }
            let line = self.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "[static-const/declaration] comma or closing bracket".to_string(),
                line,
            });
        }
        self.consume(TokenType::RBRACK)?;

        Ok(ASTNode::StaticConstTable {
            name,
            element_type: "u16".to_string(),
            values,
            span: Span::unknown(),
        })
    }

    /// 静的関数宣言をパース - static function Name() { ... }
    fn parse_static_function(&mut self) -> Result<ASTNode, ParseError> {
        self.consume(TokenType::FUNCTION)?;
        let attrs = self.take_pending_runes_for_static_function()?;

        // 関数名を取得（Box名.関数名の形式をサポート）
        let name = if let TokenType::IDENTIFIER(first_part) = &self.current_token().token_type {
            let mut full_name = first_part.clone();
            self.advance();

            // ドット記法をチェック（例：Math.min）
            if self.match_token(&TokenType::DOT) {
                self.advance(); // DOTを消費

                if let TokenType::IDENTIFIER(method_name) = &self.current_token().token_type {
                    full_name = format!("{}.{}", full_name, method_name);
                    self.advance();
                } else {
                    let line = self.current_token().line;
                    return Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: "method name after dot".to_string(),
                        line,
                    });
                }
            }

            full_name
        } else {
            let line = self.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "static function name".to_string(),
                line,
            });
        };

        // パラメータリストをパース
        self.consume(TokenType::LPAREN)?;

        // Phase 285A1.5: Use shared helper to prevent parser hangs on unsupported type annotations
        let params = crate::parser::common::params::parse_param_name_list(self, "static method")?;

        self.consume(TokenType::RPAREN)?;

        // 関数本体をパース（共通ブロックヘルパー）
        let body = self.parse_block_statements()?;

        Ok(ASTNode::FunctionDeclaration {
            name,
            params,
            body,
            is_static: true,    // 🔥 静的関数フラグを設定
            is_override: false, // デフォルトは非オーバーライド
            attrs,
            span: Span::unknown(),
        })
    }
}
