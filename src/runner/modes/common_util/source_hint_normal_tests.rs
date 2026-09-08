//! Selected preparation retains parser-owned declarations through real text merge.
use super::*;
use crate::runner::modes::common_util::normal_callable::{
    materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
};

#[test]
fn normal_preparation_preserves_local_with_and_without_prelude() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_vars(
        &[
            ("NYASH_ALLOW_USING_FILE", Some("1")),
            ("NYASH_ENABLE_USING", Some("1")),
            ("NYASH_OPERATOR_BOX_ALL", Some("1")),
        ],
        || {
            let runner = NyashRunner::new(Default::default());
            let directory = tempfile::tempdir().unwrap();
            let prelude = directory.path().join("prelude.hako");
            let nested = directory.path().join("nested.hako");
            std::fs::write(&nested, "static box Nested { get() { return 1 } }\n").unwrap();
            std::fs::write(
                &prelude,
                format!("using \"{}\" as NestedAlias\nstatic box Helper {{\nget() {{\nlocal n: i64 = 7\nreturn n\n}}\n}}\n", nested.display()),
            )
            .unwrap();
            for imported in [false, true] {
                let mut source = String::new();
                if imported {
                    source.push_str(&format!("using \"{}\"\n", prelude.display()));
                }
                source.push_str("local a: Array<i8> = [7]\nlocal b = 9\nreturn 30\n");
                let filename = directory.path().join("main.hako");
                let prepared = prepare_normal_source_with_imports(
                    &runner,
                    filename.to_str().unwrap(),
                    &source,
                )
                .unwrap();
                assert!(prepared.code.contains("local a: Array<i8> = [7]"));
                assert!(prepared.code.contains("local b = 9"));
                if imported {
                    assert!(prepared.code.contains("local n: i64 = 7"));
                    assert!(prepared.code.contains("static box Nested"));
                    assert_eq!(
                        prepared.imports.get("NestedAlias").map(String::as_str),
                        Some("Nested")
                    );
                } else {
                    assert_eq!(prepared.code, source);
                }
                assert!(!prepared.code.contains("static box StringifyOperator"));
                assert!(!prepared.code.contains("static box CompareOperator"));
                assert!(!prepared.code.contains("static box AddOperator"));
                let NormalCallableMaterializationOutcomeV1::SourceBacked(product) =
                    materialize_normal_callable_program_v1(&prepared.code, Default::default())
                        .unwrap()
                else {
                    panic!("source-backed");
                };
                if imported {
                    let result = crate::mir::MirCompiler::with_options(true)
                        .compile_normal_with_published(
                            crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(
                                product,
                                None,
                                prepared.imports,
                            ),
                            |_, _| -> Result<(), String> {
                                panic!("explicit static prefix remains stopped")
                            },
                        );
                    assert!(result
                        .err().unwrap()
                        .to_string()
                        .contains("caller-prefix-capability"));
                }
                let legacy =
                    prepare_source_with_imports(&runner, filename.to_str().unwrap(), &source)
                        .unwrap();
                assert!(legacy.code.contains("static box AddOperator"));
            }
            let malformed = prepare_normal_source_with_imports(
                &runner,
                "broken.hako",
                "local a: = []\nreturn 30\n",
            )
            .unwrap();
            assert!(malformed.code.contains("local a:"));
            assert!(
                materialize_normal_callable_program_v1(malformed.code, Default::default()).is_err()
            );
        },
    );
}
