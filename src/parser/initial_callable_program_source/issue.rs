use crate::ast::{ASTNode, BoxMethodGeneratedProvenanceV1, BoxMethodProvenanceV1};

use super::model::{
    InitialCallableFinalSlotV1, VerifiedInitialCallableProgramSourceV1,
    VerifiedInitialCallableSourceRowV1,
};
use crate::parser::build_cfg::program_item_slots::{
    ProjectedProgramItemSlotErrorV1, ProjectedProgramItemSlotSetV1,
};
use crate::parser::callable_source_anchor::{
    DirectCallableCommitPlacementV1, DirectCallableDeclarationKindV1, GeneratedCallableOriginV1,
    PreparedCallableSourceV1, PreparedDirectCallableSourceV1,
};
use crate::parser::source_authority::{MethodSourceRelationV1, PreparedBoxSourceSealV1};
use crate::parser::source_path::{
    SourceProgramCallablePathV1, SourceProgramDeclarationPathV1, SourceProgramMemberGateStepV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::parser) enum InitialCallableProgramSourceRejectV1 {
    NotProgram,
    MissingProgramSlotSet,
    ForeignParser,
    DuplicateAnchor,
    MissingProgramSlot,
    FinalSlotOutOfRange,
    WrongDeclarationKind,
    WrongDirectCommitPlacement,
    MissingBoxSourceSeal,
    SelectedMethodRelationCoverage,
    UnsupportedBoxKind,
    UnsupportedMethodProvenance,
    GeneratedOriginMismatch,
    DuplicateFinalCallableSlot,
    CallableCoverageMismatch { expected: usize, actual: usize },
}

pub(in crate::parser) fn issue_initial_callable_program_source_v1(
    ast: ASTNode,
    callable_rows: Box<[PreparedCallableSourceV1]>,
    program_slots: Option<ProjectedProgramItemSlotSetV1>,
    box_seals: &[PreparedBoxSourceSealV1],
) -> Result<VerifiedInitialCallableProgramSourceV1, InitialCallableProgramSourceRejectV1> {
    let program_slots =
        program_slots.ok_or(InitialCallableProgramSourceRejectV1::MissingProgramSlotSet)?;
    let program_brand = program_slots.brand();
    for (index, row) in callable_rows.iter().enumerate() {
        if !row.parser_brand().same_as(program_brand)
            || !row
                .source_path()
                .declaration()
                .brand()
                .same_as(program_brand)
        {
            return Err(InitialCallableProgramSourceRejectV1::ForeignParser);
        }
        if callable_rows[..index]
            .iter()
            .any(|previous| previous.anchor().same_as(row.anchor()))
        {
            return Err(InitialCallableProgramSourceRejectV1::DuplicateAnchor);
        }
    }

    let expected = expected_callable_slots(&ast)?;
    let mut verified = Vec::with_capacity(callable_rows.len());
    for row in callable_rows {
        let slot = resolve_callable_slot(&ast, &program_slots, box_seals, &row)?;
        if verified
            .iter()
            .any(|previous: &VerifiedInitialCallableSourceRowV1| previous.final_slot() == slot)
        {
            return Err(InitialCallableProgramSourceRejectV1::DuplicateFinalCallableSlot);
        }
        verified.push(VerifiedInitialCallableSourceRowV1::new(row, slot));
    }
    if !same_slot_coverage(&expected, &verified) {
        return Err(
            InitialCallableProgramSourceRejectV1::CallableCoverageMismatch {
                expected: expected.len(),
                actual: verified.len(),
            },
        );
    }
    verified.sort_by_key(|row| slot_key(row.final_slot()));
    Ok(VerifiedInitialCallableProgramSourceV1::issue(ast, verified))
}

/// Explicit admission for the historical postpass compatibility arm.
///
/// This admission is selected before issuer execution. It identifies the
/// bounded static/mixed/direct Program shapes whose existing compatibility
/// transform cannot add an unanchored callable before the atomic co-seal.
pub(in crate::parser) fn compatibility_program_can_enter_initial_callable_lane_v1(
    ast: &ASTNode,
) -> bool {
    let ASTNode::Program { statements, .. } = ast else {
        return false;
    };
    statements.iter().all(|statement| match statement {
        ASTNode::BuildGate { .. } => false,
        ASTNode::BoxDeclaration {
            methods,
            delegates,
            is_interface,
            is_record,
            ..
        } => {
            !*is_interface
                && !*is_record
                && delegates.is_empty()
                && methods.iter_selected_declaration_order().all(|entry| {
                    matches!(
                        entry.provenance(),
                        BoxMethodProvenanceV1::ExplicitSource { .. }
                            | BoxMethodProvenanceV1::Generated(
                                BoxMethodGeneratedProvenanceV1::Property { .. }
                            )
                    )
                })
        }
        _ => true,
    })
}

fn expected_callable_slots(
    ast: &ASTNode,
) -> Result<Vec<InitialCallableFinalSlotV1>, InitialCallableProgramSourceRejectV1> {
    let ASTNode::Program { statements, .. } = ast else {
        return Err(InitialCallableProgramSourceRejectV1::NotProgram);
    };
    let mut slots = Vec::new();
    for (statement, node) in statements.iter().enumerate() {
        let statement = u32::try_from(statement)
            .map_err(|_| InitialCallableProgramSourceRejectV1::FinalSlotOutOfRange)?;
        match node {
            ASTNode::FunctionDeclaration { .. } => {
                slots.push(InitialCallableFinalSlotV1::TopLevel { statement });
            }
            ASTNode::BoxDeclaration {
                methods,
                is_interface,
                is_record,
                ..
            } => {
                if *is_interface || *is_record {
                    return Err(InitialCallableProgramSourceRejectV1::UnsupportedBoxKind);
                }
                for entry in methods.iter_selected_declaration_order() {
                    if !matches!(entry.declaration(), ASTNode::FunctionDeclaration { .. }) {
                        return Err(InitialCallableProgramSourceRejectV1::WrongDeclarationKind);
                    }
                    match entry.provenance() {
                        BoxMethodProvenanceV1::ExplicitSource { .. }
                        | BoxMethodProvenanceV1::Generated(
                            BoxMethodGeneratedProvenanceV1::Property { .. }
                            | BoxMethodGeneratedProvenanceV1::Delegate { .. },
                        ) => {}
                        BoxMethodProvenanceV1::Generated(
                            BoxMethodGeneratedProvenanceV1::MacroOrImport { .. },
                        )
                        | BoxMethodProvenanceV1::CompatibilityOnly { .. } => {
                            return Err(
                                InitialCallableProgramSourceRejectV1::UnsupportedMethodProvenance,
                            )
                        }
                    }
                    slots.push(InitialCallableFinalSlotV1::BoxMethod {
                        statement,
                        method: entry.site(),
                    });
                }
            }
            ASTNode::BuildGate { .. } => {
                return Err(InitialCallableProgramSourceRejectV1::MissingProgramSlot)
            }
            _ => {}
        }
    }
    Ok(slots)
}

fn resolve_callable_slot(
    ast: &ASTNode,
    program_slots: &ProjectedProgramItemSlotSetV1,
    box_seals: &[PreparedBoxSourceSealV1],
    row: &PreparedCallableSourceV1,
) -> Result<InitialCallableFinalSlotV1, InitialCallableProgramSourceRejectV1> {
    let source_path = row.source_path();
    let statement = program_slots
        .exact_final_slot(source_path.declaration())
        .map_err(map_program_slot_error)?
        .ok_or(InitialCallableProgramSourceRejectV1::MissingProgramSlot)?;
    match source_path {
        SourceProgramCallablePathV1::TopLevel { .. } => {
            let direct = row
                .direct()
                .ok_or(InitialCallableProgramSourceRejectV1::WrongDeclarationKind)?;
            if direct.commit_placement() != DirectCallableCommitPlacementV1::TopLevel {
                return Err(InitialCallableProgramSourceRejectV1::WrongDirectCommitPlacement);
            }
            validate_top_level(ast, statement, direct)?;
            Ok(InitialCallableFinalSlotV1::TopLevel { statement })
        }
        SourceProgramCallablePathV1::BoxMethod {
            declaration,
            gate_path,
            member_ordinal,
        } => {
            let method =
                resolve_method_ordinal(declaration, gate_path, *member_ordinal, box_seals, row)?;
            validate_box_method(ast, statement, method, row)?;
            Ok(InitialCallableFinalSlotV1::BoxMethod { statement, method })
        }
    }
}

fn validate_top_level(
    ast: &ASTNode,
    statement: u32,
    direct: &PreparedDirectCallableSourceV1,
) -> Result<(), InitialCallableProgramSourceRejectV1> {
    let node = program_statement(ast, statement)?;
    let ASTNode::FunctionDeclaration { is_static, .. } = node else {
        return Err(InitialCallableProgramSourceRejectV1::WrongDeclarationKind);
    };
    let valid = matches!(direct.kind(), DirectCallableDeclarationKindV1::FreeFunction)
        && !*is_static
        || matches!(
            direct.kind(),
            DirectCallableDeclarationKindV1::FreeStaticFunction
        ) && *is_static;
    valid
        .then_some(())
        .ok_or(InitialCallableProgramSourceRejectV1::WrongDeclarationKind)
}

fn resolve_method_ordinal(
    declaration: &SourceProgramDeclarationPathV1,
    gate_path: &[SourceProgramMemberGateStepV1],
    member_ordinal: u32,
    box_seals: &[PreparedBoxSourceSealV1],
    row: &PreparedCallableSourceV1,
) -> Result<crate::ast::BoxMethodInventoryOrdinalV1, InitialCallableProgramSourceRejectV1> {
    if let Some(direct) = row.direct() {
        let DirectCallableCommitPlacementV1::BoxMethod {
            committed_inventory,
        } = direct.commit_placement()
        else {
            return Err(InitialCallableProgramSourceRejectV1::WrongDirectCommitPlacement);
        };
        if gate_path.is_empty() {
            return Ok(committed_inventory);
        }
        let seal = exact_box_seal(declaration, box_seals)?;
        let matches = seal
            .method_relations()
            .iter()
            .filter_map(|relation| match relation {
                MethodSourceRelationV1::Explicit(relation)
                    if relation.source_site().source_member_ordinal() == member_ordinal
                        && relation.source_site().box_site().path()
                            == declaration.compatibility_box_path()
                        && seal.member_gate_selection_receipts().iter().any(|receipt| {
                            receipt
                                .exact_selected_path_for_method_site(
                                    declaration,
                                    relation.source_site(),
                                )
                                .is_some_and(|selected| selected == gate_path)
                        }) =>
                {
                    Some(relation.inventory_ordinal())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        return match matches.as_slice() {
            [method] => Ok(*method),
            _ => Err(InitialCallableProgramSourceRejectV1::SelectedMethodRelationCoverage),
        };
    }

    let generated = row
        .generated()
        .ok_or(InitialCallableProgramSourceRejectV1::WrongDeclarationKind)?;
    match generated.origin() {
        GeneratedCallableOriginV1::Property(origin) => {
            if origin.source_path() != row.source_path()
                || origin.source_path().declaration() != declaration
            {
                return Err(InitialCallableProgramSourceRejectV1::GeneratedOriginMismatch);
            }
            Ok(origin.placement().inventory_ordinal())
        }
        GeneratedCallableOriginV1::Delegate(origin) => {
            if origin.source_path() != row.source_path()
                || origin.relation().host_box_path() != declaration.compatibility_box_path()
            {
                return Err(InitialCallableProgramSourceRejectV1::GeneratedOriginMismatch);
            }
            Ok(origin
                .relation()
                .generated_inventory_placement()
                .inventory_ordinal())
        }
    }
}

fn validate_box_method(
    ast: &ASTNode,
    statement: u32,
    method: crate::ast::BoxMethodInventoryOrdinalV1,
    row: &PreparedCallableSourceV1,
) -> Result<(), InitialCallableProgramSourceRejectV1> {
    let ASTNode::BoxDeclaration {
        methods,
        is_interface,
        is_record,
        is_static,
        ..
    } = program_statement(ast, statement)?
    else {
        return Err(InitialCallableProgramSourceRejectV1::WrongDeclarationKind);
    };
    if *is_interface || *is_record {
        return Err(InitialCallableProgramSourceRejectV1::UnsupportedBoxKind);
    }
    let entry = methods
        .iter_selected_declaration_order()
        .nth(method.inventory_ordinal() as usize)
        .ok_or(InitialCallableProgramSourceRejectV1::FinalSlotOutOfRange)?;
    if !matches!(entry.declaration(), ASTNode::FunctionDeclaration { .. }) {
        return Err(InitialCallableProgramSourceRejectV1::WrongDeclarationKind);
    }
    match row {
        PreparedCallableSourceV1::Direct(direct) => {
            let kind_matches = matches!(
                direct.kind(),
                DirectCallableDeclarationKindV1::StaticBoxMethod
            ) == *is_static;
            if !kind_matches
                || !matches!(
                    entry.provenance(),
                    BoxMethodProvenanceV1::ExplicitSource { .. }
                )
            {
                return Err(InitialCallableProgramSourceRejectV1::WrongDeclarationKind);
            }
        }
        PreparedCallableSourceV1::Generated(generated) => match generated.origin() {
            GeneratedCallableOriginV1::Property(origin)
                if matches!(
                    entry.provenance(),
                    BoxMethodProvenanceV1::Generated(provenance)
                        if provenance == origin.provenance()
                ) => {}
            GeneratedCallableOriginV1::Delegate(origin)
                if matches!(
                    entry.provenance(),
                    BoxMethodProvenanceV1::Generated(provenance)
                        if provenance == origin.relation().generated_name_provenance()
                ) => {}
            _ => return Err(InitialCallableProgramSourceRejectV1::GeneratedOriginMismatch),
        },
    }
    Ok(())
}

fn exact_box_seal<'a>(
    declaration: &SourceProgramDeclarationPathV1,
    seals: &'a [PreparedBoxSourceSealV1],
) -> Result<&'a PreparedBoxSourceSealV1, InitialCallableProgramSourceRejectV1> {
    let matches = seals
        .iter()
        .filter(|seal| seal.box_site().path() == declaration.compatibility_box_path())
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [seal] => Ok(*seal),
        _ => Err(InitialCallableProgramSourceRejectV1::MissingBoxSourceSeal),
    }
}

fn program_statement(
    ast: &ASTNode,
    statement: u32,
) -> Result<&ASTNode, InitialCallableProgramSourceRejectV1> {
    let ASTNode::Program { statements, .. } = ast else {
        return Err(InitialCallableProgramSourceRejectV1::NotProgram);
    };
    statements
        .get(statement as usize)
        .ok_or(InitialCallableProgramSourceRejectV1::FinalSlotOutOfRange)
}

fn map_program_slot_error(
    error: ProjectedProgramItemSlotErrorV1,
) -> InitialCallableProgramSourceRejectV1 {
    match error {
        ProjectedProgramItemSlotErrorV1::ForeignParser => {
            InitialCallableProgramSourceRejectV1::ForeignParser
        }
        ProjectedProgramItemSlotErrorV1::DuplicateSourcePath
        | ProjectedProgramItemSlotErrorV1::DuplicateFinalSlot
        | ProjectedProgramItemSlotErrorV1::FinalSlotOverflow => {
            InitialCallableProgramSourceRejectV1::FinalSlotOutOfRange
        }
    }
}

fn same_slot_coverage(
    expected: &[InitialCallableFinalSlotV1],
    actual: &[VerifiedInitialCallableSourceRowV1],
) -> bool {
    expected.len() == actual.len()
        && expected.iter().all(|slot| {
            actual
                .iter()
                .filter(|row| row.final_slot() == *slot)
                .count()
                == 1
        })
}

fn slot_key(slot: InitialCallableFinalSlotV1) -> (u32, u32) {
    match slot {
        InitialCallableFinalSlotV1::TopLevel { statement } => (statement, 0),
        InitialCallableFinalSlotV1::BoxMethod { statement, method } => {
            (statement, method.inventory_ordinal().saturating_add(1))
        }
    }
}

trait CallableSourcePathV1 {
    fn source_path(&self) -> &SourceProgramCallablePathV1;
}

impl CallableSourcePathV1 for PreparedCallableSourceV1 {
    fn source_path(&self) -> &SourceProgramCallablePathV1 {
        match self {
            PreparedCallableSourceV1::Direct(row) => row.path(),
            PreparedCallableSourceV1::Generated(row) => match row.origin() {
                GeneratedCallableOriginV1::Property(origin) => origin.source_path(),
                GeneratedCallableOriginV1::Delegate(origin) => origin.source_path(),
            },
        }
    }
}
