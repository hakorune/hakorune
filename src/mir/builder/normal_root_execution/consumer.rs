use crate::parser::{
    ParserNormalRootExecutionPreservationV1, ParserNormalRootExecutionRoleV1,
    ParserNormalRootExecutionTerminalClassV1, VerifiedFinalCallableProgramSourceV1,
};
use std::collections::HashMap;

use super::super::program_declaration_facts::PreparedNormalProgramDeclarationFactsV1;
use super::super::{MainExpansionErrorV1, PreparedAdmittedNormalRootExpansionV1};
use super::model::{AdmittedNormalRootExecutionModeV1, PreparedNormalRootExecutionConsumptionV1};
use super::NormalRootExecutionProjectionPermitV1;
use crate::mir::normal_source_plan::{require_static_main_v1, NormalSourcePlanErrorV1};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum NormalRootExecutionConsumerRejectV1 {
    SourceAuthorityUnavailable,
    Incomplete,
    IntegrityInvalid,
    SourcePolicy(NormalSourcePlanErrorV1),
    StructuralProjection(MainExpansionErrorV1),
}

#[derive(Debug)]
pub(crate) struct RejectedNormalRootExecutionConsumptionV1 {
    source: VerifiedFinalCallableProgramSourceV1,
    error: NormalRootExecutionConsumerRejectV1,
}

impl RejectedNormalRootExecutionConsumptionV1 {
    pub(crate) const fn error(&self) -> &NormalRootExecutionConsumerRejectV1 {
        &self.error
    }

    pub(in crate::mir::builder) fn static_box_construction_error(
        &self,
        imports: &HashMap<String, String>,
    ) -> Option<String> {
        if self.error != NormalRootExecutionConsumerRejectV1::IntegrityInvalid {
            return None;
        }
        // A static box with no methods can be absent from the admitted root
        // inventory while still being a declaration that rules out construction.
        let ast = self.source.ast();
        PreparedNormalProgramDeclarationFactsV1::collect(ast)
            .ok()?
            .reject_static_box_construction(ast, imports)
            .err()
    }

    pub(crate) fn discard_at_named_root_execution_terminal(self) {
        let Self { source, error } = self;
        source.discard_at_named_root_execution_terminal();
        drop(error);
    }

    #[cfg(test)]
    pub(crate) fn into_error_after_discard(self) -> NormalRootExecutionConsumerRejectV1 {
        let Self { source, error } = self;
        source.discard_at_named_root_execution_terminal();
        error
    }
}

pub(in crate::mir) struct NormalRootExecutionConsumerV1;

impl NormalRootExecutionConsumerV1 {
    pub(in crate::mir) fn consume_once(
        source: VerifiedFinalCallableProgramSourceV1,
    ) -> Result<PreparedNormalRootExecutionConsumptionV1, RejectedNormalRootExecutionConsumptionV1>
    {
        let mode = match source.normal_root_execution() {
            ParserNormalRootExecutionPreservationV1::Ready(preserved) => {
                match preserved.source().role() {
                    ParserNormalRootExecutionRoleV1::App => {
                        let policy = preserved
                            .source()
                            .app_relation()
                            .ok_or(NormalSourcePlanErrorV1::RootExecutionRelationMismatch)
                            .and_then(|relation| {
                                require_static_main_v1(relation.main_box_is_static())
                            });
                        if let Err(error) = policy {
                            return Err(RejectedNormalRootExecutionConsumptionV1 {
                                source,
                                error: NormalRootExecutionConsumerRejectV1::SourcePolicy(error),
                            });
                        }
                        AdmittedNormalRootExecutionModeV1::App
                    }
                    ParserNormalRootExecutionRoleV1::ProgramRuntime => {
                        AdmittedNormalRootExecutionModeV1::ProgramRuntime
                    }
                }
            }
            ParserNormalRootExecutionPreservationV1::Terminal(terminal) => {
                let error = match terminal
                    .terminal_class()
                    .expect("preservation Terminal cannot contain Ready")
                {
                    ParserNormalRootExecutionTerminalClassV1::SourceAuthorityUnavailable => {
                        NormalRootExecutionConsumerRejectV1::SourceAuthorityUnavailable
                    }
                    ParserNormalRootExecutionTerminalClassV1::Incomplete => {
                        NormalRootExecutionConsumerRejectV1::Incomplete
                    }
                    ParserNormalRootExecutionTerminalClassV1::IntegrityInvalid => {
                        NormalRootExecutionConsumerRejectV1::IntegrityInvalid
                    }
                };
                return Err(RejectedNormalRootExecutionConsumptionV1 { source, error });
            }
        };
        // Admission condition: a source carrying merged-source lineage must
        // also carry its issued source-admission witness. Materialization
        // issues the witness (possibly with zero rows) for every merged
        // source-backed product; a missing witness here means the source
        // bypassed that boundary and must reject before package effects.
        if let Some(lineage) = source.source_lineage() {
            if lineage.merged_source_lineage().is_some()
                && lineage.source_admission_witness().is_none()
            {
                return Err(RejectedNormalRootExecutionConsumptionV1 {
                    source,
                    error: NormalRootExecutionConsumerRejectV1::SourceAuthorityUnavailable,
                });
            }
        }
        let root_execution = match PreparedAdmittedNormalRootExpansionV1::issue(
            &source,
            NormalRootExecutionProjectionPermitV1::issue_for_consumer(),
        ) {
            Ok(root_execution) => root_execution,
            Err(error) => {
                return Err(RejectedNormalRootExecutionConsumptionV1 {
                    source,
                    error: NormalRootExecutionConsumerRejectV1::StructuralProjection(error),
                })
            }
        };
        Ok(PreparedNormalRootExecutionConsumptionV1::issue(
            source,
            mode,
            root_execution,
        ))
    }
}
