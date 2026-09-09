//! Small source-side helpers for ordinary-`New` co-sealing.
//!
//! These helpers only translate already-issued observations or validate the
//! direct initializer shape. They do not issue keys, select a target, or
//! recover missing semantic facts.

use super::{
    OrdinaryNewCoSealIssueV1, OrdinaryNewConstructorDispositionV1,
    OrdinaryNewTrivialArgumentKindV1, OrdinaryNewTrivialArgumentV1,
};
use crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1;
use crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    SelectedNewArgumentKindV1, SelectedNewArgumentObservationV1,
};
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourcePathSegmentV1};

pub(super) fn convert_selected_new_arguments(
    observation: SelectedNewArgumentObservationV1,
) -> Result<Box<[OrdinaryNewTrivialArgumentV1]>, SelectedNewArgumentUnavailableV1> {
    observation
        .arguments()
        .map(|rows| {
            rows.iter()
                .map(|row| {
                    let kind = match row.kind() {
                        SelectedNewArgumentKindV1::Integer(value) => {
                            OrdinaryNewTrivialArgumentKindV1::Integer(*value)
                        }
                        SelectedNewArgumentKindV1::Bool(value) => {
                            OrdinaryNewTrivialArgumentKindV1::Bool(*value)
                        }
                        SelectedNewArgumentKindV1::Local { binding } => {
                            OrdinaryNewTrivialArgumentKindV1::Local { binding: *binding }
                        }
                    };
                    OrdinaryNewTrivialArgumentV1::new(
                        observation.new_site().owner(),
                        observation.new_site().clone(),
                        row.ordinal(),
                        row.site().clone(),
                        kind,
                    )
                })
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
        .map_err(Clone::clone)
}

pub(super) fn is_direct_local_initializer(segments: &[SourcePathSegmentV1]) -> bool {
    matches!(
        segments,
        [
            SourcePathSegmentV1::Body(_),
            SourcePathSegmentV1::Initializer(_)
        ]
    )
}

pub(super) fn no_birth_constructor_disposition(
    site: &OwnedExprSiteV1,
    class: &str,
    arity: usize,
) -> Result<OrdinaryNewConstructorDispositionV1, OrdinaryNewCoSealIssueV1> {
    if arity == 0 {
        return Ok(OrdinaryNewConstructorDispositionV1::NoBirthZero);
    }
    Err(OrdinaryNewCoSealIssueV1::BirthConstructorMissing {
        site: site.clone(),
        class: class.into(),
        arity,
    })
}

pub(super) fn retain_child_terminal_relation(row: &TerminalRelationV1, has_map: bool) -> bool {
    has_map
        || matches!(
            row,
            TerminalRelationV1::IntegerLiteral(_) | TerminalRelationV1::I64Field(_)
        )
}
