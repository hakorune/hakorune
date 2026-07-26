use super::*;
use crate::ast::LiteralValue;
use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction};
use crate::parser::NyashParser;

fn span() -> crate::ast::Span {
    crate::ast::Span::unknown()
}

fn field_assign(field: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(ASTNode::FieldAccess {
            object: Box::new(ASTNode::Me { span: span() }),
            field: field.to_string(),
            span: span(),
        }),
        value: Box::new(value),
        span: span(),
    }
}

fn int_lit(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: span(),
    }
}

#[test]
fn inlineable_setter_accepts_simple_assignment_and_return() {
    let body = vec![
        field_assign(
            "attempt_count",
            ASTNode::BinaryOp {
                operator: crate::ast::BinaryOperator::Add,
                left: Box::new(ASTNode::FieldAccess {
                    object: Box::new(ASTNode::Me { span: span() }),
                    field: "attempt_count".to_string(),
                    span: span(),
                }),
                right: Box::new(int_lit(1)),
                span: span(),
            },
        ),
        ASTNode::Return {
            value: Some(Box::new(int_lit(1))),
            span: span(),
        },
    ];

    assert!(is_inlineable_same_module_helper_key(
        "HakoAllocObjectLifecycleAllocResult",
        "recordAttempt",
        0
    ));
    assert!(is_inlineable_same_module_helper_body(&body));
}

#[test]
fn inlineable_setter_rejects_wrapper_call_body() {
    let body = vec![ASTNode::Return {
        value: Some(Box::new(ASTNode::FunctionCall {
            name: "other".to_string(),
            arguments: Vec::new(),
            span: span(),
        })),
        span: span(),
    }];

    assert!(!is_inlineable_same_module_helper_key(
        "HakoAllocObjectLifecycleFacade",
        "recordSmallAllocFailure",
        1
    ));
    assert!(!is_inlineable_same_module_helper_body(&body));
}

#[test]
fn structured_catalog_lookup_preserves_static_and_instance_namespaces() {
    let source = r#"
        static box StaticHelpers {
            read(value) { return value }
        }
        box InstanceHelpers {
            read(value) { return value }
        }
    "#;
    let root = NyashParser::parse_from_string(source).unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .unwrap();

    let static_helper = builder
        .prepare_same_module_helper_declaration(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "StaticHelpers",
            "read",
            1,
        )
        .unwrap()
        .unwrap();
    assert_eq!(static_helper.function_name, "StaticHelpers.read/1");
    assert_eq!(static_helper.params, ["value"]);

    let instance_helper = builder
        .prepare_same_module_helper_declaration(
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
            "InstanceHelpers",
            "read",
            1,
        )
        .unwrap()
        .unwrap();
    assert_eq!(instance_helper.function_name, "InstanceHelpers.read/1");
    assert_eq!(instance_helper.params, ["value"]);
}

#[test]
fn setter_allowlist_rejects_before_catalog_query() {
    let mut builder = MirBuilder::new();
    assert_eq!(
        builder
            .try_inline_same_module_helper_setter_call("NotAllowed", "write", &[], None,)
            .unwrap(),
        None
    );
}

#[test]
fn helper_setter_completion_bypasses_generic_terminal() {
    let source = r#"
        box HakoAllocObjectLifecycleAllocResult {
            recordAttempt() { return 1 }
        }
    "#;
    let root = NyashParser::parse_from_string(source).unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .unwrap();
    builder.enter_function_for_test("record_setter_custom_terminal/0".to_string());

    let result = builder
        .try_inline_same_module_helper_setter_call(
            "HakoAllocObjectLifecycleAllocResult",
            "recordAttempt",
            &[],
            None,
        )
        .unwrap()
        .expect("allowlisted setter must complete through its custom owner");
    let instructions = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .collect::<Vec<_>>();

    assert!(instructions.iter().any(|instruction| matches!(
        instruction,
        MirInstruction::Const {
            dst,
            value: crate::mir::ConstValue::Integer(1),
        } if *dst == result
    )));
    assert!(!instructions
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::Call { .. })));
}

#[test]
fn prepared_setter_has_no_builder_effect_until_execution() {
    let source = r#"
        box HakoAllocObjectLifecycleAllocResult {
            recordAttempt() { return 1 }
        }
    "#;
    let root = NyashParser::parse_from_string(source).unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .unwrap();
    builder.enter_function_for_test("prepared_record_setter/0".to_string());

    let prepared = builder
        .prepare_same_module_helper_setter_inline(
            "HakoAllocObjectLifecycleAllocResult",
            "recordAttempt",
            &[],
        )
        .unwrap()
        .expect("allowlisted setter must prepare");
    let instruction_count = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .map(|block| block.instructions.len())
        .sum::<usize>();
    assert_eq!(instruction_count, 0, "prepare must not emit MIR");

    let mut descent = LegacyMethodCallArgumentsV1::new(&[]);
    let result = builder
        .execute_prepared_same_module_helper_setter_inline(prepared, &[], None, &mut descent)
        .unwrap();
    let instructions = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .collect::<Vec<_>>();
    assert!(instructions.iter().any(|instruction| matches!(
        instruction,
        MirInstruction::Const {
            dst,
            value: crate::mir::ConstValue::Integer(1),
        } if *dst == result
    )));
}

#[test]
fn infer_same_module_helper_receiver_box_name_follows_phi_inputs_without_hint() {
    let signature = FunctionSignature {
        name: "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    let block = function
        .get_block_mut(BasicBlockId::new(0))
        .expect("entry block");
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "FooBox".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    block.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(3),
        inputs: vec![
            (BasicBlockId::new(0), ValueId::new(1)),
            (BasicBlockId::new(0), ValueId::new(2)),
        ],
        type_hint: None,
    });

    let mut builder = MirBuilder::new();
    builder.function_state.current_function = Some(function);

    assert_eq!(
        builder
            .infer_same_module_helper_receiver_box_name(ValueId::new(3))
            .as_deref(),
        Some("FooBox")
    );
}
