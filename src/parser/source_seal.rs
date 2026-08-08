//! Final parser source product for the bounded R6-S3 slice.
//!
//! This module owns the post-prune/post-delegate boundary.  The ordinary
//! parser transaction issues only a prepared payload; the postpass product in
//! this module is the only owner that can compare that payload with the final
//! AST inventory and issue the non-Clone source seal.

use crate::ast::{
    ASTNode, BoxMethodGeneratedProvenanceV1, BoxMethodInventoryErrorV1, BoxMethodInventoryV1,
    BoxMethodProvenanceV1,
};
use crate::parser::ParserMetadata;

use super::delegate_source_relation::GeneratedDelegateSourceRelationV1;
use super::source_authority::{
    DelegateSourceDeclarationV1, MethodSourceRelationV1, ParserInvocationBrandV1,
    SourceBoxDeclarationSiteV1,
};
use super::source_gate_ledger::PreparedBuildGateSourceRecordV1;
use super::source_path::{
    SourceBoxDeclarationPathV1, SourceBoxPathSegmentV1, SourceBuildGateBranchV1,
    SourceBuildGatePathV1,
};
use super::source_seal_finalizer::GeneratedDelegateCoverageErrorV1;
use super::NyashParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BuildGateSelectionReceiptV1 {
    brand: ParserInvocationBrandV1,
    gate_id: super::source_authority::SourceBuildGateIdV1,
    gate_path: SourceBuildGatePathV1,
    selected_branch: SourceBuildGateBranchV1,
}

#[cfg(test)]
#[path = "source_seal_delegate_tests.rs"]
mod source_seal_delegate_tests;

#[cfg(test)]
#[path = "source_seal_misc_tests.rs"]
mod source_seal_misc_tests;

#[cfg(test)]
#[path = "source_seal_finalizer_tests.rs"]
mod source_seal_finalizer_tests;

impl BuildGateSelectionReceiptV1 {
    pub(super) fn issue(
        record: &PreparedBuildGateSourceRecordV1,
        selected_branch: SourceBuildGateBranchV1,
    ) -> Self {
        Self {
            brand: record.brand.clone(),
            gate_id: record.gate_id,
            gate_path: record.gate_path.clone(),
            selected_branch,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum SourceSealFinalizationErrorV1 {
    TopLevelBuildGateUnsupported,
    UnsupportedTopLevelBoxKind { ordinal: usize },
    OrdinaryBoxCountMismatch { prepared: usize, final_ast: usize },
    FinalInventoryShorter { prepared: usize, final_ast: usize },
    InventoryPrefixMismatch { ordinal: usize },
    UnexpectedGeneratedRow { ordinal: usize },
    FinalAstBoxPathCoverageMismatch { prepared: usize, final_ast: usize },
    DuplicateFinalAstBoxPath { final_index: usize },
    PreparedBoxPathMissing { prepared_index: usize },
    ForeignFinalAstBoxPath { final_index: usize },
    GeneratedDelegateCoverage(GeneratedDelegateCoverageErrorV1),
    Inventory(BoxMethodInventoryErrorV1),
}

#[derive(Debug)]
pub(super) struct PreparedBoxSourceSealV1 {
    pub(super) brand: ParserInvocationBrandV1,
    pub(super) box_site: SourceBoxDeclarationSiteV1,
    pub(super) inventory: BoxMethodInventoryV1,
    pub(super) method_relations: Box<[MethodSourceRelationV1]>,
    pub(super) delegate_source_declarations: Box<[DelegateSourceDeclarationV1]>,
    pub(super) generated_delegate_source_relations: Box<[GeneratedDelegateSourceRelationV1]>,
}

impl PreparedBoxSourceSealV1 {
    pub(super) fn inventory(&self) -> &BoxMethodInventoryV1 {
        &self.inventory
    }

    pub(super) fn method_relations(&self) -> &[MethodSourceRelationV1] {
        &self.method_relations
    }

    pub(super) fn delegate_source_declarations(&self) -> &[DelegateSourceDeclarationV1] {
        &self.delegate_source_declarations
    }

    pub(super) fn generated_delegate_source_relations(
        &self,
    ) -> &[GeneratedDelegateSourceRelationV1] {
        &self.generated_delegate_source_relations
    }

    pub(super) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.box_site
    }

    /// Consume the prepared payload only after the postpass has produced the
    /// final inventory and the generated-delegate relation coverage is exact.
    pub(super) fn finalize_against(
        self,
        final_inventory: &BoxMethodInventoryV1,
    ) -> Result<ParserBoxSourceSealV1, SourceSealFinalizationErrorV1> {
        let prepared_entries = self.inventory.clone().into_selected_declaration_order();
        let final_entries = final_inventory.clone().into_selected_declaration_order();
        if final_entries.len() < prepared_entries.len() {
            return Err(SourceSealFinalizationErrorV1::FinalInventoryShorter {
                prepared: prepared_entries.len(),
                final_ast: final_entries.len(),
            });
        }

        for (ordinal, (prepared, final_entry)) in prepared_entries
            .iter()
            .zip(final_entries.iter())
            .enumerate()
        {
            if prepared != final_entry {
                return Err(SourceSealFinalizationErrorV1::InventoryPrefixMismatch { ordinal });
            }
        }

        for (ordinal, entry) in final_entries
            .iter()
            .enumerate()
            .skip(prepared_entries.len())
        {
            if !matches!(
                entry.provenance(),
                BoxMethodProvenanceV1::Generated(BoxMethodGeneratedProvenanceV1::Delegate { .. })
            ) {
                return Err(SourceSealFinalizationErrorV1::UnexpectedGeneratedRow { ordinal });
            }
        }

        super::source_seal_finalizer::validate_generated_delegate_coverage(&self, final_inventory)
            .map_err(SourceSealFinalizationErrorV1::GeneratedDelegateCoverage)?;

        Ok(ParserBoxSourceSealV1 {
            prepared: PreparedBoxSourceSealV1 {
                brand: self.brand,
                box_site: self.box_site,
                inventory: self.inventory,
                method_relations: self.method_relations,
                delegate_source_declarations: Box::new([]),
                generated_delegate_source_relations: self.generated_delegate_source_relations,
            },
        })
    }
}

/// Single typed owner for the AST/source handoff between parser postpasses.
///
/// S3B-A intentionally keeps the existing direct ordinary-Box prune/delegate
/// behavior. The important boundary is that those postpasses consume and
/// return this product instead of separately mutating an AST and a seal list.
/// Gate path rebasing and private target lookup are parser-private slices;
/// final generated relation coverage is completed by the sole finalizer below.
/// This product is not resolver authority until that final relation coverage
/// succeeds.
#[derive(Debug)]
pub(super) struct OpenParserPostpassProductV1 {
    pub(super) ast: ASTNode,
    pub(super) source_session: ParserSourceSessionV1,
    pub(super) final_box_paths: Vec<SourceBoxDeclarationPathV1>,
    metadata: ParserMetadata,
}

/// Parser-owned source transport for the open postpass product. This wrapper
/// keeps prepared payload storage behind a named session boundary; generated
/// delegate relations stay parser-private until finalizer coverage succeeds.
#[derive(Debug)]
pub(super) struct ParserSourceSessionV1 {
    pub(super) prepared_source_seals: Vec<PreparedBoxSourceSealV1>,
    gate_records: Vec<PreparedBuildGateSourceRecordV1>,
    selection_receipts: Vec<BuildGateSelectionReceiptV1>,
}

#[derive(Debug)]
pub(super) struct PreparedParserSourcePruneV1 {
    prepared_source_seals: Vec<PreparedBoxSourceSealV1>,
    selection_receipts: Vec<BuildGateSelectionReceiptV1>,
}

impl ParserSourceSessionV1 {
    pub(super) fn from_prepared(
        prepared_source_seals: Vec<PreparedBoxSourceSealV1>,
        gate_records: Vec<PreparedBuildGateSourceRecordV1>,
    ) -> Self {
        Self {
            prepared_source_seals,
            gate_records,
            selection_receipts: Vec::new(),
        }
    }

    pub(super) fn gate_records(&self) -> &[PreparedBuildGateSourceRecordV1] {
        &self.gate_records
    }

    pub(super) fn attach_generated_delegate_relations(
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

    pub(super) fn prepare_prune(
        &self,
        receipts: &[BuildGateSelectionReceiptV1],
    ) -> Result<PreparedParserSourcePruneV1, String> {
        validate_gate_receipts(&self.gate_records, receipts)?;
        let mut retained = Vec::with_capacity(self.prepared_source_seals.len());
        for seal in &self.prepared_source_seals {
            if source_seal_survives(seal, &self.gate_records, receipts)? {
                retained.push(clone_prepared_source_seal(seal));
            }
        }
        Ok(PreparedParserSourcePruneV1 {
            prepared_source_seals: retained,
            selection_receipts: receipts.to_vec(),
        })
    }

    pub(super) fn commit_prune(self, prepared: PreparedParserSourcePruneV1) -> Self {
        Self {
            prepared_source_seals: prepared.prepared_source_seals,
            gate_records: self.gate_records,
            selection_receipts: prepared.selection_receipts,
        }
    }

    fn into_prepared(self) -> Vec<PreparedBoxSourceSealV1> {
        self.prepared_source_seals
    }
}

fn clone_prepared_source_seal(seal: &PreparedBoxSourceSealV1) -> PreparedBoxSourceSealV1 {
    PreparedBoxSourceSealV1 {
        brand: seal.brand.clone(),
        box_site: seal.box_site.clone(),
        inventory: seal.inventory.clone(),
        method_relations: seal.method_relations.clone(),
        delegate_source_declarations: seal.delegate_source_declarations.clone(),
        generated_delegate_source_relations: seal.generated_delegate_source_relations.clone(),
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
        if record.scope != super::source_gate_ledger::SourceBuildGateScopeV1::TopLevelItem {
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

fn source_seal_survives(
    seal: &PreparedBoxSourceSealV1,
    records: &[PreparedBuildGateSourceRecordV1],
    receipts: &[BuildGateSelectionReceiptV1],
) -> Result<bool, String> {
    let path = seal.box_site.path();
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
        if seal.brand != record.brand {
            return Err("foreign parser brand in Box source seal gate relation".to_owned());
        }
        let receipt = receipts
            .iter()
            .find(|receipt| receipt.gate_id == *gate_id && receipt.gate_path == gate_path)
            .ok_or_else(|| "Box source seal has no build-gate selection receipt".to_owned())?;
        if receipt.brand != record.brand || receipt.selected_branch != *branch {
            return Ok(false);
        }
    }
    Ok(true)
}

fn source_prune_error(message: String) -> crate::parser::ParseError {
    crate::parser::ParseError::BuildCfg { message, line: 0 }
}

impl OpenParserPostpassProductV1 {
    pub(super) fn new(
        ast: ASTNode,
        prepared_source_seals: Vec<PreparedBoxSourceSealV1>,
        gate_records: Vec<PreparedBuildGateSourceRecordV1>,
        metadata: ParserMetadata,
    ) -> Self {
        Self {
            ast,
            source_session: ParserSourceSessionV1::from_prepared(
                prepared_source_seals,
                gate_records,
            ),
            final_box_paths: Vec::new(),
            metadata,
        }
    }

    pub(super) fn prune_build_gates(
        self,
        parser: &NyashParser,
    ) -> Result<Self, crate::parser::ParseError> {
        let Self {
            ast,
            source_session,
            final_box_paths: _,
            metadata,
        } = self;
        let pruned = super::source_gate_prune::prune_top_level_gate_program(
            parser,
            ast,
            source_session.gate_records(),
        )?;
        let super::source_gate_prune::GatePruneOutputV1 {
            ast,
            receipts,
            final_box_paths,
        } = pruned;
        let ast = parser.prune_build_gate_program(ast)?;
        let prepared = source_session
            .prepare_prune(&receipts)
            .map_err(source_prune_error)?;
        let source_session = source_session.commit_prune(prepared);
        Ok(Self {
            ast,
            source_session,
            final_box_paths,
            metadata,
        })
    }

    pub(super) fn lower_delegates(self) -> Result<Self, crate::parser::ParseError> {
        super::delegate_batch::lower_delegates(self)
    }

    pub(super) fn commit_generated_delegate_batch(
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

    pub(super) fn finalize(
        self,
    ) -> Result<ParsedProgramWithSourceV1, SourceSealFinalizationErrorV1> {
        finalize_program(
            self.ast,
            self.source_session.into_prepared(),
            self.final_box_paths,
            self.metadata,
        )
    }
}

/// Final authority. It is intentionally non-Clone and has no public
/// constructor. Only `OpenParserPostpassProductV1::finalize` can issue it.
#[derive(Debug)]
pub(super) struct ParserBoxSourceSealV1 {
    prepared: PreparedBoxSourceSealV1,
}

impl ParserBoxSourceSealV1 {
    pub(super) fn inventory(&self) -> &BoxMethodInventoryV1 {
        &self.prepared.inventory
    }

    pub(super) fn method_relations(&self) -> &[MethodSourceRelationV1] {
        &self.prepared.method_relations
    }

    pub(super) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.prepared.box_site
    }

    pub(super) fn generated_delegate_source_relations(
        &self,
    ) -> &[GeneratedDelegateSourceRelationV1] {
        &self.prepared.generated_delegate_source_relations
    }
}

#[derive(Debug)]
pub(super) struct ParsedProgramWithSourceV1 {
    ast: ASTNode,
    source_seals: Box<[ParserBoxSourceSealV1]>,
    generated_delegate_source_relations: Box<[GeneratedDelegateSourceRelationV1]>,
    metadata: ParserMetadata,
}

impl ParsedProgramWithSourceV1 {
    pub(super) fn ast(&self) -> &ASTNode {
        &self.ast
    }

    pub(super) fn into_ast(self) -> ASTNode {
        self.ast
    }

    pub(super) fn source_seals(&self) -> &[ParserBoxSourceSealV1] {
        &self.source_seals
    }

    pub(super) fn generated_delegate_source_relations(
        &self,
    ) -> &[GeneratedDelegateSourceRelationV1] {
        &self.generated_delegate_source_relations
    }

    pub(super) fn metadata(&self) -> &ParserMetadata {
        &self.metadata
    }
}

/// Finalize only the bounded R6-S3 cohort: direct top-level ordinary Rust
/// `box` declarations, after build-gate pruning and delegate lowering.
/// `FinalizerCoveragePlanV1` is the private one-to-one source-path alignment
/// between parser-owned declarations and the final AST. Generated delegate
/// rows are retained in the final source seal only after the source-aware
/// relation/placement coverage check succeeds.
#[derive(Debug)]
struct FinalizerCoveragePlanV1 {
    prepared_to_final: Vec<usize>,
}

impl FinalizerCoveragePlanV1 {
    fn issue(
        prepared: &[PreparedBoxSourceSealV1],
        final_box_paths: &[SourceBoxDeclarationPathV1],
    ) -> Result<Self, SourceSealFinalizationErrorV1> {
        if prepared.len() != final_box_paths.len() {
            return Err(
                SourceSealFinalizationErrorV1::FinalAstBoxPathCoverageMismatch {
                    prepared: prepared.len(),
                    final_ast: final_box_paths.len(),
                },
            );
        }
        for (final_index, path) in final_box_paths.iter().enumerate() {
            if final_box_paths[..final_index]
                .iter()
                .any(|previous| previous == path)
            {
                return Err(SourceSealFinalizationErrorV1::DuplicateFinalAstBoxPath {
                    final_index,
                });
            }
            if let Some(first) = prepared.first() {
                if path.brand() != &first.brand {
                    return Err(SourceSealFinalizationErrorV1::ForeignFinalAstBoxPath {
                        final_index,
                    });
                }
            }
        }

        let mut prepared_to_final = Vec::with_capacity(prepared.len());
        for (prepared_index, seal) in prepared.iter().enumerate() {
            let Some(final_index) = final_box_paths
                .iter()
                .position(|path| path == seal.box_site.path())
            else {
                return Err(SourceSealFinalizationErrorV1::PreparedBoxPathMissing {
                    prepared_index,
                });
            };
            prepared_to_final.push(final_index);
        }
        Ok(Self { prepared_to_final })
    }
}

fn finalize_program(
    ast: ASTNode,
    prepared: Vec<PreparedBoxSourceSealV1>,
    final_box_paths: Vec<SourceBoxDeclarationPathV1>,
    metadata: ParserMetadata,
) -> Result<ParsedProgramWithSourceV1, SourceSealFinalizationErrorV1> {
    let ASTNode::Program { ref statements, .. } = ast else {
        return Err(SourceSealFinalizationErrorV1::OrdinaryBoxCountMismatch {
            prepared: prepared.len(),
            final_ast: 0,
        });
    };

    if statements
        .iter()
        .any(|statement| matches!(statement, ASTNode::BuildGate { .. }))
    {
        return Err(SourceSealFinalizationErrorV1::TopLevelBuildGateUnsupported);
    }
    let mut final_inventories = Vec::new();
    for (ordinal, statement) in statements.iter().enumerate() {
        match statement {
            ASTNode::BoxDeclaration {
                methods,
                is_interface: false,
                is_record: false,
                is_static: false,
                ..
            } => final_inventories.push(methods),
            ASTNode::BoxDeclaration { .. } => {
                return Err(SourceSealFinalizationErrorV1::UnsupportedTopLevelBoxKind { ordinal });
            }
            _ => {}
        }
    }
    if final_inventories.len() != prepared.len() {
        return Err(SourceSealFinalizationErrorV1::OrdinaryBoxCountMismatch {
            prepared: prepared.len(),
            final_ast: final_inventories.len(),
        });
    }

    let generated_delegate_source_relations = prepared
        .iter()
        .flat_map(PreparedBoxSourceSealV1::generated_delegate_source_relations)
        .cloned()
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let coverage = FinalizerCoveragePlanV1::issue(&prepared, &final_box_paths)?;
    let source_seals = prepared
        .into_iter()
        .enumerate()
        .map(|(prepared_index, prepared)| {
            let final_index = coverage.prepared_to_final[prepared_index];
            prepared.finalize_against(final_inventories[final_index])
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();

    Ok(ParsedProgramWithSourceV1 {
        ast,
        source_seals,
        generated_delegate_source_relations,
        metadata,
    })
}

pub(super) fn map_error(error: SourceSealFinalizationErrorV1) -> crate::parser::ParseError {
    let message = match error {
        SourceSealFinalizationErrorV1::TopLevelBuildGateUnsupported => {
            "R6-S3A source seal requires top-level build-gate selection to be closed first"
                .to_owned()
        }
        SourceSealFinalizationErrorV1::UnsupportedTopLevelBoxKind { ordinal } => {
            format!(
                "R6-S3A source seal supports ordinary top-level Box only at statement ordinal {ordinal}"
            )
        }
        SourceSealFinalizationErrorV1::OrdinaryBoxCountMismatch {
            prepared,
            final_ast,
        } => {
            format!(
                "R6-S3A ordinary Box seal count mismatch: prepared={prepared}, final={final_ast}"
            )
        }
        SourceSealFinalizationErrorV1::FinalInventoryShorter {
            prepared,
            final_ast,
        } => {
            format!("R6-S3A final Box inventory is shorter: prepared={prepared}, final={final_ast}")
        }
        SourceSealFinalizationErrorV1::InventoryPrefixMismatch { ordinal } => {
            format!("R6-S3A final Box inventory prefix mismatch at ordinal {ordinal}")
        }
        SourceSealFinalizationErrorV1::UnexpectedGeneratedRow { ordinal } => {
            format!("R6-S3A unexpected non-delegate generated row at ordinal {ordinal}")
        }
        SourceSealFinalizationErrorV1::FinalAstBoxPathCoverageMismatch {
            prepared,
            final_ast,
        } => {
            format!(
                "R6-S3B-B3 final AST Box source-path coverage mismatch: prepared={prepared}, final={final_ast}"
            )
        }
        SourceSealFinalizationErrorV1::DuplicateFinalAstBoxPath { final_index } => {
            format!("R6-S3B-B3 duplicate final AST Box source path at index {final_index}")
        }
        SourceSealFinalizationErrorV1::PreparedBoxPathMissing { prepared_index } => {
            format!("R6-S3B-B3 prepared Box source path is absent from final AST at index {prepared_index}")
        }
        SourceSealFinalizationErrorV1::ForeignFinalAstBoxPath { final_index } => {
            format!("R6-S3B-B3 final AST Box source path has a foreign parser brand at index {final_index}")
        }
        SourceSealFinalizationErrorV1::GeneratedDelegateCoverage(error) => {
            format!("R6-S3B-D generated delegate relation coverage is invalid: {error:?}")
        }
        SourceSealFinalizationErrorV1::Inventory(error) => {
            format!("R6-S3A final Box inventory is invalid: {error}")
        }
    };
    crate::parser::ParseError::BuildCfg { message, line: 0 }
}
