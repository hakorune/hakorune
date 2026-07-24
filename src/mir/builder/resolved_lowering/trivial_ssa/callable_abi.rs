//! Callable ABI facade for the trivial Binding SSA route.
//!
//! The route consumes sealed callable rows here and passes completed metadata
//! to the generic builder. Raw source annotation classification and function
//! admission do not belong in this box.

use crate::mir::builder::MirBuilder;
use crate::mir::resolved_value_profile::product::VerifiedTrivialCanonicalOwnerV1;

/// Install the currently sealed callable ABI before body effects.
///
pub(in crate::mir::builder::resolved_lowering) fn install_trivial_callable_abi_v1(
    builder: &mut MirBuilder,
    profile: &VerifiedTrivialCanonicalOwnerV1,
) {
    let declared_parameters = profile
        .parameter_entries()
        .iter()
        .map(|row| row.abi().mir_param_decl(row.source_name()))
        .collect();
    let declared_result = profile
        .function_return()
        .map(|row| row.abi().source_type_name().to_string());
    builder.set_current_function_declared_signature(declared_parameters, declared_result);
}
