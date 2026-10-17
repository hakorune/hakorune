//! Canonical entry for the selected Static I64 Loop collector row.
//! Source/result receipts choose the ABI; final incoming coverage remains owed.

#[path = "static_loop_root_graph_writer.rs"]
mod root_graph_writer;

use crate::ast::ASTNode;
use crate::mir::builder::calls::PendingFunctionSessionCloseV1;
use crate::mir::builder::emission::constant;
use crate::mir::builder::function_fault_frame::FunctionFaultFrameV1;
use crate::mir::builder::module_draft_collector::{DraftPublicationPolicyV1, FunctionDraftKeyV1};
use crate::mir::builder::module_lowering_invocation::ModuleLoweringPortV1;
use crate::mir::builder::normal_callable_loop_source_facts::VerifiedStaticI64LoopSemanticV2;
use crate::mir::builder::ordinary_new_admission::selected::{
    emit_root_graph_v1, RootGraphEmissionV1,
};
use crate::mir::builder::raw_loop_child_entry::{
    StaticI64LoopFunctionEntryV2, StaticI64LoopTaggedPhysicalFormalV2,
};
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::static_loop_publication::SelectedStaticLoopPublicationBatchV1;
use crate::mir::builder::{MirBuilder, NormalCatalogedBoxMethodDraftAdmissionV1};
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::function::{MirFunction, MirParamDecl};
use crate::mir::instruction::InvokeOperation;
use crate::mir::normal_callable_semantic_package::{
    BorrowedFormalActualSourceV1, OrdinaryNewClaimLedgerV1, PendingChildPhysicalValidationV1,
    PendingRootHomeExitV1, PendingSelectedStaticLoopViewV1, PreparedLoopStaticBodyRetentionV1,
    PreparedRootHomeGraphPlanV1, PreparedRootHomeOrderV1, PreparedRootLexicalCallBindingGroupV1,
    PreparedSelectedStaticLoopCallProjectionV1, PreparedStaticTerminalRootRouteV1,
    VerifiedStaticLoopI64ResultSourceV1, VerifiedStaticLoopPacketSourceV1,
};
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_semantics::{
    SourceBindingSiteV1, VerifiedResolvedBlockExpressionExpectationV1,
};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirInstruction, MirType, ValueId};

pub(in crate::mir::builder) struct PreparedSelectedStaticLoopEntryV1<'builder, 'ledger> {
    claims: &'ledger OrdinaryNewClaimLedgerV1,
    entry_owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    entry_values: Box<[(u32, crate::mir::resolved_semantics::BindingRefV1, ValueId)]>,
    pending: PendingFunctionSessionCloseV1<'builder>,
    entry_group: PreparedRootLexicalCallBindingGroupV1<'ledger>,
    body_retention: PreparedLoopStaticBodyRetentionV1<'ledger>,
    publication: SelectedStaticLoopPublicationBatchV1,
    header_route: PreparedStaticTerminalRootRouteV1<'ledger>,
    body_route: PreparedStaticTerminalRootRouteV1<'ledger>,
    tail_route: PreparedStaticTerminalRootRouteV1<'ledger>,
    early_root_pending: PendingRootHomeExitV1<'ledger>,
    tail_root_pending: PendingRootHomeExitV1<'ledger>,
    physical_validation: PendingChildPhysicalValidationV1<'ledger>,
}

/// Physical receipt for one selected root graph. DraftSeal still owns the
/// Return insertion; graph emission owns all incoming edges and projections.
pub(super) struct SelectedStaticRootGraphPhysicalV1 {
    pub(super) graph: RootGraphEmissionV1,
    pub(super) created: Vec<BasicBlockId>,
    pub(super) deferred_return: (BasicBlockId, MirInstruction),
    pub(super) home_order: PreparedRootHomeOrderV1,
}

pub(super) fn emit_selected_root_graph_v1(
    draft: &mut MirBuilder,
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    frame_owner: &mut FunctionFaultFrameV1,
    plan: PreparedRootHomeGraphPlanV1,
    return_value: Option<ValueId>,
    statement_result: ValueId,
    ingress: Option<InvokeOperation>,
) -> Result<SelectedStaticRootGraphPhysicalV1, String> {
    let PreparedRootHomeGraphPlanV1 {
        operations,
        faults,
        order,
    } = plan;
    let mut writer = root_graph_writer::CanonicalRootGraphWriterV1::new(&mut canonical.cfg);
    let graph = emit_root_graph_v1(
        draft,
        frame_owner,
        &mut writer,
        operations,
        faults,
        return_value,
        statement_result,
        ingress,
    )?;
    let [deferred_return] = writer.deferred_clean() else {
        return Err("[freeze:contract][callable-loop/root-return-projection-count]".into());
    };
    if deferred_return.0 != graph.clean
        || !matches!(&deferred_return.1, MirInstruction::Return { value } if *value == return_value)
    {
        return Err("[freeze:contract][callable-loop/root-return-projection-drift]".into());
    }
    Ok(SelectedStaticRootGraphPhysicalV1 {
        created: writer.created().to_vec(),
        deferred_return: deferred_return.clone(),
        home_order: order,
        graph,
    })
}

impl PreparedSelectedStaticLoopEntryV1<'_, '_> {
    pub(in crate::mir::builder) fn collect(
        self,
        module_port: &mut ModuleLoweringPortV1<'_>,
        admission: &NormalCatalogedBoxMethodDraftAdmissionV1,
    ) -> Result<(), String> {
        let Self {
            claims,
            entry_owner,
            entry_values,
            pending,
            entry_group,
            body_retention,
            publication,
            header_route,
            body_route,
            tail_route,
            early_root_pending,
            tail_root_pending,
            physical_validation,
        } = self;
        let (brand, caller, sites_and_targets) = publication.into_parts();
        pending.complete_before_restore(|draft| {
            let prepared = module_port
                .prepare_draft_admission(
                    FunctionDraftKeyV1::CatalogedBoxMethod(admission.source_key().clone()),
                    admission.physical_symbol().to_owned(),
                    admission.physical_arity(),
                    DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                )
                .map_err(|error| error.to_string())?;
            let (unpublished, handoffs) = prepared
                .seal(draft)
                .map_err(|error| error.to_string())?
                .consume_selected_static_result_batch(&brand, &caller, &sites_and_targets)
                .map_err(|error| {
                    format!("[freeze:contract][callable-loop/publication-batch/{error:?}]")
                })?;
            // All fallible draft and publication checks precede this record.
            // Nothing fallible follows: failed collection never lends an entry.
            claims.record_borrowed_ordinary_entry_values_v1(entry_owner, Ok(entry_values))?;
            unpublished.collect();
            let [_, _, _, tail_handoff] = handoffs;
            tail_route.commit_prechecked();
            body_route.commit_prechecked();
            header_route.commit_prechecked();
            entry_group.commit_after_collected();
            early_root_pending.commit_after_collected();
            tail_root_pending.commit_static_after_collected(tail_handoff);
            body_retention.commit_after_collected();
            physical_validation.commit_after_collected();
            Ok(())
        })
    }
}

pub(in crate::mir::builder) fn prepare_selected_static_loop_entry_v1<'builder, 'ledger>(
    builder: &'builder mut MirBuilder,
    input: ResolvedFunctionLoweringInputV1<'_>,
    expectation: &VerifiedResolvedBlockExpressionExpectationV1,
    admission: &NormalCatalogedBoxMethodDraftAdmissionV1,
    product: &StaticI64LoopFunctionEntryV2,
    formal: &StaticI64LoopTaggedPhysicalFormalV2,
    result_source: &VerifiedStaticLoopI64ResultSourceV1,
    packet_source: VerifiedStaticLoopPacketSourceV1,
    handoff: &VerifiedStaticCallResultPublicationHandoffV1,
    tail_handoff: &VerifiedStaticCallResultPublicationHandoffV1,
    publication: SelectedStaticLoopPublicationBatchV1,
    claims: &'ledger OrdinaryNewClaimLedgerV1,
) -> Result<PreparedSelectedStaticLoopEntryV1<'builder, 'ledger>, String> {
    let completion = claims.completion_for_owner(input.owner()).ok_or_else(|| {
        "[freeze:contract][callable-loop/static-entry-completion-missing]".to_owned()
    })?;
    if !result_source.corroborates(product.semantic())
        || !result_source.matches_completion(completion)
        || !packet_source.corroborates(product.semantic().source_calls().0, formal.formal())
        || formal.formal() != product.tagged_formal()
        || formal.lane_index() != 0
    {
        return Err("[freeze:contract][callable-loop/static-entry-cohort-mismatch]".to_owned());
    }
    let if_control = VerifiedResolvedFunctionIfControlV1::empty_for_owned_loop_profile(
        input,
        product.semantic().roles().loop_site.node(),
    )?;
    let physical_symbol = admission.physical_symbol();
    let shell = prepare_shell(input, physical_symbol, formal, result_source)?;

    let mut outer = Some(builder.open_resolved_function_draft_seal_session_v1(physical_symbol));
    let admitted = (|| {
        let draft = outer
            .as_mut()
            .expect("selected Static draft remains owned until DraftSeal")
            .builder_view_mut_for_lowering();
        draft
            .function_state
            .resolved_binding_state
            .install(input.function())?;
        draft.install_prepared_physical_function_skeleton(shell)?;
        let mut canonical =
            CanonicalSsaFunctionSessionV2::new_generic(input, if_control, expectation, completion)?;
        let value = canonical.adopt_exact_formal_parameter(
            draft,
            &SourceBindingSiteV1::Parameter { index: 0 },
            formal.formal(),
            formal.lane_index(),
        )?;
        verify_entry(draft, product.semantic(), formal, result_source, value)?;
        let entry_values: Box<[_]> = Box::new([(0, formal.formal(), value)]);
        claims
            .validate_borrowed_ordinary_entry_values_v1(input.owner(), &Ok(entry_values.clone()))?;
        let (entry_packet, mut frame_owner) = emit_unpublished_static_invoke(
            draft,
            &mut canonical,
            product.semantic(),
            packet_source,
            handoff,
            formal,
            value,
            claims,
        )?;
        let header = super::static_loop_header::emit_unpublished_header_v1(
            draft,
            &mut canonical,
            product.semantic(),
            &entry_packet,
            &mut frame_owner,
        )?;
        let body_source = claims
            .take_loop_static_body_scalar_source_v1(
                &product.semantic().roles().loop_site,
                product.semantic().source_calls().2.call_site(),
            )
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/body-scalar-source-missing]".to_owned()
            })??;
        let body = super::static_loop_body::emit_unpublished_body_predicate_v1(
            draft,
            &mut canonical,
            product.semantic(),
            &header,
            &mut frame_owner,
            body_source,
        )?;
        let pre_root_blocks = draft
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| "[freeze:contract][callable-loop/pre-root-function-missing]".to_owned())?
            .block_ids();
        let early_root = super::static_loop_body_exit::emit_unpublished_body_exit_v1(
            draft,
            &mut canonical,
            product.semantic(),
            &header,
            &body,
            &mut frame_owner,
            claims,
        )?;
        let tail_root = super::static_loop_tail::emit_unpublished_tail_v1(
            draft,
            &mut canonical,
            product.semantic(),
            &header,
            &mut frame_owner,
            claims,
            tail_handoff,
        )?;
        let ready = super::static_loop_draft_finish::finish_unpublished_static_loop_v1(
            draft,
            canonical,
            product.semantic(),
            &pre_root_blocks,
            &early_root,
            &tail_root.root,
            &body,
        )?;
        let open = ready.open(
            outer
                .take()
                .expect("selected Static draft moves into DraftSeal once"),
        );
        let prepared = match open.prepare_exact_two(product.semantic().tail_call().return_site()) {
            Ok(prepared) => prepared,
            Err(rejected) => return Err(rejected.into_discarded_error().to_string()),
        };
        let (site, bindings, packet) = entry_packet.into_local_group_parts()?;
        let entry_group = match claims.prepare_root_lexical_call_bindings(
            input.owner(),
            site,
            bindings,
            packet,
        ) {
            Ok(group) => group,
            Err(error) => {
                prepared.commit_pending().abort_and_restore();
                return Err(error);
            }
        };
        let super::static_loop_tail::SelectedStaticTailRootV1 {
            root: tail_root,
            route: tail_route,
            packet: tail_packet,
            prepared: tail_prepared,
            invoke: tail_invoke,
            projection: tail_projection,
            frame: tail_frame,
        } = tail_root;
        let header_route = match claims
            .prepare_static_terminal_root_route_v1(product.semantic().source_calls().1.original())
        {
            Ok(route) => route,
            Err(error) => {
                prepared.commit_pending().abort_and_restore();
                return Err(error);
            }
        };
        let body_route = match claims
            .prepare_static_terminal_root_route_v1(product.semantic().source_calls().2.original())
        {
            Ok(route) => route,
            Err(error) => {
                prepared.commit_pending().abort_and_restore();
                return Err(error);
            }
        };
        let excluded_routes = [
            header_route.site().clone(),
            body_route.site().clone(),
            tail_route.site().clone(),
        ];
        let tail_projection_block = tail_projection.0;
        let early_root_pending = match claims.prepare_selected_plain_root_exit_v1(
            early_root.home_order,
            early_root.graph.origins,
            early_root.graph.bindings,
            &entry_group,
            &excluded_routes,
        ) {
            Ok(row) => row,
            Err(error) => {
                prepared.commit_pending().abort_and_restore();
                return Err(error);
            }
        };
        let SelectedStaticRootGraphPhysicalV1 {
            graph: tail_graph,
            home_order: tail_home_order,
            ..
        } = tail_root;
        let tail_root_pending = match claims.prepare_selected_static_root_exit_v1(
            tail_home_order,
            tail_graph.origins,
            tail_graph.bindings,
            &entry_group,
            &tail_route,
            &excluded_routes,
            tail_packet,
            tail_prepared,
            tail_handoff,
            tail_invoke,
            tail_projection,
            tail_frame,
        ) {
            Ok(row) => row,
            Err(error) => {
                prepared.commit_pending().abort_and_restore();
                return Err(error);
            }
        };
        let projected_body = prepared.corroborate_detached_function(|function| {
            let mut packet = body.prepare_detached_packet(function)?;
            packet.retain_header_bindings(
                &publication,
                function,
                header.header,
                header.normal,
                header.after,
                tail_projection_block,
            )?;
            if !packet.corroborates_selected_source(
                product.semantic().source_calls().2.call_site(),
                product.semantic().roles().bin_binding,
            ) {
                return Err("[freeze:contract][callable-loop/body-detached-source-drift]".into());
            }
            let pending_view = PendingSelectedStaticLoopViewV1::new(
                &entry_group,
                &packet,
                [&early_root_pending, &tail_root_pending],
                &excluded_routes,
            );
            let physical_validation = claims.validate_selected_static_loop_child_emissions(
                input.owner(),
                function,
                &pending_view,
            )?;
            Ok((packet, physical_validation))
        });
        let retention = (|| {
            let (body_packet, physical_validation) = projected_body?;
            Ok::<_, String>((
                claims.prepare_selected_loop_body_retention(input.owner(), body_packet)?,
                physical_validation,
            ))
        })();
        let (body_retention, physical_validation) = match retention {
            Ok(value) => value,
            Err(error) => {
                prepared.commit_pending().abort_and_restore();
                return Err(error);
            }
        };
        Ok(PreparedSelectedStaticLoopEntryV1 {
            claims,
            entry_owner: input.owner(),
            entry_values,
            pending: prepared.commit_pending(),
            entry_group,
            body_retention,
            publication,
            header_route,
            body_route,
            tail_route,
            early_root_pending,
            tail_root_pending,
            physical_validation,
        })
    })();
    if let Some(outer) = outer {
        outer.discard_unpublished();
    }
    admitted
}

fn emit_unpublished_static_invoke(
    draft: &mut MirBuilder,
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    semantic: &VerifiedStaticI64LoopSemanticV2,
    packet: VerifiedStaticLoopPacketSourceV1,
    handoff: &VerifiedStaticCallResultPublicationHandoffV1,
    formal: &StaticI64LoopTaggedPhysicalFormalV2,
    entry_value: ValueId,
    claims: &OrdinaryNewClaimLedgerV1,
) -> Result<
    (
        PreparedSelectedStaticLoopCallProjectionV1,
        FunctionFaultFrameV1,
    ),
    String,
> {
    let entry = draft
        .function_state
        .current_block
        .ok_or_else(|| "[freeze:contract][callable-loop/static-invoke-entry-missing]".to_owned())?;
    let (caller, site) = packet.publication_source();
    let forwarded = packet.selected_forwarded_actual_for_call(caller, site)?;
    let BorrowedFormalActualSourceV1::Forwarded {
        binding,
        formal: source_formal,
    } = &forwarded.source
    else {
        return Err("[freeze:contract][callable-loop/static-forwarded-kind-drift]".into());
    };
    if forwarded.site != *packet.argument_site() || *source_formal != formal.formal() {
        return Err("[freeze:contract][callable-loop/static-forwarded-source-drift]".into());
    }
    canonical
        .identity
        .claim_variable_use_binding(packet.argument_site(), *binding)?;
    let actual =
        canonical
            .identity
            .read_entry_receipt(draft, &mut canonical.phis, entry, *binding)?;
    if actual.physical_block() != entry || actual.physical_value() != entry_value {
        return Err("[freeze:contract][callable-loop/static-invoke-actual-drift]".into());
    }
    let call = packet.materialize_unpublished_call(formal.formal(), actual.physical_value())?;
    let mut frame_owner = FunctionFaultFrameV1::borrowed();
    let frame = frame_owner.materialize(draft)?;
    let normal = draft.next_block_id();
    let fault = draft.next_block_id();
    let result = draft.next_value_id();
    {
        let function = draft
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/static-invoke-function-missing]".to_owned()
            })?;
        canonical
            .cfg
            .create_block(function, normal)
            .map_err(|error| error.to_string())?;
        canonical
            .cfg
            .create_block(function, fault)
            .map_err(|error| error.to_string())?;
        canonical
            .cfg
            .emit_i64_invoke(function, entry, call, frame, normal, fault)
            .map_err(|error| error.to_string())?;
        canonical
            .cfg
            .emit_invoke_fault(function, fault, frame)
            .map_err(|error| error.to_string())?;
    }
    canonical
        .cfg
        .select_block(draft, normal)
        .map_err(|error| error.to_string())?;
    draft.emit_instruction(MirInstruction::InvokeNormalResult {
        invoke_block: entry,
        dst: result,
    })?;
    draft
        .function_state
        .type_ctx
        .value_types
        .insert(result, MirType::Integer);
    let n = canonical.identity.publish_declaration_exact(
        semantic.source_calls().0.declaration(),
        semantic.roles().n_binding,
        normal,
        result,
    )?;
    let observed = canonical
        .identity
        .read_entry_receipt(draft, &mut canonical.phis, normal, n)?;
    if observed.physical_value() != result || observed.physical_block() != normal {
        return Err("[freeze:contract][callable-loop/static-invoke-n-result-drift]".into());
    }
    let bin_value = constant::emit_integer(draft, semantic.bin_initial_i64())?;
    let bin = canonical.identity.publish_declaration_exact(
        &semantic.roles().bin_declaration,
        semantic.roles().bin_binding,
        normal,
        bin_value,
    )?;
    let bin_read =
        canonical
            .identity
            .read_entry_receipt(draft, &mut canonical.phis, normal, bin)?;
    if bin_read.physical_value() != bin_value || bin_read.physical_block() != normal {
        return Err("[freeze:contract][callable-loop/static-invoke-bin-input-drift]".into());
    }
    let function = draft
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/static-invoke-function-missing]".to_owned()
        })?;
    packet.corroborate_unpublished_physical_call(
        handoff,
        formal.formal(),
        actual.physical_value(),
        function,
        entry,
        normal,
        result,
    )?;
    frame_owner.validate(function)?;
    let invoke = function
        .blocks
        .get(&entry)
        .and_then(|block| block.terminator.as_ref())
        .ok_or_else(|| "[freeze:contract][callable-loop/static-invoke-binding-missing]".to_owned())?
        .clone();
    let projection = function
        .blocks
        .get(&normal)
        .and_then(|block| block.instructions.first())
        .ok_or_else(|| "[freeze:contract][callable-loop/static-result-binding-missing]".to_owned())?
        .clone();
    let source_site = packet.local_call_site().clone();
    let prepared = PreparedSelectedStaticLoopCallProjectionV1::new(
        packet,
        actual,
        entry_value,
        (entry, invoke),
        (normal, projection),
        (
            fault,
            function
                .blocks
                .get(&fault)
                .and_then(|block| block.terminator.as_ref())
                .ok_or_else(|| {
                    "[freeze:contract][callable-loop/static-fault-binding-missing]".to_owned()
                })?
                .clone(),
        ),
        claims,
    )?;
    if prepared.source_site() != &source_site {
        return Err("[freeze:contract][callable-loop/static-packet-site-drift]".into());
    }
    Ok((prepared, frame_owner))
}

fn prepare_shell(
    input: ResolvedFunctionLoweringInputV1<'_>,
    physical_symbol: &str,
    formal: &StaticI64LoopTaggedPhysicalFormalV2,
    result_source: &VerifiedStaticLoopI64ResultSourceV1,
) -> Result<MirFunction, String> {
    let ASTNode::FunctionDeclaration {
        params,
        param_decls,
        return_type_name,
        uses,
        attrs,
        ..
    } = input.source().root()
    else {
        return Err("[freeze:contract][callable-loop/static-entry-source-missing]".to_owned());
    };
    let [name] = params.as_slice() else {
        return Err("[freeze:contract][callable-loop/static-entry-parameter-count]".to_owned());
    };
    let [decl] = param_decls.as_slice() else {
        return Err("[freeze:contract][callable-loop/static-entry-declaration-count]".to_owned());
    };
    if name.is_empty() || decl.name != *name || return_type_name.is_some() {
        return Err("[freeze:contract][callable-loop/static-entry-declaration-drift]".to_owned());
    }
    let mut shell = MirFunction::new(
        FunctionSignature {
            name: physical_symbol.to_owned(),
            params: vec![formal.carrier().mir_type()],
            return_type: result_source.physical_return_type(),
            // The executable effect proof is part of the later packet slice.
            // This unpublished shell uses the conservative upper bound.
            effects: EffectMask::ALL,
        },
        BasicBlockId::new(0),
    );
    shell.metadata.declared_param_decls = vec![MirParamDecl {
        name: name.clone(),
        declared_type_name: decl.declared_type_name.clone(),
        implicit_receiver: false,
    }];
    shell.metadata.declared_return_type_name = None;
    shell.metadata.physical_param_carriers = Some(vec![formal.carrier()].into_boxed_slice());
    shell.metadata.declared_capability_uses = uses.clone();
    shell.metadata.runes = attrs.runes.clone();
    Ok(shell)
}

fn verify_entry(
    draft: &MirBuilder,
    semantic: &VerifiedStaticI64LoopSemanticV2,
    formal: &StaticI64LoopTaggedPhysicalFormalV2,
    result_source: &VerifiedStaticLoopI64ResultSourceV1,
    value: ValueId,
) -> Result<(), String> {
    let function = draft
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/static-entry-function-missing]".to_owned()
        })?;
    if value != ValueId::new(0)
        || function.params.as_slice() != [value]
        || function.signature.params.as_slice() != [formal.carrier().mir_type()]
        || function.signature.return_type != result_source.physical_return_type()
        || function.metadata.physical_param_carriers.as_deref() != Some(&[formal.carrier()][..])
        || semantic.roles().n_binding.owner() != formal.formal().owner()
    {
        return Err("[freeze:contract][callable-loop/static-entry-physical-drift]".to_owned());
    }
    Ok(())
}

#[cfg(test)]
#[path = "static_loop_entry_pending_tests.rs"]
mod pending_tests;
