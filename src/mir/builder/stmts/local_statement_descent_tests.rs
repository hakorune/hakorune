use std::cell::RefCell;

use crate::ast::{ASTNode, FieldDecl, LiteralValue, Span};
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::{MirBuilder, MirInstruction, ValueId};

use super::super::recursive_child_lowering::RecursiveChildLoweringPortV1;
use super::local_statement_descent::{
    drive_local_statement_v1, LocalStatementDescentPortV1, LocalStatementSyntaxViewV1,
    RawLegacyLocalInputV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventV1 {
    Syntax,
    Input(usize),
    Lower(usize),
    TypedArray(usize),
    Record(usize),
}

#[derive(Clone)]
struct LocalInputV1 {
    variables: Vec<String>,
    initial_values: Vec<Option<Box<ASTNode>>>,
    declared_type_names: Vec<Option<String>>,
}

struct RecordingLocalPortV1 {
    events: RefCell<Vec<EventV1>>,
    fail_syntax: bool,
    fail_input: Option<usize>,
    fail_lower: Option<usize>,
    fail_typed_array: bool,
    fail_record: bool,
}

impl RecordingLocalPortV1 {
    fn accepting() -> Self {
        Self {
            events: RefCell::new(Vec::new()),
            fail_syntax: false,
            fail_input: None,
            fail_lower: None,
            fail_typed_array: false,
            fail_record: false,
        }
    }

    fn events(&self) -> Vec<EventV1> {
        self.events.borrow().clone()
    }
}

impl RecursiveChildLoweringPortV1 for RecordingLocalPortV1 {
    type BodyInput = ();
    type StatementInput = ();
    type ExpressionInput = usize;

    fn lower_body(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        Err("unexpected body descent".to_string())
    }

    fn lower_statement(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        Err("unexpected statement descent".to_string())
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        index: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        self.events.borrow_mut().push(EventV1::Lower(index));
        if self.fail_lower == Some(index) {
            return Err(format!("lower-{index}"));
        }
        crate::mir::builder::emission::constant::emit_integer(builder, index as i64 + 10)
    }
}

impl LocalStatementDescentPortV1 for RecordingLocalPortV1 {
    type LocalInput = LocalInputV1;

    fn local_syntax<'input>(
        &self,
        input: &'input Self::LocalInput,
    ) -> Result<LocalStatementSyntaxViewV1<'input>, String> {
        self.events.borrow_mut().push(EventV1::Syntax);
        if self.fail_syntax {
            return Err("syntax".to_string());
        }
        Ok(LocalStatementSyntaxViewV1::new(
            &input.variables,
            &input.initial_values,
            &input.declared_type_names,
        ))
    }

    fn lower_ordinary_initializer(
        &mut self,
        builder: &mut MirBuilder,
        _input: &mut Self::LocalInput,
        index: usize,
    ) -> Result<ValueId, String> {
        self.events.borrow_mut().push(EventV1::Input(index));
        if self.fail_input == Some(index) {
            return Err(format!("input-{index}"));
        }
        self.lower_expression(builder, index)
    }

    fn lower_typed_array_literal_initializer(
        &mut self,
        builder: &mut MirBuilder,
        input: &mut Self::LocalInput,
        index: usize,
    ) -> Result<(ValueId, String), String> {
        self.events.borrow_mut().push(EventV1::TypedArray(index));
        if self.fail_typed_array {
            return Err("typed-array".to_string());
        }
        let ASTNode::ArrayLiteral { elements, .. } = input.initial_values[index]
            .as_deref()
            .expect("typed array initializer")
        else {
            unreachable!("typed array hook selection")
        };
        builder.build_typed_array_literal(elements.to_vec())
    }

    fn lower_record_constructor_initializer(
        &mut self,
        builder: &mut MirBuilder,
        input: &mut Self::LocalInput,
        index: usize,
        class: &str,
    ) -> Result<ValueId, String> {
        self.events.borrow_mut().push(EventV1::Record(index));
        if self.fail_record {
            return Err("record".to_string());
        }
        let ASTNode::New { arguments, .. } = input.initial_values[index]
            .as_deref()
            .expect("record initializer")
        else {
            unreachable!("record hook selection")
        };
        builder.build_record_constructor_value(class.to_string(), arguments.to_vec())
    }
}

#[derive(Default)]
struct RecordingRawAstPortV1 {
    integer_children: Vec<i64>,
}

impl RecursiveChildLoweringPortV1 for RecordingRawAstPortV1 {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        Err("unexpected body descent".to_string())
    }

    fn lower_statement(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        Err("unexpected statement descent".to_string())
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        let ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } = input
        else {
            return Err("unexpected non-integer child".to_string());
        };
        self.integer_children.push(value);
        crate::mir::builder::emission::constant::emit_integer(builder, value)
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn local_input(
    variables: &[&str],
    initial_values: Vec<Option<Box<ASTNode>>>,
    declared_type_names: Vec<Option<&str>>,
) -> LocalInputV1 {
    LocalInputV1 {
        variables: variables.iter().map(|name| (*name).to_string()).collect(),
        initial_values,
        declared_type_names: declared_type_names
            .into_iter()
            .map(|name| name.map(str::to_string))
            .collect(),
    }
}

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn instruction_count(builder: &MirBuilder) -> usize {
    builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .map(|block| block.instructions.len())
        .sum()
}

#[test]
fn ordinary_initializers_preflight_then_descend_in_index_order_and_complete_once() {
    let mut builder = builder("lcl0_order/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    let input = local_input(
        &["x", "y"],
        vec![Some(Box::new(integer(1))), Some(Box::new(integer(2)))],
        vec![None, None],
    );
    let mut port = RecordingLocalPortV1::accepting();

    drive_local_statement_v1(&mut builder, &mut port, input).unwrap();

    assert_eq!(
        port.events(),
        vec![
            EventV1::Syntax,
            EventV1::Input(0),
            EventV1::Lower(0),
            EventV1::Input(1),
            EventV1::Lower(1),
        ]
    );
    assert!(builder.function_state.binding_ctx.contains("x"));
    assert!(builder.function_state.binding_ctx.contains("y"));
}

#[test]
fn later_exact_numeric_missing_initializer_rejects_before_first_child_effect() {
    let mut builder = builder("lcl0_preflight/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    let input = local_input(
        &["x", "y"],
        vec![Some(Box::new(integer(1))), None],
        vec![None, Some("i64")],
    );
    let mut port = RecordingLocalPortV1::accepting();

    let error = drive_local_statement_v1(&mut builder, &mut port, input).unwrap_err();

    assert!(error.contains("local_contract_uninitialized_forbidden"));
    assert_eq!(port.events(), vec![EventV1::Syntax]);
    assert_eq!(instruction_count(&builder), 0);
    assert!(!builder.function_state.binding_ctx.contains("x"));
}

#[test]
fn later_typed_array_declaration_rejects_before_first_child_effect() {
    for (declared_type, expected_error) in [
        ("Array<u8>", "local_contract_uninitialized_forbidden"),
        ("Array<String>", "typed_array_contract_unsupported_element"),
    ] {
        let mut builder = builder("lcl0_typed_array_preflight/0");
        let _scope = LexicalScopeGuard::new(&mut builder);
        let second_initializer = if declared_type == "Array<u8>" {
            None
        } else {
            Some(Box::new(integer(2)))
        };
        let input = local_input(
            &["x", "ys"],
            vec![Some(Box::new(integer(1))), second_initializer],
            vec![None, Some(declared_type)],
        );
        let mut port = RecordingLocalPortV1::accepting();

        let error = drive_local_statement_v1(&mut builder, &mut port, input).unwrap_err();

        assert!(error.contains(expected_error), "{error}");
        assert_eq!(port.events(), vec![EventV1::Syntax]);
        assert_eq!(instruction_count(&builder), 0);
        assert!(!builder.function_state.binding_ctx.contains("x"));
    }
}

#[test]
fn untyped_missing_initializer_uses_null_without_child_demand() {
    let mut builder = builder("lcl0_null/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    let input = local_input(&["x"], vec![None], vec![None]);
    let mut port = RecordingLocalPortV1::accepting();

    drive_local_statement_v1(&mut builder, &mut port, input).unwrap();

    assert_eq!(port.events(), vec![EventV1::Syntax]);
    assert!(builder.function_state.binding_ctx.contains("x"));
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .any(|instruction| matches!(instruction, MirInstruction::Const { .. })));
}

#[test]
fn syntax_failure_precedes_preflight_or_initializer_effects() {
    let mut builder = builder("lcl0_syntax_failure/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    let input = local_input(&["x"], vec![Some(Box::new(integer(1)))], vec![None]);
    let mut port = RecordingLocalPortV1::accepting();
    port.fail_syntax = true;

    let error = drive_local_statement_v1(&mut builder, &mut port, input).unwrap_err();

    assert_eq!(error, "syntax");
    assert_eq!(port.events(), vec![EventV1::Syntax]);
    assert_eq!(instruction_count(&builder), 0);
    assert!(!builder.function_state.binding_ctx.contains("x"));
}

#[test]
fn initializer_input_and_child_failures_publish_no_binding_or_later_initializer() {
    for fail_input in [true, false] {
        let mut builder = builder(if fail_input {
            "lcl0_input_failure/0"
        } else {
            "lcl0_child_failure/0"
        });
        let _scope = LexicalScopeGuard::new(&mut builder);
        let input = local_input(
            &["x", "y", "z"],
            vec![
                Some(Box::new(integer(1))),
                Some(Box::new(integer(2))),
                Some(Box::new(integer(3))),
            ],
            vec![None, None, None],
        );
        let mut port = RecordingLocalPortV1::accepting();
        if fail_input {
            port.fail_input = Some(1);
        } else {
            port.fail_lower = Some(1);
        }

        let error = drive_local_statement_v1(&mut builder, &mut port, input).unwrap_err();

        assert!(error.contains(if fail_input { "input-1" } else { "lower-1" }));
        assert!(!port.events().contains(&EventV1::Input(2)));
        assert!(!port.events().contains(&EventV1::Lower(2)));
        assert!(!builder.function_state.binding_ctx.contains("x"));
        assert!(!builder.function_state.binding_ctx.contains("y"));
        assert!(!builder.function_state.binding_ctx.contains("z"));
    }
}

#[test]
fn typed_array_special_hook_precedes_direct_builder_effects_and_preclaim_reaches_local() {
    let input = local_input(
        &["xs"],
        vec![Some(Box::new(ASTNode::ArrayLiteral {
            elements: vec![integer(1), integer(2)],
            span: Span::unknown(),
        }))],
        vec![Some("Array<u8>")],
    );

    let mut rejected_builder = builder("lcl0_array_reject/0");
    let _rejected_scope = LexicalScopeGuard::new(&mut rejected_builder);
    let mut rejected_port = RecordingLocalPortV1::accepting();
    rejected_port.fail_typed_array = true;
    drive_local_statement_v1(&mut rejected_builder, &mut rejected_port, input.clone()).unwrap_err();
    assert_eq!(
        rejected_port.events(),
        vec![EventV1::Syntax, EventV1::TypedArray(0)]
    );
    assert_eq!(instruction_count(&rejected_builder), 0);
    assert!(!rejected_builder.function_state.binding_ctx.contains("xs"));

    let mut accepted_builder = builder("lcl0_array_accept/0");
    let _accepted_scope = LexicalScopeGuard::new(&mut accepted_builder);
    let mut accepted_port = RecordingLocalPortV1::accepting();
    drive_local_statement_v1(&mut accepted_builder, &mut accepted_port, input).unwrap();
    assert!(accepted_builder.function_state.binding_ctx.contains("xs"));
    assert!(accepted_builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .any(|instruction| matches!(instruction, MirInstruction::ArrayStateContractClaim { .. })));
}

#[test]
fn record_special_hook_precedes_constructor_effects() {
    let mut builder = builder("lcl0_record/0");
    builder.comp_ctx.register_record_decl(
        "Pair".to_string(),
        Vec::new(),
        &[FieldDecl {
            name: "value".to_string(),
            declared_type_name: None,
            is_weak: false,
            default_value: None,
        }],
    );
    let _scope = LexicalScopeGuard::new(&mut builder);
    let input = local_input(
        &["pair"],
        vec![Some(Box::new(ASTNode::New {
            class: "Pair".to_string(),
            arguments: vec![integer(1)],
            type_arguments: Vec::new(),
            field_initializers: Vec::new(),
            span: Span::unknown(),
        }))],
        vec![None],
    );
    let mut port = RecordingLocalPortV1::accepting();
    port.fail_record = true;

    let error = drive_local_statement_v1(&mut builder, &mut port, input).unwrap_err();

    assert!(error.contains("record"));
    assert_eq!(port.events(), vec![EventV1::Syntax, EventV1::Record(0)]);
    assert_eq!(instruction_count(&builder), 0);
    assert!(!builder.function_state.binding_ctx.contains("pair"));
}

#[test]
fn raw_special_initializers_retain_one_caller_port_for_every_child() {
    let mut builder = builder("lcl0_special_port_continuity/0");
    builder.comp_ctx.register_record_decl(
        "Pair".to_string(),
        Vec::new(),
        &[FieldDecl {
            name: "value".to_string(),
            declared_type_name: None,
            is_weak: false,
            default_value: None,
        }],
    );
    let _scope = LexicalScopeGuard::new(&mut builder);
    let input = RawLegacyLocalInputV1::new(ASTNode::Local {
        variables: vec!["xs".to_string(), "pair".to_string()],
        initial_values: vec![
            Some(Box::new(ASTNode::ArrayLiteral {
                elements: vec![integer(1), integer(2)],
                span: Span::unknown(),
            })),
            Some(Box::new(ASTNode::New {
                class: "Pair".to_string(),
                arguments: vec![integer(7)],
                type_arguments: Vec::new(),
                field_initializers: Vec::new(),
                span: Span::unknown(),
            })),
        ],
        declared_type_names: vec![Some("Array<u8>".to_string()), None],
        span: Span::unknown(),
    });
    let mut port = RecordingRawAstPortV1::default();

    drive_local_statement_v1(&mut builder, &mut port, input).unwrap();

    assert_eq!(port.integer_children, vec![1, 2, 7]);
    assert!(builder.function_state.binding_ctx.contains("xs"));
    assert!(builder.function_state.binding_ctx.contains("pair"));
}
