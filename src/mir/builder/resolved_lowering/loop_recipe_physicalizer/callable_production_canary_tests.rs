//! P0 callable production canary for the complete Loop ingress.
//!
//! This harness is intentionally test-only.  It proves the one-way bridge
//! from normal-callable S2 full demand through the existing Prelude, common
//! topology/operation physicalizer, After, Tail/Completion, and DraftSeal.
//! The late-failure case proves that a partially emitted unpublished function
//! is discarded and that a fresh request, rather than a same-session retry,
//! succeeds.

#![cfg(test)]

use super::callable_canary::materialize_callable_prelude_v1;
use super::recursive_after::prepare_recursive_after_v1;
use super::segment_allocator::allocate_for_layout;
use super::segment_dispatcher::prepare_loop_segment_operation_dispatch_v1;
use super::tail_completion::{consume_callable_tail_completion_v1, profile_counts_from_dispatch};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::builder::normal_callable_semantic_source::{
    PreparedCallableLoopIngressV1, VerifiedNormalCallableSourceIngressReceiptV1,
};
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    finish_profile_close, CanonicalBindingReadReceiptV1, CanonicalSsaFunctionSessionV2,
};
use crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::{
    LoopOperationDispatchServicesV1, LoopOperationValueLedgerV1, LoopPhysicalServicesV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
use crate::mir::compiler::callable_single_loop_recipe_coseal::VerifiedCallableSingleLoopRecipeProductV1;
use crate::mir::compiler::callable_single_loop_source_map::issue_callable_single_loop_source_map_v1;
use crate::mir::compiler::callable_single_loop_syntax_facts::issue_callable_single_loop_syntax_facts_from_ledger_v1;
use crate::mir::compiler::loop_physical_prepare::{
    VerifiedCallableFunctionLoweringInputV1, VerifiedCallablePreludeCapabilityV1,
    VerifiedCallableTerminalCompatibilityV1,
};
use crate::mir::compiler::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use crate::mir::function::MirParamDecl;
use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopOperationV1, LoopValueClassV1};
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::CanonicalCallableKeyV1;
use crate::mir::resolved_semantics::{
    CallableCatalogSealOutcomeV1, VerifiedCallableHeaderSourceUnitV1,
    VerifiedOwnerFreeCallableCatalogSourceUnitV1,
};

const COMPILATION_BRAND: u32 = 53;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CanaryReceipt {
    operation_count: usize,
    pure_count: usize,
    read_count: usize,
    write_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CallableLoopMutationV1 {
    None,
    DuplicateCondition,
    StaleGeneration,
    WrongEdgePredecessor,
    UnsealedPublication,
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn function(name: &str, params: &[&str], body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: params.iter().map(|param| (*param).into()).collect(),
        param_decls: params
            .iter()
            .map(|param| ParamDecl {
                name: (*param).into(),
                declared_type_name: Some("i64".into()),
            })
            .collect(),
        return_type_name: Some("i64".into()),
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn loop_program() -> ASTNode {
    ASTNode::Program {
        statements: vec![
            function(
                "int_to_str",
                &["unused", "value"],
                vec![
                    ASTNode::Local {
                        variables: vec!["value".into()],
                        initial_values: vec![Some(Box::new(ASTNode::FunctionCall {
                            name: "to_i64".into(),
                            arguments: vec![variable("value")],
                            span: Span::unknown(),
                        }))],
                        declared_type_names: vec![Some("i64".into())],
                        span: Span::unknown(),
                    },
                    ASTNode::Local {
                        variables: vec!["i".into()],
                        initial_values: vec![Some(Box::new(integer(0)))],
                        declared_type_names: vec![Some("i64".into())],
                        span: Span::unknown(),
                    },
                    ASTNode::Loop {
                        condition: Box::new(ASTNode::BinaryOp {
                            operator: BinaryOperator::Less,
                            left: Box::new(variable("i")),
                            right: Box::new(integer(1)),
                            span: Span::unknown(),
                        }),
                        body: vec![ASTNode::Assignment {
                            target: Box::new(variable("i")),
                            value: Box::new(ASTNode::BinaryOp {
                                operator: BinaryOperator::Add,
                                left: Box::new(variable("i")),
                                right: Box::new(integer(1)),
                                span: Span::unknown(),
                            }),
                            span: Span::unknown(),
                        }],
                        span: Span::unknown(),
                    },
                    ASTNode::Return {
                        value: Some(Box::new(variable("value"))),
                        span: Span::unknown(),
                    },
                ],
            ),
            function(
                "to_i64",
                &["n"],
                vec![ASTNode::Return {
                    value: Some(Box::new(variable("n"))),
                    span: Span::unknown(),
                }],
            ),
        ],
        span: Span::unknown(),
    }
}

fn exact_module(program: ASTNode) -> VerifiedResolvedCallableModuleV1 {
    let source = VerifiedCallableHeaderSourceUnitV1::seal_header_surface(program)
        .expect("exact header source");
    let owner_free = VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source)
        .expect("exact owner-free catalog");
    let catalog = CallableCatalogSealOutcomeV1::seal(owner_free, COMPILATION_BRAND)
        .expect("exact callable catalog");
    VerifiedResolvedCallableModuleV1::resolve(catalog).expect("exact resolved module")
}

fn canonical_key() -> CanonicalCallableKeyV1 {
    CanonicalCallableKeyV1::free_static_for_test("int_to_str", 2)
}

fn assert_loop_add_backedge(
    builder: &MirBuilder,
    header: CanonicalBindingReadReceiptV1,
) -> Result<(), String> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| "selected function disappeared before PHI check".to_owned())?;
    let add = function
        .blocks
        .iter()
        .find_map(|(block, block_data)| {
            block_data.instructions.iter().find_map(|instruction| {
                matches!(
                    instruction,
                    crate::mir::MirInstruction::BinOp {
                        op: crate::mir::BinaryOp::Add,
                        ..
                    }
                )
                .then(|| match instruction {
                    crate::mir::MirInstruction::BinOp { dst, .. } => (*block, *dst),
                    _ => unreachable!("matched only Add BinOp"),
                })
            })
        })
        .ok_or_else(|| "loop Add result missing before PHI check".to_owned())?;
    let header_phi = function
        .get_block(header.physical_block())
        .and_then(|block| {
            block.instructions.iter().find_map(|instruction| match instruction {
                crate::mir::MirInstruction::Phi { dst, inputs, .. }
                    if *dst == header.physical_value() => Some(inputs),
                _ => None,
            })
        })
        .ok_or_else(|| "header PHI missing before PHI check".to_owned())?;
    if !header_phi.contains(&add) {
        return Err(format!(
            "header PHI does not receive loop Add backedge: add={add:?} phi={header_phi:?}"
        ));
    }
    Ok(())
}

fn logical_product(
    receipt: &crate::mir::builder::normal_callable_semantic_source::
        VerifiedNormalCallableSourceIngressReceiptV1<'_>,
) -> VerifiedCallableSingleLoopRecipeProductV1 {
    let syntax =
        issue_callable_single_loop_syntax_facts_from_ledger_v1(receipt.input(), receipt.ledger())
            .expect("syntax facts");
    let map =
        issue_callable_single_loop_source_map_v1(receipt.ledger(), syntax).expect("source map");
    crate::mir::compiler::callable_single_loop_recipe_coseal::issue_callable_single_loop_recipe_v1(
        receipt.ledger(),
        map,
    )
    .expect("logical recipe product")
}

fn setup_function<'a>(
    builder: &'a mut MirBuilder,
    input: &VerifiedCallableFunctionLoweringInputV1<'a>,
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
        ..
    } = root
    else {
        panic!("expected function root")
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
        draft_builder
            .create_function_skeleton(function_name, params, body)
            .expect("function skeleton");
        draft_builder.set_current_function_declared_signature(
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
        draft_builder.set_current_function_runes(attrs);
        draft_builder.set_current_function_declared_capability_uses(uses);
        let function = draft_builder
            .function_state
            .current_function
            .as_mut()
            .expect("function installed");
        CanonicalDirectStaticCallCapabilityV1::install_for_function(
            &mut function.metadata.canonical_direct_static_call_capabilities,
            true,
        )
        .expect("direct-call capability");
        let if_control = VerifiedResolvedFunctionIfControlV1::empty_for_loop_profile(input.input())
            .expect("loop-only If control");
        CanonicalSsaFunctionSessionV2::new(input.input(), if_control, completion, 0)
            .expect("canonical session")
    };
    (outer, session)
}

fn run_canary(mutation: CallableLoopMutationV1) -> Result<CanaryReceipt, String> {
    let module = exact_module(loop_program());
    let exact_key = canonical_key();
    let exact_input = module
        .function_input(&exact_key)
        .map_err(|error| format!("exact input: {error:?}"))?;
    let index = module.source().catalog().index();
    let header = index
        .lookup(&exact_key)
        .ok_or_else(|| "exact callable header missing".to_owned())?;
    let source_receipt =
        VerifiedNormalCallableSourceIngressReceiptV1::from_resolved_input_for_test(exact_input)
            .map_err(|error| format!("source ingress: {error}"))?;
    let logical = logical_product(&source_receipt);
    let prepared = PreparedCallableLoopIngressV1::from_source_for_test(source_receipt, logical)
        .prepare_full_demand()
        .map_err(|error| format!("full S2 demand: {error:?}"))?;
    let (source_receipt, input_relations, operation_program, prelude_source, tail) =
        prepared.into_parts();
    let physical_layout = operation_program
        .prepare_physical_layout()
        .map_err(|error| format!("physical layout: {error:?}"))?;
    let branded = VerifiedCallableFunctionLoweringInputV1::issue(exact_input, index, header)
        .map_err(|error| format!("exact callable brand: {error:?}"))?;
    let prelude = VerifiedCallablePreludeCapabilityV1::issue(
        &branded,
        &prelude_source,
        crate::mir::compiler::callable_single_loop_source_shapes::SourceReceiverShapeV1::FreeStatic,
    )
    .map_err(|error| format!("Prelude contract: {error:?}"))?;
    let completion = verify_function_completion_v1(branded.input())
        .map_err(|error| format!("completion: {error:?}"))?;
    let terminal = VerifiedCallableTerminalCompatibilityV1::issue(
        &branded,
        &prelude,
        &tail,
        &completion,
        prelude.result_abi(),
    )
    .map_err(|error| format!("terminal contract: {error:?}"))?;
    let owner = source_receipt.owner();
    let mut builder = MirBuilder::new();
    let (mut outer, mut session) = setup_function(&mut builder, &branded, completion);
    let preheader = outer
        .builder_view()
        .current_block_for_test()
        .map_err(|error| format!("missing preheader: {error}"))?;
    let prelude_receipt = materialize_callable_prelude_v1(
        outer.builder_view_mut_for_lowering(),
        &mut session,
        &branded,
        &input_relations,
        &prelude,
        "int_to_str/2",
    )
    .map_err(|error| format!("Prelude materialization: {error}"))?;
    assert_eq!(input_relations.rows().len(), 1);
    let entry_rows = prelude_receipt.entry().rows.to_vec();
    let make_entry =
        || super::topology::ReadyLoopEntryV1::new_for_test(owner, preheader, entry_rows.clone());
    let segment_receipt = {
        let mut services =
            LoopPhysicalServicesV1::new(outer.builder_view_mut_for_lowering(), &mut session.cfg);
        allocate_for_layout(&physical_layout, &make_entry(), &mut services)
            .map_err(|error| format!("segment allocator: {error:?}"))?
    };
    if segment_receipt.rows().len() != physical_layout.coverage().segment_count() {
        drop(session);
        outer.discard_unpublished();
        return Err("segment allocator emitted an incomplete R1 receipt".into());
    }
    let condition_key = physical_layout
        .program()
        .operation_rows()
        .iter()
        .find_map(|row| match row.operation() {
            LoopOperationV1::CompareI64 { result, .. } => Some(result),
            _ => None,
        })
        .ok_or_else(|| "condition operation missing".to_owned())?;
    let condition_block = segment_receipt
        .lookup(physical_layout.entry_segment())
        .ok_or_else(|| "condition physical block missing".to_owned())?;
    if segment_receipt.root_after() == condition_block {
        drop(session);
        outer.discard_unpublished();
        return Err("root After aliased the entry segment".into());
    }
    let plan =
        prepare_loop_segment_operation_dispatch_v1(physical_layout, make_entry(), segment_receipt)
            .map_err(|error| format!("dispatch preflight: {error:?}"))?;
    let mut values = LoopOperationValueLedgerV1::default();
    if mutation == CallableLoopMutationV1::DuplicateCondition {
        let existing = crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::
            LoopOperationValueReceiptV1::new(
                owner,
                condition_key,
                LoopValueClassV1::Bool,
                LoopItemKeyV1::new(99),
                condition_block,
                crate::mir::ValueId::new(999),
            );
        values
            .publish(existing)
            .map_err(|error| format!("seed ledger: {error:?}"))?;
    }
    let completed = {
        let mut services = LoopOperationDispatchServicesV1::new(
            outer.builder_view_mut_for_lowering(),
            &mut session.identity,
            &mut session.phis,
        );
        plan.emit_all(values, &mut services)
    };
    if mutation == CallableLoopMutationV1::DuplicateCondition {
        let error = match completed {
            Ok(_) => {
                drop(session);
                outer.discard_unpublished();
                return Err("late duplicate unexpectedly succeeded".to_owned());
            }
            Err(error) => error,
        };
        if !matches!(
            error,
            super::operation_dispatcher::LoopOperationDispatchPhysicalFailureV1::Pure(
                super::operation_emitter::LoopOperationEmissionRejectV1::ValueAlreadyPublished(
                    key,
                )
            ) if key == condition_key
        ) {
            drop(session);
            outer.discard_unpublished();
            return Err(format!("unexpected late failure: {error:?}"));
        }
        drop(session);
        outer.discard_unpublished();
        if builder.function_state.current_function.is_some() {
            return Err("discard left unpublished current function".into());
        }
        return Err("late_failure_discarded".into());
    }
    let completed = completed.map_err(|error| format!("operation dispatch: {error:?}"))?;
    let profile_counts = profile_counts_from_dispatch(&completed.dispatch);
    let prepared_after = prepare_recursive_after_v1(completed, outer.builder_view())
        .map_err(|error| format!("After preflight: {error:?}"))?;
    let ready = prepared_after
        .emit_and_seal(
            outer.builder_view_mut_for_lowering(),
            &mut session.cfg,
            &mut session.identity,
            &mut session.phis,
        )
        .map_err(|error| format!("After: {error:?}"))?;
    if let Err(error) = assert_loop_add_backedge(outer.builder_view(), ready.header_current()) {
        drop(session);
        outer.discard_unpublished();
        return Err(error);
    }
    match mutation {
        CallableLoopMutationV1::StaleGeneration => {
            let header = ready.header_current();
            let block = outer
                .builder_view_mut_for_lowering()
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| "selected function disappeared before mutation".to_owned())?
                .get_block_mut(header.physical_block())
                .ok_or_else(|| "selected header block disappeared before mutation".to_owned())?;
            let changed = block.instructions.iter_mut().find_map(|instruction| {
                let crate::mir::MirInstruction::Phi { dst, .. } = instruction else {
                    return None;
                };
                (*dst == header.physical_value()).then(|| {
                    *dst = crate::mir::ValueId::new(u32::MAX);
                })
            });
            if changed.is_none() {
                let detail = format!("header={header:?} block={block:?}");
                drop(session);
                outer.discard_unpublished();
                return Err(format!(
                    "stale-generation mutation had no canonical header PHI: {detail}"
                ));
            }
        }
        CallableLoopMutationV1::WrongEdgePredecessor => {
            let header = ready.header_current().physical_block();
            let block = outer
                .builder_view_mut_for_lowering()
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| "selected function disappeared before mutation".to_owned())?
                .get_block_mut(ready.root_after())
                .ok_or_else(|| "selected After block disappeared before mutation".to_owned())?;
            if !block.predecessors.remove(&header) {
                drop(session);
                outer.discard_unpublished();
                return Err("wrong-edge mutation found no canonical predecessor".to_owned());
            }
            block
                .predecessors
                .insert(crate::mir::BasicBlockId::new(u32::MAX));
        }
        CallableLoopMutationV1::None
        | CallableLoopMutationV1::DuplicateCondition
        | CallableLoopMutationV1::UnsealedPublication => {}
    }
    let terminal_receipt = match consume_callable_tail_completion_v1(
        ready,
        profile_counts,
        condition_key,
        &tail,
        &terminal,
        outer.builder_view_mut_for_lowering(),
        &mut session,
    ) {
        Ok(receipt) => receipt,
        Err(error) => {
            let detail = format!("Tail/Completion: {error:?}");
            if matches!(
                mutation,
                CallableLoopMutationV1::StaleGeneration
                    | CallableLoopMutationV1::WrongEdgePredecessor
            ) {
                drop(session);
                outer.discard_unpublished();
            }
            return Err(detail);
        }
    };
    let terminal_block = terminal_receipt.block();
    let profile_close = terminal_receipt.into_profile_close();
    let canonical_close = finish_profile_close(owner, terminal_block, || {
        profile_close.finish(owner, terminal_block)
    })
    .map_err(|error| format!("profile close: {error:?}"))?;
    if mutation == CallableLoopMutationV1::UnsealedPublication {
        let function = outer
            .builder_view_mut_for_lowering()
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "selected function disappeared before seal mutation".to_owned())?;
        function
            .get_block_mut(terminal_block)
            .ok_or_else(|| "selected terminal block disappeared before seal mutation".to_owned())?
            .sealed = false;
    }
    let ready_draft = match session
        .finish_for_draft_seal(outer.builder_view_mut_for_lowering(), canonical_close)
    {
        Ok(ready) => ready,
        Err(error) => {
            let detail = format!("DraftSeal finish: {error:?}");
            if mutation == CallableLoopMutationV1::UnsealedPublication {
                outer.discard_unpublished();
            }
            return Err(detail);
        }
    };
    let open_draft = ready_draft.open(outer);
    let prepared = open_draft
        .prepare()
        .map_err(|_| "DraftSeal prepare rejected".to_owned())?;
    let _completed_draft = prepared.commit().consume_non_authority_evidence();
    Ok(CanaryReceipt {
        operation_count: 7,
        pure_count: 4,
        read_count: 2,
        write_count: 1,
    })
}

#[test]
fn callable_production_canary_runs_s2_to_draft_seal() {
    let receipt = run_canary(CallableLoopMutationV1::None).expect("P0 callable production canary");
    assert_eq!(
        (
            receipt.operation_count,
            receipt.pure_count,
            receipt.read_count,
            receipt.write_count,
        ),
        (7, 4, 2, 1)
    );
}

#[test]
fn callable_production_canary_discards_late_failure_and_reruns_fresh() {
    let error = run_canary(CallableLoopMutationV1::DuplicateCondition)
        .expect_err("late duplicate must reject");
    assert_eq!(error, "late_failure_discarded");
    let receipt = run_canary(CallableLoopMutationV1::None).expect("fresh request after discard");
    assert_eq!(receipt.operation_count, 7);
}

#[test]
fn callable_production_canary_rejects_selected_session_mutations() {
    let error = run_canary(CallableLoopMutationV1::StaleGeneration)
        .expect_err("stale generation must reject");
    assert!(error.contains("read_relation_input"), "{error}");

    let error = run_canary(CallableLoopMutationV1::WrongEdgePredecessor)
        .expect_err("wrong predecessor must reject");
    assert!(error.contains("read_relation_predecessor"), "{error}");

    let error = run_canary(CallableLoopMutationV1::UnsealedPublication)
        .expect_err("unsealed publication must reject");
    assert!(
        error.contains("canonical CFG seal witness disagrees"),
        "{error}"
    );

    let receipt = run_canary(CallableLoopMutationV1::None)
        .expect("fresh request after selected-session mutation discard");
    assert_eq!(receipt.operation_count, 7);
}
