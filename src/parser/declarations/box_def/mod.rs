//! Box Definition Parser Module
//!
//! Box宣言（box, interface box, static box）の解析を担当
//! Nyashの中核概念「Everything is Box」を実現する重要モジュール

use crate::ast::{ASTNode, DelegateDecl, FieldDecl, Span, TransitionDecl};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::HashMap;

pub mod header;
pub mod interface;
pub mod members;
mod sync_box;
pub mod validators;

fn is_supported_record_default_expr(expr: &ASTNode) -> bool {
    match expr {
        ASTNode::Literal { value, .. } => matches!(
            value,
            crate::ast::LiteralValue::Integer(_)
                | crate::ast::LiteralValue::TypedInteger { .. }
                | crate::ast::LiteralValue::Bool(_)
                | crate::ast::LiteralValue::String(_)
                | crate::ast::LiteralValue::Null
        ),
        ASTNode::UnaryOp {
            operator, operand, ..
        } => {
            matches!(operator, crate::ast::UnaryOperator::Minus)
                && matches!(
                    operand.as_ref(),
                    ASTNode::Literal {
                        value: crate::ast::LiteralValue::Integer(_)
                            | crate::ast::LiteralValue::TypedInteger { .. },
                        ..
                    }
                )
        }
        _ => false,
    }
}

/// Thin wrappers to keep the main loop tidy (behavior-preserving)
fn box_try_block_first_property(
    p: &mut NyashParser,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if !p.match_token(&TokenType::LBRACE) {
        return Ok(false);
    }
    p.ensure_no_pending_runes("block-first property")?;
    members::properties::try_parse_block_first_property(
        p,
        &mut state.methods,
        &mut state.birth_once_props,
    )
}

fn box_try_method_postfix_after_last(
    p: &mut NyashParser,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if state.last_method_name.is_none()
        || !(p.match_token(&TokenType::CATCH) || p.match_token(&TokenType::CLEANUP))
    {
        return Ok(false);
    }
    p.ensure_no_pending_runes("method postfix")?;
    members::postfix::try_parse_method_postfix_after_last_method(
        p,
        &mut state.methods,
        &state.last_method_name,
    )
}

fn box_try_init_block(
    p: &mut NyashParser,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if !(p.match_token(&TokenType::INIT) && p.peek_token() != &TokenType::LPAREN) {
        return Ok(false);
    }
    p.ensure_no_pending_runes("init block")?;
    members::fields::parse_init_block_if_any(p, &mut state.init_fields, &mut state.weak_fields)
}

fn box_try_delegate(
    p: &mut NyashParser,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if !p.match_token(&TokenType::DELEGATE) {
        return Ok(false);
    }
    p.ensure_no_pending_runes("delegate declaration")?;
    state.delegates.push(members::delegates::parse_delegate_decl(p)?);
    Ok(true)
}

fn box_try_transition(
    p: &mut NyashParser,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if let Some(transition) = members::transitions::try_parse_transition_decl(p)? {
        p.ensure_no_pending_runes("transition declaration")?;
        state.transitions.push(transition);
        return Ok(true);
    }
    Ok(false)
}

fn box_try_constructor(
    p: &mut NyashParser,
    is_override: bool,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if let Some((key, node)) = members::constructors::try_parse_constructor(p, is_override)? {
        let mut node = node;
        p.attach_pending_runes_to_declaration(&mut node)?;
        state.constructors.insert(key, node);
        return Ok(true);
    }
    Ok(false)
}

fn box_try_visibility(
    p: &mut NyashParser,
    visibility: &str,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if visibility != "public" && visibility != "private" {
        return Ok(false);
    }
    p.ensure_no_pending_runes("visibility field/property")?;
    members::fields::try_parse_visibility_block_or_single(
        p,
        visibility,
        &mut state.methods,
        &mut state.fields,
        &mut state.field_decls,
        &mut state.field_initializers,
        &mut state.public_fields,
        &mut state.private_fields,
        &mut state.last_method_name,
        &mut state.weak_fields,
    )
}

fn box_try_build_gate_members(
    p: &mut NyashParser,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if !p.is_build_gate_head() {
        return Ok(false);
    }

    let line = p.current_token().line;
    p.consume_build_gate_head()?;
    let predicate = p.parse_build_predicate()?;

    let then_state = parse_box_member_gate_block(p)?;
    let else_state = if p.match_token(&TokenType::ELSE) {
        p.advance();
        Some(if p.is_build_gate_head() {
            parse_box_member_gate_group(p)?
        } else {
            parse_box_member_gate_block(p)?
        })
    } else {
        None
    };

    let selected_then = p.eval_build_predicate(&predicate, Span::new(0, 0, line, 1))?;
    let then_signature = then_state.signature();
    let else_signature = else_state.as_ref().map(BoxMemberState::signature);

    match &else_signature {
        Some(sig) if then_signature != *sig => {
            return Err(ParseError::BuildCfg {
                message: "member-level gate branches must preserve the same public signature"
                    .to_string(),
                line,
            });
        }
        None if !then_signature.is_empty() => {
            return Err(ParseError::BuildCfg {
                message:
                    "member-level gate inside box bodies requires an else branch with the same public signature"
                        .to_string(),
                line,
            });
        }
        _ => {}
    }

    let selected_state = if selected_then {
        then_state
    } else if let Some(else_state) = else_state {
        else_state
    } else {
        BoxMemberState::default()
    };

    state.merge_from(selected_state);
    state.last_method_name = None;
    Ok(true)
}

fn parse_box_member_gate_block(p: &mut NyashParser) -> Result<BoxMemberState, ParseError> {
    p.consume(TokenType::LBRACE)?;
    let mut state = BoxMemberState::default();
    parse_box_member_body(p, &mut state)?;
    p.consume(TokenType::RBRACE)?;
    Ok(state)
}

fn parse_box_member_gate_group(p: &mut NyashParser) -> Result<BoxMemberState, ParseError> {
    if !p.is_build_gate_head() {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "gate".to_string(),
            line: p.current_token().line,
        });
    }
    let line = p.current_token().line;
    p.consume_build_gate_head()?;
    let predicate = p.parse_build_predicate()?;
    let then_state = parse_box_member_gate_block(p)?;
    let else_state = if p.match_token(&TokenType::ELSE) {
        p.advance();
        Some(if p.is_build_gate_head() {
            parse_box_member_gate_group(p)?
        } else {
            parse_box_member_gate_block(p)?
        })
    } else {
        None
    };
    let selected_then = p.eval_build_predicate(&predicate, Span::new(0, 0, line, 1))?;
    let then_signature = then_state.signature();
    let else_signature = else_state.as_ref().map(BoxMemberState::signature);

    match &else_signature {
        Some(sig) if then_signature != *sig => {
            return Err(ParseError::BuildCfg {
                message: "member-level gate branches must preserve the same public signature"
                    .to_string(),
                line,
            });
        }
        None if !then_signature.is_empty() => {
            return Err(ParseError::BuildCfg {
                message:
                    "member-level gate inside box bodies requires an else branch with the same public signature"
                        .to_string(),
                line,
            });
        }
        _ => {}
    }

    Ok(if selected_then {
        then_state
    } else if let Some(else_state) = else_state {
        else_state
    } else {
        BoxMemberState::default()
    })
}

fn parse_box_member_body(
    p: &mut NyashParser,
    state: &mut BoxMemberState,
) -> Result<(), ParseError> {
    while !p.match_token(&TokenType::RBRACE) && !p.is_at_end() {
        if p.maybe_parse_opt_annotation_noop(
            crate::parser::statements::helpers::AnnotationSite::Member,
        )? {
            continue;
        }

        if box_try_block_first_property(p, state)? {
            continue;
        }

        if box_try_method_postfix_after_last(p, state)? {
            continue;
        }

        if p.match_token(&TokenType::RBRACE) {
            break;
        }

        if box_try_init_block(p, state)? {
            continue;
        }

        if box_try_delegate(p, state)? {
            state.last_method_name = None;
            continue;
        }

        if box_try_transition(p, state)? {
            state.last_method_name = None;
            continue;
        }

        if let Some(invariant) = p.try_parse_invariant_clause()? {
            state.invariants.push(invariant);
            state.last_method_name = None;
            continue;
        }

        if box_try_build_gate_members(p, state)? {
            continue;
        }

        let mut is_override = false;
        if p.match_token(&TokenType::OVERRIDE) {
            is_override = true;
            p.advance();
        }

        if box_try_constructor(p, is_override, state)? {
            continue;
        }

        if p.match_token(&TokenType::WEAK) {
            p.ensure_no_pending_runes("weak field")?;
            p.advance();
            if let TokenType::IDENTIFIER(field_name) = &p.current_token().token_type {
                let field_name = field_name.clone();
                p.advance();
                members::fields::parse_weak_field(
                    p,
                    field_name,
                    &mut state.methods,
                    &mut state.fields,
                    &mut state.field_decls,
                    &mut state.weak_fields,
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

        if let TokenType::IDENTIFIER(field_or_method) = &p.current_token().token_type {
            let field_or_method = field_or_method.clone();
            let field_or_method_line = p.current_token().line;
            p.advance();

            if box_try_visibility(p, &field_or_method, state)? {
                continue;
            }

            if crate::config::env::unified_members() && field_or_method == "get" {
                if let Some(_property_name) = members::fields::try_parse_get_computed_property(
                    p,
                    field_or_method_line,
                    &mut state.methods,
                )? {
                    p.ensure_no_pending_runes("get property")?;
                    state.last_method_name = None;
                    continue;
                }
            }

            if crate::config::env::unified_members()
                && (field_or_method == "once" || field_or_method == "birth_once")
            {
                p.ensure_no_pending_runes("unified property")?;
                if members::properties::try_parse_unified_property(
                    p,
                    &field_or_method,
                    &mut state.methods,
                    &mut state.birth_once_props,
                )? {
                    state.last_method_name = None;
                    continue;
                }
            }

            if box_try_method_or_field(p, field_or_method, is_override, state)? {
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
    Ok(())
}

#[derive(Debug, Default)]
struct BoxMemberState {
    fields: Vec<String>,
    field_decls: Vec<FieldDecl>,
    field_initializers: Vec<(String, ASTNode)>,
    methods: HashMap<String, ASTNode>,
    public_fields: Vec<String>,
    private_fields: Vec<String>,
    constructors: HashMap<String, ASTNode>,
    init_fields: Vec<String>,
    weak_fields: Vec<String>,
    delegates: Vec<DelegateDecl>,
    invariants: Vec<ASTNode>,
    transitions: Vec<TransitionDecl>,
    birth_once_props: Vec<String>,
    last_method_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct MethodSignature {
    name: String,
    params: Vec<String>,
    param_decls: Vec<crate::ast::ParamDecl>,
    return_type_name: Option<String>,
    uses: Vec<String>,
    contracts: Vec<crate::ast::ContractClause>,
    is_static: bool,
    is_override: bool,
    attrs: crate::ast::DeclarationAttrs,
}

impl MethodSignature {
    fn from_node(node: &ASTNode) -> Option<Self> {
        let ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            return_type_name,
            uses,
            contracts,
            is_static,
            is_override,
            attrs,
            ..
        } = node
        else {
            return None;
        };
        Some(Self {
            name: name.clone(),
            params: params.clone(),
            param_decls: param_decls.clone(),
            return_type_name: return_type_name.clone(),
            uses: uses.clone(),
            contracts: contracts.clone(),
            is_static: *is_static,
            is_override: *is_override,
            attrs: attrs.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct BoxMemberSignature {
    fields: Vec<String>,
    field_decls: Vec<FieldDecl>,
    field_initializers: Vec<(String, ASTNode)>,
    public_fields: Vec<String>,
    private_fields: Vec<String>,
    methods: std::collections::BTreeMap<String, MethodSignature>,
    constructors: std::collections::BTreeMap<String, MethodSignature>,
    init_fields: Vec<String>,
    weak_fields: Vec<String>,
    delegates: Vec<DelegateDecl>,
    invariants: Vec<ASTNode>,
    transitions: Vec<TransitionDecl>,
    birth_once_props: Vec<String>,
}

impl BoxMemberSignature {
    fn is_empty(&self) -> bool {
        self.fields.is_empty()
            && self.field_decls.is_empty()
            && self.field_initializers.is_empty()
            && self.public_fields.is_empty()
            && self.private_fields.is_empty()
            && self.methods.is_empty()
            && self.constructors.is_empty()
            && self.init_fields.is_empty()
            && self.weak_fields.is_empty()
            && self.delegates.is_empty()
            && self.invariants.is_empty()
            && self.transitions.is_empty()
            && self.birth_once_props.is_empty()
    }
}

impl BoxMemberState {
    fn merge_from(&mut self, mut other: BoxMemberState) {
        self.fields.extend(other.fields.drain(..));
        self.field_decls.extend(other.field_decls.drain(..));
        self.field_initializers
            .extend(other.field_initializers.drain(..));
        self.methods.extend(other.methods.drain());
        self.public_fields.extend(other.public_fields.drain(..));
        self.private_fields.extend(other.private_fields.drain(..));
        self.constructors.extend(other.constructors.drain());
        self.init_fields.extend(other.init_fields.drain(..));
        self.weak_fields.extend(other.weak_fields.drain(..));
        self.delegates.extend(other.delegates.drain(..));
        self.invariants.extend(other.invariants.drain(..));
        self.transitions.extend(other.transitions.drain(..));
        self.birth_once_props.extend(other.birth_once_props.drain(..));
    }

    fn signature(&self) -> BoxMemberSignature {
        let mut methods = std::collections::BTreeMap::new();
        for (name, node) in &self.methods {
            if let Some(sig) = MethodSignature::from_node(node) {
                methods.insert(name.clone(), sig);
            }
        }

        let mut constructors = std::collections::BTreeMap::new();
        for (name, node) in &self.constructors {
            if let Some(sig) = MethodSignature::from_node(node) {
                constructors.insert(name.clone(), sig);
            }
        }

        BoxMemberSignature {
            fields: self.fields.clone(),
            field_decls: self.field_decls.clone(),
            field_initializers: self.field_initializers.clone(),
            public_fields: self.public_fields.clone(),
            private_fields: self.private_fields.clone(),
            methods,
            constructors,
            init_fields: self.init_fields.clone(),
            weak_fields: self.weak_fields.clone(),
            delegates: self.delegates.clone(),
            invariants: self.invariants.clone(),
            transitions: self.transitions.clone(),
            birth_once_props: self.birth_once_props.clone(),
        }
    }
}

/// Parse either a method or a header-first field/property starting with `name`.
/// Updates `methods`/`fields` and `last_method_name` as appropriate.
fn box_try_method_or_field(
    p: &mut NyashParser,
    name: String,
    is_override: bool,
    state: &mut BoxMemberState,
) -> Result<bool, ParseError> {
    if let Some(method) = members::methods::try_parse_method(p, name.clone(), is_override)? {
        let mut method = method;
        p.attach_pending_runes_to_declaration(&mut method)?;
        state.last_method_name = Some(name.clone());
        state.methods.insert(name, method);
        return Ok(true);
    }
    // Fallback: header-first field/property (computed/once/birth_once handled inside)
    let parsed = members::fields::try_parse_header_first_field_or_property(
        p,
        name,
        &mut state.methods,
        &mut state.fields,
        &mut state.field_decls,
        &mut state.field_initializers,
        &mut state.weak_fields,
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

    Ok(ASTNode::BoxDeclaration {
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
    })
}

/// Parse C202 record declaration: `record Name { field: Type ... }`.
///
/// This row only locks the source contract. It does not add local scalar
/// replacement or packed `ArrayBox` residence.
pub fn parse_record_declaration(p: &mut NyashParser) -> Result<ASTNode, ParseError> {
    if !p.match_token(&TokenType::RECORD) {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "'record'".to_string(),
            line: p.current_token().line,
        });
    }
    p.advance(); // consume RECORD
    let attrs = p.take_pending_runes_for_box()?;
    let (name, type_parameters, extends, implements) = header::parse_header(p)?;
    if !extends.is_empty() || !implements.is_empty() {
        return Err(ParseError::UnexpectedToken {
            found: p.current_token().token_type.clone(),
            expected: "record declaration without from/implements clauses".to_string(),
            line: p.current_token().line,
        });
    }

    p.consume(TokenType::LBRACE)?;

    let mut fields = Vec::new();
    let mut field_decls = Vec::new();
    let mut invariants = Vec::new();
    while !p.match_token(&TokenType::RBRACE) && !p.is_at_end() {
        while p.match_token(&TokenType::NEWLINE) {
            p.advance();
        }
        if p.match_token(&TokenType::RBRACE) {
            break;
        }
        if let Some(invariant) = p.try_parse_invariant_clause()? {
            invariants.push(invariant);
            continue;
        }
        if p.match_token(&TokenType::WEAK) {
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "record field name; weak fields are not part of C202".to_string(),
                line: p.current_token().line,
            });
        }
        let TokenType::IDENTIFIER(field_name) = &p.current_token().token_type else {
            return Err(ParseError::UnexpectedToken {
                found: p.current_token().token_type.clone(),
                expected: "record field name".to_string(),
                line: p.current_token().line,
            });
        };
        let field_name = field_name.clone();
        p.advance();
        p.consume(TokenType::COLON)?;
        let declared_type_name =
            crate::parser::common::type_refs::parse_type_ref_text(p, "record field type")?;

        let default_value = if p.match_token(&TokenType::ASSIGN) {
            p.advance();
            let default_expr = p.parse_expression()?;
            if !is_supported_record_default_expr(&default_expr) {
                return Err(ParseError::UnexpectedToken {
                    found: p.current_token().token_type.clone(),
                    expected: "record scalar literal default expression".to_string(),
                    line: p.current_token().line,
                });
            }
            Some(Box::new(default_expr))
        } else {
            None
        };

        fields.push(field_name.clone());
        field_decls.push(FieldDecl {
            name: field_name,
            declared_type_name: Some(declared_type_name),
            is_weak: false,
            default_value,
        });

        if p.match_token(&TokenType::COMMA) {
            p.advance();
        }
    }

    if field_decls.is_empty() {
        return Err(ParseError::InvalidStatement {
            line: p.current_token().line,
        });
    }

    p.consume(TokenType::RBRACE)?;

    Ok(ASTNode::BoxDeclaration {
        name,
        fields,
        field_decls,
        public_fields: vec![],
        private_fields: vec![],
        methods: HashMap::new(),
        constructors: HashMap::new(),
        init_fields: vec![],
        weak_fields: vec![],
        delegates: vec![],
        invariants,
        transitions: vec![],
        is_interface: false,
        is_record: true,
        extends: vec![],
        implements: vec![],
        type_parameters,
        is_sync: false,
        is_static: false,
        static_init: None,
        attrs,
        span: Span::unknown(),
    })
}

/// interface box宣言をパース: interface box Name { methods... }
pub fn parse_interface_box_declaration(p: &mut NyashParser) -> Result<ASTNode, ParseError> {
    interface::parse_interface_box(p)
}
