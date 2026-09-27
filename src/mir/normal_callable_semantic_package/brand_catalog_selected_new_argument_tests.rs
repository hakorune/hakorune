#[test]
fn selected_new_arguments_reach_birth_in_issued_order() {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            "box Page { birth(integer, boolean, local_value) { } }
             static box Main { main() {
                 local local_value = 7
                 local page = new Page(11, true, local_value)
                 return 0
             } }",
            ParserBuildConfig::default(),
        )
        .unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
        else {
            panic!("source authority lost");
        };
        let result = crate::mir::MirCompiler::with_options(false)
            .compile_normal(
                crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(
                    source,
                    None,
                    Default::default(),
                ),
            )
            .expect("selected New arguments must reach Birth");
        assert!(result.verification_result.is_ok());
        let main = result.module.get_function("main").unwrap();
        let calls: Vec<_> = main
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter_map(|instruction| match instruction {
                crate::mir::MirInstruction::Invoke {
                    operation: crate::mir::instruction::InvokeOperation::Call { call, result: InvokeCallResultKind::Unit },
                    ..
                } if matches!(call.callee, crate::mir::Callee::BirthConstructor { .. }) => {
                    Some(call)
                }
                _ => None,
            })
            .collect();
        let [call] = calls.as_slice() else {
            panic!("expected exactly one Birth Call");
        };
        let [integer, boolean, local_value] = call.args.as_slice() else {
            panic!("Birth Call argument arity drifted");
        };
        let constants: Vec<_> = main
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter_map(|instruction| match instruction {
                crate::mir::MirInstruction::Const { dst, value } => Some((*dst, value)),
                _ => None,
            })
            .collect();
        assert!(constants.iter().any(|(dst, value)| {
            *dst == *integer && matches!(value, crate::mir::ConstValue::Integer(11))
        }));
        assert!(constants.iter().any(|(dst, value)| {
            *dst == *boolean && matches!(value, crate::mir::ConstValue::Bool(true))
        }));
        assert!(main
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .any(|instruction| {
                matches!(instruction, crate::mir::MirInstruction::Copy { dst, src }
                if *dst == *local_value
                    && constants.iter().any(|(constant, value)| {
                        *constant == *src
                            && matches!(value, crate::mir::ConstValue::Integer(7))
                    }))
            }));
        assert_eq!(
            main.root_ordinary_new_observation(),
            crate::mir::function::RootOrdinaryNewObservation::SourceCompleteAtFinalization
        );
    });
}

#[test]
fn selected_new_arguments_admit_inventoried_call_result_local() {
    use crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1;
    let package = issue_with_brand_catalog(
        "static box Sizes { size(v) { return v } } \
         box Page { birth(v) { } } \
         static box Main { build() { local h = Sizes.size(7)\nlocal p = new Page(h)\nreturn 0 } main() { return 0 } }",
    )
    .expect("inventoried call-result local is an admissible New argument");
    let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = claim_rows.values().collect();
    assert_eq!(claims.len(), 1);
    let rows = claims[0]
        .argument_rows()
        .expect("BoundValue provenance keeps argument rows complete");
    let [row] = rows else {
        panic!("one argument row, got {rows:?}")
    };
    let OrdinaryNewTrivialArgumentKindV1::BoundValue { binding } = row.kind() else {
        panic!(
            "call-result local must be a BoundValue row, got {:?}",
            row.kind()
        )
    };
    // The carried binding is exactly `h`'s declaration binding — the one
    // other local initializer in `build`, not the `new` site itself.
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.owner() == claims[0].site().owner())
        .expect("exact owner");
    let h_binding = package
        .batch()
        .with_lowering_input(declaration.batch_slot(), |input| {
            input
                .function()
                .expression_source()
                .initializers()
                .find(|row| row.initializer_site() != Some(claims[0].site().site()))
                .expect("the call-result local initializer")
                .binding()
        })
        .expect("lowering input");
    assert_eq!(*binding, h_binding);
}

#[test]
fn selected_new_rejects_unbound_and_call_expression_arguments() {
    use crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1;
    for body in [
        "local x = 1 + 2\nlocal p = new Page(x)\nreturn 0",
        "local p = new Page(Sizes.size(7))\nreturn 0",
    ] {
        let source = format!(
            "static box Sizes {{ size(v) {{ return v }} }} \
             box Page {{ birth(v) {{ }} }} \
             static box Main {{ build() {{ {body} }} main() {{ return 0 }} }}"
        );
        let package =
            issue_with_brand_catalog(&source).expect("non-inventoried argument stays unproven");
        let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
        let claims: Vec<_> = claim_rows.values().collect();
        assert_eq!(claims.len(), 1);
        let error = claims[0]
            .argument_rows()
            .expect_err("argument rows stay unavailable outside BoundValue scope");
        assert!(
            matches!(error, SelectedNewArgumentUnavailableV1::ArgumentNotTrivial { .. }),
            "unexpected unavailable reason: {error:?}"
        );
    }
}

#[test]
fn selected_new_rejects_nontrivial_argument_before_raw_descent() {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            "box Page { birth(value) { } }
             static box Main { main() {
                 local page = new Page(1 + 2)
                 return 0
             } }",
            ParserBuildConfig::default(),
        )
        .unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
        else {
            panic!("source authority lost");
        };
        let error = crate::mir::MirCompiler::with_options(false)
            .compile_normal(
                crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(
                    source,
                    None,
                    Default::default(),
                ),
            )
            .expect_err("nontrivial selected New argument must stop before raw descent");
        assert!(
            error.contains("ordinary-new/argument-source-unavailable"),
            "unexpected error: {error}"
        );
    });
}
