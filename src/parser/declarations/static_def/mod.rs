//! Static Box Definition (staged split)

use crate::ast::{ASTNode, BoxMethodInventoryV1, FieldDecl};
use crate::parser::common::ParserUtils;
use crate::parser::declarations::box_def::members::pending_method::PendingExplicitMethodV1;
use crate::parser::source_member_cursor::ParserBoxMemberSourceCursorV1;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::HashMap;

pub mod header;
pub mod members;

/// Parse static box declaration: static box Name { ... }
pub fn parse_static_box(p: &mut NyashParser) -> Result<ASTNode, ParseError> {
    let box_span = p.current_span();
    p.consume(TokenType::BOX)?;
    let attrs = p.take_pending_runes_for_box()?;
    let (name, type_parameters, extends, implements) = header::parse_static_header(p)?;

    p.consume(TokenType::LBRACE)?;
    let source_path =
        p.active_source_declaration_path()
            .cloned()
            .ok_or_else(|| ParseError::BuildCfg {
                message: "static Box member cursor requires an active parser source path"
                    .to_owned(),
                line: p.current_token().line,
            })?;
    let mut source_cursor =
        ParserBoxMemberSourceCursorV1::open_with_path(p.source_invocation_brand(), source_path);

    let mut fields = Vec::new();
    let mut methods = BoxMethodInventoryV1::empty();
    let constructors = HashMap::new();
    let mut init_fields = Vec::new();
    let mut weak_fields = Vec::new(); // 🔗 Track weak fields for static box
    let mut static_init: Option<Vec<ASTNode>> = None;

    let mut pending_method: Option<PendingExplicitMethodV1> = None;
    while !p.match_token(&TokenType::RBRACE) && !p.is_at_end() {
        // Tolerate blank lines between members
        while p.match_token(&TokenType::NEWLINE) {
            p.advance();
        }
        if let Some(method) = pending_method.as_mut() {
            if method.try_apply_postfix(p)? {
                continue;
            }
        }
        commit_pending_static_method(p, &mut pending_method, &mut methods, &mut source_cursor)?;
        if p.maybe_parse_opt_annotation_noop(
            crate::parser::statements::helpers::AnnotationSite::Member,
        )? {
            continue;
        }
        let trace = crate::parser::env::parser_static_trace_enabled();
        if trace {
            crate::parser::log::debug(&format!(
                "[parser][static-box] loop token={:?}",
                p.current_token().token_type
            ));
        }

        // RBRACEに到達していればループを抜ける
        if p.match_token(&TokenType::RBRACE) {
            break;
        }

        // 🔥 static 初期化子の処理（厳密ゲート互換）
        if let Some(body) = members::parse_static_initializer_if_any(p)? {
            p.ensure_no_pending_runes("static initializer")?;
            static_init = Some(body);
            finish_static_source_member(&mut source_cursor)?;
            continue;
        } else if p.match_token(&TokenType::STATIC) {
            // 互換用の暫定ガード（既定OFF）: using テキスト結合の継ぎ目で誤って 'static' が入った場合に
            // ループを抜けて外側の '}' 消費に委ねる。既定では無効化し、文脈エラーとして扱う。
            if crate::parser::env::parser_static_seam_break_on_static_enabled() {
                if crate::parser::env::cli_verbose_enabled() {
                    crate::parser::log::debug("[parser][static-box][seam] encountered 'static' inside static box; breaking (compat shim)");
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
            finish_static_source_member(&mut source_cursor)?;
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
                if crate::parser::env::cli_verbose_enabled() {
                    crate::parser::log::debug(&format!("[parser][static-box][safety] encountered statement keyword {:?} at member level (line {}); assuming premature method body exit",
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
                let seam_tolerant = crate::parser::env::parser_static_seam_tolerant_enabled();
                if seam_tolerant {
                    if crate::parser::env::cli_verbose_enabled() {
                        crate::parser::log::debug(&format!(
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
                let declaration_span = p.current_span();
                let field_or_method = field_or_method.clone();
                p.advance();
                match members::try_parse_method_or_field(p, field_or_method, declaration_span)? {
                    members::ParsedStaticMemberV1::Field(field) => {
                        fields.push(field);
                        finish_static_source_member(&mut source_cursor)?;
                    }
                    members::ParsedStaticMemberV1::Method(method) => pending_method = Some(method),
                }
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

    commit_pending_static_method(p, &mut pending_method, &mut methods, &mut source_cursor)?;

    // Tolerate trailing NEWLINE(s) before the closing '}' of the static box
    while p.match_token(&TokenType::NEWLINE) {
        p.advance();
    }
    if crate::parser::env::parser_static_trace_enabled() {
        crate::parser::log::debug(&format!(
            "[parser][static-box] closing '}}' at token={:?}",
            p.current_token().token_type
        ));
    }

    // Consume the closing RBRACE of the static box
    p.consume(TokenType::RBRACE)?;

    if crate::parser::env::parser_static_trace_enabled() {
        crate::parser::log::debug(&format!(
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

    let field_decls = fields
        .iter()
        .cloned()
        .map(|name| FieldDecl {
            is_weak: weak_fields.contains(&name),
            name,
            declared_type_name: None,
            default_value: None,
        })
        .collect();

    Ok(ASTNode::BoxDeclaration {
        name,
        fields,
        field_decls,
        public_fields: vec![],
        private_fields: vec![],
        methods,
        constructors,
        init_fields,
        weak_fields, // 🔗 Add weak fields to static box construction
        delegates: vec![],
        invariants: vec![],
        transitions: vec![],
        is_interface: false,
        is_record: false,
        extends,
        implements,
        type_parameters,
        is_sync: false,
        is_static: true, // 🔥 static boxフラグを設定
        static_init,     // 🔥 static初期化ブロック
        attrs,
        span: box_span,
    })
}

fn finish_static_source_member(
    cursor: &mut ParserBoxMemberSourceCursorV1,
) -> Result<(), ParseError> {
    cursor
        .finish_member()
        .map_err(|error| ParseError::BuildCfg {
            message: format!("static Box member source cursor failed: {error:?}"),
            line: 0,
        })
}

fn commit_pending_static_method(
    parser: &mut NyashParser,
    pending: &mut Option<PendingExplicitMethodV1>,
    methods: &mut BoxMethodInventoryV1,
    cursor: &mut ParserBoxMemberSourceCursorV1,
) -> Result<(), ParseError> {
    let Some(method) = pending.take() else {
        return Ok(());
    };
    let source_site = crate::parser::source_authority::SourceBoxMethodSiteV1::Direct {
        member: cursor.current_member_site(),
    };
    let committed = method.commit(methods)?;
    let Some((diagnostic_name, parameters)) = committed.into_parameter_source() else {
        return Err(ParseError::GrammarContract {
            stable_reject_tag: "parser/callable-parameter-source",
            detail: "direct static method omitted its parameter source product".to_owned(),
            line: 0,
        });
    };
    parser.commit_callable_parameter_source(
        source_site,
        crate::parser::callable_parameter_source::ParserCallableDeclarationKindV1::StaticBoxMethod,
        diagnostic_name,
        parameters,
    )?;
    finish_static_source_member(cursor)
}
