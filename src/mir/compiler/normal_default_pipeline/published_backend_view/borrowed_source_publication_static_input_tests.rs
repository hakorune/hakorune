//! Mutation checks for Static calls closed by the general whole-input cohort.
use super::*;

#[test]
fn scalar_input_comparison_keeps_its_numeric_protocol_through_return() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = r#"static box Main {
  sign(value) {
    if value < 0 { return 3 }
    return 4
  }
  main() { return Main.sign(-5) }
}"#;
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let compiled = MirCompiler::with_options(false)
            .compile_normal_with_published(request(source), |view, verification| {
                require_artifact_verification(verification);
                assert!(view.module().functions.contains_key("Main.sign/1"));
                Ok(())
            })
            .expect("the original numeric argument qualifies for Return");
        assert!(matches!(
            compiled,
            crate::mir::compiler::normal_default_pipeline::NormalPublishedCompileOutcome::Consumed(())
        ));
    });
}

/// The zero-input call and the former mixed cohort (one integer caller and two
/// forwarded formals) now close through the same qualified input. A completed
/// module whose call arguments are changed afterwards must not verify.
#[test]
fn static_input_general_cohort_rejects_forged_call_arguments() {
    use crate::mir::{instruction::InvokeOperation, MirInstruction, MirVerifier, ValueId};

    crate::runtime::ring0::ensure_global_ring0_initialized();
    let text = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/lang/src/hako_alloc/memory/size_class_box.hako"
    ));
    fn call_args<'a>(
        function: &'a mut crate::mir::MirFunction,
        callee: &str,
    ) -> &'a mut Vec<ValueId> {
        function
            .blocks
            .values_mut()
            .filter_map(|block| block.terminator.as_mut())
            .find_map(|terminator| match terminator {
                MirInstruction::Invoke {
                    operation: InvokeOperation::Call { call, .. },
                    ..
                } if format!("{:?}", call.callee).contains(&format!("method: \"{callee}\"")) => {
                    Some(&mut call.args)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("call to {callee}"))
    }
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for optimize in [false, true] {
            let compiled = MirCompiler::with_options(optimize)
                .compile_normal(request(text))
                .expect("size class policy compiles");
            assert!(compiled.verification_result.is_ok(), "{:?}", compiled.verification_result);
            let module = compiled.module;
            for (function, callee, mutation) in [
                ("SizeClassBox.size_to_bin/1", "max_regular_bin", "zero-input call gains an argument"),
                ("SizeClassBox.size_to_bin_usize/1", "size_to_bin", "integer caller passes another defined value"),
                ("SizeClassBox.good_size/1", "size_to_bin", "forwarding caller passes another defined value"),
                ("SizeClassBox.good_size/1", "size_to_bin", "forwarding caller passes no argument"),
            ] {
                let mut forged = module.clone();
                let target = forged.functions.get_mut(function).expect(function);
                let entry = target.params[0];
                let args = call_args(target, callee);
                match mutation {
                    "zero-input call gains an argument" => {
                        assert!(args.is_empty());
                        args.push(entry);
                    }
                    "integer caller passes another defined value" | "forwarding caller passes another defined value" => {
                        assert_eq!(args.as_slice(), &[entry], "{function}: original argument is the entry formal");
                        args[0] = ValueId(entry.0 + 1);
                    }
                    "forwarding caller passes no argument" => args.clear(),
                    _ => unreachable!(),
                }
                let errors = MirVerifier::new_strict()
                    .verify_module(&forged)
                    .expect_err(mutation);
                assert!(!errors.is_empty(), "{mutation}");
            }
        }
    });
}
