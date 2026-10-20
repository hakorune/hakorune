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
use crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    SelectedNewArgumentKindV1, SelectedNewArgumentObservationV1,
};
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourcePathSegmentV1};

pub(super) fn convert_selected_new_arguments(
    observation: SelectedNewArgumentObservationV1,
) -> Result<Box<[OrdinaryNewTrivialArgumentV1]>, SelectedNewArgumentUnavailableV1> {
    let rows = observation.arguments().map_err(Clone::clone)?;
    rows.iter().map(|row| {
        let kind = match row.kind() {
            SelectedNewArgumentKindV1::Integer(value) => OrdinaryNewTrivialArgumentKindV1::Integer(*value),
            SelectedNewArgumentKindV1::Bool(value) => OrdinaryNewTrivialArgumentKindV1::Bool(*value),
            SelectedNewArgumentKindV1::Null => OrdinaryNewTrivialArgumentKindV1::Null,
            SelectedNewArgumentKindV1::Local { binding } => OrdinaryNewTrivialArgumentKindV1::Local { binding: *binding },
            SelectedNewArgumentKindV1::Handle { binding } => OrdinaryNewTrivialArgumentKindV1::Handle { binding: *binding },
            SelectedNewArgumentKindV1::BoundValue { binding } => OrdinaryNewTrivialArgumentKindV1::BoundValue { binding: *binding },
            SelectedNewArgumentKindV1::I64Field { object } => OrdinaryNewTrivialArgumentKindV1::I64Field { object: *object },
            SelectedNewArgumentKindV1::Expression { evaluation } => {
                if evaluation.source().owner() != observation.new_site().owner()
                    || !evaluation.reference_coverage_is_exact()
                    || !matches!(evaluation.source().root(),
                        crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::NewArgument { construction, ordinal, .. }
                            if construction.site() == observation.new_site().site() && *ordinal == row.ordinal()
                                && construction.arguments().get(*ordinal as usize) == Some(row.site()))
                    || evaluation.source().nodes().first().is_none_or(|node| node.site() != row.site()) {
                    return Err(SelectedNewArgumentUnavailableV1::SourceMismatch { new_site: observation.new_site().clone() });
                }
                OrdinaryNewTrivialArgumentKindV1::Expression { evaluation: std::rc::Rc::clone(evaluation) }
            }
        };
        Ok(OrdinaryNewTrivialArgumentV1::new(observation.new_site().owner(),
            observation.new_site().clone(), row.ordinal(), row.site().clone(), kind))
    }).collect()

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

/// Retain the original source map, including canonical Static continuations
/// supplied by the same issuer. A child cannot borrow another owner's exit.
pub(super) fn retain_child_terminal_relations_v1(
    rows: std::collections::BTreeMap<crate::mir::resolved_semantics::SourceStmtSiteV1, TerminalRelationV1>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    has_map: bool,
    mut retain_source: impl FnMut(&TerminalRelationV1) -> Result<bool, OrdinaryNewCoSealIssueV1>,
) -> Result<std::collections::BTreeMap<crate::mir::resolved_semantics::SourceStmtSiteV1, TerminalRelationV1>, OrdinaryNewCoSealIssueV1> {
    let mut retained = std::collections::BTreeMap::new();
    for (site, row) in rows {
        if row.owner() != owner || row.return_site() != &site {
            return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
        }
        if retain_child_terminal_relation(&row, has_map) || retain_source(&row)? {
            retained.insert(site, row);
        }
    }
    Ok(retained)
}

pub(super) fn retain_child_terminal_relation(row: &TerminalRelationV1, has_map: bool) -> bool {
    has_map
        // The immutable Lexical argument arm is issued only after the source
        // terminal owner proves BorrowedActual; child completion must carry it
        // to the same final terminal emitter instead of dropping that loan.
        || matches!(row, TerminalRelationV1::Call(call)
            if call.arguments().iter().any(|argument| matches!(argument,
                crate::mir::resolved_semantics::home_new_prefix::TerminalCallArgumentV1::Lexical(_))))
        || matches!(
            row,
            TerminalRelationV1::Unit(_)
                | TerminalRelationV1::IntegerLiteral(_)
                | TerminalRelationV1::BoolLiteral(_)
                | TerminalRelationV1::I64Field(_)
                | TerminalRelationV1::I64Add(_)
                | TerminalRelationV1::I64Scalar(_)
                | TerminalRelationV1::MapGet(_)
        )
        // A `return new ...` relation is the co-sealed evidence that the
        // exact site owns a result claim, so it stays on the child contract
        // even without a map obligation. A `return null` relation is
        // retained explicitly for the same reason: dropping it would let a
        // mixed `new`/`null` callee masquerade as a uniform-object callee
        // downstream.
        || matches!(
            row,
            TerminalRelationV1::Value(value)
                if matches!(
                    value.returned(),
                    TerminalReturnedSourceV1::Construction(_)
                        | TerminalReturnedSourceV1::OwnedCall(_)
                        | TerminalReturnedSourceV1::BorrowedNullableLocal(_)
                        | TerminalReturnedSourceV1::StoredMapGet { .. }
                        | TerminalReturnedSourceV1::StoredMapInstalled(_)
                        | TerminalReturnedSourceV1::NullLiteral
                        | TerminalReturnedSourceV1::TextResidence { .. }
                        | TerminalReturnedSourceV1::TextExpression(_)
                )
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn child_terminal_filter_retains_borrowed_call_without_widening_strict_call() {
        for borrowed in [false, true] {
            let parameter = if borrowed { "p" } else { "" };
            let argument = if borrowed { "7" } else { "" };
            let source = format!("box Transport {{ birth() {{ }} probe({parameter}): i64 {{ return 7 }} }}
                static box Main {{ main() {{ local recv = new Transport() return recv.probe({argument}) }} }}");
            let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&source).unwrap();
            let ledger = &package.ordinary_new_claim_ledger;
            let row = ledger
                .sole_terminal_relation_for_owner(ledger.root_owner().unwrap())
                .unwrap();
            assert!(matches!(row, TerminalRelationV1::Call(_)), "{row:?}");
            assert_eq!(retain_child_terminal_relation(row, false), borrowed);
            assert!(retain_child_terminal_relation(row, true));
        }
    }
}
