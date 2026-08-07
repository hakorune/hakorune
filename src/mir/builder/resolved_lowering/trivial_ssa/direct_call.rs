//! Exact P0c direct-call materialization from one co-sealed profile row.

use crate::mir::canonical_direct_call::VerifiedCanonicalDirectCallEmissionV1;
use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
#[cfg(test)]
use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
#[cfg(test)]
use crate::mir::resolved_semantics::VerifiedCallableHeaderV1;
use crate::mir::resolved_value_profile::product::TrivialRepresentationV1;
use crate::mir::resolved_value_profile::VerifiedTrivialDirectCallV1;
use crate::mir::ValueId;

use super::super::super::MirBuilder;
use super::direct_call_type::PreparedResolvedDirectCallIntegerTypeV1;

pub(super) fn emit(
    builder: &mut MirBuilder,
    input: ResolvedFunctionLoweringInputV1<'_>,
    row: &VerifiedTrivialDirectCallV1,
    arguments: Vec<ValueId>,
) -> Result<(ValueId, TrivialRepresentationV1), String> {
    let current_header = input.callable_header().ok_or_else(|| {
        "[freeze:contract][canonical_direct_call/current_header_missing]".to_string()
    })?;
    if current_header.callable().owner() != input.owner() {
        return Err("[freeze:contract][canonical_direct_call/current_header_drift]".to_string());
    }
    let current_symbol = builder
        .function_state
        .current_function
        .as_ref()
        .map(|function| function.signature.name.as_str())
        .ok_or_else(|| "[freeze:contract][canonical_direct_call/function_missing]".to_string())?;
    if current_symbol != current_header.symbol().as_mir_name() {
        return Err(format!(
            "[freeze:contract][canonical_direct_call/symbol_drift] expected={} actual={current_symbol}",
            current_header.symbol().as_mir_name()
        ));
    }

    let capability_rows = &builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| "[freeze:contract][canonical_direct_call/function_missing]".to_string())?
        .metadata
        .canonical_direct_static_call_capabilities;
    CanonicalDirectStaticCallCapabilityV1::verify_for_emission(capability_rows)
        .map_err(str::to_string)?;

    let result = builder.next_value_id();
    let prepared = PreparedResolvedDirectCallIntegerTypeV1::prepare(
        row.result(),
        builder.function_state.type_ctx.get_type(result),
    )
    .map_err(|error| error.to_string())?;
    let instruction = VerifiedCanonicalDirectCallEmissionV1::from_verified_profile(row)
        .materialize(result, arguments)
        .map_err(|error| {
            format!("[freeze:contract][canonical_direct_call/materialization] {error:?}")
        })?;
    builder.emit_instruction(instruction)?;
    prepared.commit(result, &mut builder.function_state.type_ctx);

    Ok((result, row.result()))
}

/// Materialize a resolver-issued static call when the caller has an exact
/// header but does not belong to the whole Trivial profile.  This keeps the
/// canonical direct-call capability, emission, and type-fact owners shared by
/// profile routes without manufacturing a second profile row.
#[cfg(test)]
pub(in crate::mir::builder::resolved_lowering) fn emit_resolved_header(
    builder: &mut MirBuilder,
    input: ResolvedFunctionLoweringInputV1<'_>,
    target: &VerifiedCallableHeaderV1,
    result_abi: ExactTrivialReturnAbiV1,
    arguments: Vec<ValueId>,
) -> Result<(ValueId, TrivialRepresentationV1), String> {
    if result_abi != ExactTrivialReturnAbiV1::I64
        || ExactTrivialReturnAbiV1::classify(target.signature().result().source_type_name())
            != Some(result_abi)
    {
        return Err(
            "[freeze:contract][canonical_direct_call/resolved_header_result_abi]".to_string(),
        );
    }
    let current_header = input.callable_header().ok_or_else(|| {
        "[freeze:contract][canonical_direct_call/current_header_missing]".to_string()
    })?;
    if current_header.callable().owner() != input.owner() {
        return Err("[freeze:contract][canonical_direct_call/current_header_drift]".to_string());
    }
    let current_symbol = builder
        .function_state
        .current_function
        .as_ref()
        .map(|function| function.signature.name.as_str())
        .ok_or_else(|| "[freeze:contract][canonical_direct_call/function_missing]".to_string())?;
    if current_symbol != current_header.symbol().as_mir_name() {
        return Err(format!(
            "[freeze:contract][canonical_direct_call/symbol_drift] expected={} actual={current_symbol}",
            current_header.symbol().as_mir_name()
        ));
    }
    let capability_rows = &builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| "[freeze:contract][canonical_direct_call/function_missing]".to_string())?
        .metadata
        .canonical_direct_static_call_capabilities;
    CanonicalDirectStaticCallCapabilityV1::verify_for_emission(capability_rows)
        .map_err(str::to_string)?;

    let result = builder.next_value_id();
    let prepared = PreparedResolvedDirectCallIntegerTypeV1::prepare(
        TrivialRepresentationV1::InlineI64,
        builder.function_state.type_ctx.get_type(result),
    )
    .map_err(|error| error.to_string())?;
    let instruction = VerifiedCanonicalDirectCallEmissionV1::conservative_from_header(target)
        .materialize(result, arguments)
        .map_err(|error| {
            format!("[freeze:contract][canonical_direct_call/materialization] {error:?}")
        })?;
    builder.emit_instruction(instruction)?;
    prepared.commit(result, &mut builder.function_state.type_ctx);
    Ok((result, TrivialRepresentationV1::InlineI64))
}
