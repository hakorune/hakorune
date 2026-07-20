use std::collections::BTreeSet;

use proc_macro2::Span;
use quote::ToTokens;
use syn::ext::IdentExt;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{AttrStyle, Attribute, Item, ItemMod, Meta};

use crate::{PositionV1, SourceRangeV1};

use super::error::ModuleTopologyErrorV1;

#[derive(Clone)]
pub(super) struct ModuleDeclarationV1 {
    pub ident_syntax: String,
    pub semantic_segment: String,
    pub range: SourceRangeV1,
    pub outer_attributes: Box<[Attribute]>,
    pub inline_body_range: Option<SourceRangeV1>,
    pub inline_children: Option<Box<[ModuleDeclarationV1]>>,
}

pub(super) struct ParsedModuleSourceV1 {
    pub declarations: Box<[ModuleDeclarationV1]>,
}

pub(super) fn parse_module_source_v1(
    path: &str,
    source: &str,
) -> Result<ParsedModuleSourceV1, ModuleTopologyErrorV1> {
    let file = syn::parse_file(source).map_err(|error| ModuleTopologyErrorV1::Parse {
        path: path.to_string(),
        detail: error.to_string(),
    })?;
    reject_inner_topology_attributes(path, &file.attrs)?;
    let line_starts = line_starts(source);
    let declarations = collect_module_position_items(&file.items, &line_starts, source)?;
    let accepted_ranges = flatten_ranges(&declarations);
    let mut all_modules = AllModuleRangesV1 {
        line_starts: &line_starts,
        source,
        ranges: Vec::new(),
    };
    all_modules.visit_file(&file);
    if let Some(range) = all_modules
        .ranges
        .into_iter()
        .find(|range| !accepted_ranges.contains(range))
    {
        return Err(ModuleTopologyErrorV1::ModuleInBlock {
            path: format!("{path}:{}..{}", range.byte_start, range.byte_end),
        });
    }
    Ok(ParsedModuleSourceV1 {
        declarations: declarations.into_boxed_slice(),
    })
}

pub(super) fn outer_cfg_syntax(attributes: &[Attribute]) -> Vec<String> {
    attributes
        .iter()
        .filter(|attribute| {
            matches!(attribute.style, AttrStyle::Outer)
                && (attribute.path().is_ident("cfg") || attribute.path().is_ident("cfg_attr"))
        })
        .map(|attribute| attribute.meta.to_token_stream().to_string())
        .collect()
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

pub(super) fn direct_path_literal(
    module: &str,
    attribute: &Attribute,
) -> Result<Option<String>, ModuleTopologyErrorV1> {
    if !attribute.path().is_ident("path") {
        return Ok(None);
    }
    let Meta::NameValue(name_value) = &attribute.meta else {
        return Err(ModuleTopologyErrorV1::NonLiteralPath {
            module: module.to_string(),
        });
    };
    let syn::Expr::Lit(expression) = &name_value.value else {
        return Err(ModuleTopologyErrorV1::NonLiteralPath {
            module: module.to_string(),
        });
    };
    let syn::Lit::Str(value) = &expression.lit else {
        return Err(ModuleTopologyErrorV1::NonLiteralPath {
            module: module.to_string(),
        });
    };
    Ok(Some(value.value()))
}

fn collect_module_position_items(
    items: &[Item],
    line_starts: &[usize],
    source: &str,
) -> Result<Vec<ModuleDeclarationV1>, ModuleTopologyErrorV1> {
    let mut declarations = Vec::new();
    for item in items {
        let Item::Mod(item_mod) = item else {
            continue;
        };
        reject_inner_topology_attributes("inline-module", &item_mod.attrs)?;
        let range = source_range(item_mod.span(), line_starts, source);
        let inline_body_range = item_mod.content.as_ref().map(|(brace, _)| {
            range_between(brace.span.open(), brace.span.close(), line_starts, source)
        });
        let inline_children = item_mod
            .content
            .as_ref()
            .map(|(_, children)| collect_module_position_items(children, line_starts, source))
            .transpose()?
            .map(Vec::into_boxed_slice);
        declarations.push(ModuleDeclarationV1 {
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
            inline_body_range,
            inline_children,
        });
    }
    Ok(declarations)
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

fn flatten_ranges(declarations: &[ModuleDeclarationV1]) -> BTreeSet<SourceRangeV1> {
    fn collect(rows: &[ModuleDeclarationV1], result: &mut BTreeSet<SourceRangeV1>) {
        for row in rows {
            result.insert(row.range);
            if let Some(children) = &row.inline_children {
                collect(children, result);
            }
        }
    }
    let mut result = BTreeSet::new();
    collect(declarations, &mut result);
    result
}

struct AllModuleRangesV1<'source> {
    line_starts: &'source [usize],
    source: &'source str,
    ranges: Vec<SourceRangeV1>,
}

impl Visit<'_> for AllModuleRangesV1<'_> {
    fn visit_item_mod(&mut self, item: &ItemMod) {
        self.ranges
            .push(source_range(item.span(), self.line_starts, self.source));
        visit::visit_item_mod(self, item);
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
