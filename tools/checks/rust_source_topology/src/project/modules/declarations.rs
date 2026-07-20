use proc_macro2::Span;
use quote::ToTokens;
use syn::ext::IdentExt;
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{AttrStyle, Attribute, Item, ItemMod, LitStr, Token, UseTree};

use crate::project::CfgAttributeStreamInputRowV1;
use crate::{PositionV1, SourceRangeV1};

use super::error::ModuleTopologyErrorV1;

#[derive(Clone)]
pub(super) struct ModuleDeclarationV1 {
    pub ident_syntax: String,
    pub semantic_segment: String,
    pub range: SourceRangeV1,
    pub outer_attributes: Box<[Attribute]>,
    pub outer_topology_rows: Box<[CfgAttributeStreamInputRowV1]>,
    pub inline_body_range: Option<SourceRangeV1>,
    /// Parsed direct children of an inline body.  These are source syntax only:
    /// `CONTENTCFG0-I0` must classify the child's own inner stream before it
    /// issues `ModulePositionItemV1` declarations from them.
    pub inline_body_items: Option<Box<[Item]>>,
    pub include_macro_ambiguity: bool,
}

#[derive(Clone)]
pub(super) struct IncludeDeclarationV1 {
    pub range: SourceRangeV1,
    pub outer_attributes: Box<[Attribute]>,
    pub outer_topology_rows: Box<[CfgAttributeStreamInputRowV1]>,
    pub tokens: proc_macro2::TokenStream,
    pub include_macro_ambiguity: bool,
}

#[derive(Clone)]
pub(super) enum ModulePositionItemV1 {
    Module(ModuleDeclarationV1),
    Include(IncludeDeclarationV1),
}

pub(super) struct ParsedModuleSourceV1 {
    pub items: Box<[ModulePositionItemV1]>,
}

pub(super) fn parse_included_module_source_v1(
    path: &str,
    source: &str,
) -> Result<ParsedModuleSourceV1, ModuleTopologyErrorV1> {
    parse_source_v1(path, source, false, true)
}

/// Returns the direct raw syntax of one included fragment after the existing
/// fragment/preamble validation.  Test-only scope proof consumes this instead
/// of adding a second `syn::parse_file` authority.
#[cfg(test)]
pub(super) fn parse_included_direct_items_v1(
    path: &str,
    source: &str,
) -> Result<Box<[Item]>, ModuleTopologyErrorV1> {
    let file = parse_source_file_v1(path, source, true)?;
    reject_inner_topology_attributes(path, &file.attrs)?;
    Ok(file.items.into_boxed_slice())
}

/// Issues one direct declaration surface from already parsed, already admitted
/// module content.
///
/// Callers must have classified the enclosing file or inline body through the
/// CONTENTCFG0 gate first.  This function intentionally does not inspect an
/// inner attribute stream or recursively issue inline descendants.
pub(super) fn collect_direct_module_position_items_v1(
    path: &str,
    source: &str,
    items: &[Item],
    inherited_include_macro_ambiguity: bool,
) -> Result<ParsedModuleSourceV1, ModuleTopologyErrorV1> {
    let line_starts = line_starts(source);
    let direct_items = collect_module_position_items(
        items,
        &line_starts,
        source,
        path,
        inherited_include_macro_ambiguity,
    )?;
    reject_non_direct_topology_positions(path, source, items, &line_starts)?;
    Ok(ParsedModuleSourceV1 {
        items: direct_items.into_boxed_slice(),
    })
}

fn parse_source_v1(
    path: &str,
    source: &str,
    inherited_include_macro_ambiguity: bool,
    included_fragment: bool,
) -> Result<ParsedModuleSourceV1, ModuleTopologyErrorV1> {
    let file = parse_source_file_v1(path, source, included_fragment)?;
    reject_inner_topology_attributes(path, &file.attrs)?;
    collect_direct_module_position_items_v1(
        path,
        source,
        &file.items,
        inherited_include_macro_ambiguity,
    )
}

fn parse_source_file_v1(
    path: &str,
    source: &str,
    included_fragment: bool,
) -> Result<syn::File, ModuleTopologyErrorV1> {
    let file = syn::parse_file(source).map_err(|error| ModuleTopologyErrorV1::Parse {
        path: path.to_string(),
        detail: error.to_string(),
    })?;
    if included_fragment && (file.shebang.is_some() || !file.attrs.is_empty()) {
        return Err(ModuleTopologyErrorV1::UnsupportedIncludedPreamble {
            path: path.to_string(),
        });
    }
    Ok(file)
}

fn reject_non_direct_topology_positions(
    path: &str,
    source: &str,
    direct_items: &[Item],
    line_starts: &[usize],
) -> Result<(), ModuleTopologyErrorV1> {
    let mut nested = NestedTopologyPositionsV1 {
        line_starts,
        source,
        modules: Vec::new(),
        includes: Vec::new(),
    };
    for item in direct_items {
        match item {
            // A module's direct surface is checked only after its own content
            // gate includes it.  Do not descend through it here.
            Item::Mod(_) | Item::Macro(_) => {}
            other => nested.visit_item(other),
        }
    }
    if let Some(range) = nested.modules.into_iter().next() {
        return Err(ModuleTopologyErrorV1::ModuleInBlock {
            path: format!("{path}:{}..{}", range.byte_start, range.byte_end),
        });
    }
    if let Some(range) = nested.includes.into_iter().next() {
        return Err(ModuleTopologyErrorV1::UnsupportedIncludeContext {
            path: format!("{path}:{}..{}", range.byte_start, range.byte_end),
        });
    }
    Ok(())
}

pub(super) fn validate_module_attributes(
    module: &str,
    attributes: &[Attribute],
) -> Result<(), ModuleTopologyErrorV1> {
    for attribute in attributes {
        if !matches!(attribute.style, AttrStyle::Outer) {
            continue;
        }
        let path = attribute.path();
        let allowed = [
            "cfg",
            "cfg_attr",
            "path",
            "doc",
            "deprecated",
            "allow",
            "warn",
            "deny",
            "forbid",
            "expect",
            "no_implicit_prelude",
            "macro_use",
        ]
        .iter()
        .any(|name| path.is_ident(name));
        if !allowed {
            return Err(ModuleTopologyErrorV1::UnsupportedModuleAttribute {
                module: module.to_string(),
                attribute: path.to_token_stream().to_string(),
            });
        }
    }
    Ok(())
}

pub(super) fn validate_include_attributes(
    source_path: &str,
    attributes: &[Attribute],
) -> Result<(), ModuleTopologyErrorV1> {
    for attribute in attributes {
        if !matches!(attribute.style, AttrStyle::Outer) {
            continue;
        }
        let path = attribute.path();
        let allowed = [
            "cfg",
            "cfg_attr",
            "doc",
            "deprecated",
            "allow",
            "warn",
            "deny",
            "forbid",
            "expect",
        ]
        .iter()
        .any(|name| path.is_ident(name));
        if !allowed {
            return Err(ModuleTopologyErrorV1::UnsupportedIncludeAttribute {
                path: source_path.to_string(),
                attribute: path.to_token_stream().to_string(),
            });
        }
    }
    Ok(())
}

pub(super) fn include_literal(
    source_path: &str,
    declaration: &IncludeDeclarationV1,
) -> Result<String, ModuleTopologyErrorV1> {
    let parser = |input: syn::parse::ParseStream<'_>| {
        let value = input.parse::<LitStr>()?;
        let _ = input.parse::<Option<Token![,]>>()?;
        if !input.is_empty() {
            return Err(input.error("include! requires one literal path"));
        }
        Ok(value.value())
    };
    parser.parse2(declaration.tokens.clone()).map_err(|_| {
        ModuleTopologyErrorV1::NonLiteralInclude {
            path: source_path.to_string(),
        }
    })
}

fn collect_module_position_items(
    items: &[Item],
    line_starts: &[usize],
    source: &str,
    source_path: &str,
    inherited_include_macro_ambiguity: bool,
) -> Result<Vec<ModulePositionItemV1>, ModuleTopologyErrorV1> {
    let mut result = Vec::new();
    let scope_ambiguity =
        inherited_include_macro_ambiguity || items.iter().any(item_may_shadow_builtin_include);
    for item in items {
        if let Item::Macro(item_macro) = item {
            if item_macro.mac.path.is_ident("include") {
                result.push(ModulePositionItemV1::Include(IncludeDeclarationV1 {
                    range: source_range(item_macro.mac.span(), line_starts, source),
                    outer_attributes: item_macro
                        .attrs
                        .iter()
                        .filter(|attribute| matches!(attribute.style, AttrStyle::Outer))
                        .cloned()
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                    outer_topology_rows: collect_outer_topology_rows(
                        source_path,
                        &item_macro.attrs,
                        line_starts,
                        source,
                    )?,
                    tokens: item_macro.mac.tokens.clone(),
                    include_macro_ambiguity: scope_ambiguity,
                }));
            } else if macro_path_ends_with_include(&item_macro.mac.path) {
                let range = source_range(item_macro.mac.span(), line_starts, source);
                return Err(ModuleTopologyErrorV1::IncludeMacroIdentityUnresolved {
                    path: format!("{source_path}:{}..{}", range.byte_start, range.byte_end),
                });
            }
            continue;
        }
        let Item::Mod(item_mod) = item else {
            continue;
        };
        let range = source_range(item_mod.span(), line_starts, source);
        let inline_body_range = item_mod.content.as_ref().map(|(brace, _)| {
            range_between(brace.span.open(), brace.span.close(), line_starts, source)
        });
        let inline_body_items = item_mod
            .content
            .as_ref()
            .map(|(_, children)| children.clone().into_boxed_slice());
        result.push(ModulePositionItemV1::Module(ModuleDeclarationV1 {
            ident_syntax: item_mod.ident.to_string(),
            semantic_segment: item_mod.ident.unraw().to_string(),
            range,
            outer_attributes: item_mod
                .attrs
                .iter()
                .filter(|attribute| matches!(attribute.style, AttrStyle::Outer))
                .cloned()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            outer_topology_rows: collect_outer_topology_rows(
                source_path,
                &item_mod.attrs,
                line_starts,
                source,
            )?,
            inline_body_range,
            inline_body_items,
            include_macro_ambiguity: scope_ambiguity,
        }));
    }
    Ok(result)
}

pub(super) fn collect_outer_topology_rows(
    source_path: &str,
    attributes: &[Attribute],
    line_starts: &[usize],
    source: &str,
) -> Result<Box<[CfgAttributeStreamInputRowV1]>, ModuleTopologyErrorV1> {
    let mut rows = Vec::new();
    for attribute in attributes.iter().filter(|attribute| {
        matches!(attribute.style, AttrStyle::Outer)
            && (attribute.path().is_ident("cfg")
                || attribute.path().is_ident("cfg_attr")
                || attribute.path().is_ident("path"))
    }) {
        let source_ordinal = u32::try_from(rows.len()).map_err(|_| {
            ModuleTopologyErrorV1::AttributeOrdinalOverflow {
                path: source_path.to_string(),
            }
        })?;
        let source_range = source_range(attribute.meta.span(), line_starts, source);
        let syntax = source
            .get(source_range.byte_start..source_range.byte_end)
            .ok_or_else(|| ModuleTopologyErrorV1::AttributeRangeInvalid {
                path: source_path.to_string(),
                byte_start: source_range.byte_start,
                byte_end: source_range.byte_end,
            })?
            .to_string();
        rows.push(CfgAttributeStreamInputRowV1 {
            source_ordinal,
            source_range,
            syntax,
        });
    }
    Ok(rows.into_boxed_slice())
}

/// Projects one direct item's outer CFG surface through the declaration owner.
#[cfg(test)]
pub(super) fn collect_item_outer_topology_rows_v1(
    source_path: &str,
    attributes: &[Attribute],
    source: &str,
) -> Result<Box<[CfgAttributeStreamInputRowV1]>, ModuleTopologyErrorV1> {
    collect_outer_topology_rows(source_path, attributes, &line_starts(source), source)
}

/// Projects an exact direct-item range through the declaration source owner.
#[cfg(test)]
pub(super) fn direct_item_source_range_v1(item: &Item, source: &str) -> SourceRangeV1 {
    source_range(item.span(), &line_starts(source), source)
}

fn macro_path_ends_with_include(path: &syn::Path) -> bool {
    path.segments
        .last()
        .is_some_and(|segment| segment.ident == "include")
}

fn item_may_shadow_builtin_include(item: &Item) -> bool {
    match item {
        Item::Use(item_use) => use_tree_may_import_include(&item_use.tree),
        Item::Macro(item_macro) => {
            item_macro
                .ident
                .as_ref()
                .is_some_and(|ident| ident == "include")
                || item_macro
                    .attrs
                    .iter()
                    .any(|attribute| attribute.path().is_ident("macro_use"))
        }
        Item::ExternCrate(item_extern) => item_extern
            .attrs
            .iter()
            .any(|attribute| attribute.path().is_ident("macro_use")),
        Item::Mod(item_mod) => item_mod
            .attrs
            .iter()
            .any(|attribute| attribute.path().is_ident("macro_use")),
        _ => false,
    }
}

fn use_tree_may_import_include(tree: &UseTree) -> bool {
    match tree {
        UseTree::Path(path) => use_tree_may_import_include(&path.tree),
        UseTree::Name(name) => name.ident == "include",
        UseTree::Rename(rename) => rename.rename == "include",
        UseTree::Group(group) => group.items.iter().any(use_tree_may_import_include),
        UseTree::Glob(_) => true,
    }
}

fn reject_inner_topology_attributes(
    path: &str,
    attributes: &[Attribute],
) -> Result<(), ModuleTopologyErrorV1> {
    if attributes.iter().any(|attribute| {
        matches!(attribute.style, AttrStyle::Inner(_))
            && (attribute.path().is_ident("cfg")
                || attribute.path().is_ident("cfg_attr")
                || attribute.path().is_ident("path"))
    }) {
        return Err(ModuleTopologyErrorV1::UnsupportedInnerTopologyAttribute {
            path: path.to_string(),
        });
    }
    Ok(())
}

struct NestedTopologyPositionsV1<'source> {
    line_starts: &'source [usize],
    source: &'source str,
    modules: Vec<SourceRangeV1>,
    includes: Vec<SourceRangeV1>,
}

impl Visit<'_> for NestedTopologyPositionsV1<'_> {
    fn visit_item_mod(&mut self, item: &ItemMod) {
        self.modules
            .push(source_range(item.span(), self.line_starts, self.source));
        visit::visit_item_mod(self, item);
    }

    fn visit_macro(&mut self, mac: &syn::Macro) {
        if macro_path_ends_with_include(&mac.path) {
            self.includes
                .push(source_range(mac.span(), self.line_starts, self.source));
        }
        visit::visit_macro(self, mac);
    }
}

fn line_starts(source: &str) -> Vec<usize> {
    let mut starts = vec![0];
    for (offset, byte) in source.bytes().enumerate() {
        if byte == b'\n' {
            starts.push(offset + 1);
        }
    }
    starts
}

fn source_range(span: Span, starts: &[usize], source: &str) -> SourceRangeV1 {
    range_between(span, span, starts, source)
}

fn range_between(
    start_span: Span,
    end_span: Span,
    starts: &[usize],
    source: &str,
) -> SourceRangeV1 {
    let start = start_span.start();
    let end = end_span.end();
    SourceRangeV1 {
        start: PositionV1 {
            line: start.line,
            column: start.column,
        },
        end: PositionV1 {
            line: end.line,
            column: end.column,
        },
        byte_start: byte_offset(start.line, start.column, starts, source),
        byte_end: byte_offset(end.line, end.column, starts, source),
    }
}

fn byte_offset(line: usize, column: usize, starts: &[usize], source: &str) -> usize {
    starts
        .get(line.saturating_sub(1))
        .copied()
        .unwrap_or(source.len())
        .saturating_add(column)
        .min(source.len())
}
