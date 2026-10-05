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
        .find(|(binding, _, _)| *binding == row.receiver_binding().unwrap())
        .unwrap();
    state
        .install_single_local_for_test(
            pool_stmt,
            row.receiver_binding().unwrap(),
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
        .take_exact_lexical_read(
            owner,
            row.receiver_site().node(),
            row.receiver_binding().unwrap(),
        )
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
            inner_row.receiver_binding().unwrap(),
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

pub(in crate::mir) fn borrowed_fixture(
    package: crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
    exit: &crate::mir::resolved_semantics::SourceStmtSiteV1,
) -> (
    PreparedLexicalCallProjectionV1,
    LexicalInstanceCallDispositionRowV1,
    Box<[LocalCallArgumentV1]>,
    Rc<crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
    Vec<(BasicBlockId, MirInstruction)>,
) {
    let (mut state, locals) = package
        .batch()
        .declarations()
        .find_map(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .function()
                        .method_calls()
                        .find(|(_, call)| call.selector() == "probe")
                        .map(|_| {
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
                                locals,
                            )
                        })
                })
                .unwrap()
        })
        .unwrap();
    let owner = state.owner();
    let mut context = crate::mir::builder::CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .map_err(|(_, error)| error)
        .unwrap()
        .commit();
    let ledger = installed.ordinary_new_claim_ledger();
    let source = ledger
        .borrowed_terminal_arguments_v1(owner, exit)
        .unwrap()
        .unwrap();
    let row = ledger
        .take_borrowed_lexical_call_for_return_v1(owner, exit)
        .unwrap()
        .unwrap();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("main".into());
    let entry = PreparedCallableEntryValuesV1::static_function(&builder, 0).unwrap();
    state.install_entry_values(&entry).unwrap();
    for (i, (binding, statement, ordinal)) in locals.iter().enumerate() {
        state
            .install_single_local_for_test(
                statement,
                *binding,
                *ordinal,
                ValueId(76 + i as u32 * 2),
                ValueId(77 + i as u32 * 2),
            )
            .unwrap();
    }
    let receiver = state
        .take_exact_lexical_read(
            owner,
            row.receiver_site().node(),
            row.receiver_binding().unwrap(),
        )
        .unwrap();
    let mut bindings = Vec::new();
    let prepared =
        super::ordinary_new_admission::selected::terminal_call::prepare_lexical_arguments(
            &mut builder,
            &mut state,
            &ledger,
            owner,
            receiver,
            &source,
            &row,
            &mut bindings,
        )
        .unwrap();
    (prepared, row, source, ledger, bindings)
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

/// Real child Forwarded source/entry/completion; physical fixture, no ABI activation.
pub(in crate::mir) fn forwarded_fixture(
    package: crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
    copy_block: BasicBlockId,
) -> (
    PreparedLexicalCallProjectionV1,
    LexicalInstanceCallDispositionRowV1,
    Box<[LocalCallArgumentV1]>,
    Rc<crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
    crate::mir::resolved_semantics::SourceStmtSiteV1,
    Vec<(BasicBlockId, MirInstruction)>,
) {
    let mut context = super::CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .map_err(|(_, e)| e)
        .unwrap()
        .commit();
    let ledger = installed.ordinary_new_claim_ledger();
    let key = super::SelectedNormalCallableKeyV1::Cataloged(
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
            "Transport",
            "forward",
            1,
        ),
    );
    let (mut state, local_site, receiver_local, call_site) = installed
        .begin_lowering(&context)
        .unwrap()
        .with_selected_lowering_input(&key, |input| {
            let rows = ledger
                .borrowed_ordinary_entry_source_v1(&input)
                .map(|row| row.map(|row| row.formals().into()));
            let local_site = input
                .source()
                .function()
                .expression_source()
                .initializers()
                .find_map(|initializer| match initializer.declaration_site() {
                    SourceBindingSiteV1::Local { statement, .. } => Some(statement.node().clone()),
                    _ => None,
                })
                .unwrap();
            let mut state =
                CallableSemanticLoweringState::from_exact_source(input.source()).unwrap();
            state.lend_ordinary_new_claim_ledger(Rc::clone(&ledger));
            state
                .stage_borrowed_entry_formals(state.owner(), rows)
                .unwrap();
            let receiver_local = input
                .source()
                .function()
                .expression_source()
                .initializers()
                .filter_map(|initializer| match initializer.declaration_site() {
                    SourceBindingSiteV1::Local { statement, ordinal } => {
                        Some((initializer.binding(), statement.node().clone(), *ordinal))
                    }
                    _ => None,
                })
                .nth(1)
                .unwrap();
            let call_site = input
                .source()
                .function()
                .method_calls()
                .find(|(_, call)| call.selector() == "probe")
                .map(|(site, _)| {
                    crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                        input.source().owner(),
                        site.clone(),
                    )
                })
                .unwrap();
            (state, local_site, receiver_local, call_site)
        })
        .unwrap();
    let owner = state.owner();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("Transport/forward/1".into());
    builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .params = vec![ValueId(51), ValueId(72)];
    let entry = PreparedCallableEntryValuesV1::instance_method(&builder, 1).unwrap();
    state.install_entry_values(&entry).unwrap();
    let copy = (
        copy_block,
        MirInstruction::Copy {
            dst: ValueId(73),
            src: ValueId(72),
        },
    );
    let completed = crate::mir::builder::stmts::CompletedLocalStatementV1::from_parts(
        ValueId(73),
        vec![
            crate::mir::builder::stmts::CompletedLocalBindingV1::new(0, ValueId(72), ValueId(73))
                .with_copy(Some(copy.clone()))
                .unwrap(),
        ],
    );
    state
        .record_completed_local(&local_site, &completed)
        .unwrap();
    state
        .install_single_local_for_test(
            &receiver_local.1,
            receiver_local.0,
            receiver_local.2,
            ValueId(76),
            ValueId(77),
        )
        .unwrap();
    let exit = crate::mir::resolved_semantics::SourcePathV1::root_body(3).stmt();
    let source: Box<[LocalCallArgumentV1]> = ledger
        .lexical_i64_call_source(&call_site)
        .unwrap()
        .arguments()
        .to_vec()
        .into();
    let row = ledger
        .take_lexical_instance_call(owner, call_site.site())
        .unwrap()
        .unwrap();
    assert!(matches!(
        ledger.borrowed_call_actuals_v1(&row).unwrap().unwrap()[0].source,
        crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1::Forwarded { .. }
    ));
    let receiver = state
        .take_exact_lexical_read(
            owner,
            row.receiver_site().node(),
            row.receiver_binding().unwrap(),
        )
        .unwrap();
    let mut emitted = Vec::new();
    let prepared =
        super::ordinary_new_admission::selected::terminal_call::prepare_lexical_arguments(
            &mut builder,
            &mut state,
            &ledger,
            owner,
            receiver,
            &source,
            &row,
            &mut emitted,
        )
        .unwrap();
    assert!(
        emitted.is_empty(),
        "alias Copy is a dependency, not a call argument producer"
    );
    (prepared, row, source, ledger, exit, vec![copy])
}
