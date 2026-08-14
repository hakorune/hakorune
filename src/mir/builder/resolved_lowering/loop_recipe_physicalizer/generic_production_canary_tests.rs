//! Caller-zero Generic G0 canary for the common Loop physicalizer.
//!
//! This is deliberately a separate profile harness from the Callable canary.
//! The common dispatcher sees only the prepared program and canonical Builder
//! services; G0-specific source, Tail, and completion evidence stay here.

#![cfg(test)]

use super::operation_type::ensure_provisional_value_class;
use super::recursive_after::prepare_recursive_after_v1;
use super::segment_allocator::allocate_for_layout;
use super::segment_dispatcher::prepare_loop_segment_operation_dispatch_v1;
use super::topology::{ReadyLoopEntryRowV1, ReadyLoopEntryV1};
use crate::ast::ASTNode;
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    finish_profile_close, CanonicalSsaFunctionSessionV2,
};
use crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::{
    LoopOperationDispatchServicesV1, LoopOperationValueLedgerV1, LoopPhysicalServicesV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::generic_g0_physical_prepare::{
    issue_generic_g0_loop_ingress_v1, VerifiedGenericG0FunctionLoweringInputV1,
};
use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::function::MirParamDecl;
use crate::mir::loop_recipe_contract::{
    issue_generic_g0_recipe_demand_v1, LoopItemKeyV1, LoopOperationV1, LoopValueClassV1,
};
use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, ResolvedExitSiteV1, SourceBindingSiteV1,
};
use crate::mir::{BasicBlockId, MirType, ValueId};
use hakorune_mir_core::MirValueKind;

const EXPECTED_OPERATION_COUNT: usize = 15;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct G0CanaryReceipt {
    operation_count: usize,
    pure_count: usize,
    read_count: usize,
    carrier_count: usize,
    write_count: usize,
    condition_count: usize,
    segment_count: usize,
    predecessor_count: usize,
    tail_abi: ExactTrivialReturnAbiV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct G0TailReceipt {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    block: BasicBlockId,
    value: ValueId,
    abi: ExactTrivialReturnAbiV1,
}

fn setup_function<'a>(
    builder: &'a mut MirBuilder,
    input: &VerifiedGenericG0FunctionLoweringInputV1<'a>,
    completion: crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
) -> (
    crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1<'a>,
    CanonicalSsaFunctionSessionV2<'a>,
) {
    let root = input.input().source().root();
    let ASTNode::FunctionDeclaration {
        name,
        params,
        param_decls,
        body,
        return_type_name,
        attrs,
        uses,
        is_static,
        ..
    } = root
    else {
        panic!("expected Generic G0 function root")
    };
    let function_name = format!("{name}/{}", params.len());
    let mut outer = builder.open_resolved_function_draft_seal_session_v1(&function_name);
    let session = {
        let draft_builder = outer.builder_view_mut_for_lowering();
        draft_builder
            .function_state
            .resolved_binding_state
            .install(input.input().function())
            .expect("install resolver authority");
        if *is_static {
            draft_builder
                .create_function_skeleton(function_name, params, body)
                .expect("function skeleton");
        } else {
            draft_builder
                .create_method_skeleton(function_name, "GenericG0", params, body)
                .expect("method skeleton");
        }
        let mut declared_params = Vec::with_capacity(params.len() + usize::from(!*is_static));
        if !*is_static {
            declared_params.push(MirParamDecl {
                name: "me".into(),
                declared_type_name: None,
                implicit_receiver: true,
            });
        }
        declared_params.extend(param_decls.iter().map(|decl| MirParamDecl {
            name: decl.name.clone(),
            declared_type_name: decl.declared_type_name.clone(),
            implicit_receiver: false,
        }));
        draft_builder
            .set_current_function_declared_signature(declared_params, return_type_name.clone());
        draft_builder.set_current_function_runes(attrs);
        draft_builder.set_current_function_declared_capability_uses(uses);
        let if_control = VerifiedResolvedFunctionIfControlV1::empty_for_loop_profile(input.input())
            .expect("G0 loop-only If control");
        CanonicalSsaFunctionSessionV2::new(input.input(), if_control, completion, 0)
            .expect("canonical G0 session")
    };
    (outer, session)
}

fn materialize_parameters(
    builder: &mut MirBuilder,
    session: &mut CanonicalSsaFunctionSessionV2<'_>,
    input: &VerifiedGenericG0FunctionLoweringInputV1<'_>,
    preheader: BasicBlockId,
) -> Result<Vec<ReadyLoopEntryRowV1>, String> {
    let ASTNode::FunctionDeclaration {
        params,
        param_decls,
        ..
    } = input.input().source().root()
    else {
        return Err("G0 root is not a function".into());
    };
    let values = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or("G0 function missing")?
        .params
        .clone();
    if params.len() != 2
        || params.len() != param_decls.len()
        || (values.len() != params.len() && values.len() != params.len() + 1)
    {
        return Err("G0 parameter cardinality mismatch".into());
    }
    let has_receiver = values.len() == params.len() + 1;
    if has_receiver {
        let receiver_binding = input
            .input()
            .function()
            .declaration_binding(&SourceBindingSiteV1::Receiver)
            .ok_or("G0 receiver binding missing")?;
        let receiver_record = input
            .input()
            .function()
            .binding(receiver_binding)
            .ok_or("G0 receiver record missing")?;
        if receiver_record.kind() != BindingKindV1::Receiver
            || !matches!(
                receiver_record.origin(),
                BindingOriginV1::Source(SourceBindingSiteV1::Receiver)
            )
        {
            return Err("G0 receiver identity mismatch".into());
        }
        session.identity.publish_declaration(
            &SourceBindingSiteV1::Receiver,
            receiver_record.kind(),
            receiver_record.diagnostic_name(),
            preheader,
            values[0],
        )?;
        builder.register_value_kind(values[0], MirValueKind::Parameter(0));
        builder
            .function_state
            .type_ctx
            .set_type(values[0], MirType::Box("GenericG0".into()));
    }
    for (index, ((name, declaration), value)) in params
        .iter()
        .zip(param_decls.iter())
        .zip(values.iter().copied().skip(usize::from(has_receiver)))
        .enumerate()
    {
        if declaration.declared_type_name.as_deref() != Some("i64") {
            return Err(format!("G0 parameter {index} is not i64"));
        }
        let source_site = SourceBindingSiteV1::Parameter {
            index: index as u32,
        };
        let binding = input
            .input()
            .function()
            .declaration_binding(&source_site)
            .ok_or_else(|| format!("G0 parameter binding {index} missing"))?;
        let record = input
            .input()
            .function()
            .binding(binding)
            .ok_or_else(|| format!("G0 parameter record {index} missing"))?;
        if record.kind()
            != (BindingKindV1::Parameter {
                index: index as u32,
            })
            || !matches!(
                record.origin(),
                BindingOriginV1::Source(site) if site == &source_site
            )
        {
            return Err(format!("G0 parameter {index} identity mismatch"));
        }
        let formal_index = index as u32 + u32::from(has_receiver);
        builder.register_value_kind(value, MirValueKind::Parameter(formal_index));
        builder
            .function_state
            .type_ctx
            .set_type(value, MirType::Integer);
        session.identity.publish_declaration(
            &source_site,
            record.kind(),
            name,
            preheader,
            value,
        )?;
    }
    let rows = input
        .entries()
        .iter()
        .map(|entry| {
            let value = values
                .get(entry.parameter_index() as usize + usize::from(has_receiver))
                .copied()
                .ok_or_else(|| "G0 entry parameter index missing".to_string())?;
            Ok(ReadyLoopEntryRowV1::new(
                entry.recipe_value(),
                entry.binding(),
                value,
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;
    if rows.len() != 2 {
        return Err("G0 entry receipt count mismatch".into());
    }
    Ok(rows)
}

fn consume_g0_tail(
    ready: super::recursive_after::ReadyLoopAfterContinuationV1,
    input: &VerifiedGenericG0FunctionLoweringInputV1<'_>,
    tail: &crate::mir::loop_recipe_contract::VerifiedGenericG0TailCapabilityV1,
    builder: &mut MirBuilder,
    session: &mut CanonicalSsaFunctionSessionV2<'_>,
) -> Result<G0TailReceipt, String> {
    let owner = ready.owner();
    if tail.owner() != owner || tail.return_abi() != ExactTrivialReturnAbiV1::I64 {
        return Err("G0 tail owner/ABI mismatch".into());
    }
    let after = ready.root_after();
    let current = builder
        .function_state
        .current_block
        .ok_or("G0 tail function block missing")?;
    if current != after {
        return Err(format!(
            "G0 tail block mismatch: expected {after:?}, found {current:?}"
        ));
    }
    let post_loop = tail.post_loop_read();
    if post_loop.binding().owner() != owner {
        return Err("G0 tail binding owner mismatch".into());
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
    if receipt.owner() != owner || receipt.binding() != post_loop.binding() {
        return Err("G0 tail canonical receipt mismatch".into());
    }
    if receipt.physical_block() != after {
        return Err("G0 tail read escaped root After".into());
    }
    ensure_provisional_value_class(builder, receipt.physical_value(), LoopValueClassV1::I64)?;
    session.completion.claim_explicit_return(
        post_loop.statement(),
        input.input().function().function_region(),
        after,
        receipt.physical_value(),
    )?;
    session
        .identity
        .mark_return(ResolvedExitSiteV1::Statement(post_loop.statement().clone()))?;
    Ok(G0TailReceipt {
        owner,
        block: after,
        value: receipt.physical_value(),
        abi: tail.return_abi(),
    })
}

fn run_canary(inject_late_failure: bool) -> Result<G0CanaryReceipt, String> {
    let (unit, selection) = generic_source_unit_and_selection_for_test();
    let input = unit
        .root_function_input()
        .map_err(|error| error.to_string())?;
    let product = crate::mir::loop_recipe_contract::produce_generic_g0_recipe_v1(
        issue_generic_g0_recipe_demand_v1(selection)
            .map_err(|error| format!("G0 demand: {error:?}"))?,
    )
    .map_err(|error| format!("G0 product: {error:?}"))?;
    let prepared = issue_generic_g0_loop_ingress_v1(Some(input), product)
        .map_err(|error| format!("G0 ingress: {error:?}"))?;
    let (generic_input, program, tail, _target) = prepared.into_parts();
    if program.coverage().operation_count() != EXPECTED_OPERATION_COUNT {
        return Err("G0 prepared operation coverage is not fifteen".into());
    }
    let physical_layout = program
        .prepare_physical_layout()
        .map_err(|error| format!("G0 physical layout: {error:?}"))?;
    let completion = verify_function_completion_v1(generic_input.input())
        .map_err(|error| format!("G0 completion: {error:?}"))?;
    let owner = generic_input.owner();
    let mut builder = MirBuilder::new();
    let (mut outer, mut session) = setup_function(&mut builder, &generic_input, completion);
    let preheader = outer
        .builder_view()
        .current_block_for_test()
        .map_err(|error| format!("G0 preheader: {error}"))?;
    let entry_rows = match materialize_parameters(
        outer.builder_view_mut_for_lowering(),
        &mut session,
        &generic_input,
        preheader,
    ) {
        Ok(rows) => rows,
        Err(error) => {
            outer.discard_unpublished();
            return Err(format!("G0 parameter materialization: {error}"));
        }
    };
    macro_rules! try_after_session {
        ($expression:expr) => {{
            match $expression {
                Ok(value) => value,
                Err(error) => {
                    let detail = format!("{error:?}");
                    outer.discard_unpublished();
                    return Err(detail);
                }
            }
        }};
    }
    let make_entry = || ReadyLoopEntryV1::new_for_test(owner, preheader, entry_rows.clone());
    let segment_receipt = {
        let mut services =
            LoopPhysicalServicesV1::new(outer.builder_view_mut_for_lowering(), &mut session.cfg);
        try_after_session!(
            allocate_for_layout(&physical_layout, &make_entry(), &mut services)
                .map_err(|error| format!("G0 segment allocation: {error:?}"))
        )
    };
    let segment_count = segment_receipt.rows().len();
    if segment_count != EXPECTED_SEGMENT_COUNT
        || segment_count != physical_layout.coverage().segment_count()
        || segment_receipt.contains_physical_block(segment_receipt.root_after())
    {
        outer.discard_unpublished();
        return Err("G0 segment/root-After receipt is not exact".into());
    }
    let condition_keys = physical_layout
        .program()
        .operation_rows()
        .iter()
        .filter_map(|row| match row.operation() {
            LoopOperationV1::CompareI64 { result, .. } => Some(result),
            _ => None,
        })
        .collect::<Vec<_>>();
    if condition_keys.len() != 2 || condition_keys[0] == condition_keys[1] {
        outer.discard_unpublished();
        return Err("G0 root/child predicate values are not distinct".into());
    }
    let plan = try_after_session!(prepare_loop_segment_operation_dispatch_v1(
        physical_layout,
        make_entry(),
        segment_receipt,
    )
    .map_err(|error| format!("G0 dispatch preflight: {error:?}")));
    let mut values = LoopOperationValueLedgerV1::default();
    if inject_late_failure {
        try_after_session!(values
            .publish(super::operation_ledger::LoopOperationValueReceiptV1::new(
                owner,
                condition_keys[0],
                LoopValueClassV1::Bool,
                LoopItemKeyV1::new(99),
                BasicBlockId::new(0),
                ValueId::new(999),
            ))
            .map_err(|error| format!("G0 duplicate seed: {error:?}")));
    }
    let completed = {
        let mut services = LoopOperationDispatchServicesV1::new(
            outer.builder_view_mut_for_lowering(),
            &mut session.identity,
            &mut session.phis,
        );
        plan.emit_all(values, &mut services)
    };
    if inject_late_failure {
        let error = completed.expect_err("G0 late duplicate must reject");
        outer.discard_unpublished();
        if builder.function_state.current_function.is_some() {
            return Err("G0 discard left an unpublished function".into());
        }
        if !matches!(
            error,
            super::operation_dispatcher::LoopOperationDispatchPhysicalFailureV1::Pure(
                super::operation_emitter::LoopOperationEmissionRejectV1::ValueAlreadyPublished(key)
            ) if key == condition_keys[0]
        ) {
            return Err(format!("unexpected G0 late failure: {error:?}"));
        }
        return Err("late_failure_discarded".into());
    }
    let completed =
        try_after_session!(completed.map_err(|error| format!("G0 dispatch: {error:?}")));
    let mut pure_count = 0;
    let mut read_count = 0;
    let mut carrier_count = 0;
    let mut write_count = 0;
    for receipt in completed.dispatch.receipts() {
        match receipt {
            super::operation_dispatcher::LoopOperationDispatchReceiptV1::Pure(_) => pure_count += 1,
            super::operation_dispatcher::LoopOperationDispatchReceiptV1::Read(_) => read_count += 1,
            super::operation_dispatcher::LoopOperationDispatchReceiptV1::CarrierSeed(_) => {
                read_count += 1;
                carrier_count += 1;
            }
            super::operation_dispatcher::LoopOperationDispatchReceiptV1::Write(_) => {
                write_count += 1
            }
        }
    }
    if completed.dispatch.operation_count() != EXPECTED_OPERATION_COUNT || carrier_count != 1 {
        outer.discard_unpublished();
        return Err("G0 operation coverage/carrier count mismatch".into());
    }
    let conditions = try_after_session!(condition_keys
        .iter()
        .map(|key| {
            completed
                .values
                .receipt(*key)
                .ok_or_else(|| format!("G0 condition receipt missing for {key:?}"))
        })
        .collect::<Result<Vec<_>, _>>());
    if conditions[0].physical_block() == conditions[1].physical_block()
        || conditions[0].physical_value() == conditions[1].physical_value()
        || conditions
            .iter()
            .any(|receipt| receipt.class() != LoopValueClassV1::Bool)
    {
        outer.discard_unpublished();
        return Err("G0 root/child predicate placement is not distinct".into());
    }
    let prepared_after =
        try_after_session!(prepare_recursive_after_v1(completed, outer.builder_view())
            .map_err(|error| format!("G0 After preflight: {error:?}")));
    let ready = try_after_session!(prepared_after
        .emit_and_seal(
            outer.builder_view_mut_for_lowering(),
            &mut session.cfg,
            &mut session.identity,
            &mut session.phis,
        )
        .map_err(|error| format!("G0 After: {error:?}")));
    let predecessor_count = ready.predecessor_count();
    let tail_receipt = try_after_session!(consume_g0_tail(
        ready,
        &generic_input,
        &tail,
        outer.builder_view_mut_for_lowering(),
        &mut session,
    ));
    if tail_receipt.abi != ExactTrivialReturnAbiV1::I64 || tail_receipt.owner != owner {
        outer.discard_unpublished();
        return Err("G0 Tail/Completion receipt mismatch".into());
    }
    let terminal_block = tail_receipt.block;
    let profile_close = try_after_session!(finish_profile_close(owner, terminal_block, || {
        if predecessor_count != 1 {
            return Err(format!(
                "G0 root After predecessor count={predecessor_count}"
            ));
        }
        Ok(())
    })
    .map_err(|error| format!("G0 profile close: {error}")));
    let ready_draft = try_after_session!(session
        .finish_for_draft_seal(outer.builder_view_mut_for_lowering(), profile_close)
        .map_err(|error| format!("G0 DraftSeal finish: {error:?}")));
    let open_draft = ready_draft.open(outer);
    let prepared_draft = match open_draft.prepare() {
        Ok(prepared) => prepared,
        Err(rejected) => {
            let detail = format!(
                "G0 DraftSeal prepare rejected: stage={:?} error={:?}",
                rejected.stage(),
                rejected.error()
            );
            rejected.discard();
            return Err(detail);
        }
    };
    let _completed_draft = prepared_draft.commit().consume_non_authority_evidence();
    Ok(G0CanaryReceipt {
        operation_count: EXPECTED_OPERATION_COUNT,
        pure_count,
        read_count,
        carrier_count,
        write_count,
        condition_count: condition_keys.len(),
        segment_count,
        predecessor_count,
        tail_abi: tail_receipt.abi,
    })
}

const EXPECTED_SEGMENT_COUNT: usize = 5;

#[test]
fn generic_g0_common_physical_canary_reaches_draft_seal() {
    let receipt = run_canary(false).expect("G0 common physical canary");
    assert_eq!(receipt.operation_count, 15);
    assert_eq!(receipt.condition_count, 2);
    assert_eq!(receipt.carrier_count, 1);
    assert_eq!(receipt.segment_count, EXPECTED_SEGMENT_COUNT);
    assert_eq!(receipt.predecessor_count, 1);
    assert_eq!(receipt.tail_abi, ExactTrivialReturnAbiV1::I64);
    assert_eq!(
        receipt.pure_count + receipt.read_count + receipt.write_count,
        15
    );
}

#[test]
fn generic_g0_late_failure_discards_and_fresh_session_replays() {
    assert_eq!(
        run_canary(true).expect_err("G0 late duplicate"),
        "late_failure_discarded"
    );
    let first = run_canary(false).expect("fresh G0 session after discard");
    let second = run_canary(false).expect("second fresh G0 session");
    assert_eq!(first, second);
}
