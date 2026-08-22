//! Private source-anchor validation for the fixed S6C prephysical ingress.
//!
//! The parent ingress remains the sole semantic owner. This child only
//! validates source-backed anchors before the parent seals an operation row;
//! it never issues Facts, Recipe keys, physical IDs, or a second ingress.

use super::{S6CPrephysicalIngressRejectV2, S6CPrephysicalOperationRoleV2};

pub(super) fn verify_anchor_for_role(
    role: S6CPrephysicalOperationRoleV2,
    typed: &crate::mir::callable_semantic_batch::VerifiedS6CTypedInputRelationV1,
    calls: super::super::s6c_scan_with_init_joinir_output::S6CLogicalCallPairsRefV1<'_>,
    source: crate::mir::loop_structural_facts::S6CExitTailSourceCoSealRefV1<'_>,
) -> Result<(), S6CPrephysicalIngressRejectV2> {
    let _anchor = match role {
        S6CPrephysicalOperationRoleV2::ConditionIndexRead => binary_source(
            typed,
            crate::mir::callable_semantic_batch::S6CBinaryRoleV1::LoopConditionLess,
            "less",
        )?
        .lhs(),
        S6CPrephysicalOperationRoleV2::LengthCall => calls.length().source().call_site(),
        S6CPrephysicalOperationRoleV2::LessCondition => binary_source(
            typed,
            crate::mir::callable_semantic_batch::S6CBinaryRoleV1::LoopConditionLess,
            "less",
        )?
        .site(),
        S6CPrephysicalOperationRoleV2::BodyIndexRead => {
            let Some(argument) = calls.substring().source().arguments().first() else {
                return Err(S6CPrephysicalIngressRejectV2::Anchor("substring argument"));
            };
            let slice = typed
                .binaries()
                .iter()
                .find(|binary| {
                    binary.role()
                        == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::SliceEndAdd
                })
                .ok_or(S6CPrephysicalIngressRejectV2::Anchor("slice add"))?;
            if argument.site() == slice.source().lhs() {
                return Err(S6CPrephysicalIngressRejectV2::Anchor("body index source"));
            }
            return Ok(());
        }
        S6CPrephysicalOperationRoleV2::SliceOne => typed
            .binaries()
            .iter()
            .find(|binary| {
                binary.role() == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::SliceEndAdd
            })
            .ok_or(S6CPrephysicalIngressRejectV2::Anchor("slice add"))?
            .source()
            .rhs(),
        S6CPrephysicalOperationRoleV2::SliceEndAdd => typed
            .binaries()
            .iter()
            .find(|binary| {
                binary.role() == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::SliceEndAdd
            })
            .ok_or(S6CPrephysicalIngressRejectV2::Anchor("slice add"))?
            .source()
            .site(),
        S6CPrephysicalOperationRoleV2::SubstringCall => calls.substring().source().call_site(),
        S6CPrephysicalOperationRoleV2::TextEqual => typed
            .binaries()
            .iter()
            .find(|binary| {
                binary.role() == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::TextEqual
            })
            .ok_or(S6CPrephysicalIngressRejectV2::Anchor("TextEq"))?
            .source()
            .site(),
        S6CPrephysicalOperationRoleV2::ReturnIndexRead => source.loop_return_value(),
        S6CPrephysicalOperationRoleV2::StepIndexRead => typed
            .binaries()
            .iter()
            .find(|binary| {
                binary.role() == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::StepAdd
            })
            .ok_or(S6CPrephysicalIngressRejectV2::Anchor("step add"))?
            .source()
            .lhs(),
        S6CPrephysicalOperationRoleV2::StepOne => typed
            .binaries()
            .iter()
            .find(|binary| {
                binary.role() == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::StepAdd
            })
            .ok_or(S6CPrephysicalIngressRejectV2::Anchor("step add"))?
            .source()
            .rhs(),
        S6CPrephysicalOperationRoleV2::StepAdd => typed
            .binaries()
            .iter()
            .find(|binary| {
                binary.role() == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::StepAdd
            })
            .ok_or(S6CPrephysicalIngressRejectV2::Anchor("step add"))?
            .source()
            .site(),
        S6CPrephysicalOperationRoleV2::StepWrite => {
            let _ = typed.index_update().statement_site();
            return Ok(());
        }
    };
    Ok(())
}

fn binary_source<'a>(
    typed: &'a crate::mir::callable_semantic_batch::VerifiedS6CTypedInputRelationV1,
    role: crate::mir::callable_semantic_batch::S6CBinaryRoleV1,
    label: &'static str,
) -> Result<
    &'a crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1,
    S6CPrephysicalIngressRejectV2,
> {
    typed
        .binaries()
        .iter()
        .find(|binary| binary.role() == role)
        .map(|binary| binary.source())
        .ok_or(S6CPrephysicalIngressRejectV2::Anchor(label))
}
