//! Exact Box-row retention for parser postpass envelopes.

use crate::ast::ASTNode;
use crate::parser::source_seal::ParserBoxSourceSealV1;

use super::{ParserBoxPostpassRowV1, ParserCompatibilityCohortV1, ParserPostpassEnvelopeErrorV1};

pub(super) fn compatibility_rows(
    ast: &ASTNode,
    cohort: ParserCompatibilityCohortV1,
) -> Box<[ParserBoxPostpassRowV1]> {
    let ASTNode::Program { statements, .. } = ast else {
        return Box::new([]);
    };
    statements
        .iter()
        .enumerate()
        .filter_map(|(final_box_ordinal, statement)| {
            matches!(statement, ASTNode::BoxDeclaration { .. }).then_some(
                ParserBoxPostpassRowV1::AstOnlyCompatibility {
                    _final_box_ordinal: final_box_ordinal,
                    cohort,
                },
            )
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

/// Keep already-finalized ordinary seals in a mixed/static semantic Program.
/// Static and other compatibility-only declarations remain explicit AST-only
/// rows; this owner neither recreates an ordinary seal nor classifies names.
pub(super) fn source_backed_compatibility_rows(
    ast: &ASTNode,
    cohort: ParserCompatibilityCohortV1,
    source_seals: Box<[ParserBoxSourceSealV1]>,
    final_box_ordinals: Box<[usize]>,
) -> Result<Box<[ParserBoxPostpassRowV1]>, ParserPostpassEnvelopeErrorV1> {
    if source_seals.len() != final_box_ordinals.len() {
        return Err(ParserPostpassEnvelopeErrorV1::SourceCoverageMismatch {
            seals: source_seals.len(),
            final_box_ordinals: final_box_ordinals.len(),
        });
    }
    let ASTNode::Program { statements, .. } = ast else {
        return Err(ParserPostpassEnvelopeErrorV1::SourceOrdinalMismatch);
    };
    let mut sealed_by_statement = (0..statements.len())
        .map(|_| None)
        .collect::<Vec<Option<ParserBoxSourceSealV1>>>();
    for (seal, ordinal) in source_seals.into_vec().into_iter().zip(final_box_ordinals) {
        let Some(statement) = statements.get(ordinal) else {
            return Err(ParserPostpassEnvelopeErrorV1::SourceOrdinalMismatch);
        };
        if !matches!(
            statement,
            ASTNode::BoxDeclaration {
                is_interface: false,
                is_record: false,
                is_static: false,
                ..
            }
        ) {
            return Err(ParserPostpassEnvelopeErrorV1::SourceOrdinalMismatch);
        }
        let slot = &mut sealed_by_statement[ordinal];
        if slot.replace(seal).is_some() {
            return Err(ParserPostpassEnvelopeErrorV1::SourceOrdinalMismatch);
        }
    }

    let rows = statements
        .iter()
        .enumerate()
        .filter_map(|(final_box_ordinal, statement)| {
            matches!(statement, ASTNode::BoxDeclaration { .. }).then(|| {
                sealed_by_statement[final_box_ordinal]
                    .take()
                    .map(|seal| ParserBoxPostpassRowV1::SourceSealedOrdinary {
                        final_box_ordinal,
                        seal,
                    })
                    .unwrap_or(ParserBoxPostpassRowV1::AstOnlyCompatibility {
                        _final_box_ordinal: final_box_ordinal,
                        cohort,
                    })
            })
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    if sealed_by_statement.into_iter().any(|seal| seal.is_some()) {
        return Err(ParserPostpassEnvelopeErrorV1::SourceOrdinalMismatch);
    }
    Ok(rows)
}
