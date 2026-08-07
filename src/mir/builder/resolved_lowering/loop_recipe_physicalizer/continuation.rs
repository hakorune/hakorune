//! Caller-zero closure of the fixed callable Loop CFG.
//!
//! The topology receipt only allocates blocks.  This test-only seam consumes
//! that open receipt, emits the already verified fixed-profile edges, seals
//! CFG and BindingSSA in backedge-safe order, and returns one After receipt.
//! Tail, Completion, and DraftSeal remain outside this module.

use super::operation_dispatcher::CompletedLoopOperationDispatchV1;
use super::operation_ledger::LoopOperationValueReceiptV1;
use super::topology::{LoopAfterContinuationReceiptV1, LoopPhysicalBlockRoleV1};
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::ResolvedSsaIdentityStateV2;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{LoopNodeKeyV1, LoopValueClassV1};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlockId, MirType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CallableLoopAfterClosureRejectV1 {
    LoopCountMismatch(usize),
    OwnerMismatch,
    ConditionOwnerMismatch,
    ConditionClassMismatch,
    ConditionTypeMismatch,
    ConditionPlacementMismatch {
        expected: BasicBlockId,
        found: BasicBlockId,
    },
    CurrentBlockMismatch {
        expected: BasicBlockId,
        found: BasicBlockId,
    },
    Edge(String),
    CfgSeal(String),
    IdentitySeal(String),
    SelectAfter(String),
    OperationDispatchIncomplete,
    ConditionNotDispatched,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ReadyCallableLoopProfileCloseV1 {
    owner: FunctionOwnerIdV1,
    terminal_block: BasicBlockId,
    after_predecessors: Box<[BasicBlockId]>,
    operation_count: usize,
    pure_count: usize,
    read_count: usize,
    write_count: usize,
    condition_key: crate::mir::loop_recipe_contract::LoopValueKeyV1,
}

impl ReadyCallableLoopProfileCloseV1 {
    pub(super) fn finish(
        self,
        owner: FunctionOwnerIdV1,
        terminal_block: BasicBlockId,
    ) -> Result<(), String> {
        if self.owner != owner {
            return Err("callable profile close owner mismatch".into());
        }
        if self.terminal_block != terminal_block {
            return Err("callable profile close terminal block mismatch".into());
        }
        if self.after_predecessors.len() != 1 {
            return Err("callable profile close After predecessor mismatch".into());
        }
        if (
            self.operation_count,
            self.pure_count,
            self.read_count,
            self.write_count,
        ) != (7, 4, 2, 1)
        {
            return Err("callable profile close operation coverage mismatch".into());
        }
        let _condition_key = self.condition_key;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ReadyLoopAfterContinuationV1 {
    owner: FunctionOwnerIdV1,
    root_loop: LoopNodeKeyV1,
    root_after: BasicBlockId,
    predecessors: Box<[BasicBlockId]>,
    profile_close: ReadyCallableLoopProfileCloseV1,
}

impl ReadyLoopAfterContinuationV1 {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn root_loop(&self) -> LoopNodeKeyV1 {
        self.root_loop
    }

    pub(super) const fn root_after(&self) -> BasicBlockId {
        self.root_after
    }

    pub(super) fn predecessors(&self) -> &[BasicBlockId] {
        &self.predecessors
    }

    pub(super) fn into_profile_close(self) -> ReadyCallableLoopProfileCloseV1 {
        self.profile_close
    }
}

/// Consume the open topology receipt and close the fixed one-loop callable
/// CFG. The completed dispatch proof is consumed before any edge mutation;
/// the condition receipt must come from that same dispatch's opaque value
/// ledger.
pub(super) fn close_callable_loop_after_v1(
    open: LoopAfterContinuationReceiptV1,
    completed: CompletedLoopOperationDispatchV1,
    condition: LoopOperationValueReceiptV1,
    builder: &mut MirBuilder,
    cfg: &mut CanonicalCfgSessionV1,
    identity: &mut ResolvedSsaIdentityStateV2<'_>,
    phis: &mut PhiTxn,
) -> Result<ReadyLoopAfterContinuationV1, CallableLoopAfterClosureRejectV1> {
    if open.loop_count() != 1 {
        return Err(CallableLoopAfterClosureRejectV1::LoopCountMismatch(
            open.loop_count(),
        ));
    }
    if completed.operation_count() == 0 {
        return Err(CallableLoopAfterClosureRejectV1::OperationDispatchIncomplete);
    }
    let mut pure_count = 0;
    let mut read_count = 0;
    let mut write_count = 0;
    for receipt in completed.receipts() {
        match receipt {
            super::operation_dispatcher::LoopOperationDispatchReceiptV1::Pure(_) => pure_count += 1,
            super::operation_dispatcher::LoopOperationDispatchReceiptV1::Read(_) => read_count += 1,
            super::operation_dispatcher::LoopOperationDispatchReceiptV1::Write(_) => {
                write_count += 1
            }
        }
    }
    if (
        completed.operation_count(),
        pure_count,
        read_count,
        write_count,
    ) != (7, 4, 2, 1)
    {
        return Err(CallableLoopAfterClosureRejectV1::OperationDispatchIncomplete);
    }
    if !completed.contains_result(condition.key()) {
        return Err(CallableLoopAfterClosureRejectV1::ConditionNotDispatched);
    }
    let owner = open.owner();
    if condition.owner() != owner {
        return Err(CallableLoopAfterClosureRejectV1::ConditionOwnerMismatch);
    }
    if condition.class() != LoopValueClassV1::Bool {
        return Err(CallableLoopAfterClosureRejectV1::ConditionClassMismatch);
    }
    if builder
        .function_state
        .type_ctx
        .get_type(condition.physical_value())
        != Some(&MirType::Bool)
    {
        return Err(CallableLoopAfterClosureRejectV1::ConditionTypeMismatch);
    }

    let root = open.root_loop();
    let blocks = open.block_receipt();
    let preheader = blocks
        .lookup(root, LoopPhysicalBlockRoleV1::Preheader)
        .ok_or_else(|| CallableLoopAfterClosureRejectV1::Edge("missing preheader".into()))?;
    let header = blocks
        .lookup(root, LoopPhysicalBlockRoleV1::Header)
        .ok_or_else(|| CallableLoopAfterClosureRejectV1::Edge("missing header".into()))?;
    let body = blocks
        .lookup(root, LoopPhysicalBlockRoleV1::Body)
        .ok_or_else(|| CallableLoopAfterClosureRejectV1::Edge("missing body".into()))?;
    let step = blocks
        .lookup(root, LoopPhysicalBlockRoleV1::Step)
        .ok_or_else(|| CallableLoopAfterClosureRejectV1::Edge("missing step".into()))?;
    let after = blocks
        .lookup(root, LoopPhysicalBlockRoleV1::After)
        .ok_or_else(|| CallableLoopAfterClosureRejectV1::Edge("missing after".into()))?;
    if condition.physical_block() != header {
        return Err(
            CallableLoopAfterClosureRejectV1::ConditionPlacementMismatch {
                expected: header,
                found: condition.physical_block(),
            },
        );
    }
    let current = builder.function_state.current_block.ok_or_else(|| {
        CallableLoopAfterClosureRejectV1::CurrentBlockMismatch {
            expected: preheader,
            found: BasicBlockId::new(u32::MAX),
        }
    })?;
    if current != preheader {
        return Err(CallableLoopAfterClosureRejectV1::CurrentBlockMismatch {
            expected: preheader,
            found: current,
        });
    }

    {
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| CallableLoopAfterClosureRejectV1::Edge("missing function".into()))?;
        cfg.emit_jump(function, preheader, header)
            .map_err(|error| CallableLoopAfterClosureRejectV1::Edge(error.to_string()))?;
        cfg.emit_branch(function, header, condition.physical_value(), body, after)
            .map_err(|error| CallableLoopAfterClosureRejectV1::Edge(error.to_string()))?;
        cfg.emit_jump(function, body, step)
            .map_err(|error| CallableLoopAfterClosureRejectV1::Edge(error.to_string()))?;
        cfg.emit_jump(function, step, header)
            .map_err(|error| CallableLoopAfterClosureRejectV1::Edge(error.to_string()))?;
    }

    seal_block(builder, cfg, identity, phis, preheader)?;
    seal_block(builder, cfg, identity, phis, body)?;
    seal_block(builder, cfg, identity, phis, step)?;
    seal_block(builder, cfg, identity, phis, header)?;
    let after_witness = seal_block(builder, cfg, identity, phis, after)?;
    cfg.select_block(builder, after)
        .map_err(|error| CallableLoopAfterClosureRejectV1::SelectAfter(error.to_string()))?;

    Ok(ReadyLoopAfterContinuationV1 {
        owner,
        root_loop: root,
        root_after: after,
        predecessors: after_witness.predecessors().to_vec().into_boxed_slice(),
        profile_close: ReadyCallableLoopProfileCloseV1 {
            owner,
            terminal_block: after,
            after_predecessors: after_witness.predecessors().to_vec().into_boxed_slice(),
            operation_count: completed.operation_count(),
            pure_count,
            read_count,
            write_count,
            condition_key: condition.key(),
        },
    })
}

fn seal_block(
    builder: &mut MirBuilder,
    cfg: &mut CanonicalCfgSessionV1,
    identity: &mut ResolvedSsaIdentityStateV2<'_>,
    phis: &mut PhiTxn,
    block: BasicBlockId,
) -> Result<
    crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedPredecessorsV1,
    CallableLoopAfterClosureRejectV1,
> {
    let witness = {
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| CallableLoopAfterClosureRejectV1::CfgSeal("missing function".into()))?;
        cfg.seal_block(function, block)
            .map_err(|error| CallableLoopAfterClosureRejectV1::CfgSeal(error.to_string()))?
    };
    identity
        .seal_block(builder, phis, block, &witness)
        .map_err(CallableLoopAfterClosureRejectV1::IdentitySeal)?;
    Ok(witness)
}

#[cfg(test)]
mod tests {
    use super::super::operation_dispatcher::LoopOperationDispatchReceiptV1;
    use super::super::tail_completion::consume_callable_tail_completion_v1;
    use super::*;
    use crate::mir::builder::resolved_lowering::canonical_ssa::finish_profile_close;
    use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
    use crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::{
        physicalize_topology_for_operation_demand_v1, prepare_loop_operation_dispatch_v1,
        LoopOperationDispatchServicesV1, LoopOperationValueLedgerV1, LoopPhysicalServicesV1,
    };
    use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
    use crate::mir::compiler::callable_single_loop_operation_effect::issue_callable_operation_effect_v1;
    use crate::mir::compiler::callable_single_loop_recipe_coseal::issue_callable_single_loop_recipe_v1;
    use crate::mir::compiler::callable_single_loop_source_map::issue_callable_single_loop_source_map_v1;
    use crate::mir::compiler::callable_single_loop_source_shapes::SourceReceiverShapeV1;
    use crate::mir::compiler::callable_single_loop_static_fixture_tests::static_fixture_for_test;
    use crate::mir::compiler::callable_single_loop_syntax_facts::issue_callable_single_loop_syntax_facts_v1;
    use crate::mir::compiler::loop_physical_prepare::{
        VerifiedCallableFunctionLoweringInputV1, VerifiedCallablePreludeCapabilityV1,
        VerifiedCallableTerminalCompatibilityV1,
    };
    use crate::mir::function::MirParamDecl;
    use crate::mir::loop_recipe_contract::{
        LoopConditionV1, LoopValueKeyV1, VerifiedLoopOperationPhysicalDemandV1,
    };
    use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
    use crate::mir::resolved_control_flow::verify_function_completion_v1;
    use crate::mir::resolved_semantics::{CanonicalCallableKeyV1, FunctionOwnerIssuerV1};

    #[test]
    fn callable_profile_close_rejects_owner_and_terminal_mismatch() {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("issuer");
        let owner = issuer.issue().expect("owner");
        let foreign = issuer.issue().expect("foreign owner");
        let receipt = || ReadyCallableLoopProfileCloseV1 {
            owner,
            terminal_block: BasicBlockId::new(4),
            after_predecessors: vec![BasicBlockId::new(3)].into_boxed_slice(),
            operation_count: 7,
            pure_count: 4,
            read_count: 2,
            write_count: 1,
            condition_key: LoopValueKeyV1::new(0),
        };
        assert!(receipt().finish(foreign, BasicBlockId::new(4)).is_err());
        assert!(receipt().finish(owner, BasicBlockId::new(5)).is_err());
    }

    #[test]
    fn callable_after_closes_only_after_completed_operation_dispatch() {
        let module = static_fixture_for_test();
        let key = CanonicalCallableKeyV1::free_static_for_test("int_to_str", 1);
        let input = module.function_input(&key).expect("root input");
        let index = module.source().catalog().index();
        let header = index.lookup(&key).expect("callable header");
        let body = input.source().root_body().expect("root body");
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .expect("source ledger");
        let recipe = || {
            let loop_stmt = input.source().body_stmt(&body, 2).expect("loop statement");
            let context = ledger
                .resolved_loop_source(loop_stmt.site())
                .expect("loop source");
            let syntax = issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
                .expect("syntax facts");
            let map =
                issue_callable_single_loop_source_map_v1(&ledger, syntax).expect("source map");
            issue_callable_single_loop_recipe_v1(&ledger, map).expect("recipe product")
        };
        let operation_product =
            issue_callable_operation_effect_v1(recipe()).expect("operation product");
        let owner = operation_product.context().owner();
        let input_binding = operation_product.input().source_binding();
        let recipe_value = operation_product.input().recipe_value();
        let condition_key = match operation_product
            .operation_effect()
            .core()
            .recipe()
            .as_recipe()
            .loops[0]
            .condition
        {
            LoopConditionV1::Predicate { value, .. } => value,
            LoopConditionV1::Always => panic!("callable fixture must have predicate"),
        };
        let (effect, input_relation, context, continuation, prelude_source, tail) =
            operation_product.into_full_parts();
        let mut demand =
            VerifiedLoopOperationPhysicalDemandV1::issue(context, effect, continuation)
                .expect("operation demand");

        let input = VerifiedCallableFunctionLoweringInputV1::issue(input, index, header)
            .expect("branded callable input");
        let prelude = VerifiedCallablePreludeCapabilityV1::issue(
            &input,
            &prelude_source,
            SourceReceiverShapeV1::FreeStatic,
        )
        .expect("prepared callable prelude");
        let completion = verify_function_completion_v1(input.input()).expect("completion");
        let terminal = VerifiedCallableTerminalCompatibilityV1::issue(
            &input,
            &prelude,
            &tail,
            &completion,
            prelude.result_abi(),
        )
        .expect("terminal compatibility");

        let root = input.input().source().root();
        let crate::ast::ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            body,
            return_type_name,
            attrs,
            uses,
            ..
        } = root
        else {
            panic!("expected function root");
        };
        let function_name = format!("{name}/{}", params.len());
        let mut builder = crate::mir::builder::MirBuilder::new();
        let mut outer = builder.open_resolved_function_draft_seal_session_v1(&function_name);
        let mut session = {
            let builder = outer.builder_view_mut_for_lowering();
            builder
                .function_state
                .resolved_binding_state
                .install(input.input().function())
                .expect("install resolver authority");
            builder
                .create_function_skeleton(function_name.clone(), params, body)
                .expect("function skeleton");
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
            builder.set_current_function_runes(attrs);
            builder.set_current_function_declared_capability_uses(uses);
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .expect("function installed");
            CanonicalDirectStaticCallCapabilityV1::install_for_function(
                &mut function.metadata.canonical_direct_static_call_capabilities,
                true,
            )
            .expect("direct-call capability");
            let if_control =
                VerifiedResolvedFunctionIfControlV1::empty_for_loop_profile(input.input())
                    .expect("loop-only If control");
            CanonicalSsaFunctionSessionV2::new(input.input(), if_control, completion, 0)
                .expect("canonical session")
        };
        let preheader = outer
            .builder_view()
            .current_block_for_test()
            .expect("preheader");
        let input_value = {
            let receipt = super::super::callable_canary::materialize_callable_prelude_v1(
                outer.builder_view_mut_for_lowering(),
                &mut session,
                &input,
                &input_relation,
                &prelude,
            )
            .expect("Prelude receipt");
            receipt.entry().rows[0].value()
        };
        let make_entry = || {
            super::super::topology::ReadyLoopEntryV1::new_for_test(
                owner,
                preheader,
                vec![super::super::topology::ReadyLoopEntryRowV1::new(
                    recipe_value,
                    input_binding,
                    input_value,
                )],
            )
        };
        let entry_for_topology = make_entry();
        let entry_for_dispatch = make_entry();
        let open = {
            let mut services = LoopPhysicalServicesV1::new(
                outer.builder_view_mut_for_lowering(),
                &mut session.cfg,
            );
            physicalize_topology_for_operation_demand_v1(&demand, entry_for_topology, &mut services)
                .expect("open topology")
        };
        let program = demand.prepare_all().expect("full operation program");
        assert_eq!(program.coverage().operation_count(), 7);
        let plan = prepare_loop_operation_dispatch_v1(
            program,
            entry_for_dispatch,
            open.block_receipt().clone(),
        )
        .expect("dispatch preflight");
        let mut values_ledger = LoopOperationValueLedgerV1::default();
        let completed = {
            let mut services = LoopOperationDispatchServicesV1::new(
                outer.builder_view_mut_for_lowering(),
                &mut session.identity,
                &mut session.phis,
            );
            plan.emit_all(&mut values_ledger, &mut services)
                .expect("complete operations before CFG closure")
        };
        let condition = values_ledger
            .receipt(condition_key)
            .expect("condition receipt");
        assert_eq!(completed.operation_count(), 7);
        let mut pure_count = 0;
        let mut read_count = 0;
        let mut write_count = 0;
        for receipt in completed.receipts() {
            match receipt {
                LoopOperationDispatchReceiptV1::Pure(_) => pure_count += 1,
                LoopOperationDispatchReceiptV1::Read(_) => read_count += 1,
                LoopOperationDispatchReceiptV1::Write(_) => write_count += 1,
            }
        }
        assert_eq!((pure_count, read_count, write_count), (4, 2, 1));
        assert!(completed.contains_result(condition_key));

        let ready = close_callable_loop_after_v1(
            open,
            completed,
            condition,
            outer.builder_view_mut_for_lowering(),
            &mut session.cfg,
            &mut session.identity,
            &mut session.phis,
        )
        .expect("sealed After continuation");
        assert_eq!(ready.owner(), owner);
        assert_eq!(ready.root_loop().raw(), 0);
        assert_eq!(ready.predecessors().len(), 1);
        assert_eq!(
            outer
                .builder_view()
                .current_block_for_test()
                .expect("selected block"),
            ready.root_after()
        );
        let terminal_receipt = consume_callable_tail_completion_v1(
            ready,
            &tail,
            &terminal,
            outer.builder_view_mut_for_lowering(),
            &mut session,
        )
        .expect("Tail and Completion");
        assert_eq!(terminal_receipt.owner(), owner);
        assert_eq!(
            terminal_receipt.block(),
            outer
                .builder_view()
                .current_block_for_test()
                .expect("Tail block remains selected")
        );
        assert_eq!(terminal_receipt.abi(), prelude.result_abi());
        assert_eq!(
            outer
                .builder_view()
                .function_state
                .type_ctx
                .get_type(terminal_receipt.value()),
            Some(&terminal_receipt.abi().mir_type())
        );
        let second_claim = session.completion.claim_explicit_return(
            tail.statement(),
            terminal.target_function(),
            terminal_receipt.block(),
            terminal_receipt.value(),
        );
        assert!(second_claim.is_err(), "Completion must be one-shot");
        let terminal_block = terminal_receipt.block();
        let profile_close = terminal_receipt.into_profile_close();
        let canonical_close = finish_profile_close(owner, terminal_block, || {
            profile_close.finish(owner, terminal_block)
        })
        .expect("profile close evidence");
        let ready_draft = session
            .finish_for_draft_seal(outer.builder_view_mut_for_lowering(), canonical_close)
            .expect("typed function finish");
        let open_draft = ready_draft.open(outer);
        let prepared = match open_draft.prepare() {
            Ok(prepared) => prepared,
            Err(rejected) => {
                rejected.discard();
                panic!("DraftSeal prepare");
            }
        };
        let completed_draft = prepared.commit();
        let _draft = completed_draft.into_draft();
        assert_eq!(builder.next_value_id().0, 0);
        let fresh = builder.open_resolved_function_draft_seal_session_v1("callable_draft_seal/0");
        fresh.discard_unpublished();
    }
}
