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
                    operation: crate::mir::instruction::InvokeOperation::Call(call),
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
