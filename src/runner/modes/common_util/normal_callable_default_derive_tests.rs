use super::*;
use crate::mir::{MirCompiler, NormalCompileRequestV1};

fn enabled<R>(run: impl FnOnce() -> R) -> R {
    crate::test_support::with_env_vars(
        &[
            ("NYASH_MACRO_DISABLE", Some("0")),
            ("NYASH_MACRO_ENABLE", Some("1")),
            ("NYASH_MACRO_DERIVE", None),
            ("NYASH_MACRO_DERIVE_ALL", None),
            ("NYASH_MACRO_PATHS", None),
            ("NYASH_TEST_RUN", Some("0")),
            ("NYASH_TEST_ARGS_JSON", None),
        ],
        run,
    )
}

#[test]
fn default_pair_source_publishes_both_generated_methods_with_parameter_coverage() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    enabled(|| {
        let source = materialize_normal_callable_program_v1(
            include_str!("../../../../apps/typed-object-birth-min/main.hako"),
            ParserBuildConfig::default(),
        )
        .unwrap();
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = source else {
            panic!("source")
        };
        source
            .with_callable_semantic_syntax(|loan| {
                let mut generated = 0;
                for row in loan.rows() {
                    let crate::ast::ASTNode::FunctionDeclaration { name, .. } = row.declaration()
                    else {
                        panic!("declaration")
                    };
                    match name.as_str() {
                        "equals" => {
                            let params = row.parameters().expect("issued generated parameters");
                            assert_eq!(params.len(), 1);
                            assert_eq!(params[0].name(), "__ny_other");
                            assert_eq!(params[0].declared_type_name(), None);
                            generated += 1;
                        }
                        "toString" => {
                            assert!(row.parameters().expect("explicit zero coverage").is_empty());
                            generated += 1;
                        }
                        _ => {}
                    }
                }
                assert_eq!(generated, 2);
            })
            .unwrap();
        MirCompiler::with_options(true)
            .compile_normal_with_published(
                NormalCompileRequestV1::for_mir_mode_callable_source(
                    source,
                    None,
                    Default::default(),
                ),
                |view, _| -> Result<(), String> {
                    assert!(view.module().functions.contains_key("Pair.equals/1"));
                    assert!(view.module().functions.contains_key("Pair.toString/0"));
                    assert!(view.module().functions.contains_key("Pair.birth/2"));
                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn early_derive_snapshot_does_not_read_later_settings_or_change_ast_only_parser() {
    let (parsed, policy) = enabled(|| {
        let syntax = "box Plain {}";
        let raw = NyashParser::parse_from_string(syntax).unwrap();
        let crate::ast::ASTNode::Program { statements, .. } = raw else {
            panic!("program")
        };
        let crate::ast::ASTNode::BoxDeclaration { methods, .. } = &statements[0] else {
            panic!("box")
        };
        assert!(methods.get_declaration("equals").is_none());
        let policy = NormalMacroPolicyV1::capture();
        let parsed =
            crate::parser::string_postpass_entry::parse_with_callable_parameter_source_policy(
                syntax.into(),
                Some(100_000),
                ParserBuildConfig::default(),
                Some(&policy),
            )
            .unwrap()
            .into_normal_callable_program()
            .unwrap();
        (parsed, policy)
    });
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        let result = transform_normal_callable_program_with_policy_v1(parsed, policy).unwrap();
        let NormalCallableTransformOutcomeV1::SourceBacked(source) = result else {
            panic!("source")
        };
        source
            .with_callable_semantic_syntax(|loan| assert_eq!(loan.rows().len(), 2))
            .unwrap();
        source.discard_at_named_root_execution_terminal();
    });
}

#[test]
fn default_derive_explicit_method_precedence_and_static_omission() {
    enabled(|| {
        let result = materialize_normal_callable_program_v1(
            "box Plain { equals(other) { return false } } static box Main { main() { return 0 } }",
            ParserBuildConfig::default(),
        )
        .unwrap();
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = result else {
            panic!("source")
        };
        source.with_callable_semantic_syntax(|loan| {
            let names: Vec<_> = loan.rows().iter().map(|row| {
                let crate::ast::ASTNode::FunctionDeclaration { name, .. } = row.declaration() else { panic!("method") };
                name.as_str()
            }).collect();
            assert_eq!(names.iter().filter(|name| **name == "equals").count(), 1);
            assert_eq!(names.iter().filter(|name| **name == "toString").count(), 1);
            assert_eq!(names.len(), 3);
            let equals = loan.rows().iter().find(|row| matches!(row.declaration(), crate::ast::ASTNode::FunctionDeclaration {name, ..} if name == "equals")).unwrap();
            assert_eq!(equals.parameters().unwrap()[0].name(), "other");
        }).unwrap();
        source.discard_at_named_root_execution_terminal();
    });
}

#[test]
fn default_derive_disabled_normal_policy_keeps_no_generated_methods() {
    crate::test_support::with_env_vars(&[("NYASH_MACRO_DISABLE", Some("1"))], || {
        let result =
            materialize_normal_callable_program_v1("box Plain {}", ParserBuildConfig::default())
                .unwrap();
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = result else {
            panic!("source")
        };
        source
            .with_callable_semantic_syntax(|loan| assert!(loan.rows().is_empty()))
            .unwrap();
        source.discard_at_named_root_execution_terminal();
    });
}

#[test]
fn default_derive_public_fields_stop_at_source_contract() {
    enabled(|| {
        let result = materialize_normal_callable_program_v1(
            "box Plain { public { value } }",
            ParserBuildConfig::default(),
        );
        assert!(
            matches!(result, Err(NormalCallableMaterializationErrorV1::Parse(ParseError::GrammarContract { stable_reject_tag: "parser/default-derive-source", ref detail, .. })) if detail == "dynamic-field-text-contract-missing")
        );
    });
}

#[test]
fn default_derive_record_keeps_single_compatibility_expansion() {
    enabled(|| {
        let policy = NormalMacroPolicyV1::capture();
        let parsed =
            crate::parser::string_postpass_entry::parse_with_callable_parameter_source_policy(
                "record Pair { left: i64, right: i64 }".into(),
                Some(100_000),
                ParserBuildConfig::default(),
                Some(&policy),
            )
            .unwrap()
            .into_normal_callable_program()
            .unwrap();
        let NormalCallableTransformOutcomeV1::Compatibility { ast, .. } =
            transform_normal_callable_program_with_policy_v1(parsed, policy).unwrap()
        else {
            panic!("compatibility")
        };
        let crate::ast::ASTNode::Program { statements, .. } = ast else {
            panic!("program")
        };
        let crate::ast::ASTNode::BoxDeclaration { methods, .. } = &statements[0] else {
            panic!("record")
        };
        assert_eq!(
            methods
                .iter_selected_declaration_order()
                .filter(|entry| entry.name() == "equals")
                .count(),
            1
        );
        assert_eq!(
            methods
                .iter_selected_declaration_order()
                .filter(|entry| entry.name() == "toString")
                .count(),
            1
        );
    });
}
