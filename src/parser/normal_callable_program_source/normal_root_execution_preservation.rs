//! Exact final-transform preservation for the total normal-root execution.
//!
//! The parser source surface and App/ProgramRuntime relation are never
//! reconstructed here. This owner only validates exact whole-Program
//! preservation and moves the already-issued aggregate into the final source.

use crate::ast::ASTNode;
use crate::parser::callable_parameter_source::{
    ParserNormalProgramSourceAuthorityDispositionV1, ParserNormalRootExecutionSourceDispositionV1,
    ParserNormalRootExecutionSourceV1, ParserNormalSourcePlanSurfaceV1,
};

#[derive(Debug)]
pub(crate) struct ParserNormalRootExecutionPreservedV1 {
    source: ParserNormalRootExecutionSourceV1,
    _seal: ParserNormalRootExecutionPreservedSealV1,
}

#[derive(Debug)]
struct ParserNormalRootExecutionPreservedSealV1;

#[derive(Debug)]
pub(crate) enum ParserNormalRootExecutionPreservationV1 {
    Ready(ParserNormalRootExecutionPreservedV1),
    Terminal(ParserNormalRootExecutionSourceDispositionV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParserNormalRootExecutionPreservationRejectV1 {
    CompatibilityLoss,
    SourceWitnessMissing,
    ParserWitnessMismatch,
    InitialProgramMissing,
    TransformedProgramMissing,
    SourceBodyCardinalityMismatch {
        source: usize,
        initial: usize,
        transformed: usize,
    },
    SourceSlotMismatch {
        position: usize,
    },
    SourceStatementChanged {
        position: usize,
    },
}

pub(crate) struct ParserNormalRootExecutionPreservationIssuerV1;

impl ParserNormalRootExecutionPreservationIssuerV1 {
    pub(crate) fn seal_after_transform(
        root: ParserNormalRootExecutionSourceDispositionV1,
        source_authority: &ParserNormalProgramSourceAuthorityDispositionV1,
        initial: &ASTNode,
        transformed: &ASTNode,
    ) -> Result<
        ParserNormalRootExecutionPreservationV1,
        ParserNormalRootExecutionPreservationRejectV1,
    > {
        let ParserNormalRootExecutionSourceDispositionV1::Ready(root) = root else {
            return Ok(ParserNormalRootExecutionPreservationV1::Terminal(root));
        };
        let Some(witness) = source_authority.invocation_witness() else {
            return reject_ready_root_at_named_transform_terminal(
                root,
                ParserNormalRootExecutionPreservationRejectV1::SourceWitnessMissing,
            );
        };
        if !root.bound().invocation().same_as(witness) {
            return reject_ready_root_at_named_transform_terminal(
                root,
                ParserNormalRootExecutionPreservationRejectV1::ParserWitnessMismatch,
            );
        }
        let ASTNode::Program {
            statements: initial_statements,
            ..
        } = initial
        else {
            return reject_ready_root_at_named_transform_terminal(
                root,
                ParserNormalRootExecutionPreservationRejectV1::InitialProgramMissing,
            );
        };
        let ASTNode::Program {
            statements: transformed_statements,
            ..
        } = transformed
        else {
            return reject_ready_root_at_named_transform_terminal(
                root,
                ParserNormalRootExecutionPreservationRejectV1::TransformedProgramMissing,
            );
        };
        let source_rows = match root.bound().surface() {
            ParserNormalSourcePlanSurfaceV1::CompleteEmpty => &[][..],
            ParserNormalSourcePlanSurfaceV1::CompleteRows(rows) => rows.rows(),
        };
        if source_rows.len() != initial_statements.len()
            || initial_statements.len() != transformed_statements.len()
        {
            let error =
                ParserNormalRootExecutionPreservationRejectV1::SourceBodyCardinalityMismatch {
                    source: source_rows.len(),
                    initial: initial_statements.len(),
                    transformed: transformed_statements.len(),
                };
            return reject_ready_root_at_named_transform_terminal(root, error);
        }
        if let Some(position) = source_rows.iter().enumerate().find_map(|(position, row)| {
            (usize::try_from(row.slot().final_statement_slot()).ok() != Some(position))
                .then_some(position)
        }) {
            return reject_ready_root_at_named_transform_terminal(
                root,
                ParserNormalRootExecutionPreservationRejectV1::SourceSlotMismatch { position },
            );
        }
        if let Some(position) = initial_statements
            .iter()
            .zip(transformed_statements)
            .position(|(source, transformed)| source != transformed)
        {
            return reject_ready_root_at_named_transform_terminal(
                root,
                ParserNormalRootExecutionPreservationRejectV1::SourceStatementChanged { position },
            );
        }
        Ok(ParserNormalRootExecutionPreservationV1::Ready(
            ParserNormalRootExecutionPreservedV1 {
                source: root,
                _seal: ParserNormalRootExecutionPreservedSealV1,
            },
        ))
    }

    pub(in crate::parser) fn discard_at_named_transform_reject_terminal(
        root: ParserNormalRootExecutionSourceDispositionV1,
    ) {
        match root {
            ParserNormalRootExecutionSourceDispositionV1::Ready(source) => drop(source),
            ParserNormalRootExecutionSourceDispositionV1::SourceAuthorityUnavailable(error) => {
                let _consumed_error = error;
            }
            ParserNormalRootExecutionSourceDispositionV1::Incomplete(error) => {
                let _consumed_error = error;
            }
            ParserNormalRootExecutionSourceDispositionV1::IntegrityInvalid(error) => {
                let _consumed_error = error;
            }
        }
    }
}

fn reject_ready_root_at_named_transform_terminal(
    root: ParserNormalRootExecutionSourceV1,
    error: ParserNormalRootExecutionPreservationRejectV1,
) -> Result<ParserNormalRootExecutionPreservationV1, ParserNormalRootExecutionPreservationRejectV1>
{
    ParserNormalRootExecutionPreservationIssuerV1::discard_at_named_transform_reject_terminal(
        ParserNormalRootExecutionSourceDispositionV1::Ready(root),
    );
    Err(error)
}

impl ParserNormalRootExecutionPreservedV1 {
    pub(crate) fn source(&self) -> &ParserNormalRootExecutionSourceV1 {
        &self.source
    }
}

impl ParserNormalRootExecutionPreservationV1 {
    pub(crate) fn ready_source(&self) -> Option<&ParserNormalRootExecutionSourceV1> {
        match self {
            Self::Ready(preserved) => Some(preserved.source()),
            Self::Terminal(_) => None,
        }
    }

    pub(crate) fn discard_at_named_terminal(self) {
        match self {
            Self::Ready(preserved) => {
                let ParserNormalRootExecutionPreservedV1 { source, _seal } = preserved;
                drop((source, _seal));
            }
            Self::Terminal(root) => {
                ParserNormalRootExecutionPreservationIssuerV1::
                    discard_at_named_transform_reject_terminal(root);
            }
        }
    }
}
