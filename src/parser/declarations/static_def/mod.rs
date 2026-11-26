//! Static Box Definition (staged split)

use crate::ast::{ASTNode, BoxMethodInventoryOrdinalV1, BoxMethodInventoryV1, Span};
use crate::parser::callable_parameter_source::static_box_source::{
    OpenParserStaticBoxSourceTransactionV1,
};
use crate::parser::common::ParserUtils;
use crate::parser::declarations::box_def::members::pending_method::PendingExplicitMethodV1;
use crate::parser::source_authority::{DirectExplicitMethodSinkV1, ExplicitMethodSink};
use crate::parser::source_path::SourceProgramCallablePathV1;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::HashMap;

pub mod header;
pub mod members;

struct StaticDirectMethodSinkV1<'a> {
    methods: &'a mut BoxMethodInventoryV1,
    source: &'a OpenParserStaticBoxSourceTransactionV1,
}

impl ExplicitMethodSink for StaticDirectMethodSinkV1<'_> {
    fn commit_explicit_method_at_current(
        &mut self,
        name: String,
        declaration: ASTNode,
        diagnostic_span: Span,
    ) -> Result<BoxMethodInventoryOrdinalV1, ParseError> {
        self.methods
            .commit_explicit_method_at_current(name, declaration, diagnostic_span)
    }
}

impl DirectExplicitMethodSinkV1 for StaticDirectMethodSinkV1<'_> {
    fn current_program_callable_path(&self) -> SourceProgramCallablePathV1 {
        self.source.current_program_callable_path()
    }
}

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
    let mut source_transaction = OpenParserStaticBoxSourceTransactionV1::open(
        p.source_invocation_brand(),
        source_path,
        name.clone(),
        box_span.line,
    );

    let mut methods = BoxMethodInventoryV1::empty();
    let constructors = HashMap::new();
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
        commit_pending_static_method(
            p,
            &mut pending_method,
            &mut methods,
            &mut source_transaction,
        )?;
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

        match p.current_token().token_type {
            TokenType::INIT | TokenType::WEAK => return Err(static_box_state(p.current_token().line)),
            TokenType::STATIC => {
                let mut offset = 1;
                while matches!(p.peek_nth_token(offset), TokenType::NEWLINE) {
                    offset += 1;
                }
                if matches!(p.peek_nth_token(offset), TokenType::LBRACE) {
                    return Err(static_box_state(p.current_token().line));
                }
                if !crate::parser::env::parser_static_init_strict_enabled()
                    || !matches!(p.peek_token(), TokenType::BOX | TokenType::FUNCTION)
                {
                    p.advance();
                    p.consume(TokenType::LBRACE)?;
                } else if crate::parser::env::parser_static_seam_break_on_static_enabled() {
                    if crate::parser::env::cli_verbose_enabled() {
                        crate::parser::log::debug("[parser][static-box][seam] encountered 'static' inside static box; breaking (compat shim)");
                    }
                    break;
                }
            }
            _ => {}
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
                let mut field_or_method = field_or_method.clone();
                p.advance();
                if field_or_method == "method" {
                    if let Some(name) = members::take_method_modifier_name(p) {
                        field_or_method = name;
                    }
                }
                pending_method = Some(members::parse_method(p, field_or_method, declaration_span)?);
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

    commit_pending_static_method(
        p,
        &mut pending_method,
        &mut methods,
        &mut source_transaction,
    )?;

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

    let prepared_static_source = source_transaction.finish().map_err(static_source_issue)?;
    p.register_prepared_static_box_source(prepared_static_source)?;

    if crate::parser::env::parser_static_trace_enabled() {
        crate::parser::log::debug(&format!(
            "[parser][static-box] successfully closed static box '{}'",
            name
        ));
    }

    Ok(ASTNode::BoxDeclaration {
        name,
        fields: vec![],
        field_decls: vec![],
        public_fields: vec![],
        private_fields: vec![],
        methods,
        constructors,
        init_fields: vec![],
        weak_fields: vec![],
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
        static_init: None,
        attrs,
        span: box_span,
    })
}

fn static_box_state(line: usize) -> ParseError {
    ParseError::GrammarContract {
        stable_reject_tag: "decl/static-box-state",
        detail: "move fields and initialization into an ordinary box".to_owned(),
        line,
    }
}

fn commit_pending_static_method(
    parser: &mut NyashParser,
    pending: &mut Option<PendingExplicitMethodV1>,
    methods: &mut BoxMethodInventoryV1,
    source: &mut OpenParserStaticBoxSourceTransactionV1,
) -> Result<(), ParseError> {
    let Some(method) = pending.take() else {
        return Ok(());
    };
    let source_site = crate::parser::source_authority::SourceBoxMethodSiteV1::Direct {
        member: source.current_member_site(),
    };
    let committed = method.commit_direct(&mut StaticDirectMethodSinkV1 { methods, source })?;
    let (inventory_ordinal, diagnostic_name, parameter_source, callable_identity) =
        parser.issue_committed_static_box_method(committed)?;
    if let Some(parameters) = parameter_source {
        parser.commit_callable_parameter_source(
            source_site.clone(),
            inventory_ordinal,
            callable_identity.clone(),
            crate::parser::callable_parameter_source::ParserCallableDeclarationKindV1::StaticBoxMethod,
            diagnostic_name,
            parameters,
        )?;
    }
    source
        .commit_direct_method(source_site, callable_identity)
        .map_err(static_source_issue)
}

fn static_source_issue(
    error: crate::parser::callable_parameter_source::static_box_source::ParserStaticBoxSourceIssueV1,
) -> ParseError {
    ParseError::GrammarContract {
        stable_reject_tag: "parser/static-box-source",
        detail: format!("static Box source coverage rejected: {error:?}"),
        line: 0,
    }
}
