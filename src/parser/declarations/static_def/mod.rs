//! Static Box Definition (staged split)
#![allow(dead_code)]

use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::HashMap;

pub mod header;
pub mod members;
pub mod validators;

/// Parse static box declaration: static box Name { ... }
pub fn parse_static_box(p: &mut NyashParser) -> Result<ASTNode, ParseError> {
    p.consume(TokenType::BOX)?;
    let attrs = p.take_pending_runes_for_box()?;
    let (name, type_parameters, extends, implements) = header::parse_static_header(p)?;

    p.consume(TokenType::LBRACE)?;

    let mut fields = Vec::new();
    let mut methods = HashMap::new();
    let constructors = HashMap::new();
    let mut init_fields = Vec::new();
    let mut weak_fields = Vec::new(); // 🔗 Track weak fields for static box
    let mut static_init: Option<Vec<ASTNode>> = None;

    // Track last inserted method name to allow postfix catch/cleanup fallback parsing
    let mut last_method_name: Option<String> = None;
    while !p.match_token(&TokenType::RBRACE) && !p.is_at_end() {
        // Tolerate blank lines between members
        while p.match_token(&TokenType::NEWLINE) {
            p.advance();
        }
        if p.maybe_parse_opt_annotation_noop()? {
            continue;
        }
        let trace = std::env::var("NYASH_PARSER_TRACE_STATIC").ok().as_deref() == Some("1");
        if trace {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[parser][static-box] loop token={:?}",
                p.current_token().token_type
            ));
        }

        // Fallback: method-level postfix catch/cleanup immediately following a method
        if crate::parser::declarations::box_def::members::postfix::try_parse_method_postfix_after_last_method(
            p, &mut methods, &last_method_name,
        )? { continue; }

        // RBRACEに到達していればループを抜ける
        if p.match_token(&TokenType::RBRACE) {
            break;
        }

        // 🔥 static 初期化子の処理（厳密ゲート互換）
        if let Some(body) = members::parse_static_initializer_if_any(p)? {
            p.ensure_no_pending_runes("static initializer")?;
            static_init = Some(body);
            continue;
        } else if p.match_token(&TokenType::STATIC) {
            // 互換用の暫定ガード（既定OFF）: using テキスト結合の継ぎ目で誤って 'static' が入った場合に
            // ループを抜けて外側の '}' 消費に委ねる。既定では無効化し、文脈エラーとして扱う。
            if std::env::var("NYASH_PARSER_SEAM_BREAK_ON_STATIC")
                .ok()
                .as_deref()
                == Some("1")
            {
                if std::env::var("NYASH_CLI_VERBOSE").ok().as_deref() == Some("1") {
                    crate::runtime::get_global_ring0().log.debug("[parser][static-box][seam] encountered 'static' inside static box; breaking (compat shim)");
                }
                break;
            }
        }

        // initブロックの処理（共通ヘルパに委譲）
        if crate::parser::declarations::box_def::members::fields::parse_init_block_if_any(
            p,
            &mut init_fields,
            &mut weak_fields,
        )? {
            p.ensure_no_pending_runes("init block")?;
            continue;
        }

        // 🔧 Safety valve: if we encounter statement keywords (LOCAL, RETURN, etc.) at member level,
        // it means we've likely exited a method body prematurely. Break to close the static box.
        match p.current_token().token_type {
            TokenType::LOCAL
            | TokenType::RETURN
            | TokenType::IF
            | TokenType::LOOP
            | TokenType::BREAK
            | TokenType::CONTINUE
            | TokenType::PRINT => {
                if std::env::var("NYASH_CLI_VERBOSE").ok().as_deref() == Some("1") {
                    crate::runtime::get_global_ring0().log.debug(&format!("[parser][static-box][safety] encountered statement keyword {:?} at member level (line {}); assuming premature method body exit",
                             p.current_token().token_type, p.current_token().line));
                }
                break;
            }
            _ => {}
        }

        // Seam/robustness: tolerate stray tokens between members (text-merge or prelude seams)
        // NYASH_PARSER_SEAM_TOLERANT=1 (dev/ci既定): ASSIGN を継ぎ目として箱を閉じる（break）
        // NYASH_PARSER_SEAM_TOLERANT=0 (prod既定): ASSIGN でエラー（Fail-Fast）
        match &p.current_token().token_type {
            TokenType::SEMICOLON | TokenType::NEWLINE => {
                p.advance();
                continue;
            }
            // If we encounter a bare '=' at member level, treat as seam boundary (gated by flag)
            // Resynchronize by advancing to the closing '}' so outer logic can consume it.
            TokenType::ASSIGN => {
                let seam_tolerant =
                    std::env::var("NYASH_PARSER_SEAM_TOLERANT").ok().as_deref() == Some("1");
                if seam_tolerant {
                    if std::env::var("NYASH_CLI_VERBOSE").ok().as_deref() == Some("1") {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[parser][static-box][seam] encountered ASSIGN at member level (line {}); treating as seam boundary (closing box)",
                            p.current_token().line
                        ));
                    }
                    // advance until '}' or EOF
                    while !p.is_at_end() && !p.match_token(&TokenType::RBRACE) {
                        p.advance();
                    }
                    // do not consume RBRACE here; let trailing logic handle it
                    break; // 継ぎ目として箱を閉じる
                } else {
                    // Prod: strict mode, fail fast on unexpected ASSIGN
                    return Err(ParseError::UnexpectedToken {
                        expected: "method or field name".to_string(),
                        found: p.current_token().token_type.clone(),
                        line: p.current_token().line,
                    });
                }
            }
            TokenType::IDENTIFIER(field_or_method) => {
                let field_or_method = field_or_method.clone();
                p.advance();
                members::try_parse_method_or_field(
                    p,
                    field_or_method,
                    &mut methods,
                    &mut fields,
                    &mut last_method_name,
                )?;
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "method or field name".to_string(),
                    found: p.current_token().token_type.clone(),
                    line: p.current_token().line,
                });
            }
        }
    }

    // Tolerate trailing NEWLINE(s) before the closing '}' of the static box
    while p.match_token(&TokenType::NEWLINE) {
        p.advance();
    }
    if std::env::var("NYASH_PARSER_TRACE_STATIC").ok().as_deref() == Some("1") {
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[parser][static-box] closing '}}' at token={:?}",
            p.current_token().token_type
        ));
    }

    // Consume the closing RBRACE of the static box
    p.consume(TokenType::RBRACE)?;

    if std::env::var("NYASH_PARSER_TRACE_STATIC").ok().as_deref() == Some("1") {
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[parser][static-box] successfully closed static box '{}'",
            name
        ));
    }

    // 🔥 Static初期化ブロックから依存関係を抽出
    if let Some(ref init_stmts) = static_init {
        let dependencies = p.extract_dependencies_from_statements(init_stmts);
        p.static_box_dependencies.insert(name.clone(), dependencies);
    } else {
        p.static_box_dependencies
            .insert(name.clone(), std::collections::HashSet::new());
    }

    Ok(ASTNode::BoxDeclaration {
        name,
        fields,
        public_fields: vec![],
        private_fields: vec![],
        methods,
        constructors,
        init_fields,
        weak_fields, // 🔗 Add weak fields to static box construction
        is_interface: false,
        extends,
        implements,
        type_parameters,
        is_static: true, // 🔥 static boxフラグを設定
        static_init,     // 🔥 static初期化ブロック
        attrs,
        span: Span::unknown(),
    })
}
