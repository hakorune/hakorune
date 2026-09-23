//! Physical lowerer for the bounded Main0 derived-predicate profile.
//!
//! The source ingress and common operation demand have already been issued
//! before this module runs. This consumer only projects those products into
//! the existing canonical SSA/CFG/PHI session and closes the normal draft
//! seal. It does not inspect names, re-resolve source, or create a second
//! physical owner. The profile has no callable prelude and no explicit
//! in-loop transfer: loop carriers are seeded from the initialized-local
//! input relations, the body falls through to the loop back-edge, and the
//! tail is the post-loop `return <carrier>` outside the loop.
//!
//! The profile's predicate is derived: the compare LHS is one `Add` over the
//! carrier read and a read-only operand read, so the condition-read
//! relation is verified through the binary operation instead of a direct
//! read result.

use super::initialized_local_input_materializer::materialize_initialized_local_inputs_v1;
use super::operation_ledger::LoopOperationValueLedgerV1;
use super::recursive_after::{prepare_recursive_after_v1, ReadyLoopAfterContinuationV1};
use super::segment_allocator::allocate_for_layout;
use super::segment_dispatcher::prepare_loop_segment_operation_dispatch_v1;
use super::tail_completion::profile_counts_from_dispatch;
use super::topology::ReadyLoopEntryV1;
use super::{LoopOperationDispatchServicesV1, LoopPhysicalServicesV1};
use crate::ast::ASTNode;
use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::builder::normal_main0_derived_predicate_prepared_operation::PreparedMain0DerivedPredicateOperationProgramV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    finish_profile_close, CanonicalBindingReadReceiptV1, CanonicalSsaFunctionSessionV2,
};
use crate::mir::builder::resolved_lowering::draft_seal::ReadyFunctionDraftSealV1;
use crate::mir::builder::MirBuilder;
use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::main0_derived_predicate_recipe_coseal::{
    VerifiedMain0DerivedPredicateControlSourceV1, VerifiedMain0DerivedPredicateTailV1,
};
use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::function::MirParamDecl;
use crate::mir::loop_recipe_contract::{LoopConditionV1, LoopOperationV1, LoopValueClassV1};
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::{
    verify_function_completion_v1, DeclaredFunctionResultContractV1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId, ResolvedExitSiteV1};
use crate::mir::BasicBlockId;

/// Main0 terminal compatibility proof. The profile keeps `Main.main/0`'s
/// declared result unannotated; the Integer(i64) carrier is proven by the
/// JoinSig After binding sealed in the co-seal, so the only physical ABI
/// claim here is the fixed I64 trivial return.
#[derive(Debug)]
struct VerifiedMain0DerivedPredicateTerminalV1 {
    owner: FunctionOwnerIdV1,
    target_function: RegionId,
    abi: ExactTrivialReturnAbiV1,
}

impl VerifiedMain0DerivedPredicateTerminalV1 {
    fn issue(
        input: ResolvedFunctionLoweringInputV1<'_>,
        tail: &VerifiedMain0DerivedPredicateTailV1,
        completion: &VerifiedFunctionCompletionV1,
    ) -> Result<Self, String> {
        if completion.owner() != input.owner() || tail.owner() != input.owner() {
            return Err("[freeze:contract][main0-loop/terminal-owner]".to_owned());
        }
        let target_function = input
            .function()
            .lowering_roots()
            .function_pair()
            .region();
        if completion.target_function() != target_function {
            return Err("[freeze:contract][main0-loop/terminal-target]".to_owned());
        }
        if completion.explicit_site() != Some(tail.statement()) {
            return Err("[freeze:contract][main0-loop/terminal-site]".to_owned());
        }
        if !completion.returns_value() {
            return Err("[freeze:contract][main0-loop/terminal-not-value]".to_owned());
        }
        if !matches!(
            completion.function_exit_contract().declared_result(),
            DeclaredFunctionResultContractV1::Unannotated
        ) {
            return Err("[freeze:contract][main0-loop/terminal-annotated]".to_owned());
        }
        Ok(Self {
            owner: input.owner(),
            target_function,
            abi: ExactTrivialReturnAbiV1::I64,
        })
    }
}

#[derive(Debug)]
struct ReadyMain0DerivedPredicateProfileCloseV1 {
    owner: FunctionOwnerIdV1,
    terminal_block: BasicBlockId,
    after_predecessor_count: usize,
    operation_count: usize,
    pure_count: usize,
    read_count: usize,
    write_count: usize,
}

impl ReadyMain0DerivedPredicateProfileCloseV1 {
    fn finish(self, owner: FunctionOwnerIdV1, terminal_block: BasicBlockId) -> Result<(), String> {
        if self.owner != owner || self.terminal_block != terminal_block {
            return Err("main0 derived-predicate profile close owner/terminal mismatch".into());
        }
        if self.after_predecessor_count != 1 {
            return Err("main0 derived-predicate profile close after-predecessor mismatch".into());
        }
        let observed = (
            self.operation_count,
            self.pure_count,
            self.read_count,
            self.write_count,
        );
        // 9 ops = cond carrier read + operand read + predicate add + bound
        // read + compare + step read + delta const + step add + carrier
        // write; 4 pure (predicate add + compare + delta const + step add),
        // 4 reads, 1 write.
        if observed != (9, 4, 4, 1) {
            return Err(format!(
                "main0 derived-predicate profile close coverage mismatch: observed={observed:?}"
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
struct ReadyMain0DerivedPredicateTailCompletionV1 {
    block: BasicBlockId,
    profile_close: ReadyMain0DerivedPredicateProfileCloseV1,
}

/// Read the condition's carrier read back through the canonical identity to
/// prove the emitted Header PHI read matches the source-bound read row.
/// Unlike `validate_main0_condition_read_relation_v1`, this profile's
/// compare LHS is the predicate `Add` result, so the carrier read is
/// recovered through the binary operation's inputs: exactly one of the
/// Add's two read inputs must target the binding the body writes (the
/// carrier); the other is the read-only operand.
fn validate_main0_derived_condition_read_relation_v1(
    completed: &super::segment_dispatcher::CompletedLoopSegmentProgramV1,
    condition_block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
) -> Result<CanonicalBindingReadReceiptV1, String> {
    let operation_rows = completed.layout.program().operation_rows();
    let condition_rows = operation_rows
        .iter()
        .filter(|row| row.block() == condition_block)
        .collect::<Vec<_>>();
    let compare_left = match condition_rows
        .iter()
        .filter_map(|row| match row.operation() {
            LoopOperationV1::CompareI64 { left, .. } => Some(left),
            _ => None,
        })
        .collect::<Vec<_>>()
        .as_slice()
    {
        [condition] => *condition,
        _ => return Err("[freeze:contract][main0-loop/condition-read]".to_owned()),
    };
    let (add_left, add_right) = match condition_rows
        .iter()
        .filter_map(|row| match row.operation() {
            LoopOperationV1::BinaryI64 { result, left, right, .. } if result == compare_left => {
                Some((left, right))
            }
            _ => None,
        })
        .collect::<Vec<_>>()
        .as_slice()
    {
        [pair] => *pair,
        _ => return Err("[freeze:contract][main0-loop/condition-read]".to_owned()),
    };
    let read_rows = completed
        .layout
        .program()
        .read_binding_rows()
        .map_err(|_| "[freeze:contract][main0-loop/condition-read]".to_owned())?;
    let input_row = |result| read_rows.iter().find(|row| row.result() == result);
    let (Some(left_row), Some(right_row)) = (input_row(add_left), input_row(add_right)) else {
        return Err("[freeze:contract][main0-loop/condition-read]".to_owned());
    };
    // The carrier is the binding the loop body writes; exactly one of the
    // Add's read inputs must be that binding, the other is the operand.
    let write_rows = completed
        .layout
        .program()
        .write_binding_rows()
        .map_err(|_| "[freeze:contract][main0-loop/condition-read]".to_owned())?;
    let [write_row] = write_rows.as_ref() else {
        return Err("[freeze:contract][main0-loop/condition-read]".to_owned());
    };
    let carrier_binding = write_row.source_binding();
    let (carrier_row, operand_row) = match (
        left_row.source_binding() == carrier_binding,
        right_row.source_binding() == carrier_binding,
    ) {
        (true, false) => (left_row, right_row),
        (false, true) => (right_row, left_row),
        _ => return Err("[freeze:contract][main0-loop/condition-read]".to_owned()),
    };
    if operand_row.source_binding() == carrier_binding {
        return Err("[freeze:contract][main0-loop/condition-read]".to_owned());
    }
    let read = match completed
        .dispatch
        .receipts()
        .iter()
        .filter_map(|receipt| match receipt {
            super::operation_dispatcher::LoopOperationDispatchReceiptV1::Read(read)
                if read.result() == carrier_row.result() =>
            {
                Some(*read)
            }
            _ => None,
        })
        .collect::<Vec<_>>()
        .as_slice()
    {
        [read] => *read,
        _ => return Err("[freeze:contract][main0-loop/condition-read]".to_owned()),
    };
    let canonical = read.canonical();
    if canonical.owner() != completed.layout.program().demand().context().owner()
        || read.item() != carrier_row.item()
        || canonical.binding() != carrier_row.source_binding()
    {
        return Err("[freeze:contract][main0-loop/condition-binding]".to_owned());
    }
    Ok(canonical)
}

/// Consume the sealed After receipt exactly once at the Main0 boundary. The
/// tail `return <carrier>` is claimed through the existing
/// Completion/identity ledgers; no raw Return is inserted and no second
/// completion authority is created. The profile has no explicit in-loop
/// transfer site, so no `mark_return` beyond the tail is needed.
fn consume_main0_derived_predicate_tail_completion_v1(
    ready: ReadyLoopAfterContinuationV1,
    condition_read: CanonicalBindingReadReceiptV1,
    profile_counts: (usize, usize, usize, usize),
    tail: &VerifiedMain0DerivedPredicateTailV1,
    control: &VerifiedMain0DerivedPredicateControlSourceV1,
    terminal: &VerifiedMain0DerivedPredicateTerminalV1,
    builder: &mut MirBuilder,
    session: &mut CanonicalSsaFunctionSessionV2<'_>,
) -> Result<ReadyMain0DerivedPredicateTailCompletionV1, String> {
    let owner = ready.owner();
    let after_predecessor_count = ready.predecessor_count();
    if tail.owner() != owner || control.owner() != owner || terminal.owner != owner {
        return Err("[freeze:contract][main0-loop/tail-owner]".to_owned());
    }
    let after = ready.root_after();
    let current = builder
        .function_state
        .current_block
        .ok_or_else(|| "[freeze:contract][main0-loop/tail-block-missing]".to_owned())?;
    if current != after {
        return Err("[freeze:contract][main0-loop/tail-block-mismatch]".to_owned());
    }

    session
        .identity
        .claim_variable_use_binding(tail.value_site(), tail.binding())
        .map_err(|error| format!("[freeze:contract][main0-loop/tail-identity] {error}"))?;
    let receipt = session
        .identity
        .read_entry_receipt(builder, &mut session.phis, after, tail.binding())
        .map_err(|error| format!("[freeze:contract][main0-loop/tail-read] {error}"))?;
    if receipt.owner() != owner || receipt.binding() != tail.binding() {
        return Err("[freeze:contract][main0-loop/tail-binding]".to_owned());
    }
    if receipt.physical_block() != after {
        return Err("[freeze:contract][main0-loop/tail-physical-block]".to_owned());
    }
    let value = receipt.physical_value();
    let expected = terminal.abi.mir_type();
    let found = builder.function_state.type_ctx.get_type(value).cloned();
    if found.as_ref() != Some(&expected) {
        super::operation_type::ensure_provisional_value_class(
            builder,
            value,
            LoopValueClassV1::I64,
        )
        .map_err(|_| "[freeze:contract][main0-loop/tail-type]".to_owned())?;
    }

    let after_current = session
        .identity
        .read_entry_receipt(builder, &mut session.phis, after, condition_read.binding())
        .map_err(|error| format!("[freeze:contract][main0-loop/condition-after] {error}"))?;
    session
        .identity
        .verify_single_predecessor_read_relation(builder, after_current, condition_read)
        .map_err(|error| format!("[freeze:contract][main0-loop/condition-after] {error}"))?;

    session
        .completion
        .claim_explicit_return(tail.statement(), terminal.target_function, after, value)
        .map_err(|error| format!("[freeze:contract][main0-loop/completion] {error}"))?;
    session
        .identity
        .mark_return(ResolvedExitSiteV1::Statement(tail.statement().clone()))
        .map_err(|error| format!("[freeze:contract][main0-loop/mark-return] {error}"))?;

    Ok(ReadyMain0DerivedPredicateTailCompletionV1 {
        block: after,
        profile_close: ReadyMain0DerivedPredicateProfileCloseV1 {
            owner,
            terminal_block: after,
            after_predecessor_count,
            operation_count: profile_counts.0,
            pure_count: profile_counts.1,
            read_count: profile_counts.2,
            write_count: profile_counts.3,
        },
    })
}

/// Physical entry for the selected Main0 derived-predicate product. The
/// prepared program is consumed once; the whole function — initialized
/// locals, derived predicate loop, carrier step, and tail return — is
/// emitted into this one unpublished draft and sealed as
/// `ReadyFunctionDraftSealV1`.
pub(in crate::mir::builder) fn lower_main0_derived_predicate_function_draft_v1(
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    program: PreparedMain0DerivedPredicateOperationProgramV1<'_>,
    physical_name: String,
) -> Result<ReadyFunctionDraftSealV1, String> {
    let (input, input_relations, operation_program, tail, control) = program.into_parts();
    let loop_site = operation_program
        .demand()
        .context()
        .loop_site()
        .node()
        .clone();
    if control.owner() != input.owner() {
        return Err("[freeze:contract][main0-loop/control-owner]".to_owned());
    }
    let completion = verify_function_completion_v1(input)
        .map_err(|error| format!("[freeze:contract][main0-loop/completion] {error:?}"))?;
    let terminal = VerifiedMain0DerivedPredicateTerminalV1::issue(input, &tail, &completion)
        .map_err(|error| format!("[freeze:contract][main0-loop/terminal] {error}"))?;
    let physical_layout = operation_program
        .prepare_physical_layout()
        .map_err(|error| format!("[freeze:contract][main0-loop/layout] {error:?}"))?;

    let source_root = input.source().root();
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
        return Err("[freeze:contract][main0-loop/root-not-function]".to_owned());
    };
    let owner = input.owner();
    let ready = {
        let builder = outer.builder_view_mut_for_lowering();
        builder
            .function_state
            .resolved_binding_state
            .install(input.function())
            .map_err(|error| format!("[freeze:contract][main0-loop/install] {error}"))?;
        builder
            .create_function_skeleton(physical_name.clone(), &params, &body)
            .map_err(|error| format!("[freeze:contract][main0-loop/skeleton] {error}"))?;
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
            .ok_or_else(|| "[freeze:contract][main0-loop/function-missing]".to_owned())?;
        // The profile emits no direct call; the capability check still runs
        // once so an unexamined draft can never silently skip it.
        CanonicalDirectStaticCallCapabilityV1::install_for_function(
            &mut function.metadata.canonical_direct_static_call_capabilities,
            false,
        )
        .map_err(|error| {
            format!("[freeze:contract][main0-loop/direct-call-capability] {error}")
        })?;
        let if_control =
            VerifiedResolvedFunctionIfControlV1::empty_for_owned_loop_profile(input, &loop_site)
                .map_err(|error| format!("[freeze:contract][main0-loop/if-control] {error}"))?;
        let mut session = CanonicalSsaFunctionSessionV2::new(input, if_control, completion, 0)
            .map_err(|error| format!("[freeze:contract][main0-loop/session] {error:?}"))?;
        let preheader = builder
            .function_state
            .current_block
            .ok_or_else(|| "[freeze:contract][main0-loop/preheader-missing]".to_owned())?;
        let entry = materialize_initialized_local_inputs_v1(
            builder,
            &mut session,
            owner,
            input,
            &input_relations,
            preheader,
        )
        .map_err(|error| {
            format!("[freeze:contract][main0-loop/input-materialization] {error:?}")
        })?;
        let make_entry = || ReadyLoopEntryV1::from_rows(owner, preheader, entry.rows.to_vec());
        let segment_receipt = {
            let mut services = LoopPhysicalServicesV1::new(builder, &mut session.cfg);
            allocate_for_layout(&physical_layout, &make_entry(), &mut services)
                .map_err(|error| format!("[freeze:contract][main0-loop/segments] {error:?}"))?
        };
        if segment_receipt.rows().len() != physical_layout.coverage().segment_count() {
            return Err("[freeze:contract][main0-loop/incomplete-segments]".to_owned());
        }
        let (condition_block, condition_key) = match physical_layout
            .program()
            .demand()
            .operation_effect()
            .core()
            .recipe()
            .as_recipe()
            .loops
            .first()
            .map(|node| node.condition)
        {
            Some(LoopConditionV1::Predicate { block, value }) => (block, value),
            _ => return Err("[freeze:contract][main0-loop/condition-missing]".to_owned()),
        };
        // The recipe's declared predicate value must be the compare emitted
        // in the condition block; this is the only place the pair is pinned.
        if !physical_layout
            .program()
            .operation_rows()
            .iter()
            .filter(|row| row.block() == condition_block)
            .any(|row| matches!(row.operation(), LoopOperationV1::CompareI64 { result, .. } if result == condition_key))
        {
            return Err("[freeze:contract][main0-loop/condition-key]".to_owned());
        }
        let plan = prepare_loop_segment_operation_dispatch_v1(
            physical_layout,
            make_entry(),
            segment_receipt,
        )
        .map_err(|error| {
            format!("[freeze:contract][main0-loop/dispatch-preflight] {error:?}")
        })?;
        let values = LoopOperationValueLedgerV1::default();
        let completed = {
            let mut services = LoopOperationDispatchServicesV1::new(
                builder,
                &mut session.identity,
                &mut session.phis,
            );
            plan.emit_all(values, &mut services)
                .map_err(|error| format!("[freeze:contract][main0-loop/dispatch] {error:?}"))?
        };
        let condition_read = validate_main0_derived_condition_read_relation_v1(
            &completed,
            condition_block,
        )
        .map_err(|error| format!("{error}"))?;
        let profile_counts = profile_counts_from_dispatch(&completed.dispatch);
        let prepared_after = prepare_recursive_after_v1(completed, builder).map_err(|error| {
            format!("[freeze:contract][main0-loop/after-preflight] {error:?}")
        })?;
        let ready_after = prepared_after
            .emit_and_seal(
                builder,
                &mut session.cfg,
                &mut session.identity,
                &mut session.phis,
            )
            .map_err(|error| format!("[freeze:contract][main0-loop/after] {error:?}"))?;
        let terminal_receipt = consume_main0_derived_predicate_tail_completion_v1(
            ready_after,
            condition_read,
            profile_counts,
            &tail,
            &control,
            &terminal,
            builder,
            &mut session,
        )
        .map_err(|error| format!("[freeze:contract][main0-loop/tail] {error}"))?;
        let terminal_block = terminal_receipt.block;
        let profile_close = terminal_receipt.profile_close;
        let canonical_close = finish_profile_close(owner, terminal_block, || {
            profile_close.finish(owner, terminal_block)
        })
        .map_err(|error| format!("[freeze:contract][main0-loop/profile-close] {error:?}"))?;
        session
            .finish_for_draft_seal(builder, canonical_close)
            .map_err(|error| format!("[freeze:contract][main0-loop/session-close] {error:?}"))?
    };
    Ok(ready)
}
