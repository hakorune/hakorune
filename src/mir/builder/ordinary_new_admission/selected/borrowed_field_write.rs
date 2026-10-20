//! Original field assignment through the sole checked writer, with exact operands.
use super::*;
use crate::mir::normal_callable_semantic_package::{FieldWriteSourceLoanV1, EmittedBorrowedFieldWriteV1,
    FieldWriteValueV1};
use std::rc::Rc;

pub(in crate::mir::builder) fn emit(builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, source: FieldWriteSourceLoanV1,
    field_name: &str, reads: [FieldWriteValueV1; 2],
) -> Result<(), String> {
    source.require_current_execution(crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger))?;
    let owner = source.frame().source().owner();
    if state.owner() != owner { return Err(freeze("borrowed-field-write/source-site")); }
    let [FieldWriteValueV1::Lexical(receiver), value] = reads else { return Err(freeze("field-write/receiver-proof")) };
    let (binding, site) = source.receiver()?;
    let base = receiver.value_for(owner, site.site().node(), binding).map_err(|issue| issue.to_string())?;
    if source.field_name()? != field_name { return Err(freeze("borrowed-field-write/source-site")); }
    let field = source.field()?;
    let (value_id, prefix, relations) = match (&value, source.borrowed()) {
        (FieldWriteValueV1::Lexical(read), Some(formal)) => {
            let ty = crate::mir::numeric_substrate::exact_numeric_mir_type_from_declared_name(
                formal.field_source().declared_type_name(), crate::mir::numeric_substrate::NumericTarget::host())
                .ok_or_else(|| freeze("borrowed-field-write/numeric-declaration"))?;
            if crate::mir::numeric_substrate::exact_numeric_type_requires_dynamic_integer_range_check(&ty)
                && formal.field_source().declared_type_name() != Some("usize")
            { return Err(freeze("borrowed-field-write/exact-range-check-unavailable")); }
            let (binding, _, site) = formal.value();
            (read.value_for(owner, site.site().node(), binding).map_err(|issue| issue.to_string())?,
                Vec::new(), Vec::new())
        },
        (FieldWriteValueV1::Expression(rhs), None) => {
            if source.qualified_rhs().is_none_or(|original| !Rc::ptr_eq(original, rhs.recipe())) {
                return Err(freeze("field-write-rhs/source-drift"));
            }
            let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
            rhs.validate_lowering(ledger, function)?;
            (rhs.value(), rhs.bindings()?, rhs.relations())
        },
        _ => return Err(freeze("field-write/operand-origin")),
    };
    let operations = ledger.prior_home_unwind_for(source.frame().prior_homes())?;
    let graph = super::checked_field_write::emit(builder, state, field, base, value_id, operations)?
        .with_operand_projection(prefix, relations);
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let packet = Rc::new(EmittedBorrowedFieldWriteV1::new(source, receiver, value, graph, ledger, function)?);
    ledger.record_borrowed_field_write_bindings_v1(packet)
}
