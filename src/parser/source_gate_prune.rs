//! Source-aware top-level build-gate validation and pruning for R6-S3B-B2/B3.
//!
//! The parser-issued ledger is the identity authority.  The original AST is
//! walked in structural preorder only to validate that ledger and to produce
//! the selected AST plus parser-private final Box paths; it never issues
//! source identity from its own ordinals.

use crate::ast::{ASTNode, Span};

use super::source_authority::SourceBuildGateBranchV1;
use super::source_gate_ledger::PreparedBuildGateSourceRecordV1;
use super::source_gate_receipt::BuildGateSelectionReceiptV1;
use super::source_path::{SourceBoxDeclarationPathV1, SourceBuildGatePathV1};
use super::{NyashParser, ParseError};

struct GateCursor<'a> {
    parser: &'a NyashParser,
    records: &'a [PreparedBuildGateSourceRecordV1],
    next: usize,
    receipts: Vec<BuildGateSelectionReceiptV1>,
}

impl<'a> GateCursor<'a> {
    fn new(parser: &'a NyashParser, records: &'a [PreparedBuildGateSourceRecordV1]) -> Self {
        Self {
            parser,
            records,
            next: 0,
            receipts: Vec::with_capacity(records.len()),
        }
    }

    fn consume(
        &mut self,
        predicate: &crate::ast::BuildPredicate,
        span: Span,
        path: SourceBuildGatePathV1,
    ) -> Result<
        (
            super::source_authority::SourceBuildGateIdV1,
            SourceBuildGateBranchV1,
        ),
        ParseError,
    > {
        let record = self
            .records
            .get(self.next)
            .ok_or_else(|| ParseError::BuildCfg {
                message: "AST build-gate cursor found a gate without a parser ledger record"
                    .to_owned(),
                line: span.line,
            })?;
        if record.gate_path != path || record.predicate != *predicate {
            return Err(ParseError::BuildCfg {
                message: "AST/parser build-gate source ledger mismatch".to_owned(),
                line: span.line,
            });
        }
        if record.scope != super::source_gate_ledger::SourceBuildGateScopeV1::TopLevelItem {
            return Err(ParseError::BuildCfg {
                message: "build-gate ledger record is outside the opened top-level scope"
                    .to_owned(),
                line: span.line,
            });
        }
        let selected = self.parser.eval_build_predicate(predicate, span)?;
        let branch = if selected {
            SourceBuildGateBranchV1::Then
        } else {
            SourceBuildGateBranchV1::Else
        };
        let decision_coordinate = u32::try_from(self.next).map_err(|_| ParseError::BuildCfg {
            message: "build-gate decision coordinate exceeds u32".to_owned(),
            line: span.line,
        })?;
        self.receipts
            .push(BuildGateSelectionReceiptV1::issue_from_decision(
                record,
                decision_coordinate,
                predicate,
                branch,
            ));
        self.next += 1;
        Ok((record.gate_id, branch))
    }

    fn finish(self) -> Result<Vec<BuildGateSelectionReceiptV1>, ParseError> {
        if self.next != self.records.len() {
            return Err(ParseError::BuildCfg {
                message: format!(
                    "parser build-gate ledger has {} unconsumed records",
                    self.records.len() - self.next
                ),
                line: 0,
            });
        }
        Ok(self.receipts)
    }
}

pub(super) fn prune_top_level_gate_program(
    parser: &NyashParser,
    ast: ASTNode,
    records: &[PreparedBuildGateSourceRecordV1],
) -> Result<GatePruneOutputV1, ParseError> {
    let ASTNode::Program { statements, span } = ast else {
        return Ok(GatePruneOutputV1 {
            ast,
            receipts: Vec::new(),
            final_box_paths: Vec::new(),
        });
    };
    let mut cursor = GateCursor::new(parser, records);
    let (statements, final_box_paths) =
        prune_items(&mut cursor, statements, None, None, None, None)?;
    let receipts = cursor.finish()?;
    Ok(GatePruneOutputV1 {
        ast: ASTNode::Program { statements, span },
        receipts,
        final_box_paths,
    })
}

#[derive(Debug)]
pub(super) struct GatePruneOutputV1 {
    pub(super) ast: ASTNode,
    pub(super) receipts: Vec<BuildGateSelectionReceiptV1>,
    pub(super) final_box_paths: Vec<SourceBoxDeclarationPathV1>,
}

fn prune_items(
    cursor: &mut GateCursor<'_>,
    items: Vec<ASTNode>,
    parent_gate_path: Option<&SourceBuildGatePathV1>,
    parent_box_path: Option<&SourceBoxDeclarationPathV1>,
    branch: Option<SourceBuildGateBranchV1>,
    parent_gate_id: Option<super::source_authority::SourceBuildGateIdV1>,
) -> Result<(Vec<ASTNode>, Vec<SourceBoxDeclarationPathV1>), ParseError> {
    let mut output = Vec::with_capacity(items.len());
    let mut final_box_paths = Vec::new();
    for (child_ordinal, item) in items.into_iter().enumerate() {
        let child_ordinal = u32::try_from(child_ordinal).map_err(|_| ParseError::BuildCfg {
            message: "AST source child ordinal exceeds u32".to_owned(),
            line: 0,
        })?;
        let box_path = match (parent_box_path, branch) {
            (None, None) => SourceBoxDeclarationPathV1::root(
                cursor.parser.source_invocation_brand(),
                child_ordinal,
            ),
            (Some(parent), Some(branch)) => parent.child(
                parent_gate_id.ok_or_else(|| ParseError::BuildCfg {
                    message: "missing parent build-gate id for Box source path".to_owned(),
                    line: 0,
                })?,
                branch,
                child_ordinal,
            ),
            _ => {
                return Err(ParseError::BuildCfg {
                    message: "invalid Box source path cursor state".to_owned(),
                    line: 0,
                })
            }
        };
        let ASTNode::BuildGate {
            predicate,
            then_items,
            else_items,
            span,
        } = item
        else {
            if matches!(&item, ASTNode::BoxDeclaration { .. }) {
                final_box_paths.push(box_path);
            }
            output.push(item);
            continue;
        };
        let gate_path = match (parent_gate_path, branch) {
            (None, None) => {
                SourceBuildGatePathV1::root(cursor.parser.source_invocation_brand(), child_ordinal)
            }
            (Some(parent), Some(branch)) => parent.child(
                parent_gate_id.ok_or_else(|| ParseError::BuildCfg {
                    message: "missing parent build-gate id in structural cursor".to_owned(),
                    line: span.line,
                })?,
                branch,
                child_ordinal,
            ),
            _ => {
                return Err(ParseError::BuildCfg {
                    message: "invalid build-gate structural cursor state".to_owned(),
                    line: span.line,
                })
            }
        };
        let (gate_id, selected) = cursor.consume(&predicate, span, gate_path.clone())?;
        let then_pruned = prune_items(
            cursor,
            then_items,
            Some(&gate_path),
            Some(&box_path),
            Some(SourceBuildGateBranchV1::Then),
            Some(gate_id),
        )?;
        let else_pruned = match else_items {
            Some(items) => Some(prune_items(
                cursor,
                items,
                Some(&gate_path),
                Some(&box_path),
                Some(SourceBuildGateBranchV1::Else),
                Some(gate_id),
            )?),
            None => None,
        };
        match selected {
            SourceBuildGateBranchV1::Then => {
                output.extend(then_pruned.0);
                final_box_paths.extend(then_pruned.1);
            }
            SourceBuildGateBranchV1::Else => {
                if let Some(else_pruned) = else_pruned {
                    output.extend(else_pruned.0);
                    final_box_paths.extend(else_pruned.1);
                }
            }
        }
    }
    Ok((output, final_box_paths))
}
