//! Interface box parser: `interface box Name { methods... }`
use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::HashMap;

/// Parse `interface box Name { methods... }` and return an AST BoxDeclaration.
/// Caller must be positioned at the beginning of `interface box`.
pub(crate) fn parse_interface_box(p: &mut NyashParser) -> Result<ASTNode, ParseError> {
    p.consume(TokenType::INTERFACE)?;
    p.consume(TokenType::BOX)?;
    let attrs = p.take_pending_runes_for_box()?;

    let name = if let TokenType::IDENTIFIER(name) = &p.current_token().token_type {
        let name = name.clone();
        p.advance();
        name
    } else {
        let line = p.current_token().line;
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "identifier".to_string(),
            line,
        });
    };

    // Generic type parameters: <T, U>
    let type_parameters = if p.match_token(&TokenType::LESS) {
        p.advance(); // consume '<'
        let mut params = Vec::new();
        while !p.match_token(&TokenType::GREATER) && !p.is_at_end() {
            if let TokenType::IDENTIFIER(param) = &p.current_token().token_type {
                params.push(param.clone());
                p.advance();
                if p.match_token(&TokenType::COMMA) {
                    p.advance();
                }
            } else {
                return Err(ParseError::UnexpectedToken {
                    found: p.current_token().token_type.clone(),
                    expected: "interface type parameter name".to_string(),
                    line: p.current_token().line,
                });
            }
        }
        p.consume(TokenType::GREATER)?;
        params
    } else {
        Vec::new()
    };

    p.consume(TokenType::LBRACE)?;

    let mut methods = HashMap::new();

    while !p.match_token(&TokenType::RBRACE) && !p.is_at_end() {
        if p.match_token(&TokenType::NEWLINE) || p.match_token(&TokenType::SEMICOLON) {
            p.advance();
            continue;
        }
        if p.maybe_parse_opt_annotation_noop(
            crate::parser::statements::helpers::AnnotationSite::Member,
        )? {
            continue;
        }
        if let TokenType::IDENTIFIER(method_name) = &p.current_token().token_type {
            let method_name = method_name.clone();
            p.advance();

            // インターフェースメソッドはシグネチャのみ
            if p.match_token(&TokenType::LPAREN) {
                let attrs = p.take_pending_runes_for_interface_method()?;
                p.advance(); // consume '('

                let param_decls =
                    crate::parser::common::params::parse_param_decl_list(p, "interface method")?;
                let params = crate::ast::ParamDecl::names(&param_decls);
                p.consume(TokenType::RPAREN)?;
                let return_type_name =
                    crate::parser::common::params::parse_optional_return_type_annotation(
                        p,
                        "interface method",
                    )?;

                // インターフェースメソッドは実装なし（空のbody）
                let method_decl = ASTNode::FunctionDeclaration {
                    name: method_name.clone(),
                    params,
                    param_decls,
                    return_type_name,
                    body: vec![],       // 空の実装
                    uses: vec![],

                    contracts: vec![],
                    is_static: false,   // インターフェースメソッドは通常静的でない
                    is_override: false, // デフォルトは非オーバーライド
                    attrs,
                    span: Span::unknown(),
                };
                methods.insert(method_name, method_decl);
            } else {
                let line = p.current_token().line;
                return Err(ParseError::UnexpectedToken {
                    found: p.current_token().token_type.clone(),
                    expected: "(".to_string(),
                    line,
                });
            }
        } else {
            let line = p.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "method name".to_string(),
                line,
            });
        }
    }

    p.consume(TokenType::RBRACE)?;

    Ok(ASTNode::BoxDeclaration {
        name,
        fields: vec![], // インターフェースはフィールドなし
        field_decls: vec![],
        public_fields: vec![],
        private_fields: vec![],
        methods,
        constructors: HashMap::new(), // インターフェースにコンストラクタなし
        init_fields: vec![],          // インターフェースにinitブロックなし
        weak_fields: vec![],          // インターフェースにweak fieldsなし
        delegates: vec![],
        invariants: vec![],
        transitions: vec![],
        is_interface: true,           // インターフェースフラグ
        is_record: false,
        extends: vec![], // Multi-delegation: None → vec![] として表現
        implements: vec![],
        type_parameters,
        is_sync: false,
        is_static: false,  // インターフェースは非static
        static_init: None, // インターフェースにstatic initなし
        attrs,
        span: Span::unknown(),
    })
}
