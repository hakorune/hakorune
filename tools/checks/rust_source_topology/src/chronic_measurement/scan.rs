use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use proc_macro2::Span;
use quote::ToTokens;
use sha2::{Digest, Sha256};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Attribute, Item, Macro, Meta, Token};

use crate::model::{PositionV1, SourceRangeV1};

use super::error::ChronicScanErrorV1;
use super::manifest::{load_scope_manifest, scope_paths, ScopedRustFileV1};
use super::model::{
    ChronicAllowanceKindV1, ChronicFileObservationV1, ChronicMeasurementReportV1, ChronicMetricV1,
    ChronicModuleEdgeKindV1, ChronicObservationV1, ChronicSummaryV1, CHRONIC_MEASUREMENT_SCHEMA_V1,
};

const SCANNER_VERSION: &str = "chronic-rust-token-scanner-v1";

pub fn scan_scope_manifest(
    manifest_path: &Path,
    workspace_root: &Path,
) -> Result<ChronicMeasurementReportV1, ChronicScanErrorV1> {
    let scope = load_scope_manifest(manifest_path, workspace_root)?;
    if scope.manifest.scanner_version != SCANNER_VERSION {
        return Err(ChronicScanErrorV1::InvalidManifest {
            detail: format!(
                "scanner_version must be {SCANNER_VERSION}, got {}",
                scope.manifest.scanner_version
            ),
        });
    }
    let before_paths = scope_paths(&scope);
    let mut files = Vec::with_capacity(scope.files.len());
    for file in &scope.files {
        files.push(scan_file(file)?);
    }
    let after = load_scope_manifest(manifest_path, workspace_root)?;
    if scope_paths(&after) != before_paths || after.manifest_hash != scope.manifest_hash {
        return Err(ChronicScanErrorV1::ScopeDrift {
            detail: "scope manifest or scoped Rust file set changed during observation".into(),
        });
    }
    let mut summary = ChronicSummaryV1::default();
    for file in &files {
        for observation in &file.observations {
            match observation {
                ChronicObservationV1::CallSite { metric, .. } => match metric {
                    ChronicMetricV1::Panic => summary.panic_count += 1,
                    ChronicMetricV1::Unwrap => summary.unwrap_count += 1,
                    ChronicMetricV1::Expect => summary.expect_count += 1,
                    ChronicMetricV1::Todo => summary.todo_count += 1,
                },
                ChronicObservationV1::DeadCodeAllowance { source_range, .. } => {
                    summary.dead_code_allowance_count += 1;
                    summary.dead_code_allowance_line_count += source_range
                        .end
                        .line
                        .saturating_sub(source_range.start.line)
                        + 1;
                }
                ChronicObservationV1::ModuleEdge { .. }
                | ChronicObservationV1::OpaqueMacro { .. } => {}
            }
        }
        if file.compile_domain == "unknown" || file.role == "unknown" {
            summary.unclassified_count += 1;
        }
    }
    let source_scope_hash = hash_source_scope(&files);
    let mut report = ChronicMeasurementReportV1 {
        schema: CHRONIC_MEASUREMENT_SCHEMA_V1,
        schema_version: 1,
        scanner_version: SCANNER_VERSION.to_string(),
        scope_id: scope.manifest.scope_id,
        scope_manifest_hash: scope.manifest_hash,
        source_scope_hash,
        evidence_hash: String::new(),
        summary,
        files,
    };
    report.evidence_hash = evidence_hash(&report)?;
    Ok(report)
}

pub fn scan_scope_manifest_json(
    manifest_path: &Path,
    workspace_root: &Path,
) -> Result<String, ChronicScanErrorV1> {
    let report = scan_scope_manifest(manifest_path, workspace_root)?;
    let mut output = serde_json::to_string_pretty(&report).map_err(|error| {
        ChronicScanErrorV1::ReportSerialize {
            detail: error.to_string(),
        }
    })?;
    output.push('\n');
    Ok(output)
}

fn scan_file(file: &ScopedRustFileV1) -> Result<ChronicFileObservationV1, ChronicScanErrorV1> {
    let bytes = fs::read(&file.absolute_path).map_err(|error| ChronicScanErrorV1::SourceRead {
        path: file.relative_path.clone(),
        detail: error.to_string(),
    })?;
    let digest = sha256_bytes(&bytes);
    let source = String::from_utf8(bytes).map_err(|_| ChronicScanErrorV1::NonUtf8Source {
        path: file.relative_path.clone(),
    })?;
    let parsed = syn::parse_file(&source).map_err(|error| ChronicScanErrorV1::ParseFailed {
        path: file.relative_path.clone(),
        detail: error.to_string(),
    })?;
    let mut observer = FileObserverV1::new(&file.relative_path, &source);
    observer.record_attributes(
        &parsed.attrs,
        full_source_range(&source),
        "<file>".to_string(),
    );
    observer.visit_file(&parsed);
    if let Some(error) = observer.error {
        return Err(error);
    }
    observer.observations.sort_by_key(observation_sort_key);
    let after_bytes =
        fs::read(&file.absolute_path).map_err(|error| ChronicScanErrorV1::SourceRead {
            path: file.relative_path.clone(),
            detail: error.to_string(),
        })?;
    if sha256_bytes(&after_bytes) != digest {
        return Err(ChronicScanErrorV1::SourceChangedDuringObservation {
            path: file.relative_path.clone(),
        });
    }
    Ok(ChronicFileObservationV1 {
        path: file.relative_path.clone(),
        source_digest: digest,
        compile_domain: file.compile_domain.clone(),
        role: file.role.clone(),
        observations: observer.observations,
    })
}

struct FileObserverV1<'source> {
    path: &'source str,
    source: &'source str,
    line_starts: Vec<usize>,
    item_stack: Vec<String>,
    cfg_stack: Vec<Vec<String>>,
    observations: Vec<ChronicObservationV1>,
    observation_keys: BTreeSet<String>,
    error: Option<ChronicScanErrorV1>,
}

impl<'source> FileObserverV1<'source> {
    fn new(path: &'source str, source: &'source str) -> Self {
        Self {
            path,
            source,
            line_starts: line_starts(source),
            item_stack: Vec::new(),
            cfg_stack: Vec::new(),
            observations: Vec::new(),
            observation_keys: BTreeSet::new(),
            error: None,
        }
    }

    fn current_item(&self) -> String {
        self.item_stack
            .last()
            .cloned()
            .unwrap_or_else(|| "<module-body>".to_string())
    }

    fn inherited_cfg(&self) -> Vec<String> {
        self.cfg_stack
            .iter()
            .flat_map(|rows| rows.iter().cloned())
            .collect()
    }

    fn range(&self, span: Span) -> SourceRangeV1 {
        let start = span.start();
        let end = span.end();
        SourceRangeV1 {
            start: PositionV1 {
                line: start.line,
                column: start.column,
            },
            end: PositionV1 {
                line: end.line,
                column: end.column,
            },
            byte_start: byte_offset(&self.line_starts, start.line, start.column, self.source),
            byte_end: byte_offset(&self.line_starts, end.line, end.column, self.source),
        }
    }

    fn insert(&mut self, key: String, observation: ChronicObservationV1) {
        if self.error.is_some() {
            return;
        }
        if !self.observation_keys.insert(key.clone()) {
            self.error = Some(ChronicScanErrorV1::DuplicateObservation {
                path: self.path.to_string(),
                key,
            });
            return;
        }
        self.observations.push(observation);
    }

    fn record_attributes(&mut self, attrs: &[Attribute], target: SourceRangeV1, item_key: String) {
        for attribute in attrs {
            let parsed = match allowance(attribute) {
                Ok(value) => value,
                Err(detail) => {
                    self.error = Some(ChronicScanErrorV1::MalformedAttribute {
                        path: self.path.to_string(),
                        detail,
                    });
                    return;
                }
            };
            let Some((kind, condition)) = parsed else {
                continue;
            };
            let source_range = self.range(attribute.span());
            let key = format!(
                "{}:{}:dead_code",
                source_range.byte_start, source_range.byte_end
            );
            self.insert(
                key,
                ChronicObservationV1::DeadCodeAllowance {
                    source_range,
                    target_range: target,
                    item_key: item_key.clone(),
                    attribute_kind: kind,
                    raw_condition: condition,
                    direct_cfg_syntax: cfg_syntax(attrs),
                    inherited_cfg_syntax: self.inherited_cfg(),
                },
            );
        }
    }

    fn record_call(&mut self, metric: ChronicMetricV1, span: Span) {
        let range = self.range(span);
        let key = format!("{}:{}:{metric:?}", range.byte_start, range.byte_end);
        self.insert(
            key,
            ChronicObservationV1::CallSite {
                metric,
                source_range: range,
                item_key: self.current_item(),
                direct_cfg_syntax: Vec::new(),
                inherited_cfg_syntax: self.inherited_cfg(),
            },
        );
    }

    fn record_module(&mut self, kind: ChronicModuleEdgeKindV1, span: Span, syntax: String) {
        let range = self.range(span);
        let key = format!("{}:{}:module:{kind:?}", range.byte_start, range.byte_end);
        self.insert(
            key,
            ChronicObservationV1::ModuleEdge {
                edge_kind: kind,
                source_range: range,
                item_key: self.current_item(),
                syntax,
            },
        );
    }

    fn record_macro(&mut self, macro_node: &Macro) {
        let range = self.range(macro_node.span());
        let key = format!("{}:{}:opaque-macro", range.byte_start, range.byte_end);
        self.insert(
            key,
            ChronicObservationV1::OpaqueMacro {
                source_range: range,
                item_key: self.current_item(),
                syntax_name: macro_node.path.to_token_stream().to_string(),
            },
        );
    }

    fn fail_unsupported(&mut self, detail: impl Into<String>) {
        if self.error.is_none() {
            self.error = Some(ChronicScanErrorV1::UnsupportedTokenShape {
                path: self.path.to_string(),
                detail: detail.into(),
            });
        }
    }

    fn visit_item_with_stack<'ast>(&mut self, item: &'ast Item, visit: impl FnOnce(&mut Self)) {
        let key = item_key(item, &self.item_stack);
        self.record_attributes(item_attrs(item), self.range(item.span()), key.clone());
        if matches!(item, Item::Verbatim(_)) {
            self.fail_unsupported("Item::Verbatim");
        }
        let cfg = cfg_syntax(item_attrs(item));
        self.item_stack.push(key);
        self.cfg_stack.push(cfg);
        visit(self);
        self.cfg_stack.pop();
        self.item_stack.pop();
    }
}

impl<'ast> Visit<'ast> for FileObserverV1<'_> {
    fn visit_item(&mut self, item: &'ast Item) {
        self.visit_item_with_stack(item, |this| visit::visit_item(this, item));
    }

    fn visit_expr_macro(&mut self, expression: &'ast syn::ExprMacro) {
        if let Some(metric) = metric_for_path(&expression.mac.path) {
            self.record_call(metric, expression.span());
        }
        visit::visit_expr_macro(self, expression);
    }

    fn visit_stmt_macro(&mut self, statement: &'ast syn::StmtMacro) {
        if let Some(metric) = metric_for_path(&statement.mac.path) {
            self.record_call(metric, statement.span());
        }
        visit::visit_stmt_macro(self, statement);
    }

    fn visit_expr_method_call(&mut self, expression: &'ast syn::ExprMethodCall) {
        match expression.method.to_string().as_str() {
            "unwrap" => self.record_call(ChronicMetricV1::Unwrap, expression.span()),
            "expect" => self.record_call(ChronicMetricV1::Expect, expression.span()),
            _ => {}
        }
        visit::visit_expr_method_call(self, expression);
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        let kind = if item.content.is_some() {
            ChronicModuleEdgeKindV1::InlineModule
        } else if has_path_attribute(&item.attrs) {
            ChronicModuleEdgeKindV1::PathAttributedExternalModule
        } else {
            ChronicModuleEdgeKindV1::ExternalModule
        };
        self.record_module(kind, item.span(), item.ident.to_string());
        visit::visit_item_mod(self, item);
    }

    fn visit_macro(&mut self, macro_node: &'ast Macro) {
        if macro_node.path.is_ident("include") {
            self.record_module(
                ChronicModuleEdgeKindV1::IncludeMacro,
                macro_node.span(),
                macro_node.to_token_stream().to_string(),
            );
        }
        self.record_macro(macro_node);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        if matches!(item, syn::ImplItem::Verbatim(_)) {
            self.fail_unsupported("ImplItem::Verbatim");
        }
        let key = format!("{}::{}", self.current_item(), impl_item_name(item));
        self.record_attributes(impl_item_attrs(item), self.range(item.span()), key.clone());
        let cfg = cfg_syntax(impl_item_attrs(item));
        self.item_stack.push(key);
        self.cfg_stack.push(cfg);
        visit::visit_impl_item(self, item);
        self.cfg_stack.pop();
        self.item_stack.pop();
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        if matches!(item, syn::TraitItem::Verbatim(_)) {
            self.fail_unsupported("TraitItem::Verbatim");
        }
        let key = format!("{}::{}", self.current_item(), trait_item_name(item));
        self.record_attributes(trait_item_attrs(item), self.range(item.span()), key.clone());
        let cfg = cfg_syntax(trait_item_attrs(item));
        self.item_stack.push(key);
        self.cfg_stack.push(cfg);
        visit::visit_trait_item(self, item);
        self.cfg_stack.pop();
        self.item_stack.pop();
    }

    fn visit_foreign_item(&mut self, item: &'ast syn::ForeignItem) {
        if matches!(item, syn::ForeignItem::Verbatim(_)) {
            self.fail_unsupported("ForeignItem::Verbatim");
        }
        let key = format!("{}::{}", self.current_item(), foreign_item_name(item));
        self.record_attributes(
            foreign_item_attrs(item),
            self.range(item.span()),
            key.clone(),
        );
        let cfg = cfg_syntax(foreign_item_attrs(item));
        self.item_stack.push(key);
        self.cfg_stack.push(cfg);
        visit::visit_foreign_item(self, item);
        self.cfg_stack.pop();
        self.item_stack.pop();
    }

    fn visit_variant(&mut self, variant: &'ast syn::Variant) {
        let key = format!("{}::{}", self.current_item(), variant.ident);
        self.record_attributes(&variant.attrs, self.range(variant.span()), key.clone());
        self.item_stack.push(key);
        visit::visit_variant(self, variant);
        self.item_stack.pop();
    }

    fn visit_field(&mut self, field: &'ast syn::Field) {
        let key = format!("{}::<field>", self.current_item());
        self.record_attributes(&field.attrs, self.range(field.span()), key);
        visit::visit_field(self, field);
    }

    fn visit_local(&mut self, local: &'ast syn::Local) {
        let key = format!(
            "{}::<local:{}>",
            self.current_item(),
            self.range(local.span()).byte_start
        );
        self.record_attributes(&local.attrs, self.range(local.span()), key);
        visit::visit_local(self, local);
    }
}

fn allowance(
    attribute: &Attribute,
) -> Result<Option<(ChronicAllowanceKindV1, Option<String>)>, String> {
    if attribute.path().is_ident("allow") {
        let Meta::List(list) = &attribute.meta else {
            return Err("allow attribute must have a list".into());
        };
        let nested = parse_meta_list(&list.tokens)?;
        if nested.iter().any(is_dead_code) {
            let kind = if matches!(attribute.style, syn::AttrStyle::Inner(_)) {
                ChronicAllowanceKindV1::InnerAllow
            } else {
                ChronicAllowanceKindV1::OuterAllow
            };
            return Ok(Some((kind, None)));
        }
        return Ok(None);
    }
    if !attribute.path().is_ident("cfg_attr") {
        return Ok(None);
    }
    let Meta::List(list) = &attribute.meta else {
        return Err("cfg_attr attribute must have a list".into());
    };
    let nested = parse_meta_list(&list.tokens)?;
    let condition = nested
        .first()
        .map(|meta| meta.to_token_stream().to_string());
    for meta in nested.iter().skip(1) {
        if let Meta::List(list) = meta {
            if list.path.is_ident("allow")
                && parse_meta_list(&list.tokens)?.iter().any(is_dead_code)
            {
                return Ok(Some((ChronicAllowanceKindV1::CfgAttrAllow, condition)));
            }
        }
    }
    Ok(None)
}

fn parse_meta_list(tokens: &proc_macro2::TokenStream) -> Result<Vec<Meta>, String> {
    Punctuated::<Meta, Token![,]>::parse_terminated
        .parse2(tokens.clone())
        .map(|metas| metas.into_iter().collect())
        .map_err(|error| error.to_string())
}

fn is_dead_code(meta: &Meta) -> bool {
    matches!(meta, Meta::Path(path) if path.is_ident("dead_code"))
}

fn metric_for_path(path: &syn::Path) -> Option<ChronicMetricV1> {
    let ident = path.segments.last()?.ident.to_string();
    match ident.as_str() {
        "panic" => Some(ChronicMetricV1::Panic),
        "todo" => Some(ChronicMetricV1::Todo),
        _ => None,
    }
}

fn item_attrs(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(value) => &value.attrs,
        Item::Enum(value) => &value.attrs,
        Item::ExternCrate(value) => &value.attrs,
        Item::Fn(value) => &value.attrs,
        Item::ForeignMod(value) => &value.attrs,
        Item::Impl(value) => &value.attrs,
        Item::Macro(value) => &value.attrs,
        Item::Mod(value) => &value.attrs,
        Item::Static(value) => &value.attrs,
        Item::Struct(value) => &value.attrs,
        Item::Trait(value) => &value.attrs,
        Item::TraitAlias(value) => &value.attrs,
        Item::Type(value) => &value.attrs,
        Item::Union(value) => &value.attrs,
        Item::Use(value) => &value.attrs,
        Item::Verbatim(_) => &[],
        _ => &[],
    }
}

fn item_key(item: &Item, stack: &[String]) -> String {
    let name = match item {
        Item::Const(value) => value.ident.to_string(),
        Item::Enum(value) => value.ident.to_string(),
        Item::ExternCrate(value) => value.ident.to_string(),
        Item::Fn(value) => value.sig.ident.to_string(),
        Item::Impl(_) => "<impl>".to_string(),
        Item::Macro(value) => value
            .ident
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "<macro>".into()),
        Item::Mod(value) => value.ident.to_string(),
        Item::Static(value) => value.ident.to_string(),
        Item::Struct(value) => value.ident.to_string(),
        Item::Trait(value) => value.ident.to_string(),
        Item::TraitAlias(value) => value.ident.to_string(),
        Item::Type(value) => value.ident.to_string(),
        Item::Union(value) => value.ident.to_string(),
        Item::Use(_) => "<use>".to_string(),
        Item::ForeignMod(_) => "<foreign>".to_string(),
        Item::Verbatim(_) => "<verbatim>".to_string(),
        _ => "<item>".to_string(),
    };
    if stack.is_empty() {
        name
    } else {
        format!("{}::{name}", stack.join("::"))
    }
}

fn impl_item_attrs(item: &syn::ImplItem) -> &[Attribute] {
    match item {
        syn::ImplItem::Const(value) => &value.attrs,
        syn::ImplItem::Fn(value) => &value.attrs,
        syn::ImplItem::Macro(value) => &value.attrs,
        syn::ImplItem::Type(value) => &value.attrs,
        syn::ImplItem::Verbatim(_) => &[],
        _ => &[],
    }
}

fn impl_item_name(item: &syn::ImplItem) -> String {
    match item {
        syn::ImplItem::Const(value) => value.ident.to_string(),
        syn::ImplItem::Fn(value) => value.sig.ident.to_string(),
        syn::ImplItem::Macro(value) => value.mac.path.to_token_stream().to_string(),
        syn::ImplItem::Type(value) => value.ident.to_string(),
        syn::ImplItem::Verbatim(_) => "<verbatim>".into(),
        _ => "<item>".into(),
    }
}

fn trait_item_attrs(item: &syn::TraitItem) -> &[Attribute] {
    match item {
        syn::TraitItem::Const(value) => &value.attrs,
        syn::TraitItem::Fn(value) => &value.attrs,
        syn::TraitItem::Macro(value) => &value.attrs,
        syn::TraitItem::Type(value) => &value.attrs,
        syn::TraitItem::Verbatim(_) => &[],
        _ => &[],
    }
}

fn trait_item_name(item: &syn::TraitItem) -> String {
    match item {
        syn::TraitItem::Const(value) => value.ident.to_string(),
        syn::TraitItem::Fn(value) => value.sig.ident.to_string(),
        syn::TraitItem::Macro(value) => value.mac.path.to_token_stream().to_string(),
        syn::TraitItem::Type(value) => value.ident.to_string(),
        syn::TraitItem::Verbatim(_) => "<verbatim>".into(),
        _ => "<item>".into(),
    }
}

fn foreign_item_attrs(item: &syn::ForeignItem) -> &[Attribute] {
    match item {
        syn::ForeignItem::Fn(value) => &value.attrs,
        syn::ForeignItem::Static(value) => &value.attrs,
        syn::ForeignItem::Type(value) => &value.attrs,
        syn::ForeignItem::Macro(value) => &value.attrs,
        syn::ForeignItem::Verbatim(_) => &[],
        _ => &[],
    }
}

fn foreign_item_name(item: &syn::ForeignItem) -> String {
    match item {
        syn::ForeignItem::Fn(value) => value.sig.ident.to_string(),
        syn::ForeignItem::Static(value) => value.ident.to_string(),
        syn::ForeignItem::Type(value) => value.ident.to_string(),
        syn::ForeignItem::Macro(value) => value.mac.path.to_token_stream().to_string(),
        syn::ForeignItem::Verbatim(_) => "<verbatim>".into(),
        _ => "<item>".into(),
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

fn full_source_range(source: &str) -> SourceRangeV1 {
    let line_starts = line_starts(source);
    let end_line = source.lines().count().max(1);
    let end_column = source.rsplit('\n').next().map(str::len).unwrap_or_default();
    SourceRangeV1 {
        start: PositionV1 { line: 1, column: 0 },
        end: PositionV1 {
            line: end_line,
            column: end_column,
        },
        byte_start: 0,
        byte_end: source.len().max(line_starts.last().copied().unwrap_or(0)),
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
    line_start
        + line_source
            .char_indices()
            .nth(column)
            .map(|(offset, _)| offset)
            .unwrap_or(line_source.len())
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn observation_sort_key(observation: &ChronicObservationV1) -> (usize, usize, String) {
    let range = match observation {
        ChronicObservationV1::CallSite { source_range, .. }
        | ChronicObservationV1::DeadCodeAllowance { source_range, .. }
        | ChronicObservationV1::ModuleEdge { source_range, .. }
        | ChronicObservationV1::OpaqueMacro { source_range, .. } => source_range,
    };
    (
        range.byte_start,
        range.byte_end,
        observation_kind_tag(observation).to_string(),
    )
}

fn observation_kind_tag(observation: &ChronicObservationV1) -> &'static str {
    match observation {
        ChronicObservationV1::CallSite { .. } => "call_site",
        ChronicObservationV1::DeadCodeAllowance { .. } => "dead_code_allowance",
        ChronicObservationV1::ModuleEdge { .. } => "module_edge",
        ChronicObservationV1::OpaqueMacro { .. } => "opaque_macro",
    }
}

fn hash_source_scope(files: &[ChronicFileObservationV1]) -> String {
    let mut hasher = Sha256::new();
    for file in files {
        hasher.update(file.path.as_bytes());
        hasher.update([0]);
        hasher.update(file.source_digest.as_bytes());
        hasher.update([0]);
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn evidence_hash(report: &ChronicMeasurementReportV1) -> Result<String, ChronicScanErrorV1> {
    let mut evidence = report.clone();
    evidence.evidence_hash.clear();
    let bytes =
        serde_json::to_vec(&evidence).map_err(|error| ChronicScanErrorV1::ReportSerialize {
            detail: error.to_string(),
        })?;
    Ok(sha256_bytes(&bytes))
}
