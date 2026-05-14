/*!
 * Function declaration parsing
 */

use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    /// function宣言をパース: function name(params) { body }
    pub fn parse_function_declaration(&mut self) -> Result<ASTNode, ParseError> {
        self.consume(TokenType::FUNCTION)?;
        let attrs = self.take_pending_runes_for_free_function()?;

        // 関数名を取得
        let name = if let TokenType::IDENTIFIER(name) = &self.current_token().token_type {
            let name = name.clone();
            self.advance();
            name
        } else {
            let line = self.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "function name".to_string(),
                line,
            });
        };

        // パラメータリストをパース
        self.consume(TokenType::LPAREN)?;

        let param_decls = crate::parser::common::params::parse_param_decl_list(self, "function")?;
        let params = crate::ast::ParamDecl::names(&param_decls);

        self.consume(TokenType::RPAREN)?;
        let return_type_name =
            crate::parser::common::params::parse_optional_return_type_annotation(self, "function")?;
        let contracts = self.parse_contract_clauses_until_body()?;

        // 関数本体をパース（共通ブロックヘルパー）
        let body = self.parse_block_statements()?;

        Ok(ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            return_type_name,
            body,
            contracts,
            is_static: false,   // 通常の関数は静的でない
            is_override: false, // デフォルトは非オーバーライド
            attrs,
            span: Span::unknown(),
        })
    }
}
