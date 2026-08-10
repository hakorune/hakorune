use crate::ast::{ASTNode, BoxMethodInventoryErrorV1, BoxMethodInventoryV1};
use crate::parser::ParserMetadata;

use super::super::build_cfg::decision_set::PreparedBuildGateDecisionSetV1;
use super::super::callable_gate_projection::MemberGateSelectionReceiptV1;
use super::super::callable_source_anchor::{
    PreparedCallableSourceV1, PreparedGeneratedCallableSourceV1,
};
use super::super::delegate_source_relation::GeneratedDelegateSourceRelationV1;
use super::super::source_authority::{
    DelegateSourceDeclarationV1, MethodSourceRelationV1, ParserInvocationBrandV1,
    SourceBoxDeclarationSiteV1,
};
use super::super::source_gate_ledger::PreparedBuildGateSourceRecordV1;
use super::super::source_gate_receipt::BuildGateSelectionReceiptV1;
use super::super::source_path::SourceBoxDeclarationPathV1;
use super::super::source_seal_finalizer::GeneratedDelegateCoverageErrorV1;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::parser) enum SourceSealFinalizationErrorV1 {
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
pub(in crate::parser) struct PreparedBoxSourceSealV1 {
    pub(in crate::parser) brand: ParserInvocationBrandV1,
    pub(in crate::parser) box_site: SourceBoxDeclarationSiteV1,
    pub(in crate::parser) inventory: BoxMethodInventoryV1,
    pub(in crate::parser) method_relations: Box<[MethodSourceRelationV1]>,
    pub(in crate::parser) delegate_source_declarations: Box<[DelegateSourceDeclarationV1]>,
    pub(in crate::parser) member_gate_selection_receipts: Box<[MemberGateSelectionReceiptV1]>,
    pub(in crate::parser) generated_property_callable_rows:
        Box<[PreparedGeneratedCallableSourceV1]>,
    pub(in crate::parser) generated_delegate_source_relations:
        Box<[GeneratedDelegateSourceRelationV1]>,
}

impl PreparedBoxSourceSealV1 {
    pub(in crate::parser) fn inventory(&self) -> &BoxMethodInventoryV1 {
        &self.inventory
    }

    pub(in crate::parser) fn method_relations(&self) -> &[MethodSourceRelationV1] {
        &self.method_relations
    }

    pub(in crate::parser) fn delegate_source_declarations(&self) -> &[DelegateSourceDeclarationV1] {
        &self.delegate_source_declarations
    }

    pub(in crate::parser) fn generated_delegate_source_relations(
        &self,
    ) -> &[GeneratedDelegateSourceRelationV1] {
        &self.generated_delegate_source_relations
    }

    pub(in crate::parser) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.box_site
    }

    pub(in crate::parser) fn member_gate_selection_receipts(
        &self,
    ) -> &[MemberGateSelectionReceiptV1] {
        &self.member_gate_selection_receipts
    }
}

/// Single typed owner for the AST/source handoff between parser postpasses.
#[derive(Debug)]
pub(in crate::parser) struct OpenParserPostpassProductV1 {
    pub(in crate::parser) ast: ASTNode,
    pub(in crate::parser) source_session: ParserSourceSessionV1,
    pub(in crate::parser) final_box_paths: Vec<SourceBoxDeclarationPathV1>,
    pub(in crate::parser) build_gate_decision_set: PreparedBuildGateDecisionSetV1,
    pub(in crate::parser) explain: Option<super::super::BuildGateExplainReport>,
    pub(in crate::parser) metadata: ParserMetadata,
}

/// Parser-owned source transport for the open postpass product.
#[derive(Debug)]
pub(in crate::parser) struct ParserSourceSessionV1 {
    pub(in crate::parser) prepared_source_seals: Vec<PreparedBoxSourceSealV1>,
    pub(in crate::parser) gate_records: Vec<PreparedBuildGateSourceRecordV1>,
    pub(in crate::parser) selection_receipts: Vec<BuildGateSelectionReceiptV1>,
    pub(in crate::parser) callable_rows: Vec<PreparedCallableSourceV1>,
}

#[derive(Debug)]
pub(in crate::parser) struct PreparedParserSourcePruneV1 {
    pub(in crate::parser) prepared_source_seals: Vec<PreparedBoxSourceSealV1>,
    pub(in crate::parser) gate_records: Vec<PreparedBuildGateSourceRecordV1>,
    pub(in crate::parser) selection_receipts: Vec<BuildGateSelectionReceiptV1>,
    pub(in crate::parser) callable_rows: Vec<PreparedCallableSourceV1>,
}

impl PreparedParserSourcePruneV1 {
    pub(in crate::parser) fn retained_box_paths(&self) -> Vec<SourceBoxDeclarationPathV1> {
        self.prepared_source_seals
            .iter()
            .map(|seal| seal.box_site.path().clone())
            .collect()
    }
}

/// Final authority. It is intentionally non-Clone and has no public
/// constructor. Only `OpenParserPostpassProductV1::finalize` can issue it.
#[derive(Debug)]
pub(in crate::parser) struct ParserBoxSourceSealV1 {
    pub(in crate::parser) prepared: PreparedBoxSourceSealV1,
}

impl ParserBoxSourceSealV1 {
    pub(in crate::parser) fn inventory(&self) -> &BoxMethodInventoryV1 {
        &self.prepared.inventory
    }

    pub(in crate::parser) fn method_relations(&self) -> &[MethodSourceRelationV1] {
        &self.prepared.method_relations
    }

    pub(in crate::parser) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.prepared.box_site
    }

    pub(in crate::parser) fn generated_delegate_source_relations(
        &self,
    ) -> &[GeneratedDelegateSourceRelationV1] {
        &self.prepared.generated_delegate_source_relations
    }
}

#[derive(Debug)]
pub(in crate::parser) struct ParsedProgramWithSourceV1 {
    pub(in crate::parser) ast: ASTNode,
    pub(in crate::parser) source_seals: Box<[ParserBoxSourceSealV1]>,
    pub(in crate::parser) callable_rows: Box<[PreparedCallableSourceV1]>,
    pub(in crate::parser) final_box_ordinals: Box<[usize]>,
    pub(in crate::parser) generated_delegate_source_relations:
        Box<[GeneratedDelegateSourceRelationV1]>,
    pub(in crate::parser) metadata: ParserMetadata,
}

impl ParsedProgramWithSourceV1 {
    pub(in crate::parser) fn ast(&self) -> &ASTNode {
        &self.ast
    }

    pub(in crate::parser) fn into_ast(self) -> ASTNode {
        self.ast
    }

    pub(in crate::parser) fn source_seals(&self) -> &[ParserBoxSourceSealV1] {
        &self.source_seals
    }

    pub(in crate::parser) fn generated_delegate_source_relations(
        &self,
    ) -> &[GeneratedDelegateSourceRelationV1] {
        &self.generated_delegate_source_relations
    }

    pub(in crate::parser) fn callable_rows(&self) -> &[PreparedCallableSourceV1] {
        &self.callable_rows
    }

    pub(in crate::parser) fn into_postpass_parts(
        self,
    ) -> (
        ASTNode,
        Box<[ParserBoxSourceSealV1]>,
        Box<[PreparedCallableSourceV1]>,
        Box<[usize]>,
        ParserMetadata,
    ) {
        (
            self.ast,
            self.source_seals,
            self.callable_rows,
            self.final_box_ordinals,
            self.metadata,
        )
    }

    pub(in crate::parser) fn metadata(&self) -> &ParserMetadata {
        &self.metadata
    }
}
