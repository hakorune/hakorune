use std::fmt;

use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{
    Attribute, Expr, ExprAsync, ExprCall, ExprClosure, ExprMethodCall, ImplItemConst, ImplItemFn,
    ItemConst, ItemFn, ItemImpl, ItemMod, ItemStatic, ItemTrait, Macro, TraitItemConst,
    TraitItemFn, Type,
};

use crate::model::{
    DirectCallExpressionKindV1, DirectCallResolutionV1, DirectCallSiteV1,
    DirectCallUnresolvedReasonV1, ItemFactV1, ItemKindV1, LexicalContextKindV1, LexicalContextV1,
    OpaqueSyntaxKindV1, OpaqueSyntaxSiteV1, ParseStatusV1, PositionV1, RustSourceTopologyV1,
    SourceEditionObservationV1, SourceFileTopologyV1, SourceRangeV1, UnresolvedCallSiteV1,
    RUST_SOURCE_TOPOLOGY_SCHEMA_V1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtractErrorV1 {
    EmptyPath,
    EmptyModuleSyntaxPath,
    Parse { detail: String },
}

impl fmt::Display for ExtractErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPath => write!(formatter, "[rust-source-topology/empty-path]"),
            Self::EmptyModuleSyntaxPath => {
                write!(formatter, "[rust-source-topology/empty-module-syntax-path]")
            }
            Self::Parse { detail } => {
                write!(formatter, "[rust-source-topology/parse-failed] {detail}")
            }
        }
    }
}

impl std::error::Error for ExtractErrorV1 {}

pub fn extract_single_file_source(
    path: &str,
    module_syntax_path: &str,
    source: &str,
) -> Result<RustSourceTopologyV1, ExtractErrorV1> {
    if path.is_empty() {
        return Err(ExtractErrorV1::EmptyPath);
    }
    if module_syntax_path.is_empty() {
        return Err(ExtractErrorV1::EmptyModuleSyntaxPath);
    }
    let file = syn::parse_file(source).map_err(|error| ExtractErrorV1::Parse {
        detail: error.to_string(),
    })?;
    let mut extractor = SingleFileExtractorV1::new(path, module_syntax_path, source);
    extractor.visit_file(&file);
    Ok(extractor.finish())
}

#[derive(Clone)]
struct CurrentItemV1 {
    item_id: String,
    syntax_path: String,
}

struct SingleFileExtractorV1<'source> {
    path: &'source str,
    root_module_syntax_path: &'source str,
    source: &'source str,
    line_starts: Vec<usize>,
    module_stack: Vec<String>,
    item_stack: Vec<CurrentItemV1>,
    cfg_stack: Vec<Vec<String>>,
    lexical_stack: Vec<LexicalContextV1>,
    items: Vec<ItemFactV1>,
    calls: Vec<DirectCallSiteV1>,
    opaque_sites: Vec<OpaqueSyntaxSiteV1>,
}

impl<'source> SingleFileExtractorV1<'source> {
    fn new(path: &'source str, module_syntax_path: &'source str, source: &'source str) -> Self {
        Self {
            path,
            root_module_syntax_path: module_syntax_path,
            source,
            line_starts: line_starts(source),
            module_stack: vec![module_syntax_path.to_string()],
            item_stack: Vec::new(),
            cfg_stack: Vec::new(),
            lexical_stack: Vec::new(),
            items: Vec::new(),
            calls: Vec::new(),
            opaque_sites: Vec::new(),
        }
    }

    fn finish(mut self) -> RustSourceTopologyV1 {
        self.items
            .sort_by_key(|item| (item.source_range, item.item_id.clone()));
        self.calls
            .sort_by_key(|call| (call.source_range, call.call_site_id.clone()));
        self.opaque_sites
            .sort_by_key(|site| (site.source_range, site.opaque_site_id.clone()));
        let unresolved_call_sites = self
            .calls
            .iter()
            .filter_map(|call| match &call.resolution {
                DirectCallResolutionV1::Resolved { .. } => None,
                DirectCallResolutionV1::Unresolved { reason, evidence } => {
                    Some(UnresolvedCallSiteV1 {
                        call_site_id: call.call_site_id.clone(),
                        reason: *reason,
                        evidence: evidence.clone(),
                    })
                }
            })
            .collect();
        RustSourceTopologyV1 {
            schema: RUST_SOURCE_TOPOLOGY_SCHEMA_V1,
            schema_version: 1,
            source_file: SourceFileTopologyV1 {
                path: self.path.to_string(),
                content_digest: stable_digest(self.source),
                edition: SourceEditionObservationV1::Unknown,
                parse_status: ParseStatusV1::Success,
                root_module_syntax_path: self.root_module_syntax_path.to_string(),
                items: self.items,
                direct_call_sites: self.calls,
                unresolved_call_sites,
                opaque_syntax_sites: self.opaque_sites,
            },
        }
    }

    fn current_module(&self) -> &str {
        self.module_stack
            .last()
            .expect("single-file extractor always owns one syntax module root")
    }

    fn current_item(&self) -> Option<&CurrentItemV1> {
        self.item_stack.last()
    }

    fn current_item_syntax_path(&self) -> String {
        self.current_item()
            .map(|item| item.syntax_path.clone())
            .unwrap_or_else(|| format!("{}::<module-body>", self.current_module()))
    }

    fn parent_syntax_path(&self) -> String {
        self.current_item()
            .map(|item| item.syntax_path.clone())
            .unwrap_or_else(|| self.current_module().to_string())
    }

    fn inherited_cfg(&self) -> Vec<String> {
        self.cfg_stack
            .iter()
            .flat_map(|rows| rows.iter().cloned())
            .collect()
    }

    fn source_range(&self, span: Span) -> SourceRangeV1 {
        source_range(span, &self.line_starts, self.source)
    }

    fn source_text(&self, range: SourceRangeV1) -> String {
        self.source
            .get(range.byte_start..range.byte_end)
            .unwrap_or_default()
            .to_string()
    }

    fn make_id(&self, prefix: &str, range: SourceRangeV1, suffix: &str) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.path, range.byte_start, range.byte_end, prefix, suffix
        )
    }

    fn record_item(
        &mut self,
        syntax_path: String,
        kind: ItemKindV1,
        span: Span,
        attrs: &[Attribute],
        impl_self_type_syntax: Option<String>,
        impl_trait_syntax: Option<String>,
    ) -> CurrentItemV1 {
        let range = self.source_range(span);
        let item_id = self.make_id("item", range, item_kind_tag(kind));
        let current = CurrentItemV1 {
            item_id: item_id.clone(),
            syntax_path: syntax_path.clone(),
        };
        self.items.push(ItemFactV1 {
            item_id,
            syntax_path,
            parent_item_id: self.current_item().map(|item| item.item_id.clone()),
            module_syntax_path: self.current_module().to_string(),
            kind,
            impl_self_type_syntax,
            impl_trait_syntax,
            source_range: range,
            direct_cfg_syntax: cfg_syntax(attrs),
            inherited_cfg_syntax: self.inherited_cfg(),
        });
        current
    }

    fn with_item(
        &mut self,
        item: CurrentItemV1,
        attrs: &[Attribute],
        visit: impl FnOnce(&mut Self),
    ) {
        self.item_stack.push(item);
        self.cfg_stack.push(cfg_syntax(attrs));
        visit(self);
        self.cfg_stack.pop();
        self.item_stack.pop();
    }

    fn record_call(
        &mut self,
        kind: DirectCallExpressionKindV1,
        span: Span,
        callee: String,
        receiver_syntax: Option<String>,
        reason: DirectCallUnresolvedReasonV1,
        evidence: String,
        attrs: &[Attribute],
    ) {
        let range = self.source_range(span);
        let source_text = self.source_text(range);
        let call_site_id = self.make_id("call", range, call_kind_tag(kind));
        self.calls.push(DirectCallSiteV1 {
            call_site_id,
            path: self.path.to_string(),
            module_syntax_path: self.current_module().to_string(),
            enclosing_item_id: self.current_item().map(|item| item.item_id.clone()),
            enclosing_item_syntax_path: self.current_item_syntax_path(),
            lexical_context: self.lexical_stack.clone(),
            expression_kind: kind,
            source_range: range,
            source_digest: stable_digest(&source_text),
            source_text,
            normalized_callee_syntax: callee,
            receiver_syntax,
            explicit_receiver_type_syntax: None,
            direct_cfg_syntax: cfg_syntax(attrs),
            inherited_cfg_syntax: self.inherited_cfg(),
            resolution: DirectCallResolutionV1::Unresolved { reason, evidence },
        });
    }

    fn record_opaque(
        &mut self,
        kind: OpaqueSyntaxKindV1,
        syntax_name: String,
        span: Span,
        attrs: &[Attribute],
    ) {
        let range = self.source_range(span);
        let source_text = self.source_text(range);
        self.opaque_sites.push(OpaqueSyntaxSiteV1 {
            opaque_site_id: self.make_id("opaque", range, opaque_kind_tag(kind)),
            path: self.path.to_string(),
            module_syntax_path: self.current_module().to_string(),
            enclosing_item_id: self.current_item().map(|item| item.item_id.clone()),
            kind,
            syntax_name,
            source_range: range,
            source_digest: stable_digest(&source_text),
            source_text,
            direct_cfg_syntax: cfg_syntax(attrs),
            inherited_cfg_syntax: self.inherited_cfg(),
        });
    }

    fn with_lexical(
        &mut self,
        kind: LexicalContextKindV1,
        span: Span,
        visit: impl FnOnce(&mut Self),
    ) {
        let source_range = self.source_range(span);
        self.lexical_stack
            .push(LexicalContextV1 { kind, source_range });
        visit(self);
        self.lexical_stack.pop();
    }
}

impl<'ast> Visit<'ast> for SingleFileExtractorV1<'_> {
    fn visit_item_mod(&mut self, item: &'ast ItemMod) {
        let syntax_path = format!("{}::{}", self.current_module(), item.ident);
        let kind = if item.content.is_some() {
            ItemKindV1::InlineModule
        } else {
            ItemKindV1::ExternalModule
        };
        let current = self.record_item(
            syntax_path.clone(),
            kind,
            item.span(),
            &item.attrs,
            None,
            None,
        );
        let Some((_, items)) = &item.content else {
            let opaque_kind = if has_path_attribute(&item.attrs) {
                OpaqueSyntaxKindV1::PathAttributedExternalModule
            } else {
                OpaqueSyntaxKindV1::ExternalModule
            };
            self.record_opaque(
                opaque_kind,
                item.ident.to_string(),
                item.span(),
                &item.attrs,
            );
            return;
        };
        self.module_stack.push(syntax_path);
        self.with_item(current, &item.attrs, |this| {
            for child in items {
                this.visit_item(child);
            }
        });
        self.module_stack.pop();
    }

    fn visit_item_fn(&mut self, item: &'ast ItemFn) {
        let syntax_path = format!("{}::{}", self.parent_syntax_path(), item.sig.ident);
        let current = self.record_item(
            syntax_path,
            ItemKindV1::Function,
            item.span(),
            &item.attrs,
            None,
            None,
        );
        self.with_item(current, &item.attrs, |this| {
            visit::visit_block(this, &item.block)
        });
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        let impl_name = normalized_type(&item.self_ty);
        let trait_syntax = item
            .trait_
            .as_ref()
            .map(|(_, path, _)| normalized_path(path));
        let syntax_path = format!("{}::<impl {}>", self.current_module(), impl_name);
        let current = self.record_item(
            syntax_path,
            ItemKindV1::Impl,
            item.span(),
            &item.attrs,
            Some(impl_name),
            trait_syntax,
        );
        self.with_item(current, &item.attrs, |this| {
            visit::visit_item_impl(this, item)
        });
    }

    fn visit_impl_item_fn(&mut self, item: &'ast ImplItemFn) {
        let syntax_path = format!("{}::{}", self.current_item_syntax_path(), item.sig.ident);
        let current = self.record_item(
            syntax_path,
            ItemKindV1::ImplMethod,
            item.span(),
            &item.attrs,
            None,
            None,
        );
        self.with_item(current, &item.attrs, |this| {
            visit::visit_block(this, &item.block)
        });
    }

    fn visit_impl_item_const(&mut self, item: &'ast ImplItemConst) {
        let syntax_path = format!("{}::{}", self.parent_syntax_path(), item.ident);
        let current = self.record_item(
            syntax_path,
            ItemKindV1::ImplAssociatedConst,
            item.span(),
            &item.attrs,
            None,
            None,
        );
        self.with_item(current, &item.attrs, |this| this.visit_expr(&item.expr));
    }

    fn visit_item_trait(&mut self, item: &'ast ItemTrait) {
        let syntax_path = format!("{}::{}", self.current_module(), item.ident);
        let current = self.record_item(
            syntax_path,
            ItemKindV1::Trait,
            item.span(),
            &item.attrs,
            None,
            None,
        );
        self.with_item(current, &item.attrs, |this| {
            visit::visit_item_trait(this, item)
        });
    }

    fn visit_trait_item_fn(&mut self, item: &'ast TraitItemFn) {
        let syntax_path = format!("{}::{}", self.current_item_syntax_path(), item.sig.ident);
        let current = self.record_item(
            syntax_path,
            ItemKindV1::TraitMethod,
            item.span(),
            &item.attrs,
            None,
            None,
        );
        if let Some(block) = &item.default {
            self.with_item(current, &item.attrs, |this| visit::visit_block(this, block));
        }
    }

    fn visit_trait_item_const(&mut self, item: &'ast TraitItemConst) {
        let syntax_path = format!("{}::{}", self.parent_syntax_path(), item.ident);
        let current = self.record_item(
            syntax_path,
            ItemKindV1::TraitAssociatedConst,
            item.span(),
            &item.attrs,
            None,
            None,
        );
        if let Some((_, expression)) = &item.default {
            self.with_item(current, &item.attrs, |this| this.visit_expr(expression));
        }
    }

    fn visit_item_const(&mut self, item: &'ast ItemConst) {
        let syntax_path = format!("{}::{}", self.parent_syntax_path(), item.ident);
        let current = self.record_item(
            syntax_path,
            ItemKindV1::Const,
            item.span(),
            &item.attrs,
            None,
            None,
        );
        self.with_item(current, &item.attrs, |this| this.visit_expr(&item.expr));
    }

    fn visit_item_static(&mut self, item: &'ast ItemStatic) {
        let syntax_path = format!("{}::{}", self.parent_syntax_path(), item.ident);
        let current = self.record_item(
            syntax_path,
            ItemKindV1::Static,
            item.span(),
            &item.attrs,
            None,
            None,
        );
        self.with_item(current, &item.attrs, |this| this.visit_expr(&item.expr));
    }

    fn visit_expr_call(&mut self, expression: &'ast ExprCall) {
        let (callee, reason, evidence) = classify_expr_call_callee(expression.func.as_ref());
        self.record_call(
            DirectCallExpressionKindV1::ExprCall,
            expression.span(),
            callee,
            None,
            reason,
            evidence,
            &expression.attrs,
        );
        visit::visit_expr_call(self, expression);
    }

    fn visit_expr_method_call(&mut self, expression: &'ast ExprMethodCall) {
        let receiver = normalized_tokens(expression.receiver.as_ref());
        self.record_call(
            DirectCallExpressionKindV1::ExprMethodCall,
            expression.span(),
            expression.method.to_string(),
            Some(receiver.clone()),
            DirectCallUnresolvedReasonV1::GeneralReceiverInference,
            receiver,
            &expression.attrs,
        );
        visit::visit_expr_method_call(self, expression);
    }

    fn visit_expr_closure(&mut self, expression: &'ast ExprClosure) {
        self.with_lexical(LexicalContextKindV1::Closure, expression.span(), |this| {
            visit::visit_expr_closure(this, expression)
        });
    }

    fn visit_expr_async(&mut self, expression: &'ast ExprAsync) {
        self.with_lexical(
            LexicalContextKindV1::AsyncBlock,
            expression.span(),
            |this| visit::visit_expr_async(this, expression),
        );
    }

    fn visit_macro(&mut self, mac: &'ast Macro) {
        let name = normalized_path(&mac.path);
        let kind = if mac.path.is_ident("include") {
            OpaqueSyntaxKindV1::IncludeMacro
        } else {
            OpaqueSyntaxKindV1::MacroInvocation
        };
        self.record_opaque(kind, name, mac.span(), &[]);
    }
}

fn classify_expr_call_callee(callee: &Expr) -> (String, DirectCallUnresolvedReasonV1, String) {
    match callee {
        Expr::Path(path) => {
            let syntax = normalized_path(&path.path);
            (
                syntax.clone(),
                DirectCallUnresolvedReasonV1::ResolutionDeferredToS0c,
                syntax,
            )
        }
        Expr::Closure(closure) => {
            let syntax = normalized_tokens(closure);
            (
                syntax.clone(),
                DirectCallUnresolvedReasonV1::ClosureInvocation,
                syntax,
            )
        }
        Expr::Paren(paren) => {
            let syntax = normalized_tokens(paren);
            let reason = if matches!(paren.expr.as_ref(), Expr::Closure(_)) {
                DirectCallUnresolvedReasonV1::ClosureInvocation
            } else if matches!(paren.expr.as_ref(), Expr::Path(_)) {
                DirectCallUnresolvedReasonV1::IndirectFunctionValue
            } else {
                DirectCallUnresolvedReasonV1::UnsupportedCalleeExpression
            };
            (syntax.clone(), reason, syntax)
        }
        other => {
            let syntax = normalized_tokens(other);
            (
                syntax.clone(),
                DirectCallUnresolvedReasonV1::UnsupportedCalleeExpression,
                syntax,
            )
        }
    }
}

fn cfg_syntax(attrs: &[Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("cfg") || attr.path().is_ident("cfg_attr"))
        .map(|attr| attr.meta.to_token_stream().to_string())
        .collect()
}

fn has_path_attribute(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("path"))
}

fn normalized_path(path: &syn::Path) -> String {
    let prefix = path
        .leading_colon
        .as_ref()
        .map(|_| "::")
        .unwrap_or_default();
    let segments = path
        .segments
        .iter()
        .map(|segment| segment.to_token_stream().to_string())
        .collect::<Vec<_>>()
        .join("::");
    format!("{prefix}{segments}")
}

fn normalized_type(ty: &Type) -> String {
    normalized_tokens(ty)
}

fn normalized_tokens(tokens: &impl ToTokens) -> String {
    tokens.to_token_stream().to_string()
}

fn source_range(span: Span, line_starts: &[usize], source: &str) -> SourceRangeV1 {
    let start = span.start();
    let end = span.end();
    let byte_start = byte_offset(line_starts, start.line, start.column, source);
    let byte_end = byte_offset(line_starts, end.line, end.column, source);
    SourceRangeV1 {
        start: PositionV1 {
            line: start.line,
            column: start.column,
        },
        end: PositionV1 {
            line: end.line,
            column: end.column,
        },
        byte_start,
        byte_end,
    }
}

fn line_starts(source: &str) -> Vec<usize> {
    let mut starts = vec![0];
    for (index, byte) in source.bytes().enumerate() {
        if byte == b'\n' {
            starts.push(index + 1);
        }
    }
    starts
}

fn byte_offset(line_starts: &[usize], line: usize, column: usize, source: &str) -> usize {
    let source_len = source.len();
    let line_start = line_starts
        .get(line.saturating_sub(1))
        .copied()
        .unwrap_or(source_len);
    let line_end = line_starts
        .get(line)
        .copied()
        .unwrap_or(source_len)
        .min(source_len);
    let line_source = &source[line_start..line_end];
    let column_byte = line_source
        .char_indices()
        .nth(column)
        .map(|(offset, _)| offset)
        .unwrap_or(line_source.len());
    line_start + column_byte
}

fn stable_digest(text: &str) -> String {
    let mut value = 0xcbf29ce484222325_u64;
    for byte in text.bytes() {
        value ^= u64::from(byte);
        value = value.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{value:016x}")
}

fn item_kind_tag(kind: ItemKindV1) -> &'static str {
    match kind {
        ItemKindV1::InlineModule => "inline_module",
        ItemKindV1::ExternalModule => "external_module",
        ItemKindV1::Function => "function",
        ItemKindV1::Impl => "impl",
        ItemKindV1::ImplMethod => "impl_method",
        ItemKindV1::ImplAssociatedConst => "impl_associated_const",
        ItemKindV1::Trait => "trait",
        ItemKindV1::TraitMethod => "trait_method",
        ItemKindV1::TraitAssociatedConst => "trait_associated_const",
        ItemKindV1::Const => "const",
        ItemKindV1::Static => "static",
    }
}

fn call_kind_tag(kind: DirectCallExpressionKindV1) -> &'static str {
    match kind {
        DirectCallExpressionKindV1::ExprCall => "expr_call",
        DirectCallExpressionKindV1::ExprMethodCall => "expr_method_call",
    }
}

fn opaque_kind_tag(kind: OpaqueSyntaxKindV1) -> &'static str {
    match kind {
        OpaqueSyntaxKindV1::MacroInvocation => "macro",
        OpaqueSyntaxKindV1::IncludeMacro => "include",
        OpaqueSyntaxKindV1::ExternalModule => "external_module",
        OpaqueSyntaxKindV1::PathAttributedExternalModule => "path_external_module",
    }
}
