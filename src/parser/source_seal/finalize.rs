use crate::ast::{
    ASTNode, BoxMethodGeneratedProvenanceV1, BoxMethodInventoryV1, BoxMethodProvenanceV1,
};
use crate::parser::{NyashParser, ParserMetadata};

use super::super::source_path::SourceBoxDeclarationPathV1;
use super::model::{
    OpenParserPostpassProductV1, ParsedProgramWithSourceV1, ParserBoxSourceSealV1,
    PreparedBoxSourceSealV1, SourceSealFinalizationErrorV1,
};

impl PreparedBoxSourceSealV1 {
    /// Consume the prepared payload only after the postpass has produced the
    /// final inventory and generated-delegate relation coverage is exact.
    pub(in crate::parser) fn finalize_against(
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

        super::super::source_seal_finalizer::validate_generated_delegate_coverage(
            &self,
            final_inventory,
        )
        .map_err(SourceSealFinalizationErrorV1::GeneratedDelegateCoverage)?;

        Ok(ParserBoxSourceSealV1 {
            prepared: PreparedBoxSourceSealV1 {
                brand: self.brand,
                box_site: self.box_site,
                inventory: self.inventory,
                method_relations: self.method_relations,
                delegate_source_declarations: Box::new([]),
                member_gate_selection_receipts: self.member_gate_selection_receipts,
                generated_property_callable_rows: Box::new([]),
                generated_delegate_source_relations: self.generated_delegate_source_relations,
            },
        })
    }
}

impl OpenParserPostpassProductV1 {
    pub(in crate::parser) fn finalize(
        self,
    ) -> Result<ParsedProgramWithSourceV1, SourceSealFinalizationErrorV1> {
        let (prepared, callable_rows) = self.source_session.into_parts();
        finalize_program(
            self.ast,
            prepared,
            callable_rows.into_boxed_slice(),
            self.final_box_paths,
            self.metadata,
        )
    }

    /// S0 total postpass coordinator. Cohort admission happens after the
    /// shared prune transaction and chooses exactly one explicit arm.
    pub(in crate::parser) fn finish_total_s0(
        self,
        parser: &NyashParser,
        demand: super::super::postpass_envelope::PostpassDemandV1,
    ) -> Result<super::super::postpass_envelope::CompletedParserPostpassV1, crate::parser::ParseError>
    {
        let product = self.prune_build_gates_with_explain(
            parser,
            matches!(
                demand.explain,
                super::super::postpass_envelope::ExplainDemandV1::Capture
            ),
        )?;
        let explain = product.explain.clone();
        let cohort = super::super::postpass_envelope::classify_program(&product.ast);
        if matches!(
            cohort,
            super::super::postpass_envelope::ParserPostpassProgramCohortV1::OrdinaryTopLevelBox
        ) {
            let sealed = product.lower_delegates()?.finalize().map_err(map_error)?;
            return super::super::postpass_envelope::CompletedParserPostpassV1::from_source_product(
                sealed, explain,
            )
            .map_err(|error| error.into_parse_error());
        }

        let (ast, metadata, callable_rows) = product.into_compatibility_parts();
        let ast = super::super::postpass_compatibility::lower(ast)?;
        super::super::postpass_envelope::CompletedParserPostpassV1::from_compatibility(
            ast,
            metadata,
            explain,
            callable_rows,
        )
        .map_err(|error| error.into_parse_error())
    }

    fn into_compatibility_parts(
        self,
    ) -> (
        ASTNode,
        ParserMetadata,
        Box<[super::super::callable_source_anchor::PreparedCallableSourceV1]>,
    ) {
        let (_, callable_rows) = self.source_session.into_parts();
        (self.ast, self.metadata, callable_rows.into_boxed_slice())
    }
}

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
    callable_rows: Box<[super::super::callable_source_anchor::PreparedCallableSourceV1]>,
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
        callable_rows,
        final_box_ordinals: coverage.prepared_to_final.into_boxed_slice(),
        generated_delegate_source_relations,
        metadata,
    })
}

pub(in crate::parser) fn map_error(
    error: SourceSealFinalizationErrorV1,
) -> crate::parser::ParseError {
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
        } => format!(
            "R6-S3A ordinary Box seal count mismatch: prepared={prepared}, final={final_ast}"
        ),
        SourceSealFinalizationErrorV1::FinalInventoryShorter {
            prepared,
            final_ast,
        } => format!(
            "R6-S3A final Box inventory is shorter: prepared={prepared}, final={final_ast}"
        ),
        SourceSealFinalizationErrorV1::InventoryPrefixMismatch { ordinal } => {
            format!("R6-S3A final Box inventory prefix mismatch at ordinal {ordinal}")
        }
        SourceSealFinalizationErrorV1::UnexpectedGeneratedRow { ordinal } => {
            format!("R6-S3A unexpected non-delegate generated row at ordinal {ordinal}")
        }
        SourceSealFinalizationErrorV1::FinalAstBoxPathCoverageMismatch {
            prepared,
            final_ast,
        } => format!(
            "R6-S3B-B3 final AST Box source-path coverage mismatch: prepared={prepared}, final={final_ast}"
        ),
        SourceSealFinalizationErrorV1::DuplicateFinalAstBoxPath { final_index } => {
            format!("R6-S3B-B3 duplicate final AST Box source path at index {final_index}")
        }
        SourceSealFinalizationErrorV1::PreparedBoxPathMissing { prepared_index } => format!(
            "R6-S3B-B3 prepared Box source path is absent from final AST at index {prepared_index}"
        ),
        SourceSealFinalizationErrorV1::ForeignFinalAstBoxPath { final_index } => format!(
            "R6-S3B-B3 final AST Box source path has a foreign parser brand at index {final_index}"
        ),
        SourceSealFinalizationErrorV1::GeneratedDelegateCoverage(error) => {
            format!("R6-S3B-D generated delegate relation coverage is invalid: {error:?}")
        }
        SourceSealFinalizationErrorV1::Inventory(error) => {
            format!("R6-S3A final Box inventory is invalid: {error}")
        }
    };
    crate::parser::ParseError::BuildCfg { message, line: 0 }
}
