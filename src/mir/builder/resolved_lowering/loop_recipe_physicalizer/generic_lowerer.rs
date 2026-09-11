//! Production consumer for the bounded Generic G0 Loop profile.
//!
//! Admission already co-seals the source parent, neutral operation program,
//! entry rows, layout, control, Completion, and Generic tail.  This module
//! only materializes those products into the existing unpublished function
//! session and returns through the existing Single draft/commit lifecycle.

use super::operation_type::ensure_provisional_value_class;
use super::recursive_after::prepare_recursive_after_v1;
use super::segment_allocator::allocate_for_layout;
use super::segment_dispatcher::prepare_loop_segment_operation_dispatch_v1;
use super::topology::ready_loop_entry_from_canonical_rows;
use super::{LoopOperationDispatchServicesV1, LoopOperationValueLedgerV1, LoopPhysicalServicesV1};
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    finish_profile_close, CanonicalSsaFunctionSessionV2,
};
use crate::mir::builder::calls::{
    CanonicalFunctionLoweringSessionV1, CanonicalFunctionSessionErrorV1,
    PendingFunctionSessionCloseV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::builder::resolved_lowering::ReadyFunctionDraftSealV1;
use crate::mir::compiler::generic_g0_physical_function_entry_input::GenericG0PhysicalLaneRoleV1;
use crate::mir::compiler::generic_g0_physical_operation_cohort::PreparedGenericG0PhysicalEmitterAdmissionV1;
use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::function::MirParamDecl;
use crate::mir::loop_recipe_contract::{LoopValueClassV1, VerifiedGenericG0TailCapabilityV1};
use crate::mir::resolved_semantics::ResolvedExitSiteV1;
use crate::mir::MirFunction;

pub(in crate::mir::builder) fn lower_generic_g0_function_draft_v1(
    builder: &mut MirBuilder,
    admission: PreparedGenericG0PhysicalEmitterAdmissionV1<'_>,
) -> Result<MirFunction, String> {
    if builder.function_state.current_function.is_some()
        || builder.function_state.current_block.is_some()
    {
        return Err("[freeze:contract][generic-g0/lowerer/builder_not_empty]".to_owned());
    }

    let function_name = admission.symbol().as_mir_name().to_owned();
    let mut outer = builder.open_resolved_function_draft_seal_session_v1(&function_name);
    let ready = match lower_generic_g0_function_ready_v1(&mut outer, admission) {
        Ok(ready) => ready,
        Err(error) => {
            outer.discard_unpublished();
            return Err(error);
        }
    };
    let prepared = ready.open(outer).prepare().map_err(|rejected| {
        let detail = format!(
            "[freeze:contract][generic-g0/draft-seal/{:?}] {:?}",
            rejected.stage(),
            rejected.error()
        );
        rejected.discard();
        detail
    })?;
    Ok(prepared.commit().consume_non_authority_evidence())
}

/// Open one canonical child session around the existing Generic lowerer and
/// retain its prepared draft until the invocation collector has admitted it.
/// This is the top-level path used while the caller still owns the Main draft.
pub(in crate::mir::builder) fn lower_generic_g0_function_draft_pending_v1<'builder, 'source>(
    builder: &'builder mut MirBuilder,
    function_name: &str,
    admission: PreparedGenericG0PhysicalEmitterAdmissionV1<'source>,
) -> Result<PendingFunctionSessionCloseV1<'builder>, CanonicalFunctionSessionErrorV1> {
    let mut outer = builder.open_resolved_function_draft_seal_session_v1(function_name);
    let ready = match lower_generic_g0_function_ready_v1(&mut outer, admission) {
        Ok(ready) => ready,
        Err(error) => {
            outer.discard_unpublished();
            return Err(CanonicalFunctionSessionErrorV1::Primary(error));
        }
    };
    let prepared = match ready.open(outer).prepare() {
        Ok(prepared) => prepared,
        Err(rejected) => {
            let detail = format!(
                "[freeze:contract][generic-g0/draft-seal/{:?}] {:?}",
                rejected.stage(),
                rejected.error()
            );
            rejected.discard();
            return Err(CanonicalFunctionSessionErrorV1::Primary(detail));
        }
    };
    Ok(prepared.commit_pending())
}

fn lower_generic_g0_function_ready_v1<'builder, 'source>(
    outer: &mut CanonicalFunctionLoweringSessionV1<'builder>,
    admission: PreparedGenericG0PhysicalEmitterAdmissionV1<'source>,
) -> Result<ReadyFunctionDraftSealV1, String> {
    let mut preflight = admission.into_session_preflight();
    let input = preflight.input();
    let function_name = preflight.shell_plan().symbol().as_mir_name().to_owned();
    let expectation = preflight.take_expectation()?;
    let outer_if = preflight.take_outer_if()?;
    let tail = preflight.take_tail();
    let draft = outer.builder_view_mut_for_lowering();
    (|| {
        draft
            .function_state
            .resolved_binding_state
            .install(input.function())?;
        let param_decls = preflight
            .shell_plan()
            .descriptors()
            .iter()
            .map(|descriptor| MirParamDecl {
                name: descriptor.diagnostic_name().to_owned(),
                declared_type_name: Some("i64".to_owned()),
                implicit_receiver: descriptor.role()
                    == GenericG0PhysicalLaneRoleV1::InstanceReceiver,
            })
            .collect::<Vec<_>>();
        draft.create_resolved_function_skeleton(
            function_name,
            &param_decls,
            Some(preflight.shell_plan().result_abi().source_type_name()),
            preflight.shell_plan().effects().effect_mask(),
        )?;

        let mut session = CanonicalSsaFunctionSessionV2::new_generic(
            input,
            outer_if,
            &expectation,
            preflight.completion(),
        )?;
        session.adopt_generic_g0_entry_lanes(draft, preflight.shell_plan().descriptors())?;
        let preheader = session.entry_block(draft)?;
        let mut rows = Vec::with_capacity(preflight.entries().len());
        for entry in preflight.entries() {
            let receipt = session.identity.read_entry_receipt(
                draft,
                &mut session.phis,
                preheader,
                entry.binding(),
            )?;
            if receipt.owner() != input.owner()
                || receipt.binding() != entry.binding()
                || receipt.physical_block() != preheader
            {
                return Err("[freeze:contract][generic-g0/entry_receipt_drift]".to_owned());
            }
            rows.push((
                entry.recipe_value(),
                entry.binding(),
                receipt.physical_value(),
            ));
        }
        let ready_entry = ready_loop_entry_from_canonical_rows(input.owner(), preheader, rows);
        let segment_receipt = {
            let mut services = LoopPhysicalServicesV1::new(draft, &mut session.cfg);
            allocate_for_layout(preflight.layout(), &ready_entry, &mut services)
                .map_err(|error| format!("[freeze:contract][generic-g0/segments] {error:?}"))?
        };
        let layout = preflight.take_layout();
        let dispatch =
            prepare_loop_segment_operation_dispatch_v1(layout, ready_entry, segment_receipt)
                .map_err(|error| {
                    format!("[freeze:contract][generic-g0/dispatch-preflight] {error:?}")
                })?;
        let completed = {
            let mut services = LoopOperationDispatchServicesV1::new(
                draft,
                &mut session.identity,
                &mut session.phis,
            );
            dispatch
                .emit_all(LoopOperationValueLedgerV1::default(), &mut services)
                .map_err(|error| format!("[freeze:contract][generic-g0/dispatch] {error:?}"))?
        };
        let prepared_after = prepare_recursive_after_v1(completed, draft).map_err(|error| {
            format!("[freeze:contract][generic-g0/after-preflight] {error:?}")
        })?;
        let ready_after = prepared_after
            .emit_and_seal(
                draft,
                &mut session.cfg,
                &mut session.identity,
                &mut session.phis,
            )
            .map_err(|error| format!("[freeze:contract][generic-g0/after] {error:?}"))?;
        let predecessor_count = ready_after.predecessor_count();
        let terminal_block = consume_tail(ready_after, input, &tail, draft, &mut session)?;
        let profile_close = finish_profile_close(input.owner(), terminal_block, || {
            if predecessor_count != 1 {
                return Err(format!(
                    "[freeze:contract][generic-g0/root-after-predecessors] {predecessor_count}"
                ));
            }
            Ok(())
        })?;
        session
            .finish_for_draft_seal(draft, profile_close)
            .map_err(|error| format!("[freeze:contract][generic-g0/session-close] {error:?}"))
    })()
}

fn consume_tail(
    ready: super::recursive_after::ReadyLoopAfterContinuationV1,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    tail: &VerifiedGenericG0TailCapabilityV1,
    builder: &mut MirBuilder,
    session: &mut CanonicalSsaFunctionSessionV2<'_>,
) -> Result<crate::mir::BasicBlockId, String> {
    let owner = ready.owner();
    if owner != input.owner()
        || tail.owner() != owner
        || tail.return_abi() != ExactTrivialReturnAbiV1::I64
    {
        return Err("[freeze:contract][generic-g0/tail_owner_or_abi]".to_owned());
    }
    let after = ready.root_after();
    if builder.function_state.current_block != Some(after) {
        return Err("[freeze:contract][generic-g0/tail_block]".to_owned());
    }
    let post_loop = tail.post_loop_read();
    if post_loop.binding().owner() != owner {
        return Err("[freeze:contract][generic-g0/tail_binding_owner]".to_owned());
    }
    session
        .identity
        .claim_variable_use_binding(post_loop.value(), post_loop.binding())?;
    let receipt = session.identity.read_entry_receipt(
        builder,
        &mut session.phis,
        after,
        post_loop.binding(),
    )?;
    if receipt.owner() != owner
        || receipt.binding() != post_loop.binding()
        || receipt.physical_block() != after
    {
        return Err("[freeze:contract][generic-g0/tail_receipt]".to_owned());
    }
    ensure_provisional_value_class(builder, receipt.physical_value(), LoopValueClassV1::I64)?;
    session.completion.claim_explicit_return(
        post_loop.statement(),
        input.function().function_region(),
        after,
        receipt.physical_value(),
    )?;
    session
        .identity
        .mark_return(ResolvedExitSiteV1::Statement(post_loop.statement().clone()))?;
    Ok(after)
}
