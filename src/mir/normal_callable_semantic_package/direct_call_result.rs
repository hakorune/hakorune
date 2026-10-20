//! Typed source result classification; execution remains with its existing lender.
use super::*;

/// The callee's own terminal relation is the sole result-class evidence:
/// a Map-source `return` yields a `Map` call result, a `return new`
/// construction yields a `Handle` call result (the caller receives the
/// transferred object as an owned Home), and every other admitted
/// scalar relation keeps its exact I64/Bool kind. OpaqueCall proves no result class —
/// the callee cannot publish an Invoke result kind until its return value
/// is decomposed. Declared annotations never decide this.
pub(in crate::mir::normal_callable_semantic_package) fn call_result_kind(
    terminal_relation: Option<&TerminalRelationV1>,
) -> Option<InvokeCallResultKind> {
    match terminal_relation {
        Some(TerminalRelationV1::Call(row)) if row.source_scalar().is_some() => {
            match row.scalar_kind() {
                crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer => {
                    Some(InvokeCallResultKind::I64)
                }
                crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool => Some(InvokeCallResultKind::Bool),
            }
        }
        Some(TerminalRelationV1::BoolLiteral(_)) => Some(InvokeCallResultKind::Bool),
        Some(TerminalRelationV1::OpaqueCall(_)) => None,
        Some(TerminalRelationV1::Value(row)) => match row.returned() {
            // A Text source relation does not grant the ordinary call's
            // owned-pair ABI. Keep it closed until that transfer is admitted.
            TerminalReturnedSourceV1::TextResidence { .. } | TerminalReturnedSourceV1::TextExpression(_) => None,
            TerminalReturnedSourceV1::BorrowedNullableLocal(_) => None,
            TerminalReturnedSourceV1::Construction(_) => Some(InvokeCallResultKind::Handle),
            TerminalReturnedSourceV1::MapLiteral(_) | TerminalReturnedSourceV1::MapLocal(_) => {
                Some(InvokeCallResultKind::Map)
            }
            _ => Some(InvokeCallResultKind::I64),
        },
        _ => Some(InvokeCallResultKind::I64),
    }
}

/// A callee with several exits publishes one result kind only when every
/// retained exit relation classifies to the same kind — the caller cannot
/// observe which `return` executed, so a mixed-exit callee has no single
/// kind. An empty relation set keeps the prior `I64` answer.
pub(in crate::mir::normal_callable_semantic_package) fn uniform_call_result_kind<'a>(
    relations: impl IntoIterator<Item = &'a TerminalRelationV1>,
) -> Option<InvokeCallResultKind> {
    let mut kind = None;
    for relation in relations {
        let row = call_result_kind(Some(relation))?;
        match kind {
            None => kind = Some(row),
            Some(existing) if existing == row => {}
            Some(_) => return None,
        }
    }
    // An empty relation set keeps the prior `I64` answer.
    kind.or_else(|| call_result_kind(None))
}
