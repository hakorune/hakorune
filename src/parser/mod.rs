/*!
 * Nyash Parser - Rust Implementation
 *
 * Python版nyashc_v4.pyのNyashParserをRustで完全再実装
 * Token列をAST (Abstract Syntax Tree) に変換
 *
 * モジュール構造:
 * - common.rs: 共通ユーティリティとトレイト (ParserUtils)
 * - expressions.rs: 式パーサー (parse_expression, parse_or, parse_and等)
 * - statements.rs: 文パーサー (parse_statement, parse_if, parse_loop等)
 * - declarations/: Box宣言パーサー (box_definition, static_box, dependency_helpers)
 * - items/: トップレベル宣言 (global_vars, functions, static_items)
 *
 * 2025-08-16: 大規模リファクタリング完了
 * - 1530行 → 227行 (85%削減)
 * - 機能ごとにモジュール分離で保守性向上
 */

mod body_source;
mod build_cfg;
mod build_gate_selection;
mod callable_contract_syntax;
mod callable_gate_projection;
#[cfg(test)]
mod callable_gate_projection_tests;
mod callable_parameter_source;
mod callable_source_anchor;
mod common;
mod contracts;
mod cursor; // TokenCursor: 改行処理を一元管理
mod declarations;
mod delegate_batch;
mod delegate_lowering;
mod delegate_source_relation;
mod delegate_target_index;
pub(crate) mod env;
// depth_tracking.rs was a legacy depth counter for Smart advance.
// Phase 15.5: removed in favor of TokenCursor-centric newline handling.
pub mod entry_sugar; // helper to parse with sugar level
mod expr;
mod expr_cursor; // TokenCursorを使用した式パーサー（実験的）
mod expressions;
mod from_transport_boundary;
mod generated_callable_anchor;
mod grammar_contract;
mod initial_callable_program_source;
mod items;
mod lifecycle;
pub(crate) mod log;
mod postpass_compatibility;
mod postpass_envelope;
mod postpass_open;
mod release_source;
mod runes;
mod source_authority;
mod source_gate_ledger;
mod source_gate_receipt;
mod source_member_cursor;
mod source_path;
mod source_resolver_handoff;
mod source_seal;
mod source_seal_finalizer;
#[cfg(test)]
mod source_session_tests;
mod stage3; // Phase 152-A: Stage-3 parser extensions
mod statements; // Now uses modular structure in statements/
mod string_postpass_entry;
pub mod sugar; // Phase 12.7-B: desugar pass (basic)
pub mod sugar_gate; // thread-local gate for sugar parsing (tests/docs)
                    // mod errors;

use common::ParserUtils;

use crate::ast::{ASTNode, BuildPredicate, EnumVariantDecl, RuneAttr, Span};
use crate::tokenizer::{Token, TokenType};

#[inline]
fn is_sugar_enabled() -> bool {
    crate::parser::sugar_gate::is_enabled()
}

pub use build_cfg::BuildGateExplainReport;
pub use hakorune_frontend_parser::parser::{
    BuildMode, GrammarProfile, ParserBuildConfig, ParserMetadata,
};

// ===== 🔥 Debug Macros =====

/// Infinite loop detection macro - must be called in every loop that advances tokens
/// Prevents parser from hanging due to token consumption bugs
/// Uses parser's debug_fuel field for centralized fuel management
#[macro_export]
macro_rules! must_advance {
    ($parser:expr, $fuel:expr, $location:literal) => {
        // デバッグ燃料がSomeの場合のみ制限チェック
        if let Some(ref mut limit) = $parser.debug_fuel {
            if *limit == 0 {
                $crate::parser::log::error(&format!(
                    "🚨 PARSER INFINITE LOOP DETECTED at {}",
                    $location
                ));
                $crate::parser::log::error(&format!(
                    "🔍 Current token: {:?} at line {}",
                    $parser.current_token().token_type,
                    $parser.current_token().line
                ));
                $crate::parser::log::error(&format!(
                    "🔍 Parser position: {}/{}",
                    $parser.current,
                    $parser.tokens.len()
                ));
                return Err($crate::parser::ParseError::InfiniteLoop {
                    location: $location.to_string(),
                    token: $parser.current_token().token_type.clone(),
                    line: $parser.current_token().line,
                });
            }
            *limit -= 1;
        }
        // None の場合は無制限なのでチェックしない
    };
}

/// Initialize debug fuel for loop monitoring
#[macro_export]
macro_rules! debug_fuel {
    () => {
        100_000 // Default: 100k iterations should be enough for any reasonable program
    };
}

pub use hakorune_frontend_parser::migration_transport::{
    parse_migration_transport_with_config, MigrationTransport, MigrationTransportBundle,
    MigrationTransportKind,
};
pub use hakorune_frontend_parser::parser::ParseError;

pub(crate) use body_source::{
    ParserBoxBodySourceEnvelopeV1, ParserBoxInstanceMethodSyntaxLeaseV1,
    ParserBoxMethodBodySourceRowV1,
};
pub(crate) use callable_contract_syntax::CallableContractSyntaxV1;
pub(crate) use callable_parameter_source::{
    ParserCallableSyntaxLoanErrorV1, RetainedParserCallableSemanticSourceV1,
};
pub(crate) use source_resolver_handoff::{
    ParserBoxResolverSourceHandoffV1, ResolverBoxMethodSourceRowV1, ResolverBoxMethodSourceSiteV1,
    ResolverBoxSourceRowV1, ResolverMethodSignatureSyntaxV1, ResolverSourceInvocationProvenanceV1,
};

/// Nyashパーサー - トークン列をASTに変換
pub struct NyashParser {
    pub(super) tokens: Vec<Token>,
    pub(super) current: usize,
    /// Fresh identity for one parser invocation. Source authority products
    /// must never be reconstructed from token positions or AST names.
    pub(super) source_invocation_brand: source_authority::ParserInvocationBrandV1,
    callable_parameter_source_session:
        Option<callable_parameter_source::ParserCallableParameterSourceSessionV1>,
    callable_source_session: Option<callable_source_anchor::ParserCallableSourceSessionV1>,
    /// Top-level source statement cursor used to issue exact Box declaration
    /// sites. It is parser-session state only; no seal is issued here.
    pub(super) next_source_statement_ordinal: u32,
    pub(super) active_source_statement_ordinal: Option<u32>,
    /// Parser-issued structural path for the currently parsed declaration.
    /// This is richer than the top-level statement cursor because multiple
    /// Boxes may occur inside one build-gate branch.
    pub(super) active_source_declaration_path: Option<source_authority::SourceBoxDeclarationPathV1>,
    pub(super) next_source_build_gate_id: u32,
    pub(super) source_build_gate_scope: source_gate_ledger::SourceBuildGateScopeV1,
    pub(super) prepared_source_build_gate_records:
        Vec<source_gate_ledger::PreparedBuildGateSourceRecordV1>,
    pub(super) build_gate_observations: Vec<build_cfg::decision_set::BuildGateObservationV1>,
    /// 🔥 Static box依存関係追跡（循環依存検出用）
    pub(super) static_box_dependencies:
        std::collections::HashMap<String, std::collections::HashSet<String>>,
    /// 🔥 デバッグ燃料：無限ループ検出用制限値 (None = 無制限)
    pub(super) debug_fuel: Option<usize>,
    /// Pending rune annotations waiting for the next declaration node.
    pub(super) pending_runes: Vec<RuneAttr>,
    /// Pending `@rune Gate(...)` sugar waiting for the next declaration node.
    pub(super) pending_build_gate: Option<(BuildPredicate, usize)>,
    /// Committed rune metadata in source order.
    pub(super) rune_metadata: Vec<RuneAttr>,
    /// Enum declarations parsed so far, used to resolve shorthand enum matches.
    pub(super) known_enums: std::collections::BTreeMap<String, Vec<EnumVariantDecl>>,
    /// Build configuration used to prune AST-level `gate` conditionals.
    pub(super) build_config: ParserBuildConfig,
    /// Prepared Box source payloads retained until the final postpass.
    /// `source_seal` consumes these exactly once; AST-only paths may discard
    /// them as compatibility projections.
    pub(super) prepared_source_seals: Vec<source_authority::PreparedBoxSourceSealV1>,
}

// ParserUtils trait implementation now lives here (legacy depth tracking removed)

impl NyashParser {
    /// 新しいパーサーを作成
    pub fn new(tokens: Vec<Token>) -> Self {
        let source_invocation_brand = source_authority::ParserInvocationBrandV1::issue();
        Self {
            tokens,
            current: 0,
            callable_parameter_source_session: Some(
                callable_parameter_source::ParserCallableParameterSourceSessionV1::open(
                    source_invocation_brand.clone(),
                ),
            ),
            callable_source_session: Some(
                callable_source_anchor::ParserCallableSourceSessionV1::open(
                    source_invocation_brand.clone(),
                ),
            ),
            source_invocation_brand,
            next_source_statement_ordinal: 0,
            active_source_statement_ordinal: None,
            active_source_declaration_path: None,
            next_source_build_gate_id: 0,
            source_build_gate_scope: source_gate_ledger::SourceBuildGateScopeV1::Closed,
            prepared_source_build_gate_records: Vec::new(),
            build_gate_observations: Vec::new(),
            static_box_dependencies: std::collections::HashMap::new(),
            debug_fuel: Some(100_000), // デフォルト値
            pending_runes: Vec::new(),
            pending_build_gate: None,
            rune_metadata: Vec::new(),
            known_enums:
                hakorune_frontend_ast::result_option_prelude::result_option_prelude_enum_decls(),
            build_config: ParserBuildConfig::default(),
            prepared_source_seals: Vec::new(),
        }
    }

    pub fn with_build_config(mut self, build_config: ParserBuildConfig) -> Self {
        self.build_config = build_config;
        self
    }

    pub(super) fn register_enum_declaration(&mut self, name: &str, variants: &[EnumVariantDecl]) {
        self.known_enums.insert(name.to_string(), variants.to_vec());
    }

    /// 文字列からパース (トークナイズ + パース)
    /// Note: Reads parser_stage3_enabled() (NYASH_FEATURES=stage3 or legacy env) for using-chain parsing
    pub fn parse_from_string(input: impl Into<String>) -> Result<ASTNode, ParseError> {
        // Ensure Stage-3 features are enabled when parsing using-chain files
        // when parent requested Stage-3 parsing via NYASH_FEATURES/legacy env
        Self::parse_from_string_with_fuel(input, Some(100_000))
    }

    /// 文字列からパースし、Rune metadata sidecar も返す。
    pub fn parse_from_string_with_metadata(
        input: impl Into<String>,
    ) -> Result<(ASTNode, ParserMetadata), ParseError> {
        Self::parse_from_string_with_fuel_and_metadata(input, Some(100_000))
    }

    /// 文字列からパース (デバッグ燃料指定版)
    /// fuel: Some(n) = n回まで、None = 無制限
    pub fn parse_from_string_with_fuel(
        input: impl Into<String>,
        fuel: Option<usize>,
    ) -> Result<ASTNode, ParseError> {
        Self::parse_from_string_with_fuel_and_build_config(
            input,
            fuel,
            ParserBuildConfig::default(),
        )
    }

    pub fn parse_from_string_with_build_config(
        input: impl Into<String>,
        build_config: ParserBuildConfig,
    ) -> Result<ASTNode, ParseError> {
        Self::parse_from_string_with_fuel_and_build_config(input, Some(100_000), build_config)
    }

    /// Canonical R6-S3 source product for the bounded ordinary Rust `box`
    /// cohort. The returned product is non-Clone and is finalized only after
    /// build-gate pruning and delegate lowering. Existing AST-only APIs remain
    /// compatibility projections until the later top-level-gate cutover.
    pub(crate) fn parse_from_string_with_source_seal(
        input: impl Into<String>,
        build_config: ParserBuildConfig,
    ) -> Result<source_seal::ParsedProgramWithSourceV1, ParseError> {
        let input_s: String = input.into();
        let pre = normalize_logical_ops(&input_s);
        let mut tokenizer = crate::tokenizer::NyashTokenizer::with_grammar_profile(
            pre,
            build_config.grammar_profile,
        );
        let tokens = tokenizer.tokenize()?;
        for tok in &tokens {
            if let TokenType::IDENTIFIER(name) = &tok.token_type {
                if name == "self" {
                    return Err(ParseError::UnsupportedIdentifier {
                        name: name.clone(),
                        line: tok.line,
                    });
                }
            }
        }

        let mut parser = Self::new(tokens);
        parser.build_config = build_config;
        let ast = parser.parse_program()?;
        let product = parser.open_postpass_product(ast)?;
        let product = product.prune_build_gates(&parser)?;
        let product = product.lower_delegates()?;
        product.finalize().map_err(source_seal::map_error)
    }

    pub(super) fn parse_postpass_s0(
        &mut self,
    ) -> Result<postpass_envelope::CompletedParserPostpassV1, ParseError> {
        self.parse_postpass_with_demand(postpass_envelope::PostpassDemandV1::default())
    }

    pub(super) fn parse_postpass_with_demand(
        &mut self,
        demand: postpass_envelope::PostpassDemandV1,
    ) -> Result<postpass_envelope::CompletedParserPostpassV1, ParseError> {
        let ast = self.parse_program()?;
        let product = self.open_postpass_product(ast)?;
        product.finish_total_s0(self, demand)
    }

    /// Bounded AST-only projection for the same direct ordinary-Box rich
    /// path. This is intentionally crate-visible until all general parser
    /// cohorts have typed path/relation transport.
    pub(crate) fn parse_from_string_with_source_seal_ast(
        input: impl Into<String>,
        build_config: ParserBuildConfig,
    ) -> Result<ASTNode, ParseError> {
        Self::parse_from_string_with_source_seal(input, build_config)
            .map(source_seal::ParsedProgramWithSourceV1::into_ast)
    }

    pub fn parse_from_string_with_fuel_and_build_config(
        input: impl Into<String>,
        fuel: Option<usize>,
        build_config: ParserBuildConfig,
    ) -> Result<ASTNode, ParseError> {
        string_postpass_entry::parse(input.into(), fuel, build_config)
    }

    pub fn parse_grammar_evidence_from_string_with_build_config(
        input: impl Into<String>,
        build_config: ParserBuildConfig,
    ) -> Result<ASTNode, ParseError> {
        Self::parse_grammar_evidence_from_string_with_fuel_and_build_config(
            input,
            Some(100_000),
            build_config,
        )
    }

    fn parse_grammar_evidence_from_string_with_fuel_and_build_config(
        input: impl Into<String>,
        fuel: Option<usize>,
        build_config: ParserBuildConfig,
    ) -> Result<ASTNode, ParseError> {
        let input_s: String = input.into();
        let pre = normalize_logical_ops(&input_s);
        let mut tokenizer = crate::tokenizer::NyashTokenizer::with_grammar_profile(
            pre,
            build_config.grammar_profile,
        );
        let tokens = tokenizer.tokenize()?;

        for tok in &tokens {
            if let TokenType::IDENTIFIER(name) = &tok.token_type {
                if name == "self" {
                    return Err(ParseError::UnsupportedIdentifier {
                        name: name.clone(),
                        line: tok.line,
                    });
                }
            }
        }

        let mut parser = Self::new(tokens);
        parser.debug_fuel = fuel;
        parser.build_config = build_config;
        let ast = parser.parse_program()?;
        parser.prune_build_gate_program(ast)
    }

    pub fn parse_from_string_with_build_config_and_explain_report(
        input: impl Into<String>,
        build_config: ParserBuildConfig,
    ) -> Result<(ASTNode, BuildGateExplainReport), ParseError> {
        Self::parse_from_string_with_fuel_and_build_config_and_explain_report(
            input,
            Some(100_000),
            build_config,
        )
    }

    pub fn parse_from_string_with_fuel_and_build_config_and_explain_report(
        input: impl Into<String>,
        fuel: Option<usize>,
        build_config: ParserBuildConfig,
    ) -> Result<(ASTNode, BuildGateExplainReport), ParseError> {
        string_postpass_entry::parse_with_explain(input.into(), fuel, build_config)
    }

    /// 文字列からパースし、デバッグ燃料と metadata sidecar を返す。
    pub fn parse_from_string_with_fuel_and_metadata(
        input: impl Into<String>,
        fuel: Option<usize>,
    ) -> Result<(ASTNode, ParserMetadata), ParseError> {
        let input_s: String = input.into();
        let pre = normalize_logical_ops(&input_s);
        let mut tokenizer = crate::tokenizer::NyashTokenizer::new(pre);
        let tokens = tokenizer.tokenize()?;

        for tok in &tokens {
            if let TokenType::IDENTIFIER(name) = &tok.token_type {
                if name == "self" {
                    return Err(ParseError::UnsupportedIdentifier {
                        name: name.clone(),
                        line: tok.line,
                    });
                }
            }
        }

        let mut parser = Self::new(tokens);
        parser.debug_fuel = fuel;
        crate::parser::string_postpass_entry::parse_with_metadata(&mut parser)
    }

    /// パース実行 - Program ASTを返す
    pub fn parse(&mut self) -> Result<ASTNode, ParseError> {
        string_postpass_entry::parse_existing(self)
    }

    // ===== パース関数群 =====

    /// プログラム全体をパース
    fn parse_program(&mut self) -> Result<ASTNode, ParseError> {
        let mut statements = Vec::new();

        let allow_sc = crate::parser::env::parser_allow_semicolon_raw();

        while !self.is_at_end() {
            // EOF tokenはスキップ
            if matches!(self.current_token().token_type, TokenType::EOF) {
                break;
            }

            // NEWLINE tokenはスキップ（文の区切りとして使用）
            if matches!(self.current_token().token_type, TokenType::NEWLINE)
                || (allow_sc && matches!(self.current_token().token_type, TokenType::SEMICOLON))
            {
                self.advance();
                continue;
            }

            if self.maybe_parse_opt_annotation_noop(
                crate::parser::statements::helpers::AnnotationSite::TopLevel,
            )? {
                continue;
            }

            let statement_ordinal = self.next_source_statement_ordinal;
            self.next_source_statement_ordinal = self
                .next_source_statement_ordinal
                .checked_add(1)
                .ok_or(ParseError::BuildCfg {
                    message: "parser source statement ordinal exceeds u32".to_owned(),
                    line: self.current_token().line,
                })?;
            self.active_source_statement_ordinal = Some(statement_ordinal);
            self.active_source_declaration_path =
                Some(source_authority::SourceBoxDeclarationPathV1::root(
                    self.source_invocation_brand(),
                    statement_ordinal,
                ));
            let mut statement = if self.is_build_gate_head() {
                let previous = self.set_source_build_gate_scope(
                    source_gate_ledger::SourceBuildGateScopeV1::TopLevelItem,
                );
                let parsed = self.parse_build_gate_item();
                self.set_source_build_gate_scope(previous);
                parsed?
            } else {
                let previous = self.set_source_build_gate_scope(
                    source_gate_ledger::SourceBuildGateScopeV1::Closed,
                );
                let parsed = self.parse_statement();
                self.set_source_build_gate_scope(previous);
                parsed?
            };
            self.active_source_statement_ordinal = None;
            self.active_source_declaration_path = None;
            self.attach_pending_runes_to_declaration(&mut statement)?;
            statements.push(statement);
        }

        self.ensure_no_pending_runes("end of file")?;

        // 🔥 すべてのstatic box解析後に循環依存検出
        self.check_circular_dependencies()?;
        self.ensure_no_pending_build_gate("end of file")?;

        Ok(ASTNode::Program {
            statements,
            span: Span::unknown(),
        })
    }
    // Statement parsing methods are now in statements.rs module

    /// 代入文または関数呼び出しをパース
    fn parse_assignment_or_function_call(&mut self) -> Result<ASTNode, ParseError> {
        // まず左辺を式としてパース
        let expr = self.parse_expression()?;

        // 次のトークンが = または 複合代入演算子 なら代入文
        if self.match_token(&TokenType::ASSIGN) {
            self.advance(); // consume '='
            let value = Box::new(self.parse_expression()?);

            // 左辺が代入可能な形式かチェック
            match &expr {
                ASTNode::Variable { .. } | ASTNode::FieldAccess { .. } | ASTNode::Index { .. } => {
                    Ok(ASTNode::Assignment {
                        target: Box::new(expr),
                        value,
                        span: Span::unknown(),
                    })
                }
                _ => {
                    let line = self.current_token().line;
                    Err(ParseError::InvalidStatement { line })
                }
            }
        } else if self.match_token(&TokenType::PlusAssign)
            || self.match_token(&TokenType::MinusAssign)
            || self.match_token(&TokenType::MulAssign)
            || self.match_token(&TokenType::DivAssign)
        {
            if !is_sugar_enabled() {
                let line = self.current_token().line;
                return Err(ParseError::UnexpectedToken {
                    found: self.current_token().token_type.clone(),
                    expected: "enable NYASH_SYNTAX_SUGAR_LEVEL=basic|full for '+=' and friends"
                        .to_string(),
                    line,
                });
            }
            // determine operator
            let op = match &self.current_token().token_type {
                TokenType::PlusAssign => crate::ast::BinaryOperator::Add,
                TokenType::MinusAssign => crate::ast::BinaryOperator::Subtract,
                TokenType::MulAssign => crate::ast::BinaryOperator::Multiply,
                TokenType::DivAssign => crate::ast::BinaryOperator::Divide,
                _ => unreachable!(),
            };
            self.advance(); // consume 'op='
            let rhs = self.parse_expression()?;
            // 左辺が代入可能な形式かチェック
            match &expr {
                ASTNode::Variable { .. } | ASTNode::FieldAccess { .. } | ASTNode::Index { .. } => {
                    Ok(ASTNode::CompoundAssignment {
                        target: Box::new(expr),
                        operator: op,
                        value: Box::new(rhs),
                        span: Span::unknown(),
                    })
                }
                _ => {
                    let line = self.current_token().line;
                    Err(ParseError::InvalidStatement { line })
                }
            }
        } else {
            // 代入文でなければ式文として返す
            Ok(expr)
        }
    }

    // Expression parsing methods are now in expressions.rs module
    // Utility methods are now in common.rs module via ParserUtils trait
    // Item parsing methods are now in items.rs module

    // ===== 🔥 Static Box循環依存検出 =====
}

impl NyashParser {
    pub(super) fn take_metadata(&mut self) -> ParserMetadata {
        ParserMetadata {
            runes: std::mem::take(&mut self.rune_metadata),
        }
    }

    pub(super) fn push_pending_rune(&mut self, rune: RuneAttr) {
        self.pending_runes.push(rune);
    }

    pub(super) fn push_pending_build_gate(
        &mut self,
        predicate: BuildPredicate,
        line: usize,
    ) -> Result<(), ParseError> {
        if self.pending_build_gate.is_some() {
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "[freeze:contract][parser/build-gate] duplicate @rune Gate".to_string(),
                line,
            });
        }
        self.issue_pending_build_gate_observation(predicate.clone(), Span::new(0, 0, line, 1))?;
        self.pending_build_gate = Some((predicate, line));
        Ok(())
    }

    pub(super) fn take_pending_build_gate(&mut self) -> Option<(BuildPredicate, usize)> {
        self.pending_build_gate.take()
    }

    pub(super) fn wrap_with_pending_build_gate(
        &mut self,
        node: ASTNode,
    ) -> Result<ASTNode, ParseError> {
        if let Some((predicate, line)) = self.take_pending_build_gate() {
            Ok(ASTNode::BuildGate {
                predicate,
                then_items: vec![node],
                else_items: None,
                span: Span::new(0, 0, line, 1),
            })
        } else {
            Ok(node)
        }
    }

    pub(super) fn ensure_no_pending_build_gate(&self, context: &str) -> Result<(), ParseError> {
        if self.pending_build_gate.is_none() {
            return Ok(());
        }
        Err(ParseError::BuildCfg {
            message: format!("dangling @rune Gate before {}", context),
            line: self.current_token().line,
        })
    }
}

// ---- Minimal ParserUtils impl (depth-less; TokenCursor handles newline policy) ----
impl common::ParserUtils for NyashParser {
    fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
    fn current(&self) -> usize {
        self.current
    }
    fn current_mut(&mut self) -> &mut usize {
        &mut self.current
    }
}

fn normalize_logical_ops(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut it = src.chars().peekable();
    let mut in_str = false;
    let mut in_line = false;
    let mut in_block = false;

    while let Some(c) = it.next() {
        if in_line {
            out.push(c);
            if c == '\n' {
                in_line = false;
            }
            continue;
        }

        if in_block {
            out.push(c);
            if c == '*' && matches!(it.peek(), Some('/')) {
                out.push('/');
                it.next();
                in_block = false;
            }
            continue;
        }

        if in_str {
            out.push(c);
            if c == '\\' {
                if let Some(nc) = it.next() {
                    out.push(nc);
                }
                continue;
            }
            if c == '"' {
                in_str = false;
            }
            continue;
        }

        match c {
            '"' => {
                in_str = true;
                out.push(c);
            }
            '/' => match it.peek() {
                Some('/') => {
                    out.push('/');
                    out.push('/');
                    it.next();
                    in_line = true;
                }
                Some('*') => {
                    out.push('/');
                    out.push('*');
                    it.next();
                    in_block = true;
                }
                _ => out.push('/'),
            },
            '#' => {
                in_line = true;
                out.push('#');
            }
            '|' => {
                if matches!(it.peek(), Some('|')) {
                    out.push_str(" or ");
                    it.next();
                } else if matches!(it.peek(), Some('>')) {
                    out.push('|');
                    out.push('>');
                    it.next();
                } else {
                    out.push('|');
                }
            }
            '&' => {
                if matches!(it.peek(), Some('&')) {
                    out.push_str(" and ");
                    it.next();
                } else {
                    out.push('&');
                }
            }
            _ => out.push(c),
        }
    }

    out
}
