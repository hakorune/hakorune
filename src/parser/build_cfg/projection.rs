//! One-way projection from the parser-owned BuildGate decision set.
//!
//! The shared postpass uses this selector with the generic AST walker in
//! `prune.rs`.  The selector never evaluates a predicate.  It consumes the
//! non-Clone decision rows in preorder, validates source-record identity, and
//! emits only the selected branch while still visiting inactive branches.

use crate::ast::{ASTNode, BuildPredicate, Span};
use crate::parser::build_gate_selection::BuildGateSelectionOutcomeV1;

use super::decision_set::{
    BuildGateDecisionRowV1, BuildGateReachabilityV1, PreparedBuildGateDecisionSetV1,
};
use super::prune::{project_build_gate_program, BuildGateProjectionSelector};
use crate::parser::source_gate_ledger::PreparedBuildGateSourceRecordV1;
use crate::parser::source_gate_receipt::BuildGateSelectionReceiptV1;
use crate::parser::{BuildGateExplainReport, NyashParser, ParseError};

#[cfg(test)]
#[path = "projection_tests.rs"]
mod projection_tests;

#[derive(Debug)]
pub(crate) struct BuildGateProjectionOutputV1 {
    pub(crate) ast: ASTNode,
    pub(crate) receipts: Vec<BuildGateSelectionReceiptV1>,
    pub(crate) explain: Option<BuildGateExplainReport>,
}

pub(crate) fn project_build_gates(
    parser: &NyashParser,
    ast: ASTNode,
    decisions: &PreparedBuildGateDecisionSetV1,
    records: &[PreparedBuildGateSourceRecordV1],
    capture_explain: bool,
) -> Result<BuildGateProjectionOutputV1, ParseError> {
    let mut selector = DecisionProjectionSelector::new(
        parser.source_invocation_brand(),
        decisions.rows(),
        records,
        capture_explain,
    );
    let ast = project_build_gate_program(ast, &mut selector)?;
    selector.finish(ast)
}

struct DecisionProjectionSelector<'a> {
    brand: crate::parser::source_authority::ParserInvocationBrandV1,
    rows: &'a [BuildGateDecisionRowV1],
    records: &'a [PreparedBuildGateSourceRecordV1],
    cursor: usize,
    receipts: Vec<BuildGateSelectionReceiptV1>,
    explain: Option<BuildGateExplainReport>,
}

impl<'a> DecisionProjectionSelector<'a> {
    fn new(
        brand: crate::parser::source_authority::ParserInvocationBrandV1,
        rows: &'a [BuildGateDecisionRowV1],
        records: &'a [PreparedBuildGateSourceRecordV1],
        capture_explain: bool,
    ) -> Self {
        Self {
            brand,
            rows,
            records,
            cursor: 0,
            receipts: Vec::with_capacity(records.len()),
            explain: capture_explain.then(BuildGateExplainReport::new),
        }
    }

    fn finish(self, ast: ASTNode) -> Result<BuildGateProjectionOutputV1, ParseError> {
        if self.cursor != self.rows.len() {
            return Err(projection_error(
                "BuildGate projection did not consume every decision row",
                0,
            ));
        }
        if self.receipts.len() != self.records.len() {
            return Err(projection_error(
                "BuildGate projection did not consume every source record",
                0,
            ));
        }
        Ok(BuildGateProjectionOutputV1 {
            ast,
            receipts: self.receipts,
            explain: self.explain,
        })
    }

    fn current_row(
        &self,
        predicate: &BuildPredicate,
        span: Span,
        has_else: bool,
        reachable: bool,
    ) -> Result<&BuildGateDecisionRowV1, ParseError> {
        let row = self.rows.get(self.cursor).ok_or_else(|| {
            projection_error("AST BuildGate has no matching decision row", span.line)
        })?;
        if row.brand != self.brand {
            return Err(projection_error(
                "foreign parser brand in BuildGate decision projection",
                span.line,
            ));
        }
        if row.coordinate != u32::try_from(self.cursor).unwrap_or(u32::MAX) {
            return Err(projection_error(
                "BuildGate decision coordinate is not in structural preorder",
                span.line,
            ));
        }
        if row.predicate != *predicate || row.span.line != span.line {
            return Err(projection_error(
                "AST BuildGate does not match its parser decision row",
                span.line,
            ));
        }
        let expected_reachability = if reachable {
            BuildGateReachabilityV1::Reachable
        } else {
            BuildGateReachabilityV1::InactiveSubtree
        };
        if row.reachability != expected_reachability {
            return Err(projection_error(
                "BuildGate reachability disagrees with the selected parent branch",
                span.line,
            ));
        }
        let shape_mismatch = match row.selected_branch {
            BuildGateSelectionOutcomeV1::NoElse => has_else,
            BuildGateSelectionOutcomeV1::Else => !has_else,
            BuildGateSelectionOutcomeV1::Then => false,
        };
        if shape_mismatch {
            return Err(projection_error(
                "BuildGate decision branch shape disagrees with the AST",
                span.line,
            ));
        }
        Ok(row)
    }

    fn issue_source_receipt(&mut self, row: &BuildGateDecisionRowV1) -> Result<(), ParseError> {
        let Some(gate_id) = row.gate_id else {
            return Ok(());
        };
        let Some(path) = row.source_path.as_ref() else {
            return Err(projection_error(
                "source BuildGate decision row has no source path",
                row.span.line,
            ));
        };
        let Some(record) = self.records.iter().find(|record| {
            record.gate_id == gate_id
                && record.gate_path == *path
                && record.predicate == row.predicate
        }) else {
            return Err(projection_error(
                "source BuildGate decision row has no matching source record",
                row.span.line,
            ));
        };
        if self
            .receipts
            .iter()
            .any(|receipt| receipt.gate_id == gate_id)
        {
            return Err(projection_error(
                "duplicate source BuildGate decision receipt",
                row.span.line,
            ));
        }
        self.receipts
            .push(BuildGateSelectionReceiptV1::issue_from_decision(
                record,
                row.coordinate,
                &row.predicate,
                row.selected_branch,
            ));
        Ok(())
    }
}

impl BuildGateProjectionSelector for DecisionProjectionSelector<'_> {
    fn select(
        &mut self,
        predicate: &BuildPredicate,
        span: Span,
        has_else: bool,
        reachable: bool,
    ) -> Result<bool, ParseError> {
        let row = self
            .current_row(predicate, span, has_else, reachable)?
            .clone();
        self.issue_source_receipt(&row)?;
        if reachable {
            if let Some(report) = self.explain.as_mut() {
                report.conditional_group_count += 1;
                match row.selected_branch {
                    BuildGateSelectionOutcomeV1::Then => {
                        report.active_branch_count += 1;
                        if has_else {
                            report.inactive_branch_count += 1;
                        }
                    }
                    BuildGateSelectionOutcomeV1::Else => {
                        report.active_branch_count += 1;
                        report.inactive_branch_count += 1;
                    }
                    BuildGateSelectionOutcomeV1::NoElse => {
                        report.inactive_branch_count += 1;
                    }
                }
            }
        }
        self.cursor += 1;
        Ok(matches!(
            row.selected_branch,
            BuildGateSelectionOutcomeV1::Then
        ))
    }

    fn visit_inactive_branches(&self) -> bool {
        true
    }
}

fn projection_error(message: impl Into<String>, line: usize) -> ParseError {
    ParseError::BuildCfg {
        message: message.into(),
        line,
    }
}
