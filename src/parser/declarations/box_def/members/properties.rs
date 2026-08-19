//! Properties parsing (once/birth_once, header-first)
use crate::ast::Span;
use crate::parser::common::ParserUtils;
use crate::parser::declarations::box_def::members::{
    property_batch::{PreparedGeneratedPropertyMethodBatchV1, PropertyMemberKindV1},
    syntax::{self, PropertyBodyPostfix},
};
use crate::parser::source_authority::GeneratedPropertySink;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

fn prepare_and_commit(
    kind: PropertyMemberKindV1,
    sink: &mut impl GeneratedPropertySink,
    birth_once_props: &mut Vec<String>,
    name: String,
    body: Vec<crate::ast::ASTNode>,
    diagnostic_span: Span,
) -> Result<(), ParseError> {
    let batch = PreparedGeneratedPropertyMethodBatchV1::prepare(kind, name, body, diagnostic_span)?;
    if let Some(property_name) = batch.commit(sink)? {
        sink.record_generated_birth_trigger_at_current(
            crate::parser::source_authority::GeneratedBirthTriggerKindV1::BirthOnceProperty,
        );
        birth_once_props.push(property_name);
    }
    Ok(())
}

/// Try to parse a unified member property: `once name: Type ...` or `birth_once name: Type ...`
/// Returns Ok(true) if consumed and handled; otherwise Ok(false).
pub(crate) fn try_parse_unified_property(
    p: &mut NyashParser,
    kind_kw: &str,
    sink: &mut impl GeneratedPropertySink,
    birth_once_props: &mut Vec<String>,
) -> Result<bool, ParseError> {
    let Some(kind) = PropertyMemberKindV1::from_keyword(kind_kw) else {
        return Ok(false);
    };
    let diagnostic_span = p.current_span();

    let syntax::TypedMemberHeader {
        name,
        declared_type_name: _declared_type_name,
    } = syntax::parse_required_typed_member_header(
        p,
        "identifier after once/birth_once",
        ": type",
        "type name",
    )?;
    let body = syntax::parse_required_property_body(
        p,
        PropertyBodyPostfix::ArrowOrBlock,
        "'=>' expression or block for once/birth_once property",
    )?;
    prepare_and_commit(kind, sink, birth_once_props, name, body, diagnostic_span)?;
    Ok(true)
}

/// Try to parse a block-first unified member: `{ body } as [once|birth_once]? name : Type [postfix]`
/// Returns Ok(true) if a member was parsed and emitted into `methods`.
pub(crate) fn try_parse_block_first_property(
    p: &mut NyashParser,
    sink: &mut impl GeneratedPropertySink,
    birth_once_props: &mut Vec<String>,
) -> Result<bool, ParseError> {
    if !(crate::parser::env::unified_members() && p.match_token(&TokenType::LBRACE)) {
        return Ok(false);
    }
    // 1) Parse block body first
    let mut final_body = p.parse_block_statements()?;

    // 2) Expect 'as'
    if let TokenType::IDENTIFIER(kw) = &p.current_token().token_type {
        if kw != "as" {
            let line = p.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "'as' after block for block-first member".to_string(),
                line,
            });
        }
    } else {
        let line = p.current_token().line;
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "'as' after block for block-first member".to_string(),
            line,
        });
    }
    p.advance(); // consume 'as'

    // 3) Optional kind keyword: once | birth_once
    let mut kind = PropertyMemberKindV1::Computed;
    if let TokenType::IDENTIFIER(k) = &p.current_token().token_type {
        if let Some(parsed_kind) = PropertyMemberKindV1::from_keyword(k.as_str()) {
            kind = parsed_kind;
            p.advance();
        }
    }

    // 4) Name : Type
    let diagnostic_span = p.current_span();
    let syntax::TypedMemberHeader {
        name,
        declared_type_name: _declared_type_name,
    } = syntax::parse_required_typed_member_header(
        p,
        "identifier for member name",
        ": type",
        "type name after ':'",
    )?;

    // 5) Optional postfix handlers (Stage‑3) directly after block (shared helper)
    final_body =
        crate::parser::declarations::box_def::members::postfix::wrap_with_optional_postfix(
            p, final_body,
        )?;

    prepare_and_commit(
        kind,
        sink,
        birth_once_props,
        name,
        final_body,
        diagnostic_span,
    )?;
    Ok(true)
}
