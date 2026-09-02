use std::collections::HashMap;

use super::{
    lower_prepared_raw_function_preflight_with_port_v1, PreparedRawFunctionPreflightRouteV1,
    PreparedRawFunctionPreflightV1, PreparedRawOrdinaryFunctionCompletionV1,
    RawCompatibilityOrdinaryCallTerminalV1, RawOrdinaryFunctionRetirementV1,
};
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::builder::callable_declaration_catalog::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::builder::recursive_child_lowering::{
    AppMainDirectCallDispositionPortV1, RawFunctionHeaderLookupPortV1, RecursiveChildLoweringPortV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::instruction::FastMemRegionId;
use crate::mir::{MirInstruction, TypeOpKind, ValueId};

#[derive(Default)]
struct RecordingPortV1 {
    expression_count: usize,
    events: Vec<&'static str>,
    fail_expression: bool,
}

impl RecursiveChildLoweringPortV1 for RecordingPortV1 {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        unreachable!("FunctionCall route test does not lower a body")
    }

    fn lower_statement(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        unreachable!("FunctionCall route test does not lower a statement")
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        _input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        self.expression_count += 1;
        self.events.push("child");
        if self.fail_expression {
            return Err("direct str child failed".to_owned());
        }
        crate::mir::builder::emission::constant::emit_integer(builder, 7)
    }
}

impl RawFunctionHeaderLookupPortV1 for RecordingPortV1 {
    fn with_function_headers<R>(
        &mut self,
        observe: impl for<'headers> FnOnce(
                Option<
                    &'headers dyn crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1,
                >,
            ) -> R,
    ) -> R {
        self.events.push("header");
        observe(None)
    }
}

// The test port exercises non-App-Main routes.  Keep the new App-Main
// capability explicitly unarmed rather than weakening the production bound.
impl AppMainDirectCallDispositionPortV1 for RecordingPortV1 {}

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    literal(LiteralValue::Integer(value))
}

fn new_box(name: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::New {
        class: name.to_string(),
        type_arguments: Vec::new(),
        arguments,
        field_initializers: Vec::new(),
        span: Span::unknown(),
    }
}

fn static_function(name: &str, arity: usize) -> ASTNode {
    let params = (0..arity).map(|index| format!("arg{index}")).collect();
    let param_decls = (0..arity)
        .map(|index| ParamDecl {
            name: format!("arg{index}"),
            declared_type_name: None,
        })
        .collect();
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params,
        param_decls,
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: None,
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn static_box(owner: &str, methods: &[(&str, usize)]) -> ASTNode {
    let inventory = methods
        .iter()
        .map(|(name, arity)| ((*name).to_owned(), static_function(name, *arity)))
        .collect::<HashMap<_, _>>();
    ASTNode::BoxDeclaration {
        name: owner.to_owned(),
        fields: Vec::new(),
        field_decls: Vec::new(),
        public_fields: Vec::new(),
        private_fields: Vec::new(),
        methods: crate::ast::BoxMethodInventoryV1::from_legacy_ast_map(inventory),
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
    }
}

fn install_catalog(builder: &mut MirBuilder, boxes: Vec<ASTNode>) {
    let source = ASTNode::Program {
        statements: boxes,
        span: Span::unknown(),
    };
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&source).unwrap();
    builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .unwrap();
}

fn raw_root_main_preflight(
    builder: &MirBuilder,
    name: &str,
    arguments: Vec<ASTNode>,
) -> PreparedRawFunctionPreflightV1 {
    PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        builder,
        name.to_owned(),
        arguments,
        crate::mir::builder::calls::RawBrandCallAuthorityV1::RawRootMainParkedCompatibility,
    )
}

#[test]
fn raw_root_main_special_precedence_survives_resolved_retirement() {
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .register_brand_decl("sin".to_string(), "Integer".to_string());
    builder
        .comp_ctx
        .register_brand_decl("isType".to_string(), "Integer".to_string());
    builder
        .comp_ctx
        .register_brand_decl("mem.addr".to_string(), "Integer".to_string());
    builder
        .comp_ctx
        .register_brand_decl("str".to_string(), "Integer".to_string());
    builder.push_fastmem_region(FastMemRegionId::new(6));

    let weak = raw_root_main_preflight(&builder, "weak", vec![integer(1)]);
    assert!(matches!(
        weak.route,
        PreparedRawFunctionPreflightRouteV1::CompatibilityTerminal(
            RawCompatibilityOrdinaryCallTerminalV1::RawRootMainRetired
        )
    ));

    let generic_externcall = raw_root_main_preflight(&builder, "externcall", vec![integer(1)]);
    assert!(matches!(
        generic_externcall.route,
        PreparedRawFunctionPreflightRouteV1::CompatibilityTerminal(
            RawCompatibilityOrdinaryCallTerminalV1::RawRootMainRetired
        )
    ));

    let brand =
        PreparedRawFunctionPreflightV1::prepare(&builder, "sin".to_string(), vec![integer(1)]);
    assert!(matches!(
        brand.route,
        PreparedRawFunctionPreflightRouteV1::Brand(_)
    ));

    for (name, arguments) in [
        (
            "isType",
            vec![
                integer(1),
                literal(LiteralValue::String("Integer".to_string())),
            ],
        ),
        ("mem.addr", vec![integer(1)]),
        ("str", vec![integer(1)]),
    ] {
        let collision =
            PreparedRawFunctionPreflightV1::prepare(&builder, name.to_string(), arguments);
        assert!(matches!(
            collision.route,
            PreparedRawFunctionPreflightRouteV1::Brand(_)
        ));
    }

    let mut builder = MirBuilder::new();
    let typeop = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "isType".to_string(),
        vec![
            integer(1),
            literal(LiteralValue::String("Integer".to_string())),
        ],
    );
    assert!(matches!(
        typeop.route,
        PreparedRawFunctionPreflightRouteV1::TypeOp { .. }
    ));

    let malformed_typeop =
        raw_root_main_preflight(&builder, "isType", vec![integer(1), integer(2)]);
    assert!(matches!(
        malformed_typeop.route,
        PreparedRawFunctionPreflightRouteV1::CompatibilityTerminal(
            RawCompatibilityOrdinaryCallTerminalV1::RawRootMainRetired
        )
    ));

    let math =
        PreparedRawFunctionPreflightV1::prepare(&builder, "sqrt".to_string(), vec![integer(4)]);
    assert!(matches!(
        math.route,
        PreparedRawFunctionPreflightRouteV1::Math { .. }
    ));

    let inactive_fastmem = raw_root_main_preflight(&builder, "mem.addr", vec![integer(1)]);
    assert!(matches!(
        inactive_fastmem.route,
        PreparedRawFunctionPreflightRouteV1::CompatibilityTerminal(
            RawCompatibilityOrdinaryCallTerminalV1::RawRootMainRetired
        )
    ));

    builder.push_fastmem_region(FastMemRegionId::new(7));
    let fastmem =
        PreparedRawFunctionPreflightV1::prepare(&builder, "mem.addr".to_string(), vec![integer(1)]);
    assert!(matches!(
        fastmem.route,
        PreparedRawFunctionPreflightRouteV1::FastMem { .. }
    ));

    let ordinary = raw_root_main_preflight(&builder, "user_function", vec![integer(1)]);
    assert!(matches!(
        ordinary.route,
        PreparedRawFunctionPreflightRouteV1::CompatibilityTerminal(
            RawCompatibilityOrdinaryCallTerminalV1::RawRootMainRetired
        )
    ));

    let str_one =
        PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_string(), vec![integer(1)]);
    assert!(matches!(
        str_one.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: PreparedRawOrdinaryFunctionCompletionV1::StrNormalization { .. }
        }
    ));
    for arguments in [Vec::new(), vec![integer(1), integer(2)]] {
        let wrong_arity = raw_root_main_preflight(&builder, "str", arguments);
        assert!(matches!(
            wrong_arity.route,
            PreparedRawFunctionPreflightRouteV1::CompatibilityTerminal(
                RawCompatibilityOrdinaryCallTerminalV1::RawRootMainRetired
            )
        ));
    }
}

#[test]
fn installed_non_brand_never_reprobes_legacy_brand_map() {
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .register_brand_decl("sin".to_string(), "Integer".to_string());

    let prepared = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "sin".to_string(),
        vec![integer(1)],
        crate::mir::builder::calls::RawBrandCallAuthorityV1::InstalledNonBrand { caller: None },
    );
    assert!(matches!(
        prepared.route,
        PreparedRawFunctionPreflightRouteV1::Math { .. }
    ));
}

#[test]
fn rejecting_routes_precede_children_and_typeop_uses_one_child() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("direct_preflight_order/0".to_string());
    builder
        .comp_ctx
        .register_brand_decl("Meter".to_string(), "Integer".to_string());
    let mut port = RecordingPortV1::default();

    let compatibility_externcall =
        raw_root_main_preflight(&builder, "externcall", vec![integer(1)]);
    assert!(lower_prepared_raw_function_preflight_with_port_v1(
        &mut builder,
        &mut port,
        compatibility_externcall,
    )
    .is_err());
    assert_eq!(port.expression_count, 0);

    for (name, arguments) in [
        ("Meter", Vec::new()),
        ("Meter", vec![integer(1), integer(2)]),
    ] {
        let prepared =
            PreparedRawFunctionPreflightV1::prepare(&builder, name.to_string(), arguments);
        assert!(lower_prepared_raw_function_preflight_with_port_v1(
            &mut builder,
            &mut port,
            prepared,
        )
        .is_err());
        assert_eq!(port.expression_count, 0);
    }

    let typeop = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "asType".to_string(),
        vec![
            integer(1),
            literal(LiteralValue::String("Integer".to_string())),
        ],
    );
    lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, typeop).unwrap();
    assert_eq!(port.expression_count, 1);

    let malformed_typeop =
        raw_root_main_preflight(&builder, "isType", vec![integer(1), integer(2)]);
    let _ = lower_prepared_raw_function_preflight_with_port_v1(
        &mut builder,
        &mut port,
        malformed_typeop,
    );
    assert_eq!(port.expression_count, 3);

    let inactive_fastmem = raw_root_main_preflight(&builder, "mem.addr", vec![integer(1)]);
    let _ = lower_prepared_raw_function_preflight_with_port_v1(
        &mut builder,
        &mut port,
        inactive_fastmem,
    );
    assert_eq!(port.expression_count, 3);

    builder.push_fastmem_region(FastMemRegionId::new(8));
    let unknown_fastmem = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "mem.unknown".to_string(),
        vec![integer(1)],
    );
    let error = lower_prepared_raw_function_preflight_with_port_v1(
        &mut builder,
        &mut port,
        unknown_fastmem,
    )
    .unwrap_err();
    assert!(error.contains("[fastmem/forbidden_call]"));
    assert_eq!(port.expression_count, 3);

    let wrong_arity = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "mem.addr".to_string(),
        vec![integer(1), integer(2)],
    );
    let error =
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, wrong_arity)
            .unwrap_err();
    assert!(error.contains("[fastmem/arity] call=mem.addr expected=1 actual=2"));
    assert_eq!(port.expression_count, 3);
}

#[test]
fn selected_math_and_ordinary_str_keep_child_and_completion_order() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("direct_preflight_completion/0".to_string());
    let mut port = RecordingPortV1::default();

    let math = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "sqrt".to_string(),
        vec![new_box("IntegerBox", vec![integer(9)])],
    );
    lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, math).unwrap();
    assert_eq!(port.expression_count, 1);
    assert_eq!(port.events, vec!["child"]);
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .any(|instruction| matches!(
            instruction,
            MirInstruction::TypeOp {
                op: TypeOpKind::Cast,
                ..
            }
        )));

    let string =
        PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_string(), vec![integer(1)]);
    lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, string).unwrap();
    assert_eq!(port.expression_count, 2);
    assert_eq!(port.events, vec!["child", "child"]);

    port.events.clear();
    let ordinary = raw_root_main_preflight(&builder, "user_function", vec![integer(1)]);
    let _ = lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, ordinary);
    assert!(port.events.is_empty());

    port.events.clear();
    let forged_weak = raw_root_main_preflight(&builder, "weak", vec![integer(1)]);
    let _ =
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, forged_weak);
    assert!(port.events.is_empty());
}

#[test]
fn cataloged_target_preflight_applies_total_shadow_order() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("BoxA.caller/0".to_owned());
    install_catalog(
        &mut builder,
        vec![
            static_box("BoxA", &[("caller", 0), ("run", 1), ("arity", 2)]),
            static_box("BoxB", &[("other", 1)]),
        ],
    );
    let caller = CanonicalSameModuleCallableKeyV1::test_static_box_method("BoxA", "caller", 0);

    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("run".to_owned(), ValueId::new(77));
    let current_owner = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "run".to_owned(),
        vec![integer(1)],
        crate::mir::builder::calls::RawBrandCallAuthorityV1::InstalledNonBrand {
            caller: Some(caller.clone()),
        },
    );
    assert!(matches!(
        current_owner.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: PreparedRawOrdinaryFunctionCompletionV1::Rejected { ref error }
        } if error.contains("bare-static-method-retired")
    ));

    let builtin = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "print".to_owned(),
        vec![integer(1)],
        crate::mir::builder::calls::RawBrandCallAuthorityV1::InstalledNonBrand {
            caller: Some(caller.clone()),
        },
    );
    assert!(matches!(
        builtin.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: PreparedRawOrdinaryFunctionCompletionV1::Retired(
                RawOrdinaryFunctionRetirementV1::BuiltinPrintCataloged
            )
        }
    ));

    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("env.console.log".to_owned(), ValueId::new(91));
    let local_extern = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "env.console.log".to_owned(),
        vec![integer(1)],
        crate::mir::builder::calls::RawBrandCallAuthorityV1::InstalledNonBrand {
            caller: Some(caller.clone()),
        },
    );
    assert!(matches!(
        local_extern.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: PreparedRawOrdinaryFunctionCompletionV1::CatalogedTargeted {
                callee: crate::mir::Callee::Value(value),
                ..
            }
        } if value == ValueId::new(91)
    ));

    let unique_other = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "other".to_owned(),
        vec![integer(1)],
        crate::mir::builder::calls::RawBrandCallAuthorityV1::InstalledNonBrand {
            caller: Some(caller),
        },
    );
    assert!(matches!(
        unique_other.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: PreparedRawOrdinaryFunctionCompletionV1::Rejected { ref error }
        } if error.contains("bare-static-method-retired")
    ));
}

#[test]
fn cataloged_bare_static_rejects_before_children() {
    let caller = CanonicalSameModuleCallableKeyV1::test_static_box_method("BoxA", "caller", 0);

    let missing_catalog = MirBuilder::new();
    let missing = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &missing_catalog,
        "run".to_owned(),
        vec![integer(1)],
        crate::mir::builder::calls::RawBrandCallAuthorityV1::InstalledNonBrand {
            caller: Some(caller.clone()),
        },
    );
    assert!(matches!(
        missing.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: PreparedRawOrdinaryFunctionCompletionV1::Rejected { .. }
        }
    ));

    let mut bare_static = MirBuilder::new();
    bare_static.enter_function_for_test("BoxA.caller/0".to_owned());
    install_catalog(
        &mut bare_static,
        vec![static_box("BoxA", &[("caller", 0), ("run", 2)])],
    );
    bare_static
        .function_state
        .variable_ctx
        .variable_map
        .insert("run".to_owned(), ValueId::new(88));
    let rejected = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &bare_static,
        "run".to_owned(),
        vec![integer(1), integer(2)],
        crate::mir::builder::calls::RawBrandCallAuthorityV1::InstalledNonBrand {
            caller: Some(caller),
        },
    );
    assert!(matches!(
        rejected.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: PreparedRawOrdinaryFunctionCompletionV1::Rejected { ref error }
        } if error.contains("bare-static-method-retired")
    ));

    let foreign = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &bare_static,
        "run".to_owned(),
        vec![integer(1), integer(2)],
        crate::mir::builder::calls::RawBrandCallAuthorityV1::InstalledNonBrand {
            caller: Some(CanonicalSameModuleCallableKeyV1::test_static_box_method(
                "Foreign", "caller", 0,
            )),
        },
    );
    assert!(matches!(
        foreign.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: PreparedRawOrdinaryFunctionCompletionV1::Rejected { ref error }
        } if error.contains("foreign-caller")
    ));

    let mut ambiguous = MirBuilder::new();
    ambiguous.enter_function_for_test("BoxA.caller/0".to_owned());
    install_catalog(
        &mut ambiguous,
        vec![
            static_box("BoxA", &[("caller", 0)]),
            static_box("BoxB", &[("ambig", 1)]),
            static_box("BoxC", &[("ambig", 1)]),
        ],
    );
    let ambiguous = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &ambiguous,
        "ambig".to_owned(),
        vec![integer(1)],
        crate::mir::builder::calls::RawBrandCallAuthorityV1::InstalledNonBrand {
            caller: Some(CanonicalSameModuleCallableKeyV1::test_static_box_method(
                "BoxA", "caller", 0,
            )),
        },
    );
    assert!(matches!(
        ambiguous.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: PreparedRawOrdinaryFunctionCompletionV1::Rejected { ref error }
        } if error.contains("ambiguous-static")
    ));

    let mut port = RecordingPortV1::default();
    assert!(lower_prepared_raw_function_preflight_with_port_v1(
        &mut bare_static,
        &mut port,
        rejected,
    )
    .is_err());
    assert_eq!(port.expression_count, 0);
}

#[test]
fn cataloged_local_value_target_is_consumed_once_before_canonical_call_publication() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("BoxA.caller/0".to_owned());
    install_catalog(&mut builder, vec![static_box("BoxA", &[("caller", 0)])]);
    let caller = CanonicalSameModuleCallableKeyV1::test_static_box_method("BoxA", "caller", 0);
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("run".to_owned(), ValueId::new(88));
    let prepared = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "run".to_owned(),
        vec![integer(1)],
        crate::mir::builder::calls::RawBrandCallAuthorityV1::InstalledNonBrand {
            caller: Some(caller),
        },
    );
    let mut port = RecordingPortV1::default();
    let result =
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, prepared)
            .unwrap();
    assert_eq!(port.expression_count, 1);
    assert_eq!(port.events, vec!["child", "header"]);
    let calls = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter(|instruction| matches!(instruction, MirInstruction::LegacyCallV0 { .. }))
        .collect::<Vec<_>>();
    assert_eq!(calls.len(), 1);
    assert!(matches!(
        calls[0],
        MirInstruction::LegacyCallV0 {
            dst: Some(dst),
            callee: Some(crate::mir::Callee::Value(value)),
            args,
            ..
        } if *dst == result && *value == ValueId::new(88) && args.len() == 1
    ));
}

#[test]
fn direct_str_child_failure_does_not_retry_or_observe_headers_and_reuses_builder() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("direct_str_failure_reuse/0".to_owned());
    let mut port = RecordingPortV1 {
        fail_expression: true,
        ..Default::default()
    };

    let failing =
        PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_owned(), vec![integer(1)]);
    let error =
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, failing)
            .unwrap_err();
    assert_eq!(error, "direct str child failed");
    assert_eq!(port.events, vec!["child"]);

    port.fail_expression = false;
    port.events.clear();
    let succeeding =
        PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_owned(), vec![integer(2)]);
    lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, succeeding)
        .unwrap();
    assert_eq!(port.events, vec!["child"]);
    assert_eq!(port.expression_count, 2);
}

#[path = "function_call_installed_gc_builtin_tests.rs"]
mod installed_gc_builtin_tests;

#[path = "function_call_installed_nonbrand_reject_tests.rs"]
mod installed_nonbrand_reject_tests;

#[path = "function_call_script_compatibility_tests.rs"]
mod script_compatibility_tests;
