use crate::ast::{
    ASTNode, BoxMethodGeneratedProvenanceV1, BoxMethodInventoryV1, BoxMethodProvenanceV1,
};
use crate::parser::{NyashParser, ParserMetadata};

use super::super::source_authority::ParserBoxDeclarationKindV1;
use super::super::source_path::SourceBoxDeclarationPathV1;
use super::model::{
    OpenParserPostpassProductV1, ParsedProgramWithSourceV1, ParserBoxSourceSealV1,
    PreparedBoxSourceSealV1, SourceSealFinalizationErrorV1,
};

impl PreparedBoxSourceSealV1 {
    fn validate_against(
        &self,
        final_name: &str,
        final_is_sync: bool,
        final_inventory: &BoxMethodInventoryV1,
        final_constructors: &std::collections::HashMap<String, ASTNode>,
    ) -> Result<(), SourceSealFinalizationErrorV1> {
        if self.declaration_syntax.kind() != ParserBoxDeclarationKindV1::Ordinary {
            return Err(SourceSealFinalizationErrorV1::DeclarationKindMismatch);
        }
        if self.declaration_syntax.name() != final_name {
            return Err(SourceSealFinalizationErrorV1::DeclarationNameMismatch {
                prepared: self.declaration_syntax.name().to_owned().into_boxed_str(),
                final_ast: final_name.to_owned().into_boxed_str(),
            });
        }
        if self.declaration_syntax.is_sync() != final_is_sync {
            return Err(SourceSealFinalizationErrorV1::DeclarationSyncMismatch {
                prepared: self.declaration_syntax.is_sync(),
                final_ast: final_is_sync,
            });
        }
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
            self,
            final_inventory,
        )
        .map_err(SourceSealFinalizationErrorV1::GeneratedDelegateCoverage)?;
        super::super::source_authority::constructor_source::validate_constructor_rows(
            &self.constructor_relations,
            final_constructors,
        )
        .map_err(|_| SourceSealFinalizationErrorV1::ConstructorCoverageMismatch)
    }

    /// Consume the prepared payload only after the postpass has produced the
    /// final inventory and generated-delegate relation coverage is exact.
    pub(in crate::parser) fn finalize_against(
        self,
        final_name: &str,
        final_is_sync: bool,
        final_inventory: &BoxMethodInventoryV1,
        final_constructors: &std::collections::HashMap<String, ASTNode>,
    ) -> Result<ParserBoxSourceSealV1, SourceSealFinalizationErrorV1> {
        self.validate_against(
            final_name,
            final_is_sync,
            final_inventory,
            final_constructors,
        )?;

        Ok(ParserBoxSourceSealV1 {
            prepared: PreparedBoxSourceSealV1 {
                brand: self.brand,
                box_site: self.box_site,
                declaration_syntax: self.declaration_syntax,
                inventory: self.inventory,
                method_relations: self.method_relations,
                delegate_source_declarations: Box::new([]),
                member_gate_selection_receipts: self.member_gate_selection_receipts,
                generated_property_callable_rows: Box::new([]),
                generated_delegate_source_relations: self.generated_delegate_source_relations,
                constructor_relations: self.constructor_relations,
            },
        })
    }
}

impl OpenParserPostpassProductV1 {
    pub(in crate::parser) fn finalize(
        self,
    ) -> Result<ParsedProgramWithSourceV1, SourceSealFinalizationErrorV1> {
        let OpenParserPostpassProductV1 {
            ast,
            source_session,
            final_box_paths,
            projected_program_item_slots,
            metadata,
            ..
        } = self;
        let (prepared, callable_rows) = source_session.into_parts();
        finalize_program(
            ast,
            prepared,
            callable_rows.into_boxed_slice(),
            final_box_paths,
            projected_program_item_slots,
            metadata,
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

        let semantic_candidate = super::super::initial_callable_program_source::compatibility_program_can_enter_initial_callable_lane_v1(&product.ast);
        let (ast, metadata, callable_rows, program_slots, prepared_seals, final_box_paths) =
            product.into_compatibility_parts();
        let ast = super::super::postpass_compatibility::lower(ast)?;
        if semantic_candidate {
            let mut program = super::super::initial_callable_program_source::issue_initial_callable_program_source_v1(
                ast,
                callable_rows,
                program_slots,
                &prepared_seals,
            )
            .map_err(|error| {
                map_error(SourceSealFinalizationErrorV1::InitialCallableProgramSource(error))
            })?;
            let (seals, final_box_ordinals) =
                finalize_compatibility_source(program.ast(), prepared_seals, final_box_paths)
                    .map_err(map_error)?;
            let constructor_source =
                super::super::constructor_source_catalog::ParserConstructorSourceCatalogV1::issue(
                    program.ast(),
                    &seals,
                    &final_box_ordinals,
                )
                .map_err(|error| {
                    map_error(SourceSealFinalizationErrorV1::ConstructorSourceCatalog(
                        error,
                    ))
                })?;
            program = program.attach_constructor_source(constructor_source);
            return super::super::postpass_envelope::CompletedParserPostpassV1::from_initial_compatibility(
                program,
                metadata,
                explain,
            )
            .map_err(|error| error.into_parse_error());
        }
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
        Option<super::super::build_cfg::program_item_slots::ProjectedProgramItemSlotSetV1>,
        Vec<PreparedBoxSourceSealV1>,
        Vec<SourceBoxDeclarationPathV1>,
    ) {
        let (prepared_seals, callable_rows) = self.source_session.into_parts();
        (
            self.ast,
            self.metadata,
            callable_rows.into_boxed_slice(),
            self.projected_program_item_slots,
            prepared_seals,
            self.final_box_paths,
        )
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
    projected_program_item_slots: Option<
        super::super::build_cfg::program_item_slots::ProjectedProgramItemSlotSetV1,
    >,
    metadata: ParserMetadata,
) -> Result<ParsedProgramWithSourceV1, SourceSealFinalizationErrorV1> {
    let coverage = FinalizerCoveragePlanV1::issue(&prepared, &final_box_paths)?;
    validate_ordinary_source_seals(&ast, &prepared, &coverage)?;
    let generated_delegate_source_relations = prepared
        .iter()
        .flat_map(PreparedBoxSourceSealV1::generated_delegate_source_relations)
        .cloned()
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let initial_callable_source =
        super::super::initial_callable_program_source::issue_initial_callable_program_source_v1(
            ast,
            callable_rows,
            projected_program_item_slots,
            &prepared,
        )
        .map_err(SourceSealFinalizationErrorV1::InitialCallableProgramSource)?;
    let final_boxes = final_boxes_for_source(initial_callable_source.ast(), prepared.len(), false)?;
    let source_seals = prepared
        .into_iter()
        .enumerate()
        .map(|(prepared_index, prepared)| {
            let final_index = coverage.prepared_to_final[prepared_index];
            prepared.finalize_against(
                final_boxes[final_index].1,
                final_boxes[final_index].2,
                final_boxes[final_index].3,
                final_boxes[final_index].4,
            )
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();
    let final_box_ordinals = coverage
        .prepared_to_final
        .iter()
        .map(|final_index| final_boxes[*final_index].0)
        .collect::<Vec<_>>();
    let constructor_source =
        super::super::constructor_source_catalog::ParserConstructorSourceCatalogV1::issue(
            initial_callable_source.ast(),
            &source_seals,
            &final_box_ordinals,
        )
        .map_err(SourceSealFinalizationErrorV1::ConstructorSourceCatalog)?;
    drop(final_boxes);
    let initial_callable_source =
        initial_callable_source.attach_constructor_source(constructor_source);

    Ok(ParsedProgramWithSourceV1 {
        initial_callable_source,
        source_seals,
        final_box_ordinals: final_box_ordinals.into_boxed_slice(),
        generated_delegate_source_relations,
        metadata,
    })
}

fn finalize_compatibility_source(
    ast: &ASTNode,
    prepared: Vec<PreparedBoxSourceSealV1>,
    final_box_paths: Vec<SourceBoxDeclarationPathV1>,
) -> Result<(Box<[ParserBoxSourceSealV1]>, Box<[usize]>), SourceSealFinalizationErrorV1> {
    let coverage = FinalizerCoveragePlanV1::issue(&prepared, &final_box_paths)?;
    let final_boxes = final_boxes_for_source(ast, prepared.len(), true)?;
    let mut seals = Vec::with_capacity(prepared.len());
    let mut final_box_ordinals = Vec::with_capacity(prepared.len());
    for (prepared_index, prepared) in prepared.into_iter().enumerate() {
        let final_index = coverage.prepared_to_final[prepared_index];
        let (ordinal, name, is_sync, inventory, constructors) = final_boxes[final_index];
        seals.push(prepared.finalize_against(name, is_sync, inventory, constructors)?);
        final_box_ordinals.push(ordinal);
    }
    Ok((
        seals.into_boxed_slice(),
        final_box_ordinals.into_boxed_slice(),
    ))
}

fn validate_ordinary_source_seals(
    ast: &ASTNode,
    prepared: &[PreparedBoxSourceSealV1],
    coverage: &FinalizerCoveragePlanV1,
) -> Result<(), SourceSealFinalizationErrorV1> {
    let final_boxes = final_boxes_for_source(ast, prepared.len(), false)?;
    for (prepared_index, seal) in prepared.iter().enumerate() {
        let final_box = final_boxes[coverage.prepared_to_final[prepared_index]];
        seal.validate_against(final_box.1, final_box.2, final_box.3, final_box.4)?;
    }
    Ok(())
}

fn final_boxes_for_source(
    ast: &ASTNode,
    prepared_count: usize,
    allow_static: bool,
) -> Result<
    Vec<(
        usize,
        &str,
        bool,
        &BoxMethodInventoryV1,
        &std::collections::HashMap<String, ASTNode>,
    )>,
    SourceSealFinalizationErrorV1,
> {
    let ASTNode::Program { statements, .. } = ast else {
        return Err(SourceSealFinalizationErrorV1::OrdinaryBoxCountMismatch {
            prepared: prepared_count,
            final_ast: 0,
        });
    };
    if statements
        .iter()
        .any(|statement| matches!(statement, ASTNode::BuildGate { .. }))
    {
        return Err(SourceSealFinalizationErrorV1::TopLevelBuildGateUnsupported);
    }
    let mut final_boxes = Vec::new();
    for (ordinal, statement) in statements.iter().enumerate() {
        match statement {
            ASTNode::BoxDeclaration {
                name,
                methods,
                constructors,
                is_interface: false,
                is_record: false,
                is_static,
                is_sync,
                ..
            } if !*is_static => {
                final_boxes.push((ordinal, name.as_str(), *is_sync, methods, constructors))
            }
            ASTNode::BoxDeclaration {
                is_interface: false,
                is_record: false,
                is_static: true,
                ..
            } if allow_static => {}
            ASTNode::BoxDeclaration { .. } => {
                return Err(SourceSealFinalizationErrorV1::UnsupportedTopLevelBoxKind { ordinal });
            }
            _ => {}
        }
    }
    if final_boxes.len() != prepared_count {
        return Err(SourceSealFinalizationErrorV1::OrdinaryBoxCountMismatch {
            prepared: prepared_count,
            final_ast: final_boxes.len(),
        });
    }
    Ok(final_boxes)
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
        SourceSealFinalizationErrorV1::DeclarationNameMismatch { prepared, final_ast } => format!(
            "R6-S3B-B4 Box declaration name changed: prepared={prepared}, final={final_ast}"
        ),
        SourceSealFinalizationErrorV1::DeclarationKindMismatch => {
            "R6-S3B-B4 Box declaration kind is outside the ordinary source-seal cohort".to_owned()
        },
        SourceSealFinalizationErrorV1::DeclarationSyncMismatch { prepared, final_ast } => format!(
            "R6-S3B-B4 Box sync syntax changed: prepared={prepared}, final={final_ast}"
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
        SourceSealFinalizationErrorV1::ConstructorCoverageMismatch => {
            "parser constructor source inventory does not match final AST constructors".to_owned()
        }
        SourceSealFinalizationErrorV1::ConstructorSourceCatalog(error) => {
            format!("parser constructor source catalog rejected: {error:?}")
        }
        SourceSealFinalizationErrorV1::InitialCallableProgramSource(error) => {
            format!("initial callable Program source co-seal rejected: {error:?}")
        }
        SourceSealFinalizationErrorV1::Inventory(error) => {
            format!("R6-S3A final Box inventory is invalid: {error}")
        }
    };
    crate::parser::ParseError::BuildCfg { message, line: 0 }
}
