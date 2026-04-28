//! Box Definition Parser Module
//!
//! Box宣言（box, interface box, static box）の解析を担当
//! Nyashの中核概念「Everything is Box」を実現する重要モジュール

use crate::ast::{ASTNode, FieldDecl, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::HashMap;

pub mod header;
pub mod interface;
pub mod members;
pub mod validators;

/// Thin wrappers to keep the main loop tidy (behavior-preserving)
fn box_try_block_first_property(
    p: &mut NyashParser,
    methods: &mut HashMap<String, ASTNode>,
    birth_once_props: &mut Vec<String>,
) -> Result<bool, ParseError> {
    if !p.match_token(&TokenType::LBRACE) {
        return Ok(false);
    }
    p.ensure_no_pending_runes("block-first property")?;
    members::properties::try_parse_block_first_property(p, methods, birth_once_props)
}

fn box_try_method_postfix_after_last(
    p: &mut NyashParser,
    methods: &mut HashMap<String, ASTNode>,
    last_method_name: &Option<String>,
) -> Result<bool, ParseError> {
    if last_method_name.is_none()
        || !(p.match_token(&TokenType::CATCH) || p.match_token(&TokenType::CLEANUP))
    {
        return Ok(false);
    }
    p.ensure_no_pending_runes("method postfix")?;
    members::postfix::try_parse_method_postfix_after_last_method(p, methods, last_method_name)
}

fn box_try_init_block(
    p: &mut NyashParser,
    init_fields: &mut Vec<String>,
    weak_fields: &mut Vec<String>,
) -> Result<bool, ParseError> {
    if !(p.match_token(&TokenType::INIT) && p.peek_token() != &TokenType::LPAREN) {
        return Ok(false);
    }
    p.ensure_no_pending_runes("init block")?;
    members::fields::parse_init_block_if_any(p, init_fields, weak_fields)
}

fn box_try_constructor(
    p: &mut NyashParser,
    is_override: bool,
    constructors: &mut HashMap<String, ASTNode>,
) -> Result<bool, ParseError> {
    if let Some((key, node)) = members::constructors::try_parse_constructor(p, is_override)? {
        let mut node = node;
        p.attach_pending_runes_to_declaration(&mut node)?;
        constructors.insert(key, node);
        return Ok(true);
    }
    Ok(false)
}

fn box_try_visibility(
    p: &mut NyashParser,
    visibility: &str,
    methods: &mut HashMap<String, ASTNode>,
    fields: &mut Vec<String>,
    field_decls: &mut Vec<FieldDecl>,
    public_fields: &mut Vec<String>,
    private_fields: &mut Vec<String>,
    last_method_name: &mut Option<String>,
    weak_fields: &mut Vec<String>,
) -> Result<bool, ParseError> {
    if visibility != "public" && visibility != "private" {
        return Ok(false);
    }
    p.ensure_no_pending_runes("visibility field/property")?;
    members::fields::try_parse_visibility_block_or_single(
        p,
        visibility,
        methods,
        fields,
        field_decls,
        public_fields,
        private_fields,
        last_method_name,
        weak_fields,
    )
}

/// Parse either a method or a header-first field/property starting with `name`.
/// Updates `methods`/`fields` and `last_method_name` as appropriate.
fn box_try_method_or_field(
    p: &mut NyashParser,
    name: String,
    is_override: bool,
    methods: &mut HashMap<String, ASTNode>,
    fields: &mut Vec<String>,
    field_decls: &mut Vec<FieldDecl>,
    last_method_name: &mut Option<String>,
    weak_fields: &mut Vec<String>,
) -> Result<bool, ParseError> {
    if let Some(method) = members::methods::try_parse_method(p, name.clone(), is_override)? {
        let mut method = method;
        p.attach_pending_runes_to_declaration(&mut method)?;
        *last_method_name = Some(name.clone());
        methods.insert(name, method);
        return Ok(true);
    }
    // Fallback: header-first field/property (computed/once/birth_once handled inside)
    let parsed = members::fields::try_parse_header_first_field_or_property(
        p,
        name,
        methods,
        fields,
        field_decls,
        weak_fields,
        false,
    )?;
    if parsed {
        p.ensure_no_pending_runes("field/property")?;
    }
    Ok(parsed)
}

/// box宣言をパース: box Name { fields... methods... }
pub fn parse_box_declaration(p: &mut NyashParser) -> Result<ASTNode, ParseError> {
    // Accept either 'box' or 'flow' (flow is syntactic sugar for static box)
    if !p.match_token(&TokenType::BOX) && !p.match_token(&TokenType::FLOW) {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "'box' or 'flow'".to_string(),
            line: p.current_token().line,
        });
    }
    p.advance(); // consume BOX or FLOW
    let attrs = p.take_pending_runes_for_box()?;
    let (name, type_parameters, extends, implements) = header::parse_header(p)?;

    p.consume(TokenType::LBRACE)?;

    let mut fields = Vec::new();
    let mut field_decls = Vec::new();
    let mut methods = HashMap::new();
    let mut public_fields: Vec<String> = Vec::new();
    let mut private_fields: Vec<String> = Vec::new();
    let mut constructors = HashMap::new();
    let mut init_fields = Vec::new();
    let mut weak_fields = Vec::new(); // 🔗 Track weak fields
                                      // Track birth_once properties for constructor prologue emission.
    let mut birth_once_props: Vec<String> = Vec::new();

    let mut last_method_name: Option<String> = None;
    while !p.match_token(&TokenType::RBRACE) && !p.is_at_end() {
        if p.maybe_parse_opt_annotation_noop(
            crate::parser::statements::helpers::AnnotationSite::Member,
        )? {
            continue;
        }

        // 分類（段階移行用の観測）: 将来の分岐移譲のための前処理
        if crate::config::env::parser_stage3_enabled() {
            if let Ok(kind) = members::common::classify_member(p) {
                let _ = kind; // 現段階では観測のみ（無副作用）
            }
        }

        // nyashモード（block-first）: { body } as (once|birth_once)? name : Type
        if box_try_block_first_property(p, &mut methods, &mut birth_once_props)? {
            continue;
        }

        // Fallback: method-level postfix catch/cleanup after a method (non-static box)
        if box_try_method_postfix_after_last(p, &mut methods, &last_method_name)? {
            continue;
        }

        // RBRACEに到達していればループを抜ける
        if p.match_token(&TokenType::RBRACE) {
            break;
        }

        // initブロックの処理（initメソッドではない場合のみ）
        if box_try_init_block(p, &mut init_fields, &mut weak_fields)? {
            continue;
        }

        // overrideキーワードをチェック
        let mut is_override = false;
        if p.match_token(&TokenType::OVERRIDE) {
            is_override = true;
            p.advance();
        }

        // constructor parsing moved to members::constructors
        if box_try_constructor(p, is_override, &mut constructors)? {
            // constructor parsing returns an AST node and is a declaration target
            continue;
        }

        // 🚨 birth()統一システム: Box名コンストラクタ無効化
        validators::forbid_box_named_constructor(p, &name)?;

        // Phase 285A1.3: Delegate weak field parsing to unified fields.rs logic
        if p.match_token(&TokenType::WEAK) {
            p.ensure_no_pending_runes("weak field")?;
            p.advance(); // consume WEAK
            if let TokenType::IDENTIFIER(field_name) = &p.current_token().token_type {
                let field_name = field_name.clone();
                p.advance();
                // Unified weak field parsing (Phase 285A1.3)
                members::fields::parse_weak_field(
                    p,
                    field_name,
                    &mut methods,
                    &mut fields,
                    &mut field_decls,
                    &mut weak_fields,
                )?;
                continue;
            } else {
                return Err(ParseError::UnexpectedToken {
                    expected: "field name after 'weak'".to_string(),
                    found: p.current_token().token_type.clone(),
                    line: p.current_token().line,
                });
            }
        }

        // 通常のフィールド名またはメソッド名、または unified members の先頭キーワードを読み取り
        if let TokenType::IDENTIFIER(field_or_method) = &p.current_token().token_type {
            let field_or_method = field_or_method.clone();
            let field_or_method_line = p.current_token().line;
            p.advance();

            // 可視性: public/private ブロック/単行
            if box_try_visibility(
                p,
                &field_or_method,
                &mut methods,
                &mut fields,
                &mut field_decls,
                &mut public_fields,
                &mut private_fields,
                &mut last_method_name,
                &mut weak_fields,
            )? {
                continue;
            }

            // Unified Members canonical computed syntax: `get name: Type { ... }`.
            // `get` is contextual here; `get: Type` and `get(...)` keep their
            // existing stored-field/method meaning.
            if crate::config::env::unified_members() && field_or_method == "get" {
                if let Some(_property_name) = members::fields::try_parse_get_computed_property(
                    p,
                    field_or_method_line,
                    &mut methods,
                )? {
                    p.ensure_no_pending_runes("get property")?;
                    last_method_name = None;
                    continue;
                }
            }

            // Unified Members (header-first) gate: support once/birth_once via members::properties
            if crate::config::env::unified_members()
                && (field_or_method == "once" || field_or_method == "birth_once")
            {
                p.ensure_no_pending_runes("unified property")?;
                if members::properties::try_parse_unified_property(
                    p,
                    &field_or_method,
                    &mut methods,
                    &mut birth_once_props,
                )? {
                    last_method_name = None; // do not attach method-level postfix here
                    continue;
                }
            }

            // メソッド or フィールド（委譲）
            if box_try_method_or_field(
                p,
                field_or_method,
                is_override,
                &mut methods,
                &mut fields,
                &mut field_decls,
                &mut last_method_name,
                &mut weak_fields,
            )? {
                continue;
            }
        } else {
            return Err(ParseError::UnexpectedToken {
                expected: "method or field name".to_string(),
                found: p.current_token().token_type.clone(),
                line: p.current_token().line,
            });
        }
    }

    p.consume(TokenType::RBRACE)?;
    members::property_emit::apply_birth_once_constructor_prologues(
        &mut constructors,
        &birth_once_props,
    );
    // 🚫 Disallow method named same as the box (constructor-like confusion)
    validators::validate_no_ctor_like_name(p, &name, &methods)?;

    // 🔥 Override validation
    for parent in &extends {
        p.validate_override_methods(&name, parent, &methods)?;
    }

    // birth_once 相互依存の簡易検出（宣言間の循環）
    validators::validate_birth_once_cycles(p, &methods)?;

    Ok(ASTNode::BoxDeclaration {
        name,
        fields,
        field_decls,
        public_fields,
        private_fields,
        methods,
        constructors,
        init_fields,
        weak_fields, // 🔗 Add weak fields to AST
        is_interface: false,
        extends,
        implements,
        type_parameters,
        is_static: false,  // 通常のboxはnon-static
        static_init: None, // 通常のboxはstatic初期化ブロックなし
        attrs,
        span: Span::unknown(),
    })
}

/// interface box宣言をパース: interface box Name { methods... }
pub fn parse_interface_box_declaration(p: &mut NyashParser) -> Result<ASTNode, ParseError> {
    interface::parse_interface_box(p)
}
