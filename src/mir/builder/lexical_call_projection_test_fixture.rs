//! Build real source-issued projection packets without exposing builder state.
use super::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
use crate::mir::normal_callable_semantic_package::{
    EmittedLexicalCallProjectionV1, LexicalCallArgumentProjectionV1,
    LexicalInstanceCallDispositionRowV1, PreparedLexicalCallProjectionV1,
};
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
use crate::mir::resolved_semantics::SourceBindingSiteV1;
use crate::mir::{BasicBlockId, MirBuilder, MirInstruction, ValueId};
use std::rc::Rc;

pub(in crate::mir) fn fixture() -> (
    PreparedLexicalCallProjectionV1,
    LexicalInstanceCallDispositionRowV1,
    Vec<LocalCallArgumentV1>,
) {
    let text = "box Pool { birth() {} pair(a: i64, b: i64): i64 { return a } wrap(x: i64): i64 { return x } } static box Main { main() { local pool = new Pool() local n = 8 local r = pool.pair(n, pool.wrap(9)) return 0 } }";
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        text,
        crate::parser::ParserBuildConfig::default(),
    )
    .unwrap();
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("source-backed fixture");
    };
    let brands = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(source.ast()).unwrap();
    let source = crate::mir::builder::NormalRootExecutionConsumerV1::consume_once(source)
        .unwrap()
        .into_consumed_source();
    let mut resolver =
        crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1::new(993).unwrap();
    let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_brand_catalog_v1(&mut resolver, source, Some(&brands)).unwrap();
    let (mut state, call, locals) = package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .function()
                        .method_calls()
                        .find(|(_, call)| call.selector() == "pair")
                        .map(|(site, _)| {
                            let locals = input
                                .function()
                                .expression_source()
                                .initializers()
                                .filter_map(|initializer| {
                                    let SourceBindingSiteV1::Local { statement, ordinal } =
                                        initializer.declaration_site()
                                    else {
                                        return None;
                                    };
                                    Some((
                                        initializer.binding(),
                                        statement.node().clone(),
                                        *ordinal,
                                    ))
                                })
                                .collect::<Vec<_>>();
                            (
                                CallableSemanticLoweringState::from_exact_source(input).unwrap(),
                                crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                                    input.owner(),
                                    site.clone(),
                                ),
                                locals,
                            )
                        })
                })
                .unwrap()
        })
        .expect("fixture's exact pair source loan");
    let mut context = crate::mir::builder::CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .map_err(|(_, error)| error)
        .unwrap()
        .commit();
    let ledger = installed.ordinary_new_claim_ledger();
    let owner = state.owner();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("main".into());
    let entry = PreparedCallableEntryValuesV1::static_function(&builder, 0).unwrap();
    state.install_entry_values(&entry).unwrap();
    let source = ledger
        .lexical_i64_call_source(&call)
        .unwrap()
        .arguments()
        .to_vec();
    let row = ledger
        .take_lexical_instance_call(owner, call.site())
        .unwrap()
        .unwrap();
    let (_, pool_stmt, pool_ordinal) = locals
        .iter()
        .find(|(binding, _, _)| *binding == row.receiver_binding())
        .unwrap();
    state
        .install_single_local_for_test(
            pool_stmt,
            row.receiver_binding(),
            *pool_ordinal,
            ValueId(76),
            ValueId(77),
        )
        .unwrap();
    let LocalCallArgumentV1::Scalar(binding) = source[0] else {
        panic!("scalar source");
    };
    let (_, n_stmt, n_ordinal) = locals
        .iter()
        .find(|(actual, _, _)| *actual == binding)
        .unwrap();
    state
        .install_single_local_for_test(n_stmt, binding, *n_ordinal, ValueId(78), ValueId(79))
        .unwrap();
    let receiver = state
        .take_exact_lexical_read(owner, row.receiver_site().node(), row.receiver_binding())
        .unwrap();
    let scalar = state
        .take_exact_lexical_read(owner, row.argument_sites()[0].node(), binding)
        .unwrap();
    let LocalCallArgumentV1::CallResult(inner) = &source[1] else {
        panic!("inner source");
    };
    let inner_row = ledger
        .take_lexical_instance_call(owner, inner.site().site())
        .unwrap()
        .unwrap();
    let inner_receiver = state
        .take_exact_lexical_read(
            owner,
            inner_row.receiver_site().node(),
            inner_row.receiver_binding(),
        )
        .unwrap();
    let inner_prepared = PreparedLexicalCallProjectionV1::new(
        inner_receiver,
        vec![LexicalCallArgumentProjectionV1::Integer((
            BasicBlockId(1),
            MirInstruction::Const {
                dst: ValueId(80),
                value: crate::mir::ConstValue::Integer(9),
            },
        ))],
    );
    let inner_call = inner_prepared
        .materialize(owner, &inner_row, inner.arguments())
        .unwrap();
    let emitted = EmittedLexicalCallProjectionV1::new(
        inner_row,
        inner_prepared,
        (
            BasicBlockId(1),
            MirInstruction::Invoke {
                operation: InvokeOperation::Call {
                    call: inner_call,
                    result: InvokeCallResultKind::I64,
                },
                fault_frame: ValueId(90),
                normal_landing: BasicBlockId(2),
                fault_landing: BasicBlockId(3),
            },
        ),
        (
            BasicBlockId(2),
            MirInstruction::InvokeNormalResult {
                invoke_block: BasicBlockId(1),
                dst: ValueId(81),
            },
        ),
    );
    (
        PreparedLexicalCallProjectionV1::new(
            receiver,
            vec![
                LexicalCallArgumentProjectionV1::Scalar(scalar),
                LexicalCallArgumentProjectionV1::CallResult(Box::new(emitted)),
            ],
        ),
        row,
        source,
    )
}

pub(in crate::mir) fn artifact_fixture() -> (
    crate::mir::MirModule,
    crate::mir::finalized_root_handoff::FinalizedRootHandoffV1,
) {
    artifact_fixture_checked(false, false)
}

pub(in crate::mir) fn assert_artifact_reseal_rejected() {
    let _ = artifact_fixture_checked(true, false);
}

pub(in crate::mir) fn finished_artifact_fixture() -> (
    crate::mir::MirModule,
    crate::mir::finalized_root_handoff::FinalizedRootHandoffV1,
) {
    artifact_fixture_checked(false, true)
}

fn artifact_fixture_checked(
    check_reseal: bool,
    simplify: bool,
) -> (
    crate::mir::MirModule,
    crate::mir::finalized_root_handoff::FinalizedRootHandoffV1,
) {
    use crate::mir::builder::{
        BuilderInvocationConfigV1, CallableMainMaterializationPolicyV1,
        ModuleBuilderInvocationSessionV1, NormalRuntimeInputSnapshotV1,
        PreparedNormalDefaultProgramRootV1,
    };
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let text = if simplify {
        "box Pool { birth() {} wrap(x: i64): i64 { return x } give(x: i64): i64 { return x } child(): i64 { local peer = new Pool() local r = peer.give(peer.wrap(9)) return 0 } } static box Main { main() { local pool = new Pool() local r = pool.give(pool.wrap(9)) local c = pool.child() return 0 } }"
    } else {
        "box Pool { birth() {} wrap(x: i64): i64 { return x } give(x: i64): i64 { return x } } static box Main { main() { local pool = new Pool() local r = pool.give(pool.wrap(9)) return 0 } }"
    };
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        text,
        crate::parser::ParserBuildConfig::default(),
    )
    .unwrap();
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("source-backed fixture")
    };
    let builder = crate::mir::MirBuilder::new();
    let session = ModuleBuilderInvocationSessionV1::open(
        &builder,
        BuilderInvocationConfigV1::snapshot_for_raw(&builder, None),
    );
    let completed = session
        .complete_normal_default_program_root_catalog_lifecycle(
            PreparedNormalDefaultProgramRootV1::from_callable_source(source),
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .unwrap();
    let (root_key, ledger) = completed.ordinary_root_ledger_for_test();
    let original = Rc::downgrade(&ledger);
    let (_, mut module, validate) = completed.into_artifact_parts();
    if simplify {
        crate::mir::passes::simplify_cfg::simplify(&mut module);
    }
    let handoff = validate(&module).unwrap().unwrap();
    drop(ledger);
    assert!(original.upgrade().is_some(), "original ledger retained");
    let ledger = original.upgrade().unwrap();
    if check_reseal {
        assert_eq!(
            handoff
                .root_source()
                .unwrap()
                .local_call_binding_groups()
                .count(),
            1
        );
        let second = ledger
            .seal_finalized_root_birth_handoff(root_key, &std::collections::BTreeSet::new(), None)
            .unwrap_err();
        assert!(
            second.contains("artifact-root-already-finalized"),
            "{second}"
        );
    }
    (module, handoff)
}
