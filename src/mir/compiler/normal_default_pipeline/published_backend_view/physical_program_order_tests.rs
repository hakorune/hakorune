use super::*;
use crate::mir::compiler::normal_default_pipeline::{MirCompiler, NormalCompileRequestV1};
use crate::parser::NyashParser;
use std::collections::HashMap;

fn request(source: &str) -> NormalCompileRequestV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        crate::parser::ParserBuildConfig::default(),
    )
    .expect("exact callable parse");
    let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
        .expect("exact callable transform");
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("source identity must remain intact");
    };
    NormalCompileRequestV1::for_mir_mode_callable_source(source, None, HashMap::new())
}

#[test]
fn ordinary_calls_follow_sorted_block_order() {
    let mut function = MirFunction::new(
        crate::mir::FunctionSignature {
            name: "Main.main".into(),
            params: vec![],
            return_type: crate::mir::MirType::Integer,
            effects: crate::mir::EffectMask::CONTROL,
        },
        BasicBlockId(0),
    );
    for (block_id, method) in [(BasicBlockId(2), "zeta"), (BasicBlockId(1), "alpha")] {
        let target = hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
            "Worker".into(),
            method.into(),
            0,
        )
        .unwrap();
        let call = MirCall::new(None, Callee::Global(target), vec![]);
        let instruction = MirInstruction::Invoke {
            operation: InvokeOperation::Call {
                call,
                result: InvokeCallResultKind::I64,
            },
            fault_frame: ValueId(0),
            normal_landing: BasicBlockId(0),
            fault_landing: BasicBlockId(0),
        };
        let mut block = crate::mir::BasicBlock::new(block_id);
        block.add_instruction(instruction);
        function.add_block(block);
    }
    let keys = super::collect_ordinary_calls(&function)
        .unwrap()
        .into_iter()
        .map(|call| super::ordinary_callable_key(&call.callee).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        keys,
        vec![
            hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method(
                "Worker", "alpha", 0
            ),
            hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method(
                "Worker", "zeta", 0
            ),
        ]
    );
}

#[test]
fn repeated_physical_compiles_retain_function_and_diagnostic_order() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let source = r#"
static box Main {
  main() {
    local first = helper(10)
    local second = middle(20)
    local third = later(1)
    local root_map = %{"root" => 2}
    return other(30)
  }
  helper(value: i64): i64 { local m = %{"first" => value} return 30 }
  middle(value: i64): i64 { local m = %{"middle" => value} return 30 }
  later(value: i64): i64 { local m = %{"later" => value} return 30 }
  other(value: i64): i64 { local m = %{"other" => value} return 30 }
}
"#;
        let mut snapshots = Vec::new();
        for _ in 0..2 {
            let mut compiler = MirCompiler::with_options(false);
            let mut snapshot = None;
            let result = compiler.compile_normal_with_published(
                request(source),
                |view, _| -> Result<(), String> {
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let functions = input
                        .program()
                        .functions()
                        .iter()
                        .map(|function| {
                            (
                                function.name().to_owned(),
                                function.role().wire_name().to_owned(),
                            )
                        })
                        .collect::<Vec<_>>();
                    let diagnostics = input
                        .diagnostic_sites()
                        .iter()
                        .map(|site| {
                            (
                                site.function(),
                                site.block(),
                                site.instruction(),
                                site.kind(),
                                site.site(),
                            )
                        })
                        .collect::<Vec<_>>();
                    snapshot = Some((functions, diagnostics));
                    Err("[freeze:contract][published-lifecycle/consumer-pending]".into())
                },
            );
            match result {
                Err(error) if error.contains("consumer-pending") => {}
                Err(error) => panic!("unexpected selected consumer error: {error}"),
                Ok(_) => panic!("selected consumer must remain pending"),
            }
            snapshots.push(snapshot.expect("physical snapshot"));
        }
        assert_eq!(snapshots[0], snapshots[1]);
        assert_eq!(snapshots[0].0.len(), 5, "root plus four ordinary callees");
        assert!(snapshots[0].0[1..]
            .iter()
            .all(|(_, role)| role == "ordinary_i64"));
    });
}
