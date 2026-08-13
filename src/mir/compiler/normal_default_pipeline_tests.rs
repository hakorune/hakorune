use super::*;
use crate::ast::{LiteralValue, Span};
use crate::mir::MirPrinter;
use crate::parser::NyashParser;

fn program() -> ASTNode {
    ASTNode::Program {
        statements: Vec::new(),
        span: Span::unknown(),
    }
}

fn non_program() -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(1),
        span: Span::unknown(),
    }
}

#[test]
fn callable_source_request_retains_atomic_final_program_owner() {
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            "static box Scan { run(x) { return x } }",
            crate::parser::ParserBuildConfig::default(),
        )
        .expect("callable-aware parse");
        let transformed =
            crate::r#macro::transform_normal_callable_program_v1(parsed).expect("exact transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("static exact source must remain source-backed")
        };
        let request = NormalCompileRequestV1::for_mir_mode_callable_source(
            source,
            Some("scan.hako"),
            HashMap::new(),
        );
        let (program, source, imports, admission, _) = request.into_parts();
        assert!(program.is_callable_source_backed());
        assert_eq!(source.source_file(), Some("scan.hako"));
        assert!(imports.is_empty());
        assert_eq!(
            admission,
            NormalCompileAdmissionV1::PreparedSourceWithImports(
                NormalPreparedSourceCallerV1::MirMode
            )
        );
    });
}

#[test]
fn llvm_callable_source_request_keeps_llvm_caller_identity() {
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            "static box Scan { run(x) { return x } }",
            crate::parser::ParserBuildConfig::default(),
        )
        .expect("callable-aware parse");
        let transformed =
            crate::r#macro::transform_normal_callable_program_v1(parsed).expect("exact transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("static exact source must remain source-backed")
        };
        let request = NormalCompileRequestV1::for_llvm_callable_source(
            source,
            Some("scan.hako"),
            HashMap::new(),
        );
        let (program, source, imports, admission, _) = request.into_parts();
        assert!(program.is_callable_source_backed());
        assert_eq!(source.source_file(), Some("scan.hako"));
        assert!(imports.is_empty());
        assert_eq!(
            admission,
            NormalCompileAdmissionV1::PreparedSourceWithImports(
                NormalPreparedSourceCallerV1::LlvmSourceCompiler
            )
        );
    });
}

#[test]
fn normal_ingress_materializes_required_callable_main_without_changing_script() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_BUILD_STATIC_MAIN_ENTRY", "1", || {
        let app = NyashParser::parse_from_string(
            "static box Main { helper() { return 1 } main(p0) { return p0 } }",
        )
        .expect("App source");
        let script = program();
        let mut compiler = MirCompiler::with_options(false);

        let app_result = compiler
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(app, None, HashMap::new())
                    .expect("App request"),
            )
            .expect("App compile");
        assert!(app_result.module.functions.contains_key("Main.helper/0"));
        assert!(app_result.module.functions.contains_key("Main.main/1"));
        assert!(app_result.module.functions.contains_key("main"));

        let script_result = compiler
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(script, None, HashMap::new())
                    .expect("Script request"),
            )
            .expect("Script compile");
        assert!(script_result.module.functions.contains_key("main"));
        assert!(!script_result.module.functions.contains_key("Main.main/0"));
    });
}

#[test]
fn normal_ingress_snapshots_runtime_inputs_permissively_at_compile_time() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = "static box Main { main(args) { return 0 } }";
    let request = crate::test_support::with_env_vars(
        &[
            ("NYASH_SCRIPT_ARGS_JSON", Some(r#"["request-time"]"#)),
            ("HAKO_SCRIPT_ARGS_JSON", None),
            ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some("0")),
        ],
        || {
            let ast = NyashParser::parse_from_string(source).expect("App source");
            NormalCompileRequestV1::for_mir_mode(ast, None, HashMap::new()).expect("App request")
        },
    );
    let selected = crate::test_support::with_env_vars(
        &[
            ("NYASH_SCRIPT_ARGS_JSON", Some(r#"["ingress-time"]"#)),
            ("HAKO_SCRIPT_ARGS_JSON", Some(r#"["must-not-win"]"#)),
            ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some("On")),
        ],
        || {
            MirCompiler::with_options(false)
                .compile_normal(request)
                .expect("normal compile must snapshot at compile ingress")
        },
    );
    let selected_dump = MirPrinter::new().print_module(&selected.module);
    assert!(selected_dump.contains("safepoint"), "{selected_dump}");
    assert!(selected_dump.contains("ingress-time"), "{selected_dump}");
    assert!(!selected_dump.contains("request-time"), "{selected_dump}");
    assert!(!selected_dump.contains("must-not-win"), "{selected_dump}");

    let malformed = crate::test_support::with_env_vars(
        &[
            ("NYASH_SCRIPT_ARGS_JSON", Some("{malformed}")),
            ("HAKO_SCRIPT_ARGS_JSON", Some(r#"["must-stay-masked"]"#)),
            ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some(" maybe ")),
        ],
        || {
            let ast = NyashParser::parse_from_string(source).expect("App source");
            MirCompiler::with_options(false)
                .compile_normal(
                    NormalCompileRequestV1::for_mir_mode(ast, None, HashMap::new())
                        .expect("App request"),
                )
                .expect("malformed normal inputs remain permissive")
        },
    );
    let malformed_dump = MirPrinter::new().print_module(&malformed.module);
    assert!(!malformed_dump.contains("safepoint"), "{malformed_dump}");
    assert!(
        !malformed_dump.contains("must-stay-masked"),
        "{malformed_dump}"
    );

    let hako_fallback = crate::test_support::with_env_vars(
        &[
            ("NYASH_SCRIPT_ARGS_JSON", None),
            ("HAKO_SCRIPT_ARGS_JSON", Some(r#"["hako-fallback"]"#)),
            ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some("off")),
        ],
        || {
            let ast = NyashParser::parse_from_string(source).expect("App source");
            MirCompiler::with_options(false)
                .compile_normal(
                    NormalCompileRequestV1::for_mir_mode(ast, None, HashMap::new())
                        .expect("App request"),
                )
                .expect("HAKO fallback must remain normal-compatible")
        },
    );
    let hako_dump = MirPrinter::new().print_module(&hako_fallback.module);
    assert!(hako_dump.contains("hako-fallback"), "{hako_dump}");
    assert!(!hako_dump.contains("safepoint"), "{hako_dump}");
}

#[test]
fn all_typed_program_constructors_share_one_program_admission() {
    let imports = HashMap::from([("Alias".to_owned(), "Target".to_owned())]);
    assert!(
        NormalCompileRequestV1::for_mir_mode(program(), Some("mir.hako"), imports.clone(),).is_ok()
    );
    assert!(NormalCompileRequestV1::for_minimal_mir_json(program(), Some("minimal.hako")).is_ok());
    assert!(
        NormalCompileRequestV1::for_llvm_source(program(), Some("llvm.hako"), imports.clone(),)
            .is_ok()
    );
    assert!(NormalCompileRequestV1::for_wasm_source(program(), Some("wasm.hako"), imports).is_ok());
    assert!(NormalCompileRequestV1::for_program_json_v0_import_bundle(program()).is_ok());
    assert!(NormalCompileRequestV1::for_repl_program(program()).is_ok());

    for rejected in [
        NormalCompileRequestV1::for_mir_mode(non_program(), Some("mir.hako"), HashMap::new()),
        NormalCompileRequestV1::for_minimal_mir_json(non_program(), Some("minimal.hako")),
        NormalCompileRequestV1::for_llvm_source(non_program(), Some("llvm.hako"), HashMap::new()),
        NormalCompileRequestV1::for_wasm_source(non_program(), Some("wasm.hako"), HashMap::new()),
        NormalCompileRequestV1::for_program_json_v0_import_bundle(non_program()),
        NormalCompileRequestV1::for_repl_program(non_program()),
    ] {
        let rejected = rejected.expect_err("non-Program root must reject at request admission");
        assert_eq!(
            rejected.error(),
            NormalProgramCompileRequestErrorV1::ExpectedProgramRoot
        );
        rejected.discard();
    }
}

#[test]
fn program_json_v0_import_bundle_fixes_source_and_empty_builder_imports() {
    let request = NormalCompileRequestV1::for_program_json_v0_import_bundle(program())
        .expect("Program-v0 import bundle must use typed Program admission");
    let (_, source, imports, admission, _) = request.into_parts();

    assert_eq!(source.source_file(), Some("<json_v0/imports>"));
    assert!(imports.is_empty());
    assert_eq!(
        admission,
        NormalCompileAdmissionV1::ProgramJsonV0ImportBundleNoBuilderImports
    );
}

#[test]
fn repl_program_fixes_source_and_empty_builder_imports() {
    let request = NormalCompileRequestV1::for_repl_program(program())
        .expect("REPL must use typed Program admission");
    let (_, source, imports, admission, _) = request.into_parts();

    assert_eq!(source.source_file(), Some("<repl>"));
    assert!(imports.is_empty());
    assert_eq!(
        admission,
        NormalCompileAdmissionV1::ReplProgramNoBuilderImports
    );
}

#[test]
fn post_macro_whole_file_seal_accepts_program_and_rejects_non_program() {
    let program =
        VerifiedPostMacroWholeFileProgramV1::seal(program()).expect("Program must seal once");
    let request =
        NormalCompileRequestV1::for_stage1_direct_post_macro(program, Some("stage1.hako"));
    let (_, source, imports, admission, _) = request.into_parts();
    assert_eq!(source.source_file(), Some("stage1.hako"));
    assert!(imports.is_empty());
    assert_eq!(
        admission,
        NormalCompileAdmissionV1::Stage1DirectPostMacroProgramNoImports
    );

    let rejected = VerifiedPostMacroWholeFileProgramV1::seal(non_program())
        .expect_err("Literal must fail the whole-file Program contract");
    assert_eq!(
        rejected.error(),
        PostMacroWholeFileProgramErrorV1::ExpectedProgram
    );
    rejected.discard();
}

#[test]
fn selfhost_macro_preexpand_fixes_anonymous_source_and_empty_imports() {
    let program =
        VerifiedPostMacroWholeFileProgramV1::seal(program()).expect("Program must seal once");
    let request = NormalCompileRequestV1::for_selfhost_macro_preexpand(program);
    let (_, source, imports, admission, _) = request.into_parts();

    assert_eq!(source.source_file(), None);
    assert!(imports.is_empty());
    assert_eq!(
        admission,
        NormalCompileAdmissionV1::SelfhostMacroPreexpandProgramNoImports
    );
}

#[test]
fn vm_hako_post_macro_preserves_named_source_and_exact_imports() {
    let program =
        VerifiedPostMacroWholeFileProgramV1::seal(program()).expect("Program must seal once");
    let imports = HashMap::from([("Alias".to_owned(), "Target".to_owned())]);
    let request =
        NormalCompileRequestV1::for_vm_hako_post_macro(program, "vm-hako.hako", imports.clone());
    let (_, source, actual_imports, admission, _) = request.into_parts();

    assert_eq!(source.source_file(), Some("vm-hako.hako"));
    assert_eq!(actual_imports, imports);
    assert_eq!(
        admission,
        NormalCompileAdmissionV1::VmHakoPostMacroProgramWithImports
    );
}

#[test]
fn vm_fallback_post_macro_preserves_named_source_and_empty_imports() {
    let program =
        VerifiedPostMacroWholeFileProgramV1::seal(program()).expect("Program must seal once");
    let request = NormalCompileRequestV1::for_vm_fallback_post_macro(program, "fallback.hako");
    let (_, source, imports, admission, _) = request.into_parts();

    assert_eq!(source.source_file(), Some("fallback.hako"));
    assert!(imports.is_empty());
    assert_eq!(
        admission,
        NormalCompileAdmissionV1::VmFallbackPostMacroProgramNoImports
    );
}

#[test]
fn vm_keep_post_macro_preserves_named_source_and_exact_imports() {
    let program =
        VerifiedPostMacroWholeFileProgramV1::seal(program()).expect("Program must seal once");
    let imports = HashMap::from([("KeepAlias".to_owned(), "KeepTarget".to_owned())]);
    let request =
        NormalCompileRequestV1::for_vm_keep_post_macro(program, "vm-keep.hako", imports.clone());
    let (_, source, actual_imports, admission, _) = request.into_parts();

    assert_eq!(source.source_file(), Some("vm-keep.hako"));
    assert_eq!(actual_imports, imports);
    assert_eq!(
        admission,
        NormalCompileAdmissionV1::VmKeepPostMacroProgramWithImports
    );
}
