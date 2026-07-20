//! File-scoped inner-attribute source surfaces.
//!
//! CFGSTREAM0-P0 needs exact source rows before CONTENTCFG0 decides whether a
//! file is a reachable content candidate. This module parses one complete Rust
//! file, preserves every file-level inner `cfg` / `cfg_attr` / `path` attribute
//! in source order, and owns no module traversal, content gate, or cfg decision
//! policy. Rust doc comments desugar to attributes but have comment-shaped raw
//! source; they are deliberately outside this topology-only surface.

use std::fmt;

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::AttrStyle;

use crate::{PositionV1, SourceRangeV1};

use super::fingerprint::sha256_bytes;
use super::model::{CfgAttributeStreamInputRowV1, FileInnerTopologyAttributeSurfaceV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InnerTopologyAttributeSurfaceErrorV1 {
    EmptySourcePath,
    Parse {
        source_path_workspace_relative: String,
        detail: String,
    },
    SourceOrdinalOverflow {
        source_path_workspace_relative: String,
    },
    SourceRangeInvalid {
        source_path_workspace_relative: String,
        byte_start: usize,
        byte_end: usize,
    },
}

impl fmt::Display for InnerTopologyAttributeSurfaceErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySourcePath => write!(formatter, "[rust-source-topology/inner-cfg/empty-source-path]"),
            Self::Parse {
                source_path_workspace_relative,
                detail,
            } => write!(
                formatter,
                "[rust-source-topology/inner-cfg/parse] path={source_path_workspace_relative} detail={detail}"
            ),
            Self::SourceOrdinalOverflow {
                source_path_workspace_relative,
            } => write!(
                formatter,
                "[rust-source-topology/inner-cfg/source-ordinal-overflow] path={source_path_workspace_relative}"
            ),
            Self::SourceRangeInvalid {
                source_path_workspace_relative,
                byte_start,
                byte_end,
            } => write!(
                formatter,
                "[rust-source-topology/inner-cfg/source-range-invalid] path={source_path_workspace_relative} start={byte_start} end={byte_end}"
            ),
        }
    }
}

impl std::error::Error for InnerTopologyAttributeSurfaceErrorV1 {}

/// Collects every file-level inner topology attribute in exact source order.
///
/// `syntax` is sliced from the original source at the parsed `Meta` span. It
/// is deliberately not reconstructed from token display, so comments and
/// whitespace remain available to the CFG stream parser as source evidence.
pub fn collect_file_inner_topology_attribute_surface_v1(
    source_path_workspace_relative: &str,
    source: &str,
) -> Result<FileInnerTopologyAttributeSurfaceV1, InnerTopologyAttributeSurfaceErrorV1> {
    if source_path_workspace_relative.is_empty() {
        return Err(InnerTopologyAttributeSurfaceErrorV1::EmptySourcePath);
    }
    let file =
        syn::parse_file(source).map_err(|error| InnerTopologyAttributeSurfaceErrorV1::Parse {
            source_path_workspace_relative: source_path_workspace_relative.to_string(),
            detail: error.to_string(),
        })?;
    let line_starts = line_starts(source);
    let mut rows = Vec::new();
    let mut inner_cfg_count = 0;
    let mut inner_cfg_attr_count = 0;
    let mut inner_path_count = 0;

    for attribute in file.attrs.iter().filter(|attribute| {
        matches!(attribute.style, AttrStyle::Inner(_))
            && (attribute.path().is_ident("cfg")
                || attribute.path().is_ident("cfg_attr")
                || attribute.path().is_ident("path"))
    }) {
        let source_ordinal = u32::try_from(rows.len()).map_err(|_| {
            InnerTopologyAttributeSurfaceErrorV1::SourceOrdinalOverflow {
                source_path_workspace_relative: source_path_workspace_relative.to_string(),
            }
        })?;
        if attribute.path().is_ident("cfg") {
            inner_cfg_count += 1;
        } else if attribute.path().is_ident("cfg_attr") {
            inner_cfg_attr_count += 1;
        } else if attribute.path().is_ident("path") {
            inner_path_count += 1;
        }
        let source_range = source_range(attribute.meta.span(), &line_starts, source);
        let syntax = source
            .get(source_range.byte_start..source_range.byte_end)
            .ok_or_else(
                || InnerTopologyAttributeSurfaceErrorV1::SourceRangeInvalid {
                    source_path_workspace_relative: source_path_workspace_relative.to_string(),
                    byte_start: source_range.byte_start,
                    byte_end: source_range.byte_end,
                },
            )?
            .to_string();
        rows.push(CfgAttributeStreamInputRowV1 {
            source_ordinal,
            source_range,
            syntax,
        });
    }

    Ok(FileInnerTopologyAttributeSurfaceV1 {
        source_path_workspace_relative: source_path_workspace_relative.to_string(),
        source_digest: sha256_bytes(source.as_bytes()),
        rows: rows.into_boxed_slice(),
        inner_cfg_count,
        inner_cfg_attr_count,
        inner_path_count,
    })
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
