//! Final parser source product for the bounded R6-S3 slice.
//!
//! This module owns the post-prune/post-delegate boundary.  The ordinary
//! parser transaction issues only a prepared payload; the postpass product in
//! this module is the only owner that can compare that payload with the final
//! AST inventory and issue the non-Clone source seal.

use crate::ast::{
    ASTNode, BoxMethodGeneratedProvenanceV1, BoxMethodInventoryErrorV1, BoxMethodInventoryV1,
    BoxMethodProvenanceV1, PreparedBoxMethodInventoryAppendV1,
};
use crate::parser::ParserMetadata;

use super::source_authority::{
    MethodSourceRelationV1, ParserInvocationBrandV1, SourceBoxDeclarationSiteV1,
};
use super::source_gate_ledger::PreparedBuildGateSourceRecordV1;
use super::source_path::{SourceBoxPathSegmentV1, SourceBuildGateBranchV1, SourceBuildGatePathV1};
use super::{delegate_lowering, NyashParser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BuildGateSelectionReceiptV1 {
    brand: ParserInvocationBrandV1,
    gate_id: super::source_authority::SourceBuildGateIdV1,
    gate_path: SourceBuildGatePathV1,
    selected_branch: SourceBuildGateBranchV1,
}

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
    Inventory(BoxMethodInventoryErrorV1),
}

#[derive(Debug)]
pub(super) struct PreparedBoxSourceSealV1 {
    pub(super) brand: ParserInvocationBrandV1,
    pub(super) box_site: SourceBoxDeclarationSiteV1,
    pub(super) inventory: BoxMethodInventoryV1,
    pub(super) method_relations: Box<[MethodSourceRelationV1]>,
}

impl PreparedBoxSourceSealV1 {
    pub(super) fn inventory(&self) -> &BoxMethodInventoryV1 {
        &self.inventory
    }

    pub(super) fn method_relations(&self) -> &[MethodSourceRelationV1] {
        &self.method_relations
    }

    pub(super) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.box_site
    }

    /// Consume the prepared payload only after the postpass has produced the
    /// final inventory. Delegate rows may be appended, but the original
    /// ordered rows must remain byte-for-byte equivalent at the AST level.
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

        let mut inventory = BoxMethodInventoryV1::empty();
        inventory
            .commit_prepared_append(
                PreparedBoxMethodInventoryAppendV1::try_new(final_entries)
                    .map_err(SourceSealFinalizationErrorV1::Inventory)?,
            )
            .map_err(SourceSealFinalizationErrorV1::Inventory)?;

        Ok(ParserBoxSourceSealV1 {
            prepared: PreparedBoxSourceSealV1 {
                brand: self.brand,
                box_site: self.box_site,
                inventory,
                method_relations: self.method_relations,
            },
        })
    }
}

/// Single typed owner for the AST/source handoff between parser postpasses.
///
/// S3B-A intentionally keeps the existing direct ordinary-Box prune/delegate
/// behavior. The important boundary is that those postpasses consume and
/// return this product instead of separately mutating an AST and a seal list.
/// Gate path rebasing and source-aware delegate relations remain later S3B
/// slices; this product must not be treated as resolver authority until the
/// final relation coverage is complete.
#[derive(Debug)]
pub(super) struct OpenParserPostpassProductV1 {
    ast: ASTNode,
    source_session: ParserSourceSessionV1,
    metadata: ParserMetadata,
}

/// Parser-owned source transport for the open postpass product. This wrapper
/// keeps prepared payload storage behind a named session boundary; gate path
/// and generated-delegate relation expansion belong to later S3B slices.
#[derive(Debug)]
pub(super) struct ParserSourceSessionV1 {
    prepared_source_seals: Vec<PreparedBoxSourceSealV1>,
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
            metadata,
        } = self;
        let (ast, receipts) = super::source_gate_prune::prune_top_level_gate_program(
            parser,
            ast,
            source_session.gate_records(),
        )?;
        let ast = parser.prune_build_gate_program(ast)?;
        let prepared = source_session
            .prepare_prune(&receipts)
            .map_err(source_prune_error)?;
        let source_session = source_session.commit_prune(prepared);
        Ok(Self {
            ast,
            source_session,
            metadata,
        })
    }

    pub(super) fn lower_delegates(self) -> Result<Self, crate::parser::ParseError> {
        let ast = delegate_lowering::lower_delegate_exposes(self.ast)?;
        Ok(Self { ast, ..self })
    }

    pub(super) fn finalize(
        self,
    ) -> Result<ParsedProgramWithSourceV1, SourceSealFinalizationErrorV1> {
        finalize_program(self.ast, self.source_session.into_prepared(), self.metadata)
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
}

#[derive(Debug)]
pub(super) struct ParsedProgramWithSourceV1 {
    ast: ASTNode,
    source_seals: Box<[ParserBoxSourceSealV1]>,
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

    pub(super) fn metadata(&self) -> &ParserMetadata {
        &self.metadata
    }
}

/// Finalize only the bounded R6-S3 cohort: direct top-level ordinary Rust
/// `box` declarations, after build-gate pruning and delegate lowering.
fn finalize_program(
    ast: ASTNode,
    prepared: Vec<PreparedBoxSourceSealV1>,
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

    let source_seals = prepared
        .into_iter()
        .zip(final_inventories)
        .map(|(prepared, final_inventory)| prepared.finalize_against(final_inventory))
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();

    Ok(ParsedProgramWithSourceV1 {
        ast,
        source_seals,
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
        SourceSealFinalizationErrorV1::Inventory(error) => {
            format!("R6-S3A final Box inventory is invalid: {error}")
        }
    };
    crate::parser::ParseError::BuildCfg { message, line: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};

    #[test]
    fn r6_s3_finalizes_ordinary_box_after_final_parse_postpass() {
        let parsed = NyashParser::parse_from_string_with_source_seal(
            r#"
box Plain {
    run() { return 1 }
}
"#,
            ParserBuildConfig::default(),
        )
        .expect("ordinary Box should issue the final source seal");

        assert_eq!(parsed.source_seals().len(), 1);
        let seal = &parsed.source_seals()[0];
        assert_eq!(seal.inventory().len(), 1);
        assert_eq!(seal.inventory().get("run").unwrap().name(), "run");
        assert_eq!(seal.method_relations().len(), 1);
        assert!(matches!(parsed.ast(), ASTNode::Program { .. }));
    }

    #[test]
    fn r6_s3b_b2_prunes_selected_top_level_gate_and_preserves_box_path() {
        let parsed = NyashParser::parse_from_string_with_source_seal(
            r#"
gate Build.test {
    box ThenBox { run() { return 1 } }
} else {
    box ElseBox { run() { return 2 } }
}
"#,
            ParserBuildConfig::default(),
        )
        .expect("release config should select the else branch");

        assert_eq!(parsed.source_seals().len(), 1);
        assert!(matches!(
            parsed.ast(),
            ASTNode::Program { statements, .. }
                if matches!(statements.as_slice(), [ASTNode::BoxDeclaration { name, .. }] if name == "ElseBox")
        ));
        assert!(matches!(
            parsed.source_seals()[0].box_site().path().segments(),
            [
                crate::parser::source_path::SourceBoxPathSegmentV1::RootStatement { ordinal: 0 },
                crate::parser::source_path::SourceBoxPathSegmentV1::BuildGate {
                    branch: crate::parser::source_authority::SourceBuildGateBranchV1::Else,
                    ..
                }
            ]
        ));
    }

    #[test]
    fn r6_s3b_b2_prunes_nested_top_level_gate_once() {
        let config = ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        };
        let parsed = NyashParser::parse_from_string_with_source_seal(
            r#"
gate Build.test {
            gate Build.test {
        box NestedBox { run() { return 1 } }
    }
}
"#,
            config,
        )
        .expect("nested selected gate should issue one rich source product");

        assert_eq!(parsed.source_seals().len(), 1);
        assert!(matches!(
            parsed.source_seals()[0].box_site().path().segments(),
            [
                crate::parser::source_path::SourceBoxPathSegmentV1::RootStatement { ordinal: 0 },
                crate::parser::source_path::SourceBoxPathSegmentV1::BuildGate { .. },
                crate::parser::source_path::SourceBoxPathSegmentV1::BuildGate { .. }
            ]
        ));
    }

    #[test]
    fn r6_s3b_b2_empty_gate_has_no_source_seal_and_still_finalizes() {
        let parsed = NyashParser::parse_from_string_with_source_seal(
            "gate Build.test { } else { }",
            ParserBuildConfig::default(),
        )
        .expect("empty gate should have exact ledger/receipt coverage");
        assert_eq!(parsed.source_seals().len(), 0);
        assert!(
            matches!(parsed.ast(), ASTNode::Program { statements, .. } if statements.is_empty())
        );
    }

    #[test]
    fn r6_s3b_a_ast_projection_matches_the_rich_product() {
        let source = r#"
box Plain {
    run() { return 1 }
}
"#;
        let rich =
            NyashParser::parse_from_string_with_source_seal(source, ParserBuildConfig::default())
                .expect("rich direct-Box product should finalize");
        let projected = NyashParser::parse_from_string_with_source_seal_ast(
            source,
            ParserBuildConfig::default(),
        )
        .expect("AST projection should use the rich path");

        assert_eq!(rich.into_ast(), projected);
    }

    #[test]
    fn r6_s3b_a_rich_product_keeps_diagnostic_metadata_outside_source_seal() {
        let parsed = NyashParser::parse_from_string_with_source_seal(
            r#"@rune Public
box Plain {
    run() { return 1 }
}
"#,
            ParserBuildConfig::default(),
        )
        .expect("diagnostic rune metadata must not block the bounded product");

        assert_eq!(parsed.source_seals().len(), 1);
        assert_eq!(parsed.metadata().runes.len(), 1);
        assert_eq!(parsed.metadata().runes[0].name, "Public");
    }

    #[test]
    fn r6_s3_accepts_delegate_generated_suffix_as_generated_provenance() {
        let parsed = NyashParser::parse_from_string_with_source_seal(
            r#"
box Target { run() { return 1 } }
box Host {
    target: Target
    delegate target exposes { run as runAlias }
}
"#,
            ParserBuildConfig::default(),
        )
        .expect("delegate postpass should be included before the final seal");

        assert_eq!(parsed.source_seals().len(), 2);
        let host = &parsed.source_seals()[1];
        let generated = host
            .inventory()
            .get("runAlias")
            .expect("delegate generated method must be in the final inventory");
        assert!(matches!(
            generated.provenance(),
            BoxMethodProvenanceV1::Generated(BoxMethodGeneratedProvenanceV1::Delegate { .. })
        ));
    }

    #[test]
    fn r6_s3_does_not_issue_a_partial_seal_for_unsupported_top_level_box() {
        let error = NyashParser::parse_from_string_with_source_seal(
            r#"
static box StaticOnly { run() { return 1 } }
"#,
            ParserBuildConfig::default(),
        )
        .expect_err("static Box must remain outside the bounded rich product");
        assert!(error.to_string().contains("ordinary top-level Box only"));
    }
}
