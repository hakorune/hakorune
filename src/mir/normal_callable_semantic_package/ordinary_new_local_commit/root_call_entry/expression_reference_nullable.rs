//! Original nullable producer dispatch; neither source lends a payload view.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::NullableResultReferenceV1;

pub(super) fn nullable_result_read_v1(
    source: &NullableResultReferenceV1, read: &ExactLexicalReadV1,
    context: CallValidationContextV1<'_>, function: &MirFunction, finishing: Option<&Finished>,
) -> Result<(ValueId, [Binding; 2]), String> {
    let loan = match source {
        NullableResultReferenceV1::StoredMapGet(frame) =>
            return context.stored_map_get_read_v1(frame, read, function, finishing),
        NullableResultReferenceV1::DeclaredFrom(loan) => loan,
    };
    let original = context.ledger().borrowed_nullable_call_source_v1(loan.call())
        .ok_or_else(|| freeze("expression/reference-source-node"))?;
    if original.borrowed_nullable() != Some(loan.as_ref())
        || original.local_binding().map(|(_, binding)| binding) != Some(loan.destination()) {
        return Err(freeze("expression/reference-source-node"));
    }
    let (initializer, _) = read.loan_local_initializer_v1(loan.destination())?;
    context.with_taken_packet_v1(loan.call(), |packet| {
        let row = packet.original_row()?;
        if row.source_target().borrowed_nullable_local_source() != Some(loan.as_ref())
            || context.ledger().corroborate_borrowed_nullable_call_v1(row)? != loan.result_kind_v1()
            || packet.value_in_context_v1(original.owner(), original.arguments(), context)? != initializer {
            return Err(freeze("expression/reference-source-node"));
        }
        let (invoke, normal) = packet.outer_bindings()?;
        if let Some(finishing) = finishing {
            finishing.relation_binding(function, invoke)?;
            finishing.relation_binding(function, normal)?;
        }
        Ok((read.source_value().2, [invoke.clone(), normal.clone()]))
    })
}
