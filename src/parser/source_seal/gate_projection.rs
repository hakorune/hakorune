use crate::ast::ASTNode;
use crate::parser::{NyashParser, ParserMetadata};

use super::super::build_cfg::decision_set::PreparedBuildGateDecisionSetV1;
use super::super::build_cfg::project_build_gates;
use super::super::callable_gate_projection::prune_direct_callable_rows;
use super::super::callable_source_anchor::PreparedDirectCallableSourceV1;
use super::super::delegate_source_relation::GeneratedDelegateSourceRelationV1;
use super::super::source_gate_ledger::PreparedBuildGateSourceRecordV1;
use super::super::source_gate_receipt::{selection_matches_path, BuildGateSelectionReceiptV1};
use super::super::source_path::{
    SourceBoxDeclarationPathV1, SourceBoxPathSegmentV1, SourceBuildGatePathV1,
};
use super::model::{
    OpenParserPostpassProductV1, ParserSourceSessionV1, PreparedBoxSourceSealV1,
    PreparedParserSourcePruneV1,
};

impl ParserSourceSessionV1 {
    pub(in crate::parser) fn from_prepared(
        prepared_source_seals: Vec<PreparedBoxSourceSealV1>,
        gate_records: Vec<PreparedBuildGateSourceRecordV1>,
        direct_callable_rows: Vec<PreparedDirectCallableSourceV1>,
    ) -> Self {
        Self {
            prepared_source_seals,
            gate_records,
            selection_receipts: Vec::new(),
            direct_callable_rows,
        }
    }

    pub(in crate::parser) fn gate_records(&self) -> &[PreparedBuildGateSourceRecordV1] {
        &self.gate_records
    }

    pub(in crate::parser) fn attach_generated_delegate_relations(
        &mut self,
        box_path: &SourceBoxDeclarationPathV1,
        relations: Box<[GeneratedDelegateSourceRelationV1]>,
    ) -> Result<(), String> {
        let Some(seal) = self
            .prepared_source_seals
            .iter_mut()
            .find(|seal| seal.box_site.path() == box_path)
        else {
            return Err("generated delegate relation host path is absent".to_owned());
        };
        if !seal.generated_delegate_source_relations.is_empty() {
            return Err("generated delegate relation host is already committed".to_owned());
        }
        seal.generated_delegate_source_relations = relations;
        Ok(())
    }

    pub(in crate::parser) fn prepare_prune(
        self,
        receipts: Vec<BuildGateSelectionReceiptV1>,
    ) -> Result<PreparedParserSourcePruneV1, String> {
        validate_gate_receipts(&self.gate_records, &receipts)?;
        let mut retained = Vec::with_capacity(self.prepared_source_seals.len());
        for seal in self.prepared_source_seals {
            if declaration_path_survives(
                seal.box_site.path(),
                &self.gate_records,
                &receipts,
                Some(&seal.brand),
            )? {
                retained.push(seal);
            }
        }
        let direct_callable_rows =
            prune_direct_callable_rows(self.direct_callable_rows, &retained, |declaration| {
                declaration_path_survives(
                    declaration.compatibility_box_path(),
                    &self.gate_records,
                    &receipts,
                    Some(declaration.brand()),
                )
            })?;
        Ok(PreparedParserSourcePruneV1 {
            prepared_source_seals: retained,
            gate_records: self.gate_records,
            selection_receipts: receipts,
            direct_callable_rows,
        })
    }

    pub(in crate::parser) fn commit_prune(prepared: PreparedParserSourcePruneV1) -> Self {
        Self {
            prepared_source_seals: prepared.prepared_source_seals,
            gate_records: prepared.gate_records,
            selection_receipts: prepared.selection_receipts,
            direct_callable_rows: prepared.direct_callable_rows,
        }
    }

    pub(in crate::parser) fn into_parts(
        self,
    ) -> (
        Vec<PreparedBoxSourceSealV1>,
        Vec<PreparedDirectCallableSourceV1>,
    ) {
        (self.prepared_source_seals, self.direct_callable_rows)
    }

    pub(in crate::parser) fn direct_callable_rows(&self) -> &[PreparedDirectCallableSourceV1] {
        &self.direct_callable_rows
    }
}

fn validate_gate_receipts(
    records: &[PreparedBuildGateSourceRecordV1],
    receipts: &[BuildGateSelectionReceiptV1],
) -> Result<(), String> {
    if records.len() != receipts.len() {
        return Err(format!(
            "build-gate receipt coverage mismatch: records={}, receipts={}",
            records.len(),
            receipts.len()
        ));
    }
    for (index, record) in records.iter().enumerate() {
        if record.scope != super::super::source_gate_ledger::SourceBuildGateScopeV1::TopLevelItem {
            return Err(
                "build-gate source record is outside the opened top-level scope".to_owned(),
            );
        }
        if records[..index].iter().any(|previous| {
            previous.gate_id == record.gate_id || previous.gate_path == record.gate_path
        }) {
            return Err("duplicate build-gate source record id/path".to_owned());
        }
        if let Some(receipt) = receipts.iter().find(|receipt| {
            receipt.gate_id == record.gate_id && receipt.gate_path == record.gate_path
        }) {
            if receipt.brand != record.brand {
                return Err("foreign parser brand in build-gate receipt".to_owned());
            }
            if receipt.predicate != record.predicate {
                return Err("build-gate receipt predicate disagrees with source record".to_owned());
            }
        } else {
            return Err("missing build-gate selection receipt".to_owned());
        }
    }
    for (index, receipt) in receipts.iter().enumerate() {
        if receipts[..index].iter().any(|previous| {
            previous.gate_id == receipt.gate_id || previous.gate_path == receipt.gate_path
        }) {
            return Err("duplicate build-gate selection receipt id/path".to_owned());
        }
        if !records.iter().any(|record| {
            record.gate_id == receipt.gate_id && record.gate_path == receipt.gate_path
        }) {
            return Err("foreign build-gate selection receipt".to_owned());
        }
    }
    Ok(())
}

fn declaration_path_survives(
    path: &SourceBoxDeclarationPathV1,
    records: &[PreparedBuildGateSourceRecordV1],
    receipts: &[BuildGateSelectionReceiptV1],
    expected_brand: Option<&super::super::source_authority::ParserInvocationBrandV1>,
) -> Result<bool, String> {
    for (segment_index, segment) in path.segments().iter().enumerate() {
        let SourceBoxPathSegmentV1::BuildGate {
            gate_id, branch, ..
        } = segment
        else {
            continue;
        };
        let gate_path = SourceBuildGatePathV1::from_box_prefix(path, segment_index)
            .ok_or_else(|| "cannot derive gate path from Box source path".to_owned())?;
        let record = records
            .iter()
            .find(|record| record.gate_id == *gate_id && record.gate_path == gate_path)
            .ok_or_else(|| "Box source seal references an unknown build gate".to_owned())?;
        if expected_brand.is_some_and(|brand| !brand.same_as(&record.brand)) {
            return Err("foreign parser brand in declaration gate relation".to_owned());
        }
        let receipt = receipts
            .iter()
            .find(|receipt| receipt.gate_id == *gate_id && receipt.gate_path == gate_path)
            .ok_or_else(|| "Box source seal has no build-gate selection receipt".to_owned())?;
        if receipt.brand != record.brand
            || receipt.predicate != record.predicate
            || !selection_matches_path(receipt.selected_branch, *branch)
        {
            return Ok(false);
        }
    }
    Ok(true)
}

fn source_prune_error(message: String) -> crate::parser::ParseError {
    crate::parser::ParseError::BuildCfg { message, line: 0 }
}

impl OpenParserPostpassProductV1 {
    pub(in crate::parser) fn new(
        ast: ASTNode,
        prepared_source_seals: Vec<PreparedBoxSourceSealV1>,
        gate_records: Vec<PreparedBuildGateSourceRecordV1>,
        direct_callable_rows: Vec<PreparedDirectCallableSourceV1>,
        metadata: ParserMetadata,
        build_gate_decision_set: PreparedBuildGateDecisionSetV1,
    ) -> Self {
        Self {
            ast,
            source_session: ParserSourceSessionV1::from_prepared(
                prepared_source_seals,
                gate_records,
                direct_callable_rows,
            ),
            final_box_paths: Vec::new(),
            build_gate_decision_set,
            explain: None,
            metadata,
        }
    }

    pub(in crate::parser) fn prune_build_gates(
        self,
        parser: &NyashParser,
    ) -> Result<Self, crate::parser::ParseError> {
        self.prune_build_gates_with_explain(parser, false)
    }

    pub(in crate::parser) fn prune_build_gates_with_explain(
        self,
        parser: &NyashParser,
        capture_explain: bool,
    ) -> Result<Self, crate::parser::ParseError> {
        let Self {
            ast,
            source_session,
            final_box_paths: _,
            build_gate_decision_set,
            explain: _,
            metadata,
        } = self;
        let projection = project_build_gates(
            parser,
            ast,
            &build_gate_decision_set,
            source_session.gate_records(),
            capture_explain,
        )?;
        let prepared = source_session
            .prepare_prune(projection.receipts)
            .map_err(source_prune_error)?;
        let final_box_paths = prepared.retained_box_paths();
        let source_session = ParserSourceSessionV1::commit_prune(prepared);
        Ok(Self {
            ast: projection.ast,
            source_session,
            final_box_paths,
            build_gate_decision_set,
            explain: projection.explain,
            metadata,
        })
    }

    pub(in crate::parser) fn lower_delegates(self) -> Result<Self, crate::parser::ParseError> {
        super::super::delegate_batch::lower_delegates(self)
    }

    pub(in crate::parser) fn commit_generated_delegate_batch(
        self,
        ast: ASTNode,
        relation_batches: Vec<(
            SourceBoxDeclarationPathV1,
            Box<[GeneratedDelegateSourceRelationV1]>,
        )>,
    ) -> Result<Self, String> {
        let mut source_session = self.source_session;
        for (path, relations) in relation_batches {
            source_session.attach_generated_delegate_relations(&path, relations)?;
        }
        Ok(Self {
            ast,
            source_session,
            ..self
        })
    }
}
