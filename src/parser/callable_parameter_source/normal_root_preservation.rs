//! Parser-owned final-transform preservation for the normal root role.
//!
//! The initial App/Script admission is issued by `normal_root_source.rs`.
//! This module only validates that the exact source-root statement cohort
//! survives the final callable transform and seals an opaque, non-Clone
//! handoff token.

mod consumer_loan;

pub(crate) use consumer_loan::{
    with_parser_normal_root_consumer_loan, ParserNormalAppProgramCursorV1,
    ParserNormalAppProgramItemLoanV1, ParserNormalAppResultSyntaxV1,
    ParserNormalAppRootBodyLoanV1, ParserNormalAppRootLoanV1,
    ParserNormalRootConsumerIncompleteV1, ParserNormalRootConsumerIntegrityIssueV1,
    ParserNormalRootConsumerLoanRejectV1, ParserNormalRootConsumerLoanV1,
    ParserNormalRootConsumerSourceUnavailableV1, ParserNormalScriptRootLoanV1,
    ParserNormalScriptStatementCursorV1, ParserNormalScriptStatementLoanV1,
};

use crate::ast::ASTNode;
use crate::parser::callable_source_anchor::{
    DirectCallableCommitPlacementV1, DirectCallableDeclarationKindV1, PreparedCallableSourceV1,
};
use crate::parser::initial_callable_program_source::InitialCallableFinalSlotV1;

use super::main_app_entry::ParserMainAppEntrySealV1;
use super::normal_root_source::ParserNormalRootSourceDispositionV1;
use super::parser_invocation_witness::ParserInvocationWitnessV1;
use super::script_source_authority::ParserNormalProgramSourceAuthorityDispositionV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalRootRoleV1 {
    App,
    Script,
}

#[derive(Debug)]
pub(crate) struct ParserNormalRootPreservedV1 {
    relation: ParserNormalRootRelationV1,
    _invocation: ParserInvocationWitnessV1,
    _seal: ParserNormalRootPreservedSealV1,
}

#[derive(Debug)]
enum ParserNormalRootRelationV1 {
    App(ParserNormalAppRootRelationV1),
    Script,
}

/// Private proof that the parser-admitted `Main.main/0` callable is the root
/// body and that its source parent admitted no static child.
#[derive(Debug)]
struct ParserNormalAppRootRelationV1 {
    _app_entry: ParserMainAppEntrySealV1,
    _final_slot: InitialCallableFinalSlotV1,
    _main_is_root: ParserCallableMainIsRootV1,
    _no_static_children: ParserNoStaticChildrenV1,
}

#[derive(Debug)]
struct ParserCallableMainIsRootV1;

#[derive(Debug)]
struct ParserNoStaticChildrenV1;

#[derive(Debug)]
struct ParserNormalRootPreservedSealV1;

#[derive(Debug)]
pub(crate) enum ParserNormalRootPreservationV1 {
    Ready(ParserNormalRootPreservedV1),
    Terminal(ParserNormalRootSourceDispositionV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParserNormalRootPreservationRejectV1 {
    SourceWitnessMissing,
    InitialProgramMissing,
    TransformedProgramMissing,
    SourceBodyCardinalityMismatch {
        source: usize,
        initial: usize,
        transformed: usize,
    },
    SourceStatementChanged {
        position: usize,
    },
    ParserWitnessMismatch,
    CallablePairingCardinalityMismatch {
        sources: usize,
        slots: usize,
    },
    AppCallableIdentityMissing,
    AppCallableIdentityDuplicate,
    AppCallableParserMismatch,
    AppCallableKindMismatch,
    AppCallableSourceRelationMismatch,
    AppCallableFinalSlotMismatch,
}

pub(crate) struct ParserNormalRootPreservationIssuerV1;

impl ParserNormalRootPreservationIssuerV1 {
    pub(crate) fn seal_after_transform(
        root: ParserNormalRootSourceDispositionV1,
        source_authority: &ParserNormalProgramSourceAuthorityDispositionV1,
        initial: &ASTNode,
        transformed: &ASTNode,
        callable_rows: &[PreparedCallableSourceV1],
        final_slots: &[InitialCallableFinalSlotV1],
    ) -> Result<ParserNormalRootPreservationV1, ParserNormalRootPreservationRejectV1> {
        match &root {
            ParserNormalRootSourceDispositionV1::AppReady(seal) => {
                let Some(witness) = source_authority.invocation_witness() else {
                    return Err(ParserNormalRootPreservationRejectV1::SourceWitnessMissing);
                };
                if !seal.same_parser_source(witness) {
                    return Err(ParserNormalRootPreservationRejectV1::ParserWitnessMismatch);
                }
            }
            ParserNormalRootSourceDispositionV1::ScriptReady(admission) => {
                let Some(witness) = source_authority.invocation_witness() else {
                    return Err(ParserNormalRootPreservationRejectV1::SourceWitnessMissing);
                };
                if !admission.same_parser_source_witness(witness) {
                    return Err(ParserNormalRootPreservationRejectV1::ParserWitnessMismatch);
                }
            }
            ParserNormalRootSourceDispositionV1::Outside(_)
            | ParserNormalRootSourceDispositionV1::ScriptTerminal(_)
            | ParserNormalRootSourceDispositionV1::SourceAuthorityUnavailable(_)
            | ParserNormalRootSourceDispositionV1::Incomplete(_)
            | ParserNormalRootSourceDispositionV1::IntegrityInvalid(_)
            | ParserNormalRootSourceDispositionV1::DiscardedBeforeA => {
                return Ok(ParserNormalRootPreservationV1::Terminal(root));
            }
        };

        let ASTNode::Program {
            statements: initial_statements,
            ..
        } = initial
        else {
            return Err(ParserNormalRootPreservationRejectV1::InitialProgramMissing);
        };
        let ASTNode::Program {
            statements: transformed_statements,
            ..
        } = transformed
        else {
            return Err(ParserNormalRootPreservationRejectV1::TransformedProgramMissing);
        };

        let source_body_count = match source_authority {
            ParserNormalProgramSourceAuthorityDispositionV1::Ready(authority) => {
                authority.body_rows().len()
            }
            ParserNormalProgramSourceAuthorityDispositionV1::SourceAuthorityUnavailable(_)
            | ParserNormalProgramSourceAuthorityDispositionV1::Incomplete(_)
            | ParserNormalProgramSourceAuthorityDispositionV1::IntegrityInvalid(_) => 0,
        };
        let initial_body_count = initial_statements.len();
        let transformed_body_count = transformed_statements.len();
        if source_body_count != initial_body_count || initial_body_count != transformed_body_count {
            return Err(
                ParserNormalRootPreservationRejectV1::SourceBodyCardinalityMismatch {
                    source: source_body_count,
                    initial: initial_body_count,
                    transformed: transformed_body_count,
                },
            );
        }
        if let Some(position) = initial_statements
            .iter()
            .zip(transformed_statements)
            .position(|(source, transformed)| source != transformed)
        {
            return Err(ParserNormalRootPreservationRejectV1::SourceStatementChanged { position });
        }

        let witness = source_authority
            .invocation_witness()
            .ok_or(ParserNormalRootPreservationRejectV1::SourceWitnessMissing)?;
        let relation = match root {
            ParserNormalRootSourceDispositionV1::AppReady(app_entry) => {
                ParserNormalRootRelationV1::App(seal_app_root_relation(
                    app_entry,
                    witness,
                    callable_rows,
                    final_slots,
                )?)
            }
            ParserNormalRootSourceDispositionV1::ScriptReady(_) => {
                ParserNormalRootRelationV1::Script
            }
            ParserNormalRootSourceDispositionV1::Outside(_)
            | ParserNormalRootSourceDispositionV1::ScriptTerminal(_)
            | ParserNormalRootSourceDispositionV1::SourceAuthorityUnavailable(_)
            | ParserNormalRootSourceDispositionV1::Incomplete(_)
            | ParserNormalRootSourceDispositionV1::IntegrityInvalid(_)
            | ParserNormalRootSourceDispositionV1::DiscardedBeforeA => {
                unreachable!("terminal root disposition returned before exact preservation")
            }
        };
        Ok(ParserNormalRootPreservationV1::Ready(
            ParserNormalRootPreservedV1 {
                relation,
                _invocation: witness.clone(),
                _seal: ParserNormalRootPreservedSealV1,
            },
        ))
    }
}

fn seal_app_root_relation(
    app_entry: ParserMainAppEntrySealV1,
    witness: &ParserInvocationWitnessV1,
    callable_rows: &[PreparedCallableSourceV1],
    final_slots: &[InitialCallableFinalSlotV1],
) -> Result<ParserNormalAppRootRelationV1, ParserNormalRootPreservationRejectV1> {
    if callable_rows.len() != final_slots.len() {
        return Err(
            ParserNormalRootPreservationRejectV1::CallablePairingCardinalityMismatch {
                sources: callable_rows.len(),
                slots: final_slots.len(),
            },
        );
    }

    let mut matches = callable_rows
        .iter()
        .zip(final_slots.iter().copied())
        .filter(|(source, _)| {
            source
                .anchor()
                .identity()
                .same_as(app_entry.callable_identity())
        });
    let Some((source, final_slot)) = matches.next() else {
        return Err(ParserNormalRootPreservationRejectV1::AppCallableIdentityMissing);
    };
    if matches.next().is_some() {
        return Err(ParserNormalRootPreservationRejectV1::AppCallableIdentityDuplicate);
    }
    if !witness.same_parser_brand(source.parser_brand()) {
        return Err(ParserNormalRootPreservationRejectV1::AppCallableParserMismatch);
    }

    let direct = source
        .direct()
        .ok_or(ParserNormalRootPreservationRejectV1::AppCallableKindMismatch)?;
    if direct.kind() != DirectCallableDeclarationKindV1::StaticBoxMethod {
        return Err(ParserNormalRootPreservationRejectV1::AppCallableKindMismatch);
    }
    let Some((declaration, gate_path, member_ordinal)) = direct.path().box_method_parts() else {
        return Err(ParserNormalRootPreservationRejectV1::AppCallableSourceRelationMismatch);
    };
    if !app_entry.method_site().is_direct()
        || !gate_path.is_empty()
        || declaration.compatibility_box_path() != app_entry.box_site().path()
        || member_ordinal != app_entry.method_site().source_member_ordinal()
    {
        return Err(ParserNormalRootPreservationRejectV1::AppCallableSourceRelationMismatch);
    }

    let InitialCallableFinalSlotV1::BoxMethod { statement, method } = final_slot else {
        return Err(ParserNormalRootPreservationRejectV1::AppCallableFinalSlotMismatch);
    };
    if statement != app_entry.box_site().statement_ordinal()
        || !matches!(
            direct.commit_placement(),
            DirectCallableCommitPlacementV1::BoxMethod {
                committed_inventory
            } if committed_inventory == method
        )
    {
        return Err(ParserNormalRootPreservationRejectV1::AppCallableFinalSlotMismatch);
    }

    Ok(ParserNormalAppRootRelationV1 {
        _app_entry: app_entry,
        _final_slot: final_slot,
        _main_is_root: ParserCallableMainIsRootV1,
        _no_static_children: ParserNoStaticChildrenV1,
    })
}

impl ParserNormalRootPreservedV1 {
    pub(crate) const fn role(&self) -> ParserNormalRootRoleV1 {
        match self.relation {
            ParserNormalRootRelationV1::App(_) => ParserNormalRootRoleV1::App,
            ParserNormalRootRelationV1::Script => ParserNormalRootRoleV1::Script,
        }
    }
}

impl ParserNormalRootPreservationV1 {
    pub(crate) fn role(&self) -> Option<ParserNormalRootRoleV1> {
        match self {
            Self::Ready(preserved) => Some(preserved.role()),
            Self::Terminal(_) => None,
        }
    }

    pub(crate) fn is_discarded_before_a(&self) -> bool {
        matches!(
            self,
            Self::Terminal(ParserNormalRootSourceDispositionV1::DiscardedBeforeA)
        )
    }
}
