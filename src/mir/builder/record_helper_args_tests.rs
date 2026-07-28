use super::*;
use crate::ast::LiteralValue;
use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::builder::recursive_child_lowering::drive_raw_legacy_expression_v1;
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

fn builder_with_catalog(source: &str) -> MirBuilder {
    let root = NyashParser::parse_from_string(source).unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .unwrap();
    builder
}

#[test]
fn setter_boundary_keeps_allowlist_and_body_shape_separate() {
    let valid = [
        field_assign("attempt_count", int_lit(1)),
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
    assert!(is_inlineable_same_module_helper_body(&valid));
    let wrapper = [ASTNode::Return {
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
    assert!(!is_inlineable_same_module_helper_body(&wrapper));
    assert!(MirBuilder::new()
        .prepare_same_module_helper_setter_inline("NotAllowed", "write", &[])
        .unwrap()
        .is_none());
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
    let builder = builder_with_catalog(source);
    for (namespace, owner) in [
        (
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "StaticHelpers",
        ),
        (
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
            "InstanceHelpers",
        ),
    ] {
        let helper = builder
            .prepare_same_module_helper_declaration(namespace, owner, "read", 1)
            .unwrap()
            .unwrap();
        assert_eq!(helper.function_name, format!("{owner}.read/1"));
        assert_eq!(helper.params, ["value"]);
    }
}

fn helper_builder() -> MirBuilder {
    let source = r#"
        box HakoAllocObjectLifecycleAllocResult {
            recordAttempt() {
                me.attempt_count = 1
                return 2
            }
        }"#;
    let mut builder = builder_with_catalog(source);
    builder.enter_function_for_test("catalog_helper_descent/0".to_string());
    builder
}

fn prepare_setter(builder: &MirBuilder) -> PreparedSameModuleHelperSetterInlineV1 {
    builder
        .prepare_same_module_helper_setter_inline(
            "HakoAllocObjectLifecycleAllocResult",
            "recordAttempt",
            &[],
        )
        .unwrap()
        .unwrap()
}

#[derive(Default)]
struct RecordingHelperDescent {
    events: Vec<&'static str>,
    fail: Option<&'static str>,
}

impl MethodCallArgumentDescentV1 for RecordingHelperDescent {
    fn lower_all(&mut self, _: &mut MirBuilder) -> Result<Vec<ValueId>, String> {
        unreachable!("zero-argument helper")
    }

    fn lower_index(&mut self, _: &mut MirBuilder, _: usize) -> Result<ValueId, String> {
        unreachable!("zero-argument helper")
    }

    fn lower_catalog_helper_child(
        &mut self,
        builder: &mut MirBuilder,
        child: CatalogHelperChildV1,
    ) -> Result<ValueId, String> {
        match child {
            CatalogHelperChildV1::Statement(_) => {
                self.events.push("statement");
                if self.fail == Some("statement") {
                    return Err("injected helper statement failure".to_string());
                }
                crate::mir::builder::emission::constant::emit_void(builder)
            }
            CatalogHelperChildV1::Expression(expression) => {
                self.events.push("expression");
                if self.fail == Some("expression") {
                    return Err("injected helper expression failure".to_string());
                }
                drive_raw_legacy_expression_v1(builder, expression)
            }
        }
    }
}

fn assert_no_helper_terminal(builder: &MirBuilder) {
    assert!(!current_function(builder).blocks.values().any(|block| {
        block
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Call { .. }))
            || matches!(block.terminator, Some(MirInstruction::Return { .. }))
    }));
}

fn current_function(builder: &MirBuilder) -> &MirFunction {
    builder.function_state.current_function.as_ref().unwrap()
}

fn execute_setter(
    builder: &mut MirBuilder,
    descent: &mut RecordingHelperDescent,
) -> Result<ValueId, String> {
    let prepared = prepare_setter(builder);
    builder.execute_prepared_same_module_helper_setter_inline(
        prepared,
        &[],
        Some(ValueId::new(90)),
        descent,
    )
}

#[test]
fn helper_body_continuity_failure_restore_and_reuse() {
    let mut builder = helper_builder();
    assert!(current_function(&builder).blocks[&BasicBlockId::new(0)]
        .instructions
        .is_empty());
    let mut descent = RecordingHelperDescent::default();
    let result = execute_setter(&mut builder, &mut descent).unwrap();
    assert_eq!(descent.events, ["statement", "expression"]);
    assert!(current_function(&builder)
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .any(|inst| matches!(inst, MirInstruction::Const { dst, value: crate::mir::ConstValue::Integer(2) } if *dst == result)));
    assert_no_helper_terminal(&builder);

    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("outer".to_string(), ValueId::new(7));
    let saved = builder.function_state.variable_ctx.variable_map.clone();
    let mut descent = RecordingHelperDescent::default();
    for fail in ["statement", "expression"] {
        descent.fail = Some(fail);
        assert!(execute_setter(&mut builder, &mut descent).is_err());
        assert_eq!(builder.function_state.variable_ctx.variable_map, saved);
    }
    descent.fail = None;
    execute_setter(&mut builder, &mut descent).unwrap();
    assert_eq!(builder.function_state.variable_ctx.variable_map, saved);
    assert_no_helper_terminal(&builder);
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
