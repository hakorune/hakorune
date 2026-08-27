//! PHASE2160-RAW-COMPAT-RUNTIME-BOX-FATE-I0
//!
//! These tests keep the old reentrant fixtures as typed negative evidence.
//! The phase2160 RawCompatibility runtime-Box route must retire before any
//! nested declaration can register symbols, lower fields, or descend into a
//! constructor/method body.  Generic RawInvocation and the legacy facade stay
//! covered by their original sibling tests.

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::builder::module_lowering_invocation::{
    LegacyChildDraftAdmissionV1, ModuleLoweringInvocationV1,
};
use crate::mir::builder::raw_compat_runtime_box_fate::{
    RawCompatibilityRuntimeBoxFateV1, RawRuntimeBoxFateDispositionV1, RuntimeBoxFateScopeV1,
};
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationSourceTransportV1, RawSourceTransportPortV1,
};
use crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1;
use crate::mir::MirBuilder;
use std::collections::HashMap;

fn seeded<'builder>(builder: &'builder mut MirBuilder) -> ModuleLoweringInvocationV1<'builder> {
    builder.root_is_app_mode = Some(false);
    builder.enter_function_for_test("reentrant_parent/0".to_owned());
    ModuleLoweringInvocationV1::open(builder)
}

fn function(name: &str, is_static: bool) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: Vec::new(),
        param_decls: Vec::<ParamDecl>::new(),
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn function_with_body(name: &str, is_static: bool, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: Vec::new(),
        param_decls: Vec::<ParamDecl>::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn nested_box_with_body(name: &str, is_static: bool, body: Vec<ASTNode>) -> ASTNode {
    let method = function_with_body("run", is_static, body);
    let ASTNode::FunctionDeclaration {
        name: method_name, ..
    } = &method
    else {
        unreachable!()
    };
    ASTNode::BoxDeclaration {
        name: name.to_owned(),
        fields: Vec::new(),
        field_decls: Vec::new(),
        public_fields: Vec::new(),
        private_fields: Vec::new(),
        methods: crate::ast::BoxMethodInventoryV1::from_legacy_ast_map(HashMap::from([(
            method_name.clone(),
            method,
        )])),
        constructors: HashMap::new(),
        init_fields: Vec::new(),
        weak_fields: Vec::new(),
        delegates: Vec::new(),
        invariants: Vec::new(),
        transitions: Vec::new(),
        is_interface: false,
        is_record: false,
        is_static,
        extends: Vec::new(),
        implements: Vec::new(),
        type_parameters: Vec::new(),
        is_sync: false,
        static_init: None,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn nested_box(name: &str, is_static: bool) -> ASTNode {
    nested_box_with_body(name, is_static, vec![function_return_value(1)])
}

fn function_return_value(value: i64) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        })),
        span: Span::unknown(),
    }
}

fn nested_box_with_constructor(name: &str) -> ASTNode {
    let mut node = nested_box(name, false);
    let ASTNode::BoxDeclaration { constructors, .. } = &mut node else {
        unreachable!()
    };
    constructors.insert("birth/0".to_owned(), function("birth", false));
    node
}

fn outer_source() -> RawInvocationSourceTransportV1<()> {
    RawInvocationSourceTransportV1::script_root(())
}

#[derive(Debug, PartialEq, Eq)]
struct RuntimeBoxFateSnapshot {
    header_symbols: Vec<String>,
    module_functions: Vec<String>,
    module_metadata: String,
    current_function: Option<(String, usize, usize)>,
    instruction_rows: Vec<String>,
    compilation_context: String,
    metadata_context: String,
    recursion_depth: usize,
}

fn snapshot(invocation: &mut ModuleLoweringInvocationV1<'_>) -> RuntimeBoxFateSnapshot {
    invocation.with_header_port(|builder, headers| {
        let mut header_symbols = Vec::new();
        headers.visit_symbols(&mut |symbol| header_symbols.push(symbol.to_owned()));

        let mut instruction_rows = builder
            .function_state
            .current_function
            .as_ref()
            .expect("current function")
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .map(|row| format!("{row:?}"))
            .collect::<Vec<_>>();
        instruction_rows.sort();

        let current_function = builder
            .function_state
            .current_function
            .as_ref()
            .map(|function| {
                (
                    function.signature.name.clone(),
                    function.blocks.len(),
                    function
                        .blocks
                        .values()
                        .map(|block| block.instructions.len())
                        .sum(),
                )
            });
        let mut module_functions = builder
            .current_module
            .as_ref()
            .map(|module| module.functions.keys().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        module_functions.sort();
        let module_metadata = builder
            .current_module
            .as_ref()
            .map(|module| format!("{:?}", module.metadata))
            .unwrap_or_default();

        RuntimeBoxFateSnapshot {
            header_symbols,
            module_functions,
            module_metadata,
            current_function,
            instruction_rows,
            compilation_context: format!("{:?}", builder.comp_ctx),
            metadata_context: format!(
                "{:?}",
                (
                    builder.metadata_ctx.current_span(),
                    builder.metadata_ctx.current_region_stack()
                )
            ),
            recursion_depth: builder.recursion_depth,
        }
    })
}

fn assert_zero_delta(
    invocation: &mut ModuleLoweringInvocationV1<'_>,
    before: RuntimeBoxFateSnapshot,
) {
    let after = snapshot(invocation);
    assert_eq!(after, before, "retired route must leave no module delta");
}

fn assert_retired_body(body: Vec<ASTNode>, label: &str) {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);
    let before = snapshot(&mut invocation);

    let result = invocation.with_module_port(|builder, module_port| {
        let mut raw_port = RawInvocationChildPortV1::new(module_port);
        raw_port.with_phase2160_raw_compat_runtime_box_fate_v1(|scoped| {
            scoped.with_source_transport_v1(outer_source(), |port, ()| {
                port.capture_static_box_method_pending_v1(
                    builder,
                    "Outer.run/0".to_owned(),
                    Vec::new(),
                    Vec::new(),
                    None,
                    body,
                    Vec::new(),
                    DeclarationAttrs::default(),
                )
                .map(drop)
                .map_err(|error| error.to_string())
            })
        })
    });

    let error = result.expect_err(label);
    assert!(
        error.contains("[freeze:contract][raw-compat/runtime-box-fate-retired/"),
        "unexpected retirement error: {error}"
    );
    assert_zero_delta(&mut invocation, before);
}

fn assert_unarmed_body(body: Vec<ASTNode>, label: &str) {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);
    let result = invocation.with_module_port(|builder, module_port| {
        let pending = {
            let mut raw_port = RawInvocationChildPortV1::new(module_port);
            raw_port.with_source_transport_v1(outer_source(), |port, ()| {
                port.capture_static_box_method_pending_v1(
                    builder,
                    "Outer.run/0".to_owned(),
                    Vec::new(),
                    Vec::new(),
                    None,
                    body,
                    Vec::new(),
                    DeclarationAttrs::default(),
                )
                .map_err(|error| error.to_string())
            })?
        };
        module_port
            .commit_legacy_pending(
                pending,
                LegacyChildDraftAdmissionV1::legacy_symbol("Outer.run/0".into(), 0),
            )
            .map_err(|error| error.to_string())
    });
    result.expect(label);
    invocation.with_header_port(|_builder, headers| {
        assert!(headers.contains_symbol("NestedStatic.run/0"));
        assert!(headers.contains_symbol("Outer.run/0"));
        assert_eq!(headers.symbol_count(), 2);
    });
}

#[test]
fn phase2160_runtime_box_fate_rejects_second_take() {
    let mut fate = RawCompatibilityRuntimeBoxFateV1::issue_retire();
    assert_eq!(
        fate.take_retire().expect("first fate take"),
        RawRuntimeBoxFateDispositionV1::Retire
    );
    assert!(fate
        .take_retire()
        .expect_err("second fate take must reject")
        .contains("raw-compat/runtime-box-fate-second-take"));
}

#[test]
fn generic_raw_invocation_runtime_box_fate_stays_unarmed() {
    let mut scope = RuntimeBoxFateScopeV1::Unarmed;
    assert_eq!(
        scope.take_retire().expect("unarmed fate disposition"),
        RawRuntimeBoxFateDispositionV1::Continue
    );
}

#[test]
fn phase2160_runtime_box_fate_rejects_nested_scope() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);
    let result = invocation.with_module_port(|_builder, module_port| {
        let mut raw_port = RawInvocationChildPortV1::new(module_port);
        raw_port.with_phase2160_raw_compat_runtime_box_fate_v1(|scoped| {
            scoped.with_phase2160_raw_compat_runtime_box_fate_v1(|_| Ok(()))
        })
    });
    assert!(result
        .expect_err("nested phase2160 scope must reject")
        .contains("raw-compat/runtime-box-fate-scope"));
}

#[test]
fn generic_raw_invocation_keeps_unarmed_nested_box_success() {
    assert_unarmed_body(
        vec![nested_box("NestedStatic", true)],
        "generic RawInvocation must keep unarmed nested Box success",
    );
}

#[test]
fn phase2160_raw_compat_static_nested_static_box_is_typed_retire() {
    assert_retired_body(
        vec![nested_box("NestedStatic", true)],
        "static child must retire",
    );
}

#[test]
fn phase2160_raw_compat_static_nested_instance_box_is_typed_retire() {
    assert_retired_body(
        vec![nested_box("NestedInstance", false)],
        "instance child must retire",
    );
}

#[test]
fn phase2160_raw_compat_nested_instance_constructor_is_typed_retire() {
    assert_retired_body(
        vec![nested_box_with_constructor("NestedCtor")],
        "constructor child must retire",
    );
}

#[test]
fn phase2160_raw_compat_nested_depth_three_is_typed_retire() {
    let leaf = nested_box_with_constructor("Leaf");
    let middle = nested_box_with_body("Middle", false, vec![leaf, function_return_value(2)]);
    assert_retired_body(
        vec![middle, function_return_value(3)],
        "depth-three child must retire",
    );
}
