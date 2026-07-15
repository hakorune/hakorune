//! Callable ABI facade for the trivial Binding SSA route.
//!
//! The route consumes sealed callable rows here and passes completed metadata
//! to the generic builder. Raw source annotation classification and function
//! admission do not belong in this box.

use crate::mir::builder::MirBuilder;
use crate::mir::resolved_value_profile::product::VerifiedTrivialParameterEntryV1;
use crate::mir::MirFunction;

/// Install the currently sealed callable ABI before body effects.
///
/// R0a-L0 remains behavior-neutral: the closed profile has parameter rows but
/// no typed-return witness yet, so the declared result is intentionally None.
/// R0a-S0 will extend this facade with the sealed return witness rather than
/// restoring a raw AST read in resolved lowering.
pub(in crate::mir::builder::resolved_lowering) fn install_trivial_callable_abi_v1(
    builder: &mut MirBuilder,
    parameter_entries: &[VerifiedTrivialParameterEntryV1],
) {
    let declared_parameters = parameter_entries
        .iter()
        .map(|row| row.abi().mir_param_decl(row.source_name()))
        .collect();
    builder.set_current_function_declared_signature(declared_parameters, None);
}

/// Refresh callable-boundary carriers in their canonical order on the
/// unpublished function draft.
pub(in crate::mir::builder::resolved_lowering) fn refresh_trivial_callable_boundary_contracts_v1(
    function: &mut MirFunction,
) {
    crate::mir::type_contracts::parameter_entry::refresh_function_parameter_entry_contracts(
        function,
    );
    crate::mir::type_contracts::return_exit::refresh_function_return_exit_contract(function);
}
