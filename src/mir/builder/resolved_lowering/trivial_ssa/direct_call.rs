//! Exact P0c direct-call materialization from one co-sealed profile row.

use crate::mir::canonical_direct_call::VerifiedCanonicalDirectCallEmissionV1;
use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_value_profile::product::TrivialRepresentationV1;
use crate::mir::resolved_value_profile::VerifiedTrivialDirectCallV1;
use crate::mir::ValueId;

use super::super::super::MirBuilder;
use super::operation::mir_type;

pub(super) fn emit(
    builder: &mut MirBuilder,
    input: ResolvedFunctionLoweringInputV1<'_>,
    row: &VerifiedTrivialDirectCallV1,
    arguments: Vec<ValueId>,
) -> Result<(ValueId, TrivialRepresentationV1), String> {
    if row.target().callable().owner() != input.owner() {
        return Err("[freeze:contract][canonical_direct_call/target_owner_mismatch]".to_string());
    }
    let current_symbol = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|function| function.signature.name.as_str())
        .ok_or_else(|| "[freeze:contract][canonical_direct_call/function_missing]".to_string())?;
    if current_symbol != row.target().symbol().as_mir_name() {
        return Err(format!(
            "[freeze:contract][canonical_direct_call/symbol_drift] expected={} actual={current_symbol}",
            row.target().symbol().as_mir_name()
        ));
    }

    let result = builder.next_value_id();
    let instruction = VerifiedCanonicalDirectCallEmissionV1::from_verified_profile(row)
        .materialize(result, arguments)
        .map_err(|error| {
            format!("[freeze:contract][canonical_direct_call/materialization] {error:?}")
        })?;
    builder.emit_instruction(instruction)?;
    builder
        .type_ctx
        .value_types
        .insert(result, mir_type(row.result()));

    let function =
        builder.scope_ctx.current_function.as_mut().ok_or_else(|| {
            "[freeze:contract][canonical_direct_call/function_missing]".to_string()
        })?;
    if !function
        .metadata
        .canonical_direct_static_call_capabilities
        .is_empty()
    {
        return Err("[freeze:contract][canonical_direct_call/capability_duplicate]".to_string());
    }
    function
        .metadata
        .canonical_direct_static_call_capabilities
        .push(CanonicalDirectStaticCallCapabilityV1::v1());
    Ok((result, row.result()))
}
