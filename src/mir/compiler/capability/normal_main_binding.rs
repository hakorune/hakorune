//! Binding bridge for one already-sealed normal Main lowering proof.

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::lowering_input::CanonicalLoweringErrorV1;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_value_profile::product::VerifiedTrivialCanonicalOwnerV1;

use super::CanonicalTrivialBindingSsaPlanV1;

/// Binds sealed facts to one fresh exact input without source reclassification.
pub(in crate::mir) fn bind_sealed_normal_main_parts_v1<'a>(
    function: ResolvedFunctionLoweringInputV1<'a>,
    if_control: VerifiedResolvedFunctionIfControlV1,
    completion: VerifiedFunctionCompletionV1,
    profile: VerifiedTrivialCanonicalOwnerV1,
    block_expr_count: usize,
) -> Result<CanonicalTrivialBindingSsaPlanV1<'a>, CanonicalLoweringErrorV1> {
    let owner = function.owner();
    if if_control.owner() != owner
        || completion.owner() != owner
        || profile.owner() != owner
        || function.function().owner() != owner
        || function.source().owner() != owner
    {
        return Err(CanonicalLoweringErrorV1::SourceUnitResolution {
            detail: "normal_main_sealed_fact_owner_mismatch".to_owned(),
        });
    }
    Ok(CanonicalTrivialBindingSsaPlanV1 {
        function,
        if_control,
        completion,
        profile,
        block_expr_count,
    })
}
