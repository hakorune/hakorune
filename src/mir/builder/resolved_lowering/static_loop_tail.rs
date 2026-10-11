//! Original post-loop `return me.huge_bin()` in the selected unpublished draft.

use crate::mir::builder::function_fault_frame::FunctionFaultFrameV1;
use crate::mir::builder::normal_callable_loop_source_facts::VerifiedStaticI64LoopSemanticV2;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::static_loop_entry_session::{
    emit_selected_root_graph_v1, SelectedStaticRootGraphPhysicalV1,
};
use crate::mir::builder::resolved_lowering::static_loop_header::StaticLoopHeaderContinuationV1;
use crate::mir::builder::MirBuilder;
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1;
use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
use crate::mir::loop_recipe_contract::LoopValueClassV2;
use crate::mir::normal_callable_semantic_package::{
    CallPacketSourceV1, OrdinaryNewClaimLedgerV1, PreparedLexicalCallProjectionV1,
    PreparedStaticTerminalPacketV1, PreparedStaticTerminalRootRouteV1,
};
use crate::mir::resolved_semantics::{ResolvedExitSiteV1, ResolvedMethodCallReceiverSourceV1};
use crate::mir::{BasicBlockId, MirInstruction, MirType};

pub(super) struct SelectedStaticTailRootV1<'ledger> {
    pub(super) root: SelectedStaticRootGraphPhysicalV1,
    pub(super) route: PreparedStaticTerminalRootRouteV1<'ledger>,
    pub(super) packet: PreparedStaticTerminalPacketV1,
    pub(super) prepared: PreparedLexicalCallProjectionV1,
    pub(super) invoke: (BasicBlockId, MirInstruction),
    pub(super) projection: (BasicBlockId, MirInstruction),
    pub(super) frame: (BasicBlockId, MirInstruction),
}

pub(super) fn emit_unpublished_tail_v1<'ledger>(
    draft: &mut MirBuilder,
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    semantic: &VerifiedStaticI64LoopSemanticV2,
    header: &StaticLoopHeaderContinuationV1,
    frame_owner: &mut FunctionFaultFrameV1,
    claims: &'ledger OrdinaryNewClaimLedgerV1,
    handoff: &VerifiedStaticCallResultPublicationHandoffV1,
) -> Result<SelectedStaticTailRootV1<'ledger>, String> {
    let reject = || "[freeze:contract][callable-loop/static-tail-source-drift]".to_owned();
    let recipe = semantic.recipe().as_recipe();
    let transfer = semantic
        .join()
        .logical_transfer_view()
        .map_err(|_| reject())?;
    let after = transfer.after();
    let [carrier] = recipe.carriers.as_slice() else {
        return Err(reject());
    };
    if after.loop_key() != recipe.root_loop
        || after.binding() != carrier.binding
        || after.class() != LoopValueClassV2::I64
        || header.after == header.body
    {
        return Err(reject());
    }
    let tail = semantic.tail_call();
    let original = tail.original();
    if tail.loop_site() != &semantic.roles().loop_site
        || tail.call_site().owner() != canonical.owner()
        || !tail.claim().is_current_owner_zeroarg()
        || !tail.claim().corroborates_source(
            tail.call_site(),
            ResolvedMethodCallReceiverSourceV1::CurrentOwner,
            0,
        )
        || original.target().arity() != 0
    {
        return Err(reject());
    }
    let route = claims.prepare_static_terminal_root_route_v1(original)?;
    let packet = CallPacketSourceV1::prepare_static_i64(original.clone(), handoff, claims)?;
    let prepared = packet.prepare_zeroarg_projection(handoff)?;
    let call = prepared.materialize_using(
        canonical.owner(),
        packet.loan(handoff),
        packet.observation().arguments(),
        Some(claims),
    )?;
    canonical
        .cfg
        .select_block(draft, header.after)
        .map_err(|error| error.to_string())?;
    let value = canonical.issue_physical_value_id(draft)?;
    canonical.publish_physical_value_type(draft, value, MirType::Integer)?;
    let plan = claims.prepared_root_home_graph_plan_v1(canonical.owner(), tail.return_site())?;
    let root = emit_selected_root_graph_v1(
        draft,
        canonical,
        frame_owner,
        plan,
        Some(value),
        value,
        Some(InvokeOperation::Call {
            call,
            result: InvokeCallResultKind::I64,
        }),
    )?;
    if !matches!(&root.graph.ingress,
        Some(((source, _), _)) if *source == header.after
    ) {
        return Err("[freeze:contract][callable-loop/static-tail-ingress-drift]".into());
    }
    let (invoke, projection) =
        root.graph.ingress.clone().ok_or_else(|| {
            "[freeze:contract][callable-loop/static-tail-ingress-missing]".to_owned()
        })?;
    let after_witness = {
        let function = draft
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/static-tail-function-missing]".to_owned()
            })?;
        let witness = canonical
            .cfg
            .seal_block(function, header.after)
            .map_err(|error| error.to_string())?;
        if witness.predecessors() != [header.normal]
            || !matches!(
                function.blocks.get(&header.normal).and_then(|block| block.terminator.as_ref()),
                Some(MirInstruction::Branch { then_bb, else_bb, .. })
                    if *then_bb == header.body && *else_bb == header.after
            )
        {
            return Err("[freeze:contract][callable-loop/static-tail-after-edge-drift]".into());
        }
        witness
    };
    canonical
        .identity
        .seal_block(draft, &mut canonical.phis, header.after, &after_witness)?;
    canonical.completion.claim_explicit_return(
        tail.return_site(),
        canonical.target_function(),
        root.graph.clean,
        value,
    )?;
    canonical
        .identity
        .mark_return(ResolvedExitSiteV1::Statement(tail.return_site().clone()))?;
    canonical
        .cfg
        .select_block(draft, root.graph.clean)
        .map_err(|error| error.to_string())?;
    let function = draft
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/static-tail-function-missing]".to_owned()
        })?;
    let frame = frame_owner.binding(function)?;
    Ok(SelectedStaticTailRootV1 {
        root,
        route,
        packet,
        prepared,
        invoke,
        projection,
        frame,
    })
}
