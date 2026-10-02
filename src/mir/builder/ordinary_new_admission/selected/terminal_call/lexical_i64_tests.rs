//! Corroborate physical arguments against real source-issued call rows.
use super::*;
use crate::mir::builder::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use crate::mir::resolved_semantics::SourceBindingSiteV1;

fn fixture() -> (
    PreparedCall,
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
    let inner_prepared = PreparedCall {
        receiver: inner_receiver,
        arguments: vec![ArgumentProjection::Integer((
            BasicBlockId(1),
            MirInstruction::Const {
                dst: ValueId(80),
                value: crate::mir::ConstValue::Integer(9),
            },
        ))],
    };
    let inner_call = inner_prepared
        .materialize(owner, &inner_row, inner.arguments())
        .unwrap();
    let emitted = EmittedCall {
        row: inner_row,
        prepared: inner_prepared,
        invoke: (
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
        projection: (
            BasicBlockId(2),
            MirInstruction::InvokeNormalResult {
                invoke_block: BasicBlockId(1),
                dst: ValueId(81),
            },
        ),
    };
    (
        PreparedCall {
            receiver,
            arguments: vec![
                ArgumentProjection::Scalar(scalar),
                ArgumentProjection::CallResult(Box::new(emitted)),
            ],
        },
        row,
        source,
    )
}

#[test]
fn ordered_projection_uses_exact_scalar_and_original_nested_result() {
    let (prepared, row, source) = fixture();
    let call = prepared
        .materialize(row.call_site().owner(), &row, &source)
        .unwrap();
    assert_eq!(call.args, vec![ValueId(79), ValueId(81)]);
    assert!(matches!(
        call.callee,
        crate::mir::definitions::Callee::SameModuleInstance {
            receiver: ValueId(77),
            ..
        }
    ));
    assert_eq!(
        prepared.arguments.len(),
        2,
        "arity follows ordinals, not recorded instruction count"
    );
}

#[test]
fn ordered_projection_refuses_swapped_arguments_and_foreign_receiver_read() {
    let (mut prepared, row, source) = fixture();
    let owner = row.call_site().owner();
    prepared.arguments.swap(0, 1);
    assert!(prepared
        .materialize(owner, &row, &source)
        .unwrap_err()
        .contains("argument-projection-drift"));
    prepared.arguments.swap(0, 1);
    let ArgumentProjection::Scalar(scalar) = prepared.arguments.remove(0) else {
        panic!("scalar");
    };
    prepared.arguments.insert(
        0,
        ArgumentProjection::Scalar(std::mem::replace(&mut prepared.receiver, scalar)),
    );
    assert!(prepared
        .materialize(owner, &row, &source)
        .unwrap_err()
        .contains("receiver"));
}

#[test]
fn ordered_projection_refuses_nested_target_landing_and_literal_drift() {
    for mutation in 0..3 {
        let (mut prepared, row, source) = fixture();
        let ArgumentProjection::CallResult(inner) = &mut prepared.arguments[1] else {
            panic!("nested");
        };
        match mutation {
            0 => {
                let MirInstruction::Invoke {
                    operation: InvokeOperation::Call { call, .. },
                    ..
                } = &mut inner.invoke.1
                else {
                    panic!("invoke");
                };
                call.args[0] = ValueId(999);
            }
            1 => inner.projection.0 = BasicBlockId(999),
            _ => {
                let ArgumentProjection::Integer((_, MirInstruction::Const { value, .. })) =
                    &mut inner.prepared.arguments[0]
                else {
                    panic!("constant");
                };
                *value = crate::mir::ConstValue::Integer(10);
            }
        }
        assert!(prepared
            .materialize(row.call_site().owner(), &row, &source)
            .is_err());
    }
}
