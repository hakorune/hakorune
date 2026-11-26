//! Canonical route probing for the normal callable adapter.
//!
//! This module only selects an already-issued source-bound product. It does
//! not issue source facts, repair targets, or own physical Builder state.

use crate::mir::compiler::capability::CanonicalGenericG0PlanV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::generic_g0_capability::probe_generic_g0_function_v1;
use crate::mir::compiler::CanonicalLoweringErrorV1;
use crate::mir::loop_route_policy::GenericG0PolicyModeV1;

pub(in crate::mir::builder) fn try_classify_generic_g0_route_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    mode: Option<GenericG0PolicyModeV1>,
) -> Result<Option<CanonicalGenericG0PlanV1<'_>>, String> {
    probe_generic_g0_function_v1(input, mode)
        .map_err(|error: CanonicalLoweringErrorV1| {
            format!("[freeze:contract][mir/callable-generic-g0-preflight] {error:?}")
        })
}
