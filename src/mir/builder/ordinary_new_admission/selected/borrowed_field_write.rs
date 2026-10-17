//! One guarded original field assignment through the shared checked writer.
use super::*;
use crate::mir::normal_callable_semantic_package::{FieldWriteSourceLoanV1, EmittedBorrowedFieldWriteV1};
use std::rc::Rc;


pub(in crate::mir::builder) fn emit(builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, source: FieldWriteSourceLoanV1,
    field_name: &str, reads: [Rc<crate::mir::builder::ExactLexicalReadV1>; 2],
) -> Result<(), String> {
    source.validate_source(ledger)?;
    let formal = source.borrowed()
        .ok_or_else(|| freeze("owned-field-write/rhs-physical-consumer-pending"))?;
    let owner = source.frame().source().owner();
    if state.owner() != owner || formal.field_name() != field_name { return Err(freeze("borrowed-field-write/source-site")); }
    let ty = crate::mir::numeric_substrate::exact_numeric_mir_type_from_declared_name(
        formal.field_source().declared_type_name(), crate::mir::numeric_substrate::NumericTarget::host())
        .ok_or_else(|| freeze("borrowed-field-write/numeric-declaration"))?;
    if crate::mir::numeric_substrate::exact_numeric_type_requires_dynamic_integer_range_check(&ty)
        && formal.field_source().declared_type_name() != Some("usize")
    { return Err(freeze("borrowed-field-write/exact-range-check-unavailable")); }
    // usize retains its existing canonical numeric obligation. Final source
    // use closure and backend/ABI checks must all accept this exact Invoke.
    let operations = ledger.prior_home_unwind_for(source.frame().prior_homes())?;
    let values = [formal.receiver(), formal.value()].into_iter().enumerate().map(|(index, (binding, _, site))|
        reads[index].value_for(owner, site.site().node(), binding).map_err(|issue| issue.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    let (receiver, value) = (values[0], values[1]);
    let graph = super::checked_field_write::emit(builder, state, formal.field_source().field(), receiver, value, operations)?;
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let packet = Rc::new(EmittedBorrowedFieldWriteV1::new(source, reads, graph, ledger, function)?);
    ledger.record_borrowed_field_write_bindings_v1(packet)
}
