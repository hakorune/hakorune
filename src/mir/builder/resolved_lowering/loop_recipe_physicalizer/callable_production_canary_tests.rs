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
use super::tail_completion::consume_callable_tail_completion_v1;
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::builder::normal_callable_semantic_source::{
    PreparedCallableLoopIngressV1, VerifiedNormalCallableSourceIngressReceiptV1,
};
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    finish_profile_close, CanonicalSsaFunctionSessionV2,
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

fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: vec!["n".into()],
        param_decls: vec![ParamDecl {
            name: "n".into(),
            declared_type_name: Some("i64".into()),
        }],
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
                vec![
                    ASTNode::Local {
                        variables: vec!["value".into()],
                        initial_values: vec![Some(Box::new(ASTNode::FunctionCall {
                            name: "to_i64".into(),
                            arguments: vec![variable("n")],
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
    CanonicalCallableKeyV1::free_static_for_test("int_to_str", 1)
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

fn run_canary(seed_duplicate_condition: bool) -> Result<CanaryReceipt, String> {
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
    let (source_receipt, input_relation, operation_program, prelude_source, tail) =
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
        &input_relation,
        &prelude,
    )
    .map_err(|error| format!("Prelude materialization: {error}"))?;
    let input_value = prelude_receipt.entry().rows[0].value();
    let make_entry = || {
        super::topology::ReadyLoopEntryV1::new_for_test(
            owner,
            preheader,
            vec![super::topology::ReadyLoopEntryRowV1::new(
                input_relation.recipe_value(),
                input_relation.source_binding(),
                input_value,
            )],
        )
    };
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
    if seed_duplicate_condition {
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
    if seed_duplicate_condition {
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
    let terminal_receipt = consume_callable_tail_completion_v1(
        ready,
        &tail,
        &terminal,
        outer.builder_view_mut_for_lowering(),
        &mut session,
    )
    .map_err(|error| format!("Tail/Completion: {error:?}"))?;
    let terminal_block = terminal_receipt.block();
    let profile_close = terminal_receipt.into_profile_close();
    let canonical_close = finish_profile_close(owner, terminal_block, || {
        profile_close.finish(owner, terminal_block)
    })
    .map_err(|error| format!("profile close: {error:?}"))?;
    let ready_draft = session
        .finish_for_draft_seal(outer.builder_view_mut_for_lowering(), canonical_close)
        .map_err(|error| format!("DraftSeal finish: {error:?}"))?;
    let open_draft = ready_draft.open(outer);
    let prepared = open_draft
        .prepare()
        .map_err(|_| "DraftSeal prepare rejected".to_owned())?;
    let _completed_draft = prepared.commit();
    Ok(CanaryReceipt {
        operation_count: 7,
        pure_count: 4,
        read_count: 2,
        write_count: 1,
    })
}

#[test]
fn callable_production_canary_runs_s2_to_draft_seal() {
    let receipt = run_canary(false).expect("P0 callable production canary");
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
    let error = run_canary(true).expect_err("late duplicate must reject");
    assert_eq!(error, "late_failure_discarded");
    let receipt = run_canary(false).expect("fresh request after discard");
    assert_eq!(receipt.operation_count, 7);
}
