//! Parser-owned final-transform preservation for the normal root role.
//!
//! The initial App/Script admission is issued by `normal_root_source.rs`.
//! This module only validates that the same source-root prefix survives the
//! final callable transform and seals an opaque, non-Clone handoff token.

use crate::ast::ASTNode;

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
    role: ParserNormalRootRoleV1,
    _invocation: ParserInvocationWitnessV1,
    _seal: ParserNormalRootPreservedSealV1,
}

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
    SourcePrefixLengthMismatch,
    SourcePrefixChanged,
    RootRoleDrift,
    ParserWitnessMismatch,
}

pub(crate) struct ParserNormalRootPreservationIssuerV1;

impl ParserNormalRootPreservationIssuerV1 {
    pub(crate) fn seal_after_transform(
        root: ParserNormalRootSourceDispositionV1,
        source_authority: &ParserNormalProgramSourceAuthorityDispositionV1,
        initial: &ASTNode,
        transformed: &ASTNode,
    ) -> Result<ParserNormalRootPreservationV1, ParserNormalRootPreservationRejectV1> {
        let role = match &root {
            ParserNormalRootSourceDispositionV1::AppReady(seal) => {
                let Some(witness) = source_authority.invocation_witness() else {
                    return Err(ParserNormalRootPreservationRejectV1::SourceWitnessMissing);
                };
                if !seal.same_parser_source(witness) {
                    return Err(ParserNormalRootPreservationRejectV1::ParserWitnessMismatch);
                }
                ParserNormalRootRoleV1::App
            }
            ParserNormalRootSourceDispositionV1::ScriptReady(admission) => {
                let Some(witness) = source_authority.invocation_witness() else {
                    return Err(ParserNormalRootPreservationRejectV1::SourceWitnessMissing);
                };
                if !admission.same_parser_source_witness(witness) {
                    return Err(ParserNormalRootPreservationRejectV1::ParserWitnessMismatch);
                }
                ParserNormalRootRoleV1::Script
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

        let expected_prefix_len = source_authority
            .invocation_witness()
            .map(|_| initial_statements.len())
            .ok_or(ParserNormalRootPreservationRejectV1::SourceWitnessMissing)?;
        let source_body_count = match source_authority {
            ParserNormalProgramSourceAuthorityDispositionV1::Ready(authority) => {
                authority.body_rows().len()
            }
            ParserNormalProgramSourceAuthorityDispositionV1::SourceAuthorityUnavailable(_)
            | ParserNormalProgramSourceAuthorityDispositionV1::Incomplete(_)
            | ParserNormalProgramSourceAuthorityDispositionV1::IntegrityInvalid(_) => 0,
        };
        if source_body_count != expected_prefix_len
            || transformed_statements.len() < expected_prefix_len
        {
            return Err(ParserNormalRootPreservationRejectV1::SourcePrefixLengthMismatch);
        }
        if transformed_statements[..expected_prefix_len] != initial_statements[..] {
            return Err(ParserNormalRootPreservationRejectV1::SourcePrefixChanged);
        }
        if transformed_statements[expected_prefix_len..]
            .iter()
            .any(is_static_main_box)
        {
            return Err(ParserNormalRootPreservationRejectV1::RootRoleDrift);
        }

        let witness = source_authority
            .invocation_witness()
            .ok_or(ParserNormalRootPreservationRejectV1::SourceWitnessMissing)?
            .clone();
        Ok(ParserNormalRootPreservationV1::Ready(
            ParserNormalRootPreservedV1 {
                role,
                _invocation: witness,
                _seal: ParserNormalRootPreservedSealV1,
            },
        ))
    }
}

impl ParserNormalRootPreservedV1 {
    pub(crate) const fn role(&self) -> ParserNormalRootRoleV1 {
        self.role
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

fn is_static_main_box(statement: &ASTNode) -> bool {
    matches!(
        statement,
        ASTNode::BoxDeclaration {
            name,
            is_static: true,
            ..
        } if name == "Main"
    )
}
