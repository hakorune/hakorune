//! Parser-private source receipt for one selected BuildGate.
//!
//! The receipt is emitted only after the shared decision projection has
//! matched a parser-issued source record.  It is transport evidence, not a
//! second predicate evaluator.

use crate::ast::BuildPredicate;

use super::source_authority::{ParserInvocationBrandV1, SourceBuildGateIdV1};
use super::source_gate_ledger::PreparedBuildGateSourceRecordV1;
use super::source_path::{SourceBuildGateBranchV1, SourceBuildGatePathV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BuildGateSelectionReceiptV1 {
    pub(super) brand: ParserInvocationBrandV1,
    pub(super) gate_id: SourceBuildGateIdV1,
    pub(super) gate_path: SourceBuildGatePathV1,
    pub(super) predicate: BuildPredicate,
    pub(super) decision_coordinate: u32,
    pub(super) selected_branch: SourceBuildGateBranchV1,
}

impl BuildGateSelectionReceiptV1 {
    pub(super) fn issue_from_decision(
        record: &PreparedBuildGateSourceRecordV1,
        decision_coordinate: u32,
        predicate: &BuildPredicate,
        selected_branch: SourceBuildGateBranchV1,
    ) -> Self {
        Self {
            brand: record.brand.clone(),
            gate_id: record.gate_id,
            gate_path: record.gate_path.clone(),
            predicate: predicate.clone(),
            decision_coordinate,
            selected_branch,
        }
    }
}
