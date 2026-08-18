//! Caller-zero checked materializer for the S6C Substring V9 result.
//!
//! This child consumes only an already-admitted target, the canonical
//! V6/V7/V8 operand receipt, and the exact Body segment.  It validates the
//! checked wire and adopts the runtime End lease without emitting a CallOut,
//! ValueId, TextEq, or control-flow instruction.

use std::num::NonZeroU64;

use crate::abi::dynamic_call_slot_wire::{
    DynamicV2CallDispositionV1, DynamicV2CallOutV1, DynamicV2CallStatusV1,
    DynamicV2WireSchemaRejectV1, DynamicV2WireTagV1, DYNAMIC_V2_FORWARDED_NONE_V1,
};
use crate::mir::loop_recipe_contract::LoopValueKeyV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::runtime::dynamic_v2_lease::{
    EndAuthorizedTextBorrowRejectV1, EndAuthorizedTextV1, LeaseConsumeRejectV1,
};

use super::super::common_v2_s6c_substring_callout_admission::{
    CommonV2SubstringCallOutAdmissionRejectV1, CommonV2SubstringEndConsumerRefV1,
};
use super::super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1;
use super::s6c_operand_issuer::S6CTextEqOperandReceiptV1;
use super::PreparedCommonV2SubstringCallOutAdmissionV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CommonV2SubstringV9IssuerRejectV1 {
    Admission(String),
    Callback(String),
    OwnerMismatch,
    BodySegmentMismatch,
    BodySegmentDuplicate,
    OperandResultMismatch,
    SegmentBrandMismatch,
    PlanStampMismatch,
    SiteShapeMismatch,
    Wire(DynamicV2WireSchemaRejectV1),
    NonNormalStatus(DynamicV2CallStatusV1),
    ZeroHandle,
    ZeroLease,
    Lease(EndAuthorizedTextBorrowRejectV1),
}

/// Move-only V9 lifetime.  The text is lendable only through the checked
/// End-authorized runtime owner; source/result metadata is callback-scoped and
/// cannot be split into a detached ValueId or raw handle tuple.
#[must_use = "a V9 materialization must be explicitly finished"]
#[derive(Debug)]
pub(in crate::mir::builder) struct CommonV2SubstringV9MaterializationV1 {
    owner: FunctionOwnerIdV1,
    site: crate::mir::checked_callout::CheckedCallOutSiteIdV1,
    result: LoopValueKeyV1,
    physical_block: crate::mir::BasicBlockId,
    text: Option<EndAuthorizedTextV1>,
}

impl CommonV2SubstringV9MaterializationV1 {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn site(
        &self,
    ) -> crate::mir::checked_callout::CheckedCallOutSiteIdV1 {
        self.site
    }

    pub(in crate::mir::builder) const fn result(&self) -> LoopValueKeyV1 {
        self.result
    }

    pub(in crate::mir::builder) const fn physical_block(&self) -> crate::mir::BasicBlockId {
        self.physical_block
    }

    pub(in crate::mir::builder) fn with_text<R>(
        &self,
        callback: impl FnOnce(&str) -> R,
    ) -> Result<R, EndAuthorizedTextBorrowRejectV1> {
        self.text
            .as_ref()
            .ok_or(EndAuthorizedTextBorrowRejectV1::UnknownOrAlreadyConsumed)?
            .with_text(callback)
    }

    pub(in crate::mir::builder) fn finish(mut self) -> Result<(), LeaseConsumeRejectV1> {
        self.text
            .take()
            .expect("V9 materialization finished once")
            .finish()
    }
}

impl Drop for CommonV2SubstringV9MaterializationV1 {
    fn drop(&mut self) {
        if let Some(text) = self.text.take() {
            let _ = text.finish();
        }
    }
}

pub(in crate::mir::builder) fn issue_s6c_substring_v9_from_wire_v1(
    admission: PreparedCommonV2SubstringCallOutAdmissionV1,
    operands: &S6CTextEqOperandReceiptV1<'_, '_, '_>,
    segment: &PreparedSegmentBlockReceiptV1,
    wire: DynamicV2CallOutV1,
) -> Result<CommonV2SubstringV9MaterializationV1, CommonV2SubstringV9IssuerRejectV1> {
    let expected_brand = admission.invocation_brand();
    admission.consume(|target, site_plan, end| {
        validate_static_relation(target, site_plan, end, expected_brand, operands, segment)?;
        validate_wire_shape(&wire)?;
        let handle = wire.value_payload;
        let token = NonZeroU64::new(wire.lease_token)
            .ok_or(CommonV2SubstringV9IssuerRejectV1::ZeroLease)?;
        let text = EndAuthorizedTextV1::adopt(handle, token)
            .map_err(CommonV2SubstringV9IssuerRejectV1::Lease)?;
        Ok(CommonV2SubstringV9MaterializationV1 {
            owner: target.owner(),
            site: site_plan.site_id(),
            result: end.result(),
            physical_block: operands.physical_block(),
            text: Some(text),
        })
    })
}

impl<'source, 'envelope> super::CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    /// Thread one caller-zero V9 issuer through the existing common admission
    /// and operand receipt.  The callback receives only the scoped operand
    /// proof and the move-only runtime result; no MIR callout is emitted.
    pub(in crate::mir::builder) fn with_s6c_substring_v9_issuer<R>(
        &mut self,
        builder: &mut crate::mir::builder::MirBuilder,
        segment: &PreparedSegmentBlockReceiptV1,
        physical_effects: &crate::mir::normal_callable_semantic_package::
            VerifiedS6CPhysicalFunctionEffectsV1,
        wire: DynamicV2CallOutV1,
        callback: impl for<'receipt> FnOnce(
            S6CTextEqOperandReceiptV1<'receipt, 'source, 'envelope>,
            &CommonV2SubstringV9MaterializationV1,
        ) -> Result<R, String>,
    ) -> Result<R, CommonV2SubstringV9IssuerRejectV1> {
        validate_wire_shape(&wire)?;
        self.with_s6c_substring_callout_admission(physical_effects, |session, admission| {
            session
                .with_s6c_text_eq_operands(builder, segment, |_, operands| {
                    let materialization =
                        issue_s6c_substring_v9_from_wire_v1(admission, &operands, segment, wire)
                            .map_err(|error| format!("{error:?}"))?;
                    let callback_result = callback(operands, &materialization);
                    let finish_result = materialization.finish();
                    match (callback_result, finish_result) {
                        (Ok(value), Ok(())) => Ok(value),
                        (Err(callback), Ok(())) => Err(format!("callback: {callback}")),
                        (Ok(_), Err(finish)) => Err(format!("finish: {finish:?}")),
                        (Err(callback), Err(finish)) => {
                            Err(format!("callback: {callback}; finish: {finish:?}"))
                        }
                    }
                })
                .map_err(|error| format!("{error:?}"))
        })
        .map_err(|error| match error {
            CommonV2SubstringCallOutAdmissionRejectV1::Callback(detail) => {
                CommonV2SubstringV9IssuerRejectV1::Callback(detail)
            }
            other => CommonV2SubstringV9IssuerRejectV1::Admission(format!("{other:?}")),
        })
    }
}

fn validate_wire_shape(wire: &DynamicV2CallOutV1) -> Result<(), CommonV2SubstringV9IssuerRejectV1> {
    let status = wire
        .validate_transport()
        .map_err(CommonV2SubstringV9IssuerRejectV1::Wire)?;
    if status != DynamicV2CallStatusV1::Normal {
        if status == DynamicV2CallStatusV1::Suspended {
            return Err(CommonV2SubstringV9IssuerRejectV1::Wire(
                DynamicV2WireSchemaRejectV1::SuspendedNotSupported,
            ));
        }
        return Err(CommonV2SubstringV9IssuerRejectV1::NonNormalStatus(status));
    }
    if wire.result_tag != DynamicV2WireTagV1::HostHandle as u32
        || wire.disposition != DynamicV2CallDispositionV1::EndAuthorized as u32
        || wire.forwarded_input != DYNAMIC_V2_FORWARDED_NONE_V1
        || wire.continuation_token != 0
    {
        return Err(CommonV2SubstringV9IssuerRejectV1::Wire(
            DynamicV2WireSchemaRejectV1::InvalidNormalOutcome,
        ));
    }
    if wire.value_payload == 0 {
        return Err(CommonV2SubstringV9IssuerRejectV1::ZeroHandle);
    }
    if wire.lease_token == 0 {
        return Err(CommonV2SubstringV9IssuerRejectV1::ZeroLease);
    }
    Ok(())
}

fn validate_static_relation(
    target: &crate::mir::loop_recipe_contract::PreparedLoopV2SubstringCallTargetPlanV1,
    site_plan: &crate::mir::checked_callout::CheckedCallOutSitePlanV1,
    end: CommonV2SubstringEndConsumerRefV1,
    expected_brand: crate::mir::module_invocation_identity::ModuleInvocationBrandV1,
    operands: &S6CTextEqOperandReceiptV1<'_, '_, '_>,
    segment: &PreparedSegmentBlockReceiptV1,
) -> Result<(), CommonV2SubstringV9IssuerRejectV1> {
    let owner = target.owner();
    if operands.owner() != owner || segment.owner() != owner || end.owner() != owner {
        return Err(CommonV2SubstringV9IssuerRejectV1::OwnerMismatch);
    }
    if target.block() != operands.body_block()
        || end.result() != target.result()
        || operands.substring_result() != target.result()
    {
        return Err(CommonV2SubstringV9IssuerRejectV1::OperandResultMismatch);
    }
    let mut rows = segment
        .rows()
        .iter()
        .filter(|row| row.logical_block() == target.block());
    let Some(row) = rows.next() else {
        return Err(CommonV2SubstringV9IssuerRejectV1::BodySegmentMismatch);
    };
    let duplicate = rows.next().is_some();
    let brand_matches = segment.belongs_to(&operands.segment_brand());
    if duplicate || row.physical_block() != operands.physical_block() || !brand_matches {
        return Err(if duplicate {
            CommonV2SubstringV9IssuerRejectV1::BodySegmentDuplicate
        } else if !brand_matches {
            CommonV2SubstringV9IssuerRejectV1::SegmentBrandMismatch
        } else {
            CommonV2SubstringV9IssuerRejectV1::BodySegmentMismatch
        });
    }
    if site_plan.plan_stamp() != expected_brand {
        return Err(CommonV2SubstringV9IssuerRejectV1::PlanStampMismatch);
    }
    if !matches!(
        site_plan.normal_shape(),
        crate::mir::checked_callout::CheckedCallOutNormalShapeV1::EndAuthorizedHandle { .. }
    ) || site_plan.effects() != crate::mir::EffectMask::READ
    {
        return Err(CommonV2SubstringV9IssuerRejectV1::SiteShapeMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::CompilationContext;
    use crate::mir::compiler::common_v2_physical_function_entry_input::issue_common_v2_physical_function_entry_input;
    use crate::mir::compiler::common_v2_physical_function_skeleton::reserve_common_v2_physical_function_skeleton;
    use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
    use crate::mir::resolved_semantics::{
        FunctionOwnerIssuerV1, FunctionSemanticResolverSessionV1,
    };
    use crate::mir::MirBuilder;
    use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};
    use crate::runtime::dynamic_v2_lease::publish_end_authorized_text;

    fn final_source() -> VerifiedFinalCallableProgramSourceV1 {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            include_str!("../../../../apps/tests/scan_with_init_typed_ok_min.hako"),
            ParserBuildConfig::default(),
        )
        .expect("S6C issuer source");
        crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
                .expect("source-backed transform");
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                transformed
            else {
                panic!("fixture must remain source-backed")
            };
            source
        })
    }

    fn end_wire(handle: u64, token: NonZeroU64) -> DynamicV2CallOutV1 {
        DynamicV2CallOutV1 {
            status: DynamicV2CallStatusV1::Normal as u32,
            fault_code: 0,
            result_tag: DynamicV2WireTagV1::HostHandle as u32,
            disposition: DynamicV2CallDispositionV1::EndAuthorized as u32,
            forwarded_input: DYNAMIC_V2_FORWARDED_NONE_V1,
            reserved: 0,
            value_payload: handle,
            lease_token: token.get(),
            continuation_token: 0,
        }
    }

    #[test]
    fn issuer_adopts_checked_v9_and_lends_only_inside_callback() {
        let mut resolver = FunctionSemanticResolverSessionV1::new(2211).expect("resolver");
        let package = issue_normal_callable_semantic_package_v1(&mut resolver, final_source())
            .expect("semantic package");
        let mut context = CompilationContext::new();
        let installed = package
            .prepare_install(&mut context)
            .expect("vacant catalog")
            .commit();
        let mut port = installed.begin_lowering(&context).expect("same catalog");

        let _ = port.with_s6c_common_v2_pre_session(|loan| {
            let prepared = issue_common_v2_physical_function_entry_input(loan)
                .expect("physical entry input");
            let skeleton = reserve_common_v2_physical_function_skeleton(prepared)
                .expect("physical skeleton");
            let mut builder = MirBuilder::new();
            let published = publish_end_authorized_text("b").expect("runtime end lease");
            let wire = end_wire(published.handle(), published.token());
            crate::mir::builder::resolved_lowering::with_common_v2_physical_entry_session_with_s6c_effects(
                &mut builder,
                skeleton.into_session_input(),
                |canonical, draft, physical_effects| {
                    let seed = canonical
                        .emit_initial_index_seed(draft)
                        .expect("initial index seed");
                    drop(seed);
                    canonical
                        .with_shared_segment_scope(draft, |canonical, draft, scope| {
                            canonical
                                .with_s6c_substring_v9_issuer(
                                    draft,
                                    scope.receipt(),
                                    physical_effects,
                                    wire,
                                    |operands, materialization| {
                                        assert_eq!(operands.substring_result().raw(), 9);
                                        assert_eq!(materialization.result().raw(), 9);
                                        assert_eq!(
                                            materialization.with_text(str::to_owned),
                                            Ok("b".to_owned())
                                        );
                                        Ok::<(), String>(())
                                    },
                                )
                                .expect("checked V9 issuer");
                            Ok::<(), String>(())
                        })
                        .expect("shared segment");
                    Ok::<(), String>(())
                },
            )
            .expect("unpublished V9 seam");
            Ok::<(), String>(())
        })
        .expect("one installed S6C callback");
        port.complete().expect("selected child coverage");
    }

    #[test]
    fn issuer_rejects_immediate_wire_before_any_operand_effect() {
        let wire = DynamicV2CallOutV1 {
            status: DynamicV2CallStatusV1::Normal as u32,
            fault_code: 0,
            result_tag: DynamicV2WireTagV1::ImmediateI64 as u32,
            disposition: 0,
            forwarded_input: DYNAMIC_V2_FORWARDED_NONE_V1,
            reserved: 0,
            value_payload: 0,
            lease_token: 0,
            continuation_token: 0,
        };
        assert!(matches!(
            validate_wire_shape(&wire),
            Err(CommonV2SubstringV9IssuerRejectV1::Wire(
                DynamicV2WireSchemaRejectV1::InvalidNormalOutcome
            ))
        ));
    }

    #[test]
    fn dropped_v9_materialization_rolls_back_runtime_lease() {
        let mut owner_issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        let owner = owner_issuer.issue().expect("owner");
        let published = publish_end_authorized_text("rollback").expect("runtime end lease");
        let handle = published.handle();
        let token = published.token();
        {
            let materialization = CommonV2SubstringV9MaterializationV1 {
                owner,
                site: crate::mir::checked_callout::CheckedCallOutSiteIdV1::from_test(0),
                result: LoopValueKeyV1::new(9),
                physical_block: crate::mir::BasicBlockId::new(1),
                text: Some(published),
            };
            assert_eq!(
                materialization.with_text(str::to_owned),
                Ok("rollback".to_owned())
            );
        }
        assert_eq!(
            EndAuthorizedTextV1::adopt(handle, token),
            Err(EndAuthorizedTextBorrowRejectV1::UnknownOrAlreadyConsumed)
        );
    }
}
