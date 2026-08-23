//! Parser-owned source-root disposition for the normal callable transport.
//!
//! This module co-seals the already-issued App and Script parser products at
//! one boundary.  It owns only an opaque root witness.  Script source rows
//! remain a separate A-handoff product and never enter this disposition.

use super::canonical_script_source_admission::{
    CanonicalScriptCohortAdmissionV1, CanonicalScriptCohortDispositionV1,
};
use super::main_app_entry::{
    ParserMainAppEntryDispositionV1, ParserMainAppEntryIncompleteV1,
    ParserMainAppEntryIntegrityIssueV1, ParserMainAppEntryOutsideReasonV1,
    ParserMainAppEntrySealV1, ParserMainAppEntryUnavailableV1,
};
use super::parser_invocation_witness::ParserInvocationWitnessV1;
use super::script_source_authority::{
    ParserNormalProgramSourceAuthorityDispositionV1,
    ParserNormalProgramSourceAuthorityIncompleteV1,
    ParserNormalProgramSourceAuthorityIntegrityIssueV1,
    ParserNormalProgramSourceAuthorityUnavailableV1,
};
use super::script_source_rows::CanonicalScriptSourceRowsDispositionV1;

#[derive(Debug)]
pub(in crate::parser) enum ParserNormalRootSourceDispositionV1 {
    AppReady(ParserMainAppEntrySealV1),
    ScriptReady(CanonicalScriptCohortAdmissionV1),
    Outside(ParserMainAppEntryOutsideReasonV1),
    ScriptTerminal(ParserNormalRootScriptTerminalV1),
    SourceAuthorityUnavailable(ParserNormalRootSourceUnavailableV1),
    Incomplete(ParserNormalRootSourceIncompleteV1),
    IntegrityInvalid(ParserNormalRootSourceIntegrityIssueV1),
    DiscardedBeforeA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalRootScriptTerminalV1 {
    NotApplicable,
    CompatibilitySource,
    Deferred,
    AdmissionMissing,
    CohortUnresolved,
    ObservationIncomplete,
    NonCandidate,
    DispositionTransported,
    RowsNotHandoffReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalRootSourceUnavailableV1 {
    SourceAuthority(ParserNormalProgramSourceAuthorityUnavailableV1),
    AppEntry(ParserMainAppEntryUnavailableV1),
    ScriptAdmission,
    ScriptRows,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalRootSourceIncompleteV1 {
    SourceAuthority(ParserNormalProgramSourceAuthorityIncompleteV1),
    AppEntry(ParserMainAppEntryIncompleteV1),
    ScriptAdmission,
    ScriptRows,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalRootSourceIntegrityIssueV1 {
    SourceAuthority(ParserNormalProgramSourceAuthorityIntegrityIssueV1),
    AppEntry(ParserMainAppEntryIntegrityIssueV1),
    ScriptAdmission,
    ScriptRows,
    ParserWitnessMismatch,
    ContradictoryScriptEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalRootSourceDiscardErrorV1 {
    AppReadyRequiresNormalRootConsumer,
}

pub(super) fn issue_parser_normal_root_source_v1(
    app_entry: ParserMainAppEntryDispositionV1,
    script_admission: CanonicalScriptCohortDispositionV1,
    script_rows: CanonicalScriptSourceRowsDispositionV1,
    source_authority: &ParserNormalProgramSourceAuthorityDispositionV1,
) -> (
    ParserNormalRootSourceDispositionV1,
    CanonicalScriptSourceRowsDispositionV1,
) {
    let Some(source_witness) = source_authority.invocation_witness() else {
        return (map_source_authority_failure(source_authority), script_rows);
    };

    match app_entry {
        ParserMainAppEntryDispositionV1::AppMainReady(seal) => {
            let root = if !seal.same_parser_source(source_witness) {
                ParserNormalRootSourceDispositionV1::IntegrityInvalid(
                    ParserNormalRootSourceIntegrityIssueV1::ParserWitnessMismatch,
                )
            } else if matches!(
                script_admission,
                CanonicalScriptCohortDispositionV1::CompatibilitySource
            ) && matches!(
                script_rows,
                CanonicalScriptSourceRowsDispositionV1::CompatibilitySource
            ) {
                ParserNormalRootSourceDispositionV1::AppReady(seal)
            } else {
                ParserNormalRootSourceDispositionV1::IntegrityInvalid(
                    ParserNormalRootSourceIntegrityIssueV1::ContradictoryScriptEvidence,
                )
            };
            (root, script_rows)
        }
        ParserMainAppEntryDispositionV1::Outside(reason) => {
            issue_outside_root(reason, script_admission, script_rows, source_witness)
        }
        ParserMainAppEntryDispositionV1::SourceAuthorityUnavailable(reason) => (
            ParserNormalRootSourceDispositionV1::SourceAuthorityUnavailable(
                ParserNormalRootSourceUnavailableV1::AppEntry(reason),
            ),
            script_rows,
        ),
        ParserMainAppEntryDispositionV1::Incomplete(reason) => (
            ParserNormalRootSourceDispositionV1::Incomplete(
                ParserNormalRootSourceIncompleteV1::AppEntry(reason),
            ),
            script_rows,
        ),
        ParserMainAppEntryDispositionV1::IntegrityInvalid(reason) => (
            ParserNormalRootSourceDispositionV1::IntegrityInvalid(
                ParserNormalRootSourceIntegrityIssueV1::AppEntry(reason),
            ),
            script_rows,
        ),
    }
}

fn map_source_authority_failure(
    source_authority: &ParserNormalProgramSourceAuthorityDispositionV1,
) -> ParserNormalRootSourceDispositionV1 {
    match source_authority {
        ParserNormalProgramSourceAuthorityDispositionV1::SourceAuthorityUnavailable(reason) => {
            ParserNormalRootSourceDispositionV1::SourceAuthorityUnavailable(
                ParserNormalRootSourceUnavailableV1::SourceAuthority(*reason),
            )
        }
        ParserNormalProgramSourceAuthorityDispositionV1::Incomplete(reason) => {
            ParserNormalRootSourceDispositionV1::Incomplete(
                ParserNormalRootSourceIncompleteV1::SourceAuthority(*reason),
            )
        }
        ParserNormalProgramSourceAuthorityDispositionV1::IntegrityInvalid(reason) => {
            ParserNormalRootSourceDispositionV1::IntegrityInvalid(
                ParserNormalRootSourceIntegrityIssueV1::SourceAuthority(*reason),
            )
        }
        ParserNormalProgramSourceAuthorityDispositionV1::Ready(_) => {
            unreachable!("ready source authority has an invocation witness")
        }
    }
}

fn issue_outside_root(
    reason: ParserMainAppEntryOutsideReasonV1,
    script_admission: CanonicalScriptCohortDispositionV1,
    script_rows: CanonicalScriptSourceRowsDispositionV1,
    source_witness: &ParserInvocationWitnessV1,
) -> (
    ParserNormalRootSourceDispositionV1,
    CanonicalScriptSourceRowsDispositionV1,
) {
    if reason != ParserMainAppEntryOutsideReasonV1::ProgramCohort {
        if matches!(
            script_admission,
            CanonicalScriptCohortDispositionV1::CanonicalScriptCohortAdmitted(_)
        ) || matches!(
            script_rows,
            CanonicalScriptSourceRowsDispositionV1::HandoffReady(_)
        ) {
            return (
                ParserNormalRootSourceDispositionV1::IntegrityInvalid(
                    ParserNormalRootSourceIntegrityIssueV1::ContradictoryScriptEvidence,
                ),
                script_rows,
            );
        }
        return (
            ParserNormalRootSourceDispositionV1::Outside(reason),
            script_rows,
        );
    }

    let CanonicalScriptCohortDispositionV1::CanonicalScriptCohortAdmitted(admission) =
        script_admission
    else {
        if matches!(
            script_admission,
            CanonicalScriptCohortDispositionV1::NotApplicable
                | CanonicalScriptCohortDispositionV1::CompatibilitySource
                | CanonicalScriptCohortDispositionV1::DispositionTransported
        ) {
            return (
                ParserNormalRootSourceDispositionV1::Outside(reason),
                script_rows,
            );
        }
        return (
            map_script_admission_terminal(script_admission),
            script_rows,
        );
    };

    let CanonicalScriptSourceRowsDispositionV1::HandoffReady(rows) = script_rows else {
        let root = map_script_rows_terminal(&script_rows);
        return (root, script_rows);
    };

    if !admission.same_parser_source_witness(source_witness)
        || !rows.parser_invocation_witness().same_as(source_witness)
    {
        return (
            ParserNormalRootSourceDispositionV1::IntegrityInvalid(
                ParserNormalRootSourceIntegrityIssueV1::ParserWitnessMismatch,
            ),
            CanonicalScriptSourceRowsDispositionV1::HandoffReady(rows),
        );
    }

    (
        ParserNormalRootSourceDispositionV1::ScriptReady(admission),
        CanonicalScriptSourceRowsDispositionV1::HandoffReady(rows),
    )
}

fn map_script_admission_terminal(
    disposition: CanonicalScriptCohortDispositionV1,
) -> ParserNormalRootSourceDispositionV1 {
    match disposition {
        CanonicalScriptCohortDispositionV1::NotApplicable => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::NotApplicable,
            )
        }
        CanonicalScriptCohortDispositionV1::CompatibilitySource => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::CompatibilitySource,
            )
        }
        CanonicalScriptCohortDispositionV1::Deferred => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::Deferred,
            )
        }
        CanonicalScriptCohortDispositionV1::SourceAuthorityUnavailable => {
            ParserNormalRootSourceDispositionV1::SourceAuthorityUnavailable(
                ParserNormalRootSourceUnavailableV1::ScriptAdmission,
            )
        }
        CanonicalScriptCohortDispositionV1::CohortUnresolved => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::CohortUnresolved,
            )
        }
        CanonicalScriptCohortDispositionV1::IntegrityInvalid => {
            ParserNormalRootSourceDispositionV1::IntegrityInvalid(
                ParserNormalRootSourceIntegrityIssueV1::ScriptAdmission,
            )
        }
        CanonicalScriptCohortDispositionV1::ObservationIncomplete => {
            ParserNormalRootSourceDispositionV1::Incomplete(
                ParserNormalRootSourceIncompleteV1::ScriptAdmission,
            )
        }
        CanonicalScriptCohortDispositionV1::NonCandidate => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::NonCandidate,
            )
        }
        CanonicalScriptCohortDispositionV1::DispositionTransported => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::DispositionTransported,
            )
        }
        CanonicalScriptCohortDispositionV1::CanonicalScriptCohortAdmitted(_) => {
            unreachable!("admitted Script state is handled before terminal mapping")
        }
    }
}

fn map_script_rows_terminal(
    disposition: &CanonicalScriptSourceRowsDispositionV1,
) -> ParserNormalRootSourceDispositionV1 {
    match disposition {
        CanonicalScriptSourceRowsDispositionV1::NotApplicable => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::NotApplicable,
            )
        }
        CanonicalScriptSourceRowsDispositionV1::CompatibilitySource => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::CompatibilitySource,
            )
        }
        CanonicalScriptSourceRowsDispositionV1::Deferred => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::Deferred,
            )
        }
        CanonicalScriptSourceRowsDispositionV1::AdmissionMissing => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::AdmissionMissing,
            )
        }
        CanonicalScriptSourceRowsDispositionV1::SourceAuthorityUnavailable => {
            ParserNormalRootSourceDispositionV1::SourceAuthorityUnavailable(
                ParserNormalRootSourceUnavailableV1::ScriptRows,
            )
        }
        CanonicalScriptSourceRowsDispositionV1::CohortUnresolved => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::CohortUnresolved,
            )
        }
        CanonicalScriptSourceRowsDispositionV1::ObservationIncomplete => {
            ParserNormalRootSourceDispositionV1::Incomplete(
                ParserNormalRootSourceIncompleteV1::ScriptRows,
            )
        }
        CanonicalScriptSourceRowsDispositionV1::IntegrityInvalid => {
            ParserNormalRootSourceDispositionV1::IntegrityInvalid(
                ParserNormalRootSourceIntegrityIssueV1::ScriptRows,
            )
        }
        CanonicalScriptSourceRowsDispositionV1::NonCandidate => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::NonCandidate,
            )
        }
        CanonicalScriptSourceRowsDispositionV1::MovedToParallelHandoff => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::DispositionTransported,
            )
        }
        CanonicalScriptSourceRowsDispositionV1::DispositionTransported => {
            ParserNormalRootSourceDispositionV1::ScriptTerminal(
                ParserNormalRootScriptTerminalV1::DispositionTransported,
            )
        }
        CanonicalScriptSourceRowsDispositionV1::HandoffReady(_) => {
            unreachable!("handoff-ready rows are handled before terminal mapping")
        }
    }
}

impl ParserNormalRootSourceDispositionV1 {
    pub(in crate::parser) fn discard_before_a(
        self,
    ) -> Result<Self, ParserNormalRootSourceDiscardErrorV1> {
        match self {
            Self::AppReady(_) => Err(
                ParserNormalRootSourceDiscardErrorV1::AppReadyRequiresNormalRootConsumer,
            ),
            Self::DiscardedBeforeA => Ok(Self::DiscardedBeforeA),
            Self::ScriptReady(_)
            | Self::Outside(_)
            | Self::ScriptTerminal(_)
            | Self::SourceAuthorityUnavailable(_)
            | Self::Incomplete(_)
            | Self::IntegrityInvalid(_) => Ok(Self::DiscardedBeforeA),
        }
    }

    pub(in crate::parser) const fn is_discarded_before_a(&self) -> bool {
        matches!(self, Self::DiscardedBeforeA)
    }
}
