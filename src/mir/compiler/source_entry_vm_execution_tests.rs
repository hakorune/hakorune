use super::*;
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, Span};
use crate::backend::vm_types::VMValue;
use crate::mir::compiler::source_entry_result::SourceEntryResultV1;
use crate::mir::compiler::source_entry_vm_invocation::decode_vm_value;
use crate::mir::compiler::source_entry_vm_reference::VmSourceEntryDecodePlanV1;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static ENTRY_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
use crate::mir::compiler::source_entry_result::UnitOriginV1;

fn empty_script() -> ASTNode {
    ASTNode::Program {
        statements: Vec::new(),
        span: Span::unknown(),
    }
}

fn literal_script(value: crate::ast::LiteralValue) -> ASTNode {
    ASTNode::Program {
        statements: vec![ASTNode::Literal {
            value,
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn binary_script(
    left: crate::ast::LiteralValue,
    operator: BinaryOperator,
    right: crate::ast::LiteralValue,
) -> ASTNode {
    ASTNode::Program {
        statements: vec![ASTNode::BinaryOp {
            operator,
            left: Box::new(ASTNode::Literal {
                value: left,
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: right,
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn print_script() -> ASTNode {
    ASTNode::Program {
        statements: vec![ASTNode::Print {
            expression: Box::new(ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn assignment_script() -> ASTNode {
    ASTNode::Program {
        statements: vec![
            ASTNode::Local {
                variables: vec!["x".into()],
                initial_values: vec![Some(Box::new(ASTNode::Literal {
                    value: crate::ast::LiteralValue::Integer(1),
                    span: Span::unknown(),
                }))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "x".into(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::Literal {
                    value: crate::ast::LiteralValue::Integer(3),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    }
}

fn compound_assignment_script() -> ASTNode {
    ASTNode::Program {
        statements: vec![
            ASTNode::Local {
                variables: vec!["x".into()],
                initial_values: vec![Some(Box::new(ASTNode::Literal {
                    value: crate::ast::LiteralValue::Integer(1),
                    span: Span::unknown(),
                }))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::CompoundAssignment {
                target: Box::new(ASTNode::Variable {
                    name: "x".into(),
                    span: Span::unknown(),
                }),
                operator: BinaryOperator::Add,
                value: Box::new(ASTNode::Literal {
                    value: crate::ast::LiteralValue::Integer(2),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    }
}

fn app_with_body(body: Vec<ASTNode>) -> ASTNode {
    let main = ASTNode::FunctionDeclaration {
        name: "main".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    let mut methods = HashMap::new();
    methods.insert("main".into(), main);
    ASTNode::Program {
        statements: vec![ASTNode::BoxDeclaration {
            name: "Main".into(),
            fields: Vec::new(),
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            methods,
            constructors: HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_record: false,
            extends: Vec::new(),
            implements: Vec::new(),
            type_parameters: Vec::new(),
            is_sync: false,
            is_static: true,
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn empty_app() -> ASTNode {
    app_with_body(Vec::new())
}

fn scalar_app() -> ASTNode {
    app_with_body(vec![ASTNode::Literal {
        value: crate::ast::LiteralValue::Integer(9),
        span: Span::unknown(),
    }])
}

#[test]
fn unit_plan_does_not_promote_print_payload_to_a_value() {
    let result = decode_vm_value(
        VmSourceEntryDecodePlanV1::Unit {
            origin: UnitOriginV1::PrintStatement,
            requires_void: false,
        },
        VMValue::Integer(1),
    );
    assert!(matches!(
        result,
        SourceEntryResultV1::Unit(UnitOriginV1::PrintStatement)
    ));
}

#[test]
fn integer_plan_preserves_exact_integer_value() {
    assert!(matches!(
        decode_vm_value(VmSourceEntryDecodePlanV1::Integer, VMValue::Integer(255)),
        SourceEntryResultV1::Integer(255)
    ));
}

#[test]
fn synthetic_unit_requires_void_payload() {
    let result = decode_vm_value(
        VmSourceEntryDecodePlanV1::Unit {
            origin: UnitOriginV1::EmptyBody,
            requires_void: true,
        },
        VMValue::Integer(0),
    );
    assert!(matches!(result, SourceEntryResultV1::Fault(_)));
}

#[test]
fn raw_vm_entry_executes_published_empty_script() {
    let mut compiler = crate::mir::compiler::MirCompiler::new();
    let report = compiler
        .run_raw_vm_reference(empty_script(), Some("raw-vm-empty.hako"))
        .expect("explicit Raw VM-reference entry should execute empty Script");
    assert_eq!(report.status_code(), 0);
    assert_eq!(report.diagnostic_tag(), None);
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn raw_vm_entry_preserves_integer_process_status_boundaries() {
    for value in [0, 255] {
        let mut compiler = crate::mir::compiler::MirCompiler::new();
        let report = compiler
            .run_raw_vm_reference(
                literal_script(crate::ast::LiteralValue::Integer(value)),
                Some("raw-vm-integer.hako"),
            )
            .expect("integer Raw VM-reference entry should execute");
        assert_eq!(report.status_code(), value as u8);
        assert_eq!(report.diagnostic_tag(), None);
    }
}

#[test]
fn raw_vm_entry_keeps_explicit_void_as_unit() {
    let mut compiler = crate::mir::compiler::MirCompiler::new();
    let report = compiler
        .run_raw_vm_reference(
            literal_script(crate::ast::LiteralValue::Void),
            Some("raw-vm-void.hako"),
        )
        .expect("explicit Void should execute as Unit");
    assert_eq!(report.status_code(), 0);
    assert_eq!(report.diagnostic_tag(), None);
}

#[test]
fn raw_vm_entry_reports_out_of_range_integer_without_zero_fallback() {
    for value in [-1, 256] {
        let mut compiler = crate::mir::compiler::MirCompiler::new();
        let report = compiler
            .run_raw_vm_reference(
                literal_script(crate::ast::LiteralValue::Integer(value)),
                Some("raw-vm-range.hako"),
            )
            .expect("out-of-range result should be a typed process fault");
        assert_eq!(report.status_code(), 70);
        assert_eq!(
            report.diagnostic_tag(),
            Some("[process/exit-code-out-of-range]")
        );
    }
}

#[test]
fn raw_vm_entry_reports_unsupported_process_result_kinds() {
    let cases = [
        (crate::ast::LiteralValue::Bool(true), "Bool"),
        (crate::ast::LiteralValue::Float(1.5), "Float"),
        (crate::ast::LiteralValue::String("raw".into()), "String"),
    ];
    for (value, _kind) in cases {
        let mut compiler = crate::mir::compiler::MirCompiler::new();
        let report = compiler
            .run_raw_vm_reference(literal_script(value), Some("raw-vm-kind.hako"))
            .expect("unsupported process result should remain a typed fault");
        assert_eq!(report.status_code(), 70);
        assert_eq!(
            report.diagnostic_tag(),
            Some("[process/unsupported-result]")
        );
    }
}

#[test]
fn raw_vm_entry_keeps_print_statement_as_unit() {
    let mut compiler = crate::mir::compiler::MirCompiler::new();
    let report = compiler
        .run_raw_vm_reference(print_script(), Some("raw-vm-print.hako"))
        .expect("Print statement should execute as a Unit source result");
    assert_eq!(report.status_code(), 0);
    assert_eq!(report.diagnostic_tag(), None);
}

#[test]
fn raw_vm_entry_keeps_assignment_statement_as_unit() {
    let mut compiler = crate::mir::compiler::MirCompiler::new();
    let report = compiler
        .run_raw_vm_reference(assignment_script(), Some("raw-vm-assignment.hako"))
        .expect("assignment statement should execute as Unit");
    assert_eq!(report.status_code(), 0);
    assert_eq!(report.diagnostic_tag(), None);
}

#[test]
fn raw_vm_entry_keeps_compound_assignment_statement_as_unit() {
    let mut compiler = crate::mir::compiler::MirCompiler::new();
    let report = compiler
        .run_raw_vm_reference(
            compound_assignment_script(),
            Some("raw-vm-compound-assignment.hako"),
        )
        .expect("compound assignment statement should execute as Unit");
    assert_eq!(report.status_code(), 0);
    assert_eq!(report.diagnostic_tag(), None);
}

#[test]
fn raw_vm_entry_executes_empty_app_main_without_symbol_discovery() {
    let mut compiler = crate::mir::compiler::MirCompiler::new();
    let report = compiler
        .run_raw_vm_reference(empty_app(), Some("raw-vm-app.hako"))
        .expect("empty App Main should execute through the sealed target");
    assert_eq!(report.status_code(), 0);
    assert_eq!(report.diagnostic_tag(), None);
}

#[test]
fn raw_vm_entry_ignores_decoy_nyash_entry_environment() {
    let _lock = ENTRY_ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("entry environment lock");
    let previous = std::env::var_os("NYASH_ENTRY");
    std::env::set_var("NYASH_ENTRY", "Decoy.main/0");
    let mut compiler = crate::mir::compiler::MirCompiler::new();
    let result = compiler.run_raw_vm_reference(empty_app(), Some("raw-vm-decoy.hako"));
    match previous {
        Some(value) => std::env::set_var("NYASH_ENTRY", value),
        None => std::env::remove_var("NYASH_ENTRY"),
    }
    let report = result.expect("sealed Main target must ignore NYASH_ENTRY");
    assert_eq!(report.status_code(), 0);
    assert_eq!(report.diagnostic_tag(), None);
}

#[test]
fn raw_vm_entry_keeps_app_scalar_fallthrough_as_unit() {
    let mut compiler = crate::mir::compiler::MirCompiler::new();
    let report = compiler
        .run_raw_vm_reference(scalar_app(), Some("raw-vm-app-scalar.hako"))
        .expect("App scalar fallthrough should follow the sealed App Unit plan");
    assert_eq!(report.status_code(), 0);
    assert_eq!(report.diagnostic_tag(), None);
}

#[test]
fn raw_vm_entry_maps_division_fault_to_source_diagnostic() {
    let mut compiler = crate::mir::compiler::MirCompiler::new();
    let report = compiler
        .run_raw_vm_reference(
            binary_script(
                crate::ast::LiteralValue::Integer(1),
                BinaryOperator::Divide,
                crate::ast::LiteralValue::Integer(0),
            ),
            Some("raw-vm-div-zero.hako"),
        )
        .expect("division fault should be a typed process fault");
    assert_eq!(report.status_code(), 70);
    assert_eq!(report.diagnostic_tag(), Some("[process/source-fault]"));
}

#[test]
fn raw_vm_reference_reuses_compiler_after_success() {
    let mut compiler = crate::mir::compiler::MirCompiler::new();
    for source_file in ["raw-vm-reuse-1.hako", "raw-vm-reuse-2.hako"] {
        let report = compiler
            .run_raw_vm_reference(empty_script(), Some(source_file))
            .expect("fresh Raw VM-reference execution should remain reusable");
        assert_eq!(report.status_code(), 0);
        assert_eq!(report.diagnostic_tag(), None);
    }
}

#[test]
fn raw_vm_reference_reuses_compiler_after_entry_rejection() {
    let mut compiler = crate::mir::compiler::MirCompiler::new();
    compiler.builder.repl_mode = true;
    assert!(compiler
        .run_raw_vm_reference(empty_script(), Some("raw-vm-rejected.hako"))
        .is_err());
    compiler.builder.repl_mode = false;
    let report = compiler
        .run_raw_vm_reference(empty_script(), Some("raw-vm-recovered.hako"))
        .expect("entry rejection must not poison the compiler");
    assert_eq!(report.status_code(), 0);
    assert_eq!(report.diagnostic_tag(), None);
}

#[test]
fn owned_run_retains_compile_rejection_evidence() {
    let mut compiler = crate::mir::compiler::MirCompiler::new();
    let rejected = compiler
        .run_raw_vm_reference_owned_v1(RawVmReferenceInvocationV1::narrow_v1(
            app_with_body(vec![ASTNode::Return {
                value: Some(Box::new(ASTNode::Literal {
                    value: crate::ast::LiteralValue::Integer(1),
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            }]),
            Some("raw-vm-owned-rejection.hako"),
        ))
        .expect_err("unsupported Main return stays a compile rejection");
    assert_eq!(rejected.stage(), RawVmReferenceRunStageV1::Compile);
    assert!(matches!(
        rejected.evidence(),
        RawVmReferenceRunEvidenceV1::Compile(compile)
            if compile.stage()
                == crate::mir::compiler::raw_published_compile::RawPublishedCompileStageV1::Eligibility
    ));
    rejected.discard();
}
