use super::{
    module_session::CanonicalModuleLoweringSessionV1, require_canonical_verification,
    CanonicalFinishScheduleV1, CanonicalLoweringErrorV1, LegacyRcInsertionScheduleV1, MirCompiler,
    MirFinishScheduleV1,
};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::exact_numeric_value_facts::{ExactNumericReturnFact, ExactNumericValueFactSource};
use crate::mir::function::ExactNumericRuntimeCheckContractKind;
use crate::mir::string_corridor::StringCorridorOp;
use crate::mir::string_corridor_placement::StringCorridorCandidateKind;
use crate::mir::{
    BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule,
    MirPrinter, MirType,
};
use crate::parser::NyashParser;

#[test]
fn trivial_binding_ssa_finish_schedule_skips_legacy_rc() {
    assert_eq!(
        MirFinishScheduleV1::Canonical(CanonicalFinishScheduleV1::TrivialBindingSsa)
            .legacy_rc_insertion(),
        LegacyRcInsertionScheduleV1::Skip
    );
}

#[test]
fn current_canonical_and_legacy_finish_schedules_keep_legacy_rc() {
    assert_eq!(
        MirFinishScheduleV1::Canonical(CanonicalFinishScheduleV1::CurrentCanonicalAPlus)
            .legacy_rc_insertion(),
        LegacyRcInsertionScheduleV1::Run
    );
    assert_eq!(
        MirFinishScheduleV1::Legacy.legacy_rc_insertion(),
        LegacyRcInsertionScheduleV1::Run
    );
}

#[test]
fn selected_dynamic_finish_schedule_skips_legacy_postseal_mutators() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "ParserScanLoopBox.skip_while/4".to_owned(),
            params: vec![MirType::Unknown; 4],
            return_type: MirType::Integer,
            effects: EffectMask::READ,
        },
        BasicBlockId::new(0),
    );
    function
        .metadata
        .install_a_prime_i64_physical_receipt_for_test(crate::mir::test_support::a_prime_receipt())
        .expect("receipt install");
    function
        .metadata
        .install_dynamic_v2_aot_metadata_for_test(
            crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1::for_test(
            ),
        )
        .expect("AOT metadata install");

    let mut module = MirModule::new("selected".to_owned());
    module.add_function(function);

    let schedule = super::finish_schedule_for_normal_module(&module)
        .expect("selected pair should select the closed schedule");
    assert_eq!(schedule, MirFinishScheduleV1::SelectedDynamic);
    assert_eq!(
        schedule.legacy_rc_insertion(),
        LegacyRcInsertionScheduleV1::Skip
    );
}

#[test]
fn selected_dynamic_finish_schedule_rejects_scrubbed_or_partial_metadata() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "selected/0".to_owned(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function
        .metadata
        .install_a_prime_i64_physical_receipt_for_test(crate::mir::test_support::a_prime_receipt())
        .expect("receipt install");
    let mut partial = MirModule::new("partial".to_owned());
    partial.add_function(function);
    assert!(super::finish_schedule_for_normal_module(&partial)
        .unwrap_err()
        .contains("partial"));

    let mut function = MirFunction::new(
        FunctionSignature {
            name: "selected/0".to_owned(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function
        .metadata
        .install_a_prime_i64_physical_receipt_for_test(crate::mir::test_support::a_prime_receipt())
        .expect("receipt install");
    function
        .metadata
        .install_dynamic_v2_aot_metadata_for_test(
            crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1::for_test(
            ),
        )
        .expect("AOT metadata install");
    let mut scrubbed_function = function.clone();
    scrubbed_function.signature.name = "scrubbed/0".to_owned();
    let mut scrubbed = MirModule::new("scrubbed".to_owned());
    scrubbed.add_function(function);
    scrubbed.add_function(scrubbed_function);
    assert!(super::finish_schedule_for_normal_module(&scrubbed)
        .unwrap_err()
        .contains("scrubbed"));
}

#[test]
fn test_basic_mir_compilation() {
    let mut compiler = MirCompiler::new();

    // Create a simple literal AST node
    let ast = ASTNode::Literal {
        value: LiteralValue::Integer(42),
        span: crate::ast::Span::unknown(),
    };

    // Compile to MIR
    let result = compiler.compile(ast);
    assert!(result.is_ok(), "Basic MIR compilation should succeed");

    let compile_result = result.unwrap();
    assert!(
        !compile_result.module.functions.is_empty(),
        "Module should contain at least one function"
    );
}

#[test]
fn canonical_verification_failure_discards_candidate_before_commit() {
    let mut compiler = MirCompiler::with_options(false);
    compiler.builder.repl_mode = true;
    let mut session = CanonicalModuleLoweringSessionV1::open(&compiler.builder);
    session.builder_mut().repl_mode = false;

    let error = require_canonical_verification(Err(vec![
        crate::mir::VerificationError::UnreachableBlock {
            block: crate::mir::BasicBlockId::new(900),
        },
    ]))
    .unwrap_err();
    drop(session);

    assert!(matches!(
        error,
        CanonicalLoweringErrorV1::MirVerificationFailed { .. }
    ));
    assert!(compiler.builder.repl_mode);
}

#[test]
fn compile_attaches_dynamic_integer_range_contract_before_verify() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(
        r#"
box Page {
  capacity: usize = 0
}

static box Main {
  main(x) {
    local p = new Page()
    p.capacity = x
    return 0
  }
}
"#,
    )
    .expect("parse");
    let mut compiler = MirCompiler::with_options(false);
    let result = compiler.compile(ast).expect("compile");

    assert!(
        result.verification_result.is_ok(),
        "pre-verify contract attach should satisfy exact numeric verifier: {:?}",
        result.verification_result
    );
    let contracts: Vec<_> = result
        .module
        .functions
        .values()
        .flat_map(|function| {
            function
                .metadata
                .exact_numeric_runtime_check_contracts
                .iter()
        })
        .collect();

    assert_eq!(contracts.len(), 1);
    assert_eq!(contracts[0].field, "capacity");
    assert_eq!(contracts[0].declared_type_name, "usize");
    assert_eq!(
        contracts[0].kind,
        ExactNumericRuntimeCheckContractKind::DynamicIntegerRange
    );
}

#[test]
fn compile_preserves_exact_numeric_signature_facts() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(
        r#"
static box Main {
  id(x: usize): u64 {
    return x
  }

  main() {
    return 0
  }
}
"#,
    )
    .expect("parse");
    let mut compiler = MirCompiler::with_options(false);
    let result = compiler.compile(ast).expect("compile");
    let function = result.module.get_function("Main.id/1").expect("Main.id/1");
    let param = function.params[0];

    assert_eq!(
        function
            .metadata
            .declared_param_decls
            .iter()
            .map(|decl| (
                decl.name.as_str(),
                decl.declared_type_name.as_deref().unwrap_or("<none>")
            ))
            .collect::<Vec<_>>(),
        vec![("x", "usize")]
    );
    assert_eq!(
        function.metadata.declared_return_type_name.as_deref(),
        Some("u64")
    );
    let fact = function
        .metadata
        .exact_numeric_value_facts
        .get(&param)
        .expect("param exact numeric fact");
    assert_eq!(fact.declared_type_name, "usize");
    assert_eq!(
        fact.source,
        ExactNumericValueFactSource::Param {
            index: 0,
            name: "x".to_string(),
        }
    );
    assert_eq!(
        function.metadata.exact_numeric_return_fact,
        Some(ExactNumericReturnFact {
            declared_type_name: "u64".to_string(),
        })
    );
}

#[test]
fn compile_publishes_declared_method_param_types_to_signature() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(
        r#"
box Scanner {
  text: StringBox

  birth(input_text: StringBox) {
    me.text = input_text
  }
}

static box Main {
  main() {
    return 0
  }
}
"#,
    )
    .expect("parse");
    let mut compiler = MirCompiler::with_options(false);
    let result = compiler.compile(ast).expect("compile");
    let function = result
        .module
        .get_function("Scanner.birth/1")
        .expect("Scanner.birth/1");

    assert_eq!(
        function.signature.params.get(1),
        Some(&MirType::String),
        "declared method parameter type should be callable signature truth"
    );
    assert_eq!(
        function.metadata.value_types.get(&function.params[1]),
        Some(&MirType::String),
        "method parameter value type should be seeded from signature"
    );
}

#[test]
fn compile_publishes_exact_numeric_box_field_proof_from_ordinary_literal() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(
        r#"
box Page {
  capacity: usize = 0
}

static box Main {
  main() {
    local page = new Page()
    page.capacity = 7
    return 0
  }
}
"#,
    )
    .expect("parse");
    let mut compiler = MirCompiler::with_options(false);
    let result = compiler.compile(ast).expect("compile");
    let proof = result
        .module
        .functions
        .values()
        .flat_map(|function| function.metadata.exact_numeric_field_contract_proofs.iter())
        .next()
        .expect("exact numeric Box field proof");

    assert_eq!(proof.field, "capacity");
    assert_eq!(proof.expected_type, "usize");
    assert_eq!(
        proof.proof_kind,
        crate::mir::type_contracts::proof::TypeContractProofKind::ExactNumericConstantInRange
    );
}

#[test]
fn compile_rejects_out_of_range_ordinary_literal_at_exact_numeric_box_field() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(
        r#"
box ByteCell {
  value: u8 = 0
}

static box Main {
  main() {
    local cell = new ByteCell()
    cell.value = 256
    return 0
  }
}
"#,
    )
    .expect("parse");
    let mut compiler = MirCompiler::with_options(false);
    let result = compiler.compile(ast).expect("MIR build should complete");
    let errors = result
        .verification_result
        .expect_err("verifier should reject before execution");
    let err = errors[0].to_string();

    assert!(
        err.contains("[mir/verify:numeric_range]"),
        "unexpected error: {}",
        err
    );
}

#[test]
fn test_mir_dump() {
    let mut compiler = MirCompiler::new();

    let ast = ASTNode::Literal {
        value: LiteralValue::Integer(42),
        span: crate::ast::Span::unknown(),
    };

    let result = compiler.compile(ast).unwrap();
    let mir_dump = compiler.dump_mir(&result.module);

    assert!(!mir_dump.is_empty(), "MIR dump should not be empty");
    assert!(
        mir_dump.contains("define"),
        "MIR dump should contain function definition"
    );
}

#[test]
fn test_lowering_is_type_function_call_in_print() {
    // Build AST: print(isType(42, "Integer"))
    let ast = ASTNode::Print {
        expression: Box::new(ASTNode::FunctionCall {
            name: "isType".to_string(),
            arguments: vec![
                ASTNode::Literal {
                    value: LiteralValue::Integer(42),
                    span: crate::ast::Span::unknown(),
                },
                ASTNode::Literal {
                    value: LiteralValue::String("Integer".to_string()),
                    span: crate::ast::Span::unknown(),
                },
            ],
            span: crate::ast::Span::unknown(),
        }),
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");

    // Ensure TypeOp exists in the resulting MIR
    let has_typeop = result.module.functions.values().any(|f| {
        f.blocks.values().any(|b| {
            b.all_spanned_instructions()
                .any(|sp| matches!(sp.inst, MirInstruction::TypeOp { .. }))
        })
    });
    assert!(
        has_typeop,
        "Expected TypeOp lowering for print(isType(...))"
    );
}

#[test]
fn test_lowering_is_method_call_in_print() {
    // Build AST: print( (42).is("Integer") )
    let ast = ASTNode::Print {
        expression: Box::new(ASTNode::MethodCall {
            object: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(42),
                span: crate::ast::Span::unknown(),
            }),
            method: "is".to_string(),
            arguments: vec![ASTNode::Literal {
                value: LiteralValue::String("Integer".to_string()),
                span: crate::ast::Span::unknown(),
            }],
            span: crate::ast::Span::unknown(),
        }),
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");

    // Ensure TypeOp exists in the resulting MIR
    let has_typeop = result.module.functions.values().any(|f| {
        f.blocks.values().any(|b| {
            b.all_spanned_instructions()
                .any(|sp| matches!(sp.inst, MirInstruction::TypeOp { .. }))
        })
    });
    assert!(
        has_typeop,
        "Expected TypeOp lowering for print(obj.is(...))"
    );
}

#[test]
#[ignore = "MIR13 migration: extern console.log expectation pending"]
fn test_lowering_extern_console_log() {
    // Build AST: console.log("hi") → ExternCall env.console.log
    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: "console".to_string(),
            span: crate::ast::Span::unknown(),
        }),
        method: "log".to_string(),
        arguments: vec![ASTNode::Literal {
            value: LiteralValue::String("hi".to_string()),
            span: crate::ast::Span::unknown(),
        }],
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");
    let dump = MirPrinter::verbose().print_module(&result.module);

    assert!(
        dump.contains("extern_call env.console.log"),
        "Expected extern_call env.console.log in MIR dump. Got:\n{}",
        dump
    );
}

#[test]
fn test_lowering_boxcall_array_push() {
    // Build AST: (new ArrayBox()).push(1)
    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::New {
            class: "ArrayBox".to_string(),
            arguments: vec![],
            field_initializers: vec![],
            type_arguments: vec![],
            span: crate::ast::Span::unknown(),
        }),
        method: "push".to_string(),
        arguments: vec![ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: crate::ast::Span::unknown(),
        }],
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");
    let dump = MirPrinter::new().print_module(&result.module);
    // Known Array writes converge before downstream planners observe MIR.
    assert!(
        dump.contains("array.write #0 push"),
        "Expected canonical ArrayElementWrite push. Got:\n{}",
        dump
    );
}

#[test]
fn test_compile_attaches_string_corridor_fact_for_string_length() {
    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::Literal {
            value: LiteralValue::String("hello".to_string()),
            span: crate::ast::Span::unknown(),
        }),
        method: "length".to_string(),
        arguments: vec![],
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");

    let len_fact_count = result
        .module
        .functions
        .values()
        .flat_map(|function| function.metadata.string_corridor_facts.values())
        .filter(|fact| fact.op == StringCorridorOp::StrLen)
        .count();

    assert!(
        len_fact_count >= 1,
        "expected at least one str.len fact in compiled MIR"
    );
}

#[test]
fn test_compile_attaches_string_corridor_candidate_for_string_length() {
    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::Literal {
            value: LiteralValue::String("hello".to_string()),
            span: crate::ast::Span::unknown(),
        }),
        method: "length".to_string(),
        arguments: vec![],
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");

    let direct_kernel_candidate_count = result
        .module
        .functions
        .values()
        .flat_map(|function| function.metadata.string_corridor_candidates.values())
        .flatten()
        .filter(|candidate| candidate.kind == StringCorridorCandidateKind::DirectKernelEntry)
        .count();

    assert!(
        direct_kernel_candidate_count >= 1,
        "expected at least one direct-kernel-entry candidate in compiled MIR"
    );
}

#[test]
#[ignore = "MIR13 migration: method id naming in printer pending"]
fn test_boxcall_method_id_on_universal_slot() {
    // Build AST: (new ArrayBox()).toString()
    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::New {
            class: "ArrayBox".to_string(),
            arguments: vec![],
            field_initializers: vec![],
            type_arguments: vec![],
            span: crate::ast::Span::unknown(),
        }),
        method: "toString".to_string(),
        arguments: vec![],
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");
    let dump = MirPrinter::new().print_module(&result.module);
    // Expect a BoxCall with numeric method id [#0] for toString universal slot
    assert!(
        dump.contains("toString[#0]"),
        "Expected method_id #0 for toString. Dump:\n{}",
        dump
    );
}

#[test]
fn test_lowering_await_expression() {
    if crate::config::env::mir_core13_pure() {
        if crate::config::env::joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug("[TEST] skip await under Core-13 pure mode");
        }
        return;
    }
    // Build AST: await 1  (semantic is nonsensical but should emit Await)
    let ast = ASTNode::AwaitExpression {
        expression: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: crate::ast::Span::unknown(),
        }),
        span: crate::ast::Span::unknown(),
    };
    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");
    let dump = MirPrinter::new().print_module(&result.module);
    assert!(
        dump.contains("await"),
        "Expected await in MIR dump. Got:\n{}",
        dump
    );
}

// Legacy await / safepoint モデルのテスト（Core-13/Pure 以降とは挙動差あり）.
#[test]
#[ignore]
fn test_await_has_checkpoints() {
    if crate::config::env::mir_core13_pure() {
        if crate::config::env::joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug("[TEST] skip await under Core-13 pure mode");
        }
        return;
    }
    use crate::ast::{LiteralValue, Span};
    // Build: await 1
    let ast = ASTNode::AwaitExpression {
        expression: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile");
    // Verifier should pass (await flanked by safepoints)
    assert!(
        result.verification_result.is_ok(),
        "Verifier failed for await checkpoints: {:?}",
        result.verification_result
    );
    let dump = compiler.dump_mir(&result.module);
    // Expect at least two safepoints in the function (before/after await)
    let sp_count = dump.matches("safepoint").count();
    assert!(
        sp_count >= 2,
        "Expected >=2 safepoints around await, got {}. Dump:\n{}",
        sp_count,
        dump
    );
}

// Legacy await rewrite テスト（現行の Future 統合とは独立にアーカイブ扱い）.
#[test]
#[ignore]
fn test_rewritten_await_still_checkpoints() {
    if crate::config::env::mir_core13_pure() {
        if crate::config::env::joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug("[TEST] skip await under Core-13 pure mode");
        }
        return;
    }
    use crate::ast::{LiteralValue, Span};
    // Enable rewrite so Await → ExternCall(env.future.await)
    std::env::set_var("NYASH_REWRITE_FUTURE", "1");
    let ast = ASTNode::AwaitExpression {
        expression: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile");
    // Verifier should still pass (checkpoint verification includes ExternCall await)
    assert!(
        result.verification_result.is_ok(),
        "Verifier failed for rewritten await checkpoints: {:?}",
        result.verification_result
    );
    let dump = compiler.dump_mir(&result.module);
    assert!(
        dump.contains("env.future.await"),
        "Expected rewritten await extern call. Dump:\n{}",
        dump
    );
    let sp_count = dump.matches("safepoint").count();
    assert!(
        sp_count >= 2,
        "Expected >=2 safepoints around rewritten await, got {}. Dump:\n{}",
        sp_count,
        dump
    );
    // Cleanup env
    std::env::remove_var("NYASH_REWRITE_FUTURE");
}

#[test]
#[ignore = "MIR13 migration: throw/safepoint expectations pending"]
fn test_throw_compilation() {
    let mut compiler = MirCompiler::new();

    let throw_ast = ASTNode::Throw {
        expression: Box::new(ASTNode::Literal {
            value: LiteralValue::String("Test exception".to_string()),
            span: crate::ast::Span::unknown(),
        }),
        span: crate::ast::Span::unknown(),
    };

    let result = compiler.compile(throw_ast);
    assert!(result.is_ok(), "Throw compilation should succeed");

    let compile_result = result.unwrap();
    let mir_dump = compiler.dump_mir(&compile_result.module);
    assert!(
        mir_dump.contains("throw"),
        "MIR should contain throw instruction"
    );
    assert!(
        mir_dump.contains("safepoint"),
        "MIR should contain safepoint instruction"
    );
}

#[test]
#[ignore = "MIR13 migration: loop safepoint expectation pending"]
fn test_loop_compilation() {
    let mut compiler = MirCompiler::new();

    let loop_ast = ASTNode::Loop {
        condition: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: crate::ast::Span::unknown(),
        }),
        body: vec![ASTNode::Print {
            expression: Box::new(ASTNode::Literal {
                value: LiteralValue::String("Loop body".to_string()),
                span: crate::ast::Span::unknown(),
            }),
            span: crate::ast::Span::unknown(),
        }],
        span: crate::ast::Span::unknown(),
    };

    let result = compiler.compile(loop_ast);
    assert!(result.is_ok(), "Loop compilation should succeed");

    let compile_result = result.unwrap();
    let mir_dump = compiler.dump_mir(&compile_result.module);
    assert!(
        mir_dump.contains("br"),
        "MIR should contain branch instructions"
    );
    assert!(
        mir_dump.contains("safepoint"),
        "MIR should contain safepoint instructions"
    );
}

#[test]
fn test_try_catch_compilation() {
    // Core-13 pure モードでは Try/Catch 命令は許容集合外のためスキップ
    if crate::config::env::mir_core13_pure() {
        if crate::config::env::joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[TEST] skip try/catch under Core-13 pure mode");
        }
        return;
    }
    let mut compiler = MirCompiler::new();

    let try_catch_ast = ASTNode::Program {
        statements: vec![ASTNode::TryCatch {
            try_body: vec![ASTNode::Print {
                expression: Box::new(ASTNode::Literal {
                    value: LiteralValue::String("Try block".to_string()),
                    span: crate::ast::Span::unknown(),
                }),
                span: crate::ast::Span::unknown(),
            }],
            catch_clauses: vec![crate::ast::CatchClause {
                exception_type: Some("Exception".to_string()),
                variable_name: Some("e".to_string()),
                body: vec![ASTNode::Print {
                    expression: Box::new(ASTNode::Literal {
                        value: LiteralValue::String("Catch block".to_string()),
                        span: crate::ast::Span::unknown(),
                    }),
                    span: crate::ast::Span::unknown(),
                }],
                span: crate::ast::Span::unknown(),
            }],
            finally_body: None,
            span: crate::ast::Span::unknown(),
        }],
        span: crate::ast::Span::unknown(),
    };

    let result = compiler.compile(try_catch_ast);
    assert!(
        result.is_ok(),
        "TryCatch compilation should succeed: {result:?}"
    );

    let compile_result = result.unwrap();
    let mir_dump = compiler.dump_mir(&compile_result.module);
    assert!(
        mir_dump.contains("catch"),
        "MIR should contain catch instruction"
    );
}
