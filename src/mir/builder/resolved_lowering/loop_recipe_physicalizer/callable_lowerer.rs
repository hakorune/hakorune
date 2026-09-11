//! Production consumer for the bounded CallableSingleLoop profile.
//!
//! The source ingress and common operation demand have already been issued
//! before this module runs.  This consumer only projects those products into
//! the existing canonical SSA/CFG/PHI session and closes the normal draft
//! seal.  It does not inspect names, re-resolve source, or create a second
//! physical owner.

use super::callable_canary::materialize_callable_prelude_v1;
use super::recursive_after::prepare_recursive_after_v1;
use super::segment_allocator::allocate_for_layout;
use super::segment_dispatcher::prepare_loop_segment_operation_dispatch_v1;
use super::tail_completion::{consume_callable_tail_completion_v1, profile_counts_from_dispatch};
use super::topology::ReadyLoopEntryV1;
use super::{LoopOperationDispatchServicesV1, LoopOperationValueLedgerV1, LoopPhysicalServicesV1};
use crate::ast::ASTNode;
use crate::mir::builder::normal_callable_prepared_operation::
    PreparedCallableLoopOperationProgramV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    finish_profile_close, CanonicalSsaFunctionSessionV2,
};
use crate::mir::builder::resolved_lowering::draft_seal::ReadyFunctionDraftSealV1;
use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
use crate::mir::compiler::callable_single_loop_source_shapes::{
    SourceCallKindV1, SourceReceiverShapeV1,
};
use crate::mir::compiler::loop_physical_prepare::{
    VerifiedCallableFunctionLoweringInputV1, VerifiedCallablePreludeCapabilityV1,
    VerifiedCallableTerminalCompatibilityV1,
};
use crate::mir::function::MirParamDecl;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;

pub(in crate::mir::builder) fn lower_callable_single_loop_function_draft_v1(
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    program: PreparedCallableLoopOperationProgramV1<'_>,
    physical_name: String,
) -> Result<ReadyFunctionDraftSealV1, String> {
    let (source, input_relations, operation_program, prelude_source, tail) = program.into_parts();
    let input = source.input();
    let index = input
        .callable_index()
        .ok_or_else(|| "[freeze:contract][callable-loop/missing-index]".to_owned())?;
    let header = input
        .callable_header()
        .ok_or_else(|| "[freeze:contract][callable-loop/missing-header]".to_owned())?;
    let branded = VerifiedCallableFunctionLoweringInputV1::issue(input, index, header)
        .map_err(|error| format!("[freeze:contract][callable-loop/input] {error:?}"))?;
    let expected_receiver = match prelude_source.call().kind() {
        SourceCallKindV1::Method(receiver) => receiver,
        SourceCallKindV1::FreeStatic => SourceReceiverShapeV1::FreeStatic,
    };
    let prelude = VerifiedCallablePreludeCapabilityV1::issue(
        &branded,
        &prelude_source,
        expected_receiver,
    )
    .map_err(|error| format!("[freeze:contract][callable-loop/prelude] {error:?}"))?;
    let completion = verify_function_completion_v1(branded.input())
        .map_err(|error| format!("[freeze:contract][callable-loop/completion] {error:?}"))?;
    let terminal = VerifiedCallableTerminalCompatibilityV1::issue(
        &branded,
        &prelude,
        &tail,
        &completion,
        declared_result_abi(&branded, &completion)?,
    )
    .map_err(|error| format!("[freeze:contract][callable-loop/terminal] {error:?}"))?;
    let physical_layout = operation_program
        .prepare_physical_layout()
        .map_err(|error| format!("[freeze:contract][callable-loop/layout] {error:?}"))?;

    lower_inside_session(
        outer,
        &branded,
        input_relations,
        physical_layout,
        prelude,
        tail,
        terminal,
        completion,
        physical_name,
    )
}

#[allow(clippy::too_many_arguments)]
fn lower_inside_session<'builder>(
    outer: &mut CanonicalFunctionLoweringSessionV1<'builder>,
    input: &VerifiedCallableFunctionLoweringInputV1<'_>,
    input_relations: crate::mir::loop_recipe_contract::VerifiedLoopInitializedLocalInputSourceSetV1,
    physical_layout: crate::mir::loop_recipe_contract::PreparedLoopPhysicalLayoutV1,
    prelude: VerifiedCallablePreludeCapabilityV1,
    tail: crate::mir::compiler::callable_single_loop_recipe_coseal::VerifiedCallableTailV1,
    terminal: VerifiedCallableTerminalCompatibilityV1,
    completion: crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    physical_name: String,
) -> Result<ReadyFunctionDraftSealV1, String> {
    let source_root = input.input().source().root();
    let ASTNode::FunctionDeclaration {
        params,
        param_decls,
        body,
        return_type_name,
        attrs,
        uses,
        ..
    } = source_root
    else {
        return Err("[freeze:contract][callable-loop/root-not-function]".to_owned());
    };
    let prelude_name = physical_name.clone();
    let owner = input.owner();
    let ready = {
        let builder = outer.builder_view_mut_for_lowering();
        builder
            .function_state
            .resolved_binding_state
            .install(input.input().function())
            .map_err(|error| format!("[freeze:contract][callable-loop/install] {error}"))?;
        builder
            .create_function_skeleton(prelude_name, &params, &body)
            .map_err(|error| format!("[freeze:contract][callable-loop/skeleton] {error}"))?;
        builder.set_current_function_declared_signature(
            param_decls
                .iter()
                .map(|decl| MirParamDecl {
                    name: decl.name.clone(),
                    declared_type_name: decl.declared_type_name.clone(),
                    implicit_receiver: false,
                })
                .collect(),
            return_type_name.clone(),
        );
        builder.set_current_function_runes(&attrs);
        builder.set_current_function_declared_capability_uses(&uses);
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][callable-loop/function-missing]".to_owned())?;
        CanonicalDirectStaticCallCapabilityV1::install_for_function(
            &mut function.metadata.canonical_direct_static_call_capabilities,
            true,
        )
        .map_err(|error| format!("[freeze:contract][callable-loop/direct-call-capability] {error}"))?;
        let if_control = VerifiedResolvedFunctionIfControlV1::empty_for_loop_profile(input.input())
            .map_err(|error| format!("[freeze:contract][callable-loop/if-control] {error:?}"))?;
        let mut session = CanonicalSsaFunctionSessionV2::new(
            input.input(),
            if_control,
            completion,
            0,
        )
        .map_err(|error| format!("[freeze:contract][callable-loop/session] {error:?}"))?;
        let preheader = builder
            .function_state
            .current_block
            .ok_or_else(|| "[freeze:contract][callable-loop/preheader-missing]".to_owned())?;
        let prelude_receipt = materialize_callable_prelude_v1(
            builder,
            &mut session,
            input,
            &input_relations,
            &prelude,
            physical_name.as_str(),
        )
        .map_err(|error| format!("[freeze:contract][callable-loop/prelude-materialization] {error:?}"))?;
        let entry_rows = prelude_receipt
            .entry()
            .rows
            .iter()
            .map(|row| *row)
            .collect::<Vec<_>>();
        let make_entry = || ReadyLoopEntryV1::from_rows(owner, preheader, entry_rows.clone());
        let segment_receipt = {
            let mut services = LoopPhysicalServicesV1::new(
                builder,
                &mut session.cfg,
            );
            allocate_for_layout(&physical_layout, &make_entry(), &mut services)
                .map_err(|error| format!("[freeze:contract][callable-loop/segments] {error:?}"))?
        };
        if segment_receipt.rows().len() != physical_layout.coverage().segment_count() {
            return Err("[freeze:contract][callable-loop/incomplete-segments]".to_owned());
        }
        let condition_key = physical_layout
            .program()
            .operation_rows()
            .iter()
            .find_map(|row| match row.operation() {
                crate::mir::loop_recipe_contract::LoopOperationV1::CompareI64 {
                    result, ..
                } => Some(result),
                _ => None,
            })
            .ok_or_else(|| "[freeze:contract][callable-loop/condition-missing]".to_owned())?;
        let plan = prepare_loop_segment_operation_dispatch_v1(
            physical_layout,
            make_entry(),
            segment_receipt,
        )
        .map_err(|error| format!("[freeze:contract][callable-loop/dispatch-preflight] {error:?}"))?;
        let values = LoopOperationValueLedgerV1::default();
        let completed = {
            let mut services = LoopOperationDispatchServicesV1::new(
                builder,
                &mut session.identity,
                &mut session.phis,
            );
            plan.emit_all(values, &mut services)
                .map_err(|error| format!("[freeze:contract][callable-loop/dispatch] {error:?}"))?
        };
        let profile_counts = profile_counts_from_dispatch(&completed.dispatch);
        let prepared_after = prepare_recursive_after_v1(completed, builder)
            .map_err(|error| format!("[freeze:contract][callable-loop/after-preflight] {error:?}"))?;
        let ready_after = prepared_after
            .emit_and_seal(
                builder,
                &mut session.cfg,
                &mut session.identity,
                &mut session.phis,
            )
            .map_err(|error| format!("[freeze:contract][callable-loop/after] {error:?}"))?;
        let terminal_receipt = consume_callable_tail_completion_v1(
            ready_after,
            profile_counts,
            condition_key,
            &tail,
            &terminal,
            builder,
            &mut session,
        )
        .map_err(|error| format!("[freeze:contract][callable-loop/tail] {error:?}"))?;
        let terminal_block = terminal_receipt.block();
        let profile_close = terminal_receipt.into_profile_close();
        let canonical_close = finish_profile_close(owner, terminal_block, || {
            profile_close.finish(owner, terminal_block)
        })
        .map_err(|error| format!("[freeze:contract][callable-loop/profile-close] {error:?}"))?;
        let ready = session
            .finish_for_draft_seal(builder, canonical_close)
            .map_err(|error| format!("[freeze:contract][callable-loop/session-close] {error:?}"))?;
        ready
    };
    Ok(ready)
}

fn declared_result_abi(
    input: &VerifiedCallableFunctionLoweringInputV1<'_>,
    completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
) -> Result<crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1, String> {
    let declared = completion.function_exit_contract().declared_result();
    let crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1::Annotated(name) =
        declared
    else {
        return Err("[freeze:contract][callable-loop/declared-result-unsupported]".to_owned());
    };
    let completion_abi = crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1::classify(name)
        .ok_or_else(|| "[freeze:contract][callable-loop/declared-result-unsupported]".to_owned())?;
    let header_abi = crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1::classify(
        input.header().signature().result().source_type_name(),
    )
    .ok_or_else(|| "[freeze:contract][callable-loop/header-result-unsupported]".to_owned())?;
    if completion_abi != header_abi {
        return Err("[freeze:contract][callable-loop/declared-result-mismatch]".to_owned());
    }
    Ok(completion_abi)
}
