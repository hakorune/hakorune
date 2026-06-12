//! Box Definition Parser Module
//!
//! Box宣言（box, interface box, static box）の解析を担当
//! Nyashの中核概念「Everything is Box」を実現する重要モジュール

use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

mod body;
pub mod header;
pub mod interface;
pub mod members;
mod record;
mod state;
mod sync_box;
pub mod validators;

use body::parse_box_member_body;
use state::BoxMemberState;

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
    parse_box_declaration_after_box_keyword(p, false)
}

/// Parse canonical `sync box Name { ... }`.
///
/// `sync` remains contextual: only the `sync box` statement head is special.
/// Runtime serialization is intentionally not implemented by this parser row.
pub fn parse_sync_box_declaration(p: &mut NyashParser) -> Result<ASTNode, ParseError> {
    let TokenType::IDENTIFIER(name) = &p.current_token().token_type else {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "`sync box`".to_string(),
            line: p.current_token().line,
        });
    };
    if name != "sync" {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "`sync box`".to_string(),
            line: p.current_token().line,
        });
    }
    p.advance(); // consume contextual `sync`
    p.consume(TokenType::BOX)?;
    parse_box_declaration_after_box_keyword(p, true)
}

fn parse_box_declaration_after_box_keyword(
    p: &mut NyashParser,
    is_sync: bool,
) -> Result<ASTNode, ParseError> {
    let attrs = p.take_pending_runes_for_box()?;
    let (name, type_parameters, extends, implements) = header::parse_header(p)?;

    p.consume(TokenType::LBRACE)?;
    let mut state = BoxMemberState::default();
    parse_box_member_body(p, &mut state)?;
    p.consume(TokenType::RBRACE)?;
    members::property_emit::apply_birth_once_constructor_prologues(
        &mut state.constructors,
        &state.birth_once_props,
    );
    members::property_emit::apply_stored_field_initializer_constructor_prologues(
        &mut state.constructors,
        &state.field_initializers,
    );
    // 🚫 Disallow method named same as the box (constructor-like confusion)
    validators::validate_no_ctor_like_name(p, &name, &state.methods)?;

    // 🔥 Override validation
    for parent in &extends {
        p.validate_override_methods(&name, parent, &state.methods)?;
    }

    // birth_once 相互依存の簡易検出（宣言間の循環）
    validators::validate_birth_once_cycles(p, &state.methods)?;
    if is_sync {
        sync_box::validate_no_waits_in_sync_box(p, &name, &state.methods, &state.constructors)?;
    }

    let node = ASTNode::BoxDeclaration {
        name,
        fields: state.fields,
        field_decls: state.field_decls,
        public_fields: state.public_fields,
        private_fields: state.private_fields,
        methods: state.methods,
        constructors: state.constructors,
        init_fields: state.init_fields,
        weak_fields: state.weak_fields, // 🔗 Add weak fields to AST
        delegates: state.delegates,
        invariants: state.invariants,
        transitions: state.transitions,
        is_interface: false,
        is_record: false,
        extends,
        implements,
        type_parameters,
        is_sync,
        is_static: false,  // 通常のboxはnon-static
        static_init: None, // 通常のboxはstatic初期化ブロックなし
        attrs,
        span: Span::unknown(),
    };

    p.wrap_with_pending_build_gate(node)
}

/// Parse C202 record declaration: `record Name { field: Type ... }`.
///
/// This row only locks the source contract. It does not add local scalar
/// replacement or packed `ArrayBox` residence.
pub fn parse_record_declaration(p: &mut NyashParser) -> Result<ASTNode, ParseError> {
    record::parse_record_declaration(p)
}

/// interface box宣言をパース: interface box Name { methods... }
pub fn parse_interface_box_declaration(p: &mut NyashParser) -> Result<ASTNode, ParseError> {
    interface::parse_interface_box(p)
}
