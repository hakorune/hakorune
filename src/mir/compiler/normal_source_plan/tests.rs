use std::collections::HashMap;

use crate::ast::{
    ASTNode, DeclarationAttrs, EnumVariantDecl, FieldDecl, LiteralValue, RuneAttr, Span,
};

use super::test_support::*;
use super::*;

fn integer_local_return_function(
    name: &str,
    local_name: &str,
    declared_type: Option<&str>,
    initializer: Option<ASTNode>,
    returned_name: &str,
) -> ASTNode {
    function_with_body(
        name,
        integer_local_return_body(local_name, declared_type, initializer, returned_name),
        false,
    )
}

fn main_box(methods: Vec<(&str, ASTNode)>, is_static: bool) -> ASTNode {
    ASTNode::BoxDeclaration {
        name: "Main".to_owned(),
        fields: Vec::new(),
        field_decls: Vec::new(),
        public_fields: Vec::new(),
        private_fields: Vec::new(),
        methods: methods
            .into_iter()
            .map(|(name, method)| (name.to_owned(), method))
            .collect::<HashMap<_, _>>(),
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
        is_static,
        static_init: None,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn main_only() -> ASTNode {
    main_box(vec![("main", function("main", 0, true))], true)
}

fn instance_box(name: &str, methods: Vec<(&str, ASTNode)>) -> ASTNode {
    ASTNode::BoxDeclaration {
        name: name.to_owned(),
        fields: vec!["value".to_owned()],
        field_decls: vec![FieldDecl {
            name: "value".to_owned(),
            declared_type_name: Some("Integer".to_owned()),
            is_weak: false,
            default_value: Some(Box::new(literal(1))),
        }],
        public_fields: vec!["value".to_owned()],
        private_fields: Vec::new(),
        methods: methods
            .into_iter()
            .map(|(method_name, method)| (method_name.to_owned(), method))
            .collect(),
        constructors: HashMap::new(),
        init_fields: vec!["value".to_owned()],
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
        is_static: false,
        static_init: None,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn seal(source: ASTNode) -> Result<SealedNormalSourcePlanV1, RejectedNormalSourcePlanV1> {
    NormalSourcePlanClassifierV1::seal(input(source))
}

fn seal_module(
    source: ASTNode,
) -> Result<VerifiedNormalModuleSourceV1, RejectedNormalModuleSourceV1> {
    let inventory = inventory::NormalSourceSurfaceInventoryV1::collect(input(source))
        .expect("Program inventory");
    VerifiedNormalModuleSourceV1::seal(inventory)
}

fn assert_error(source: ASTNode, expected: NormalSourcePlanErrorV1) {
    let rejected = seal(source).unwrap_err();
    assert_eq!(rejected.error(), &expected);
    rejected.discard();
}

#[test]
fn empty_and_scalar_programs_are_scripts() {
    for source in [program(Vec::new()), program(vec![literal(42)])] {
        assert!(matches!(
            seal(source).unwrap(),
            SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(_))
        ));
    }
}

#[test]
fn sealed_script_source_consumes_into_the_shared_recipe_once() {
    let SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(script)) =
        seal(program(vec![literal(42)])).expect("Script source plan")
    else {
        panic!("expected Script source family")
    };

    let recipe = script
        .prepare_script_recipe()
        .expect("shared Script recipe");

    assert_eq!(recipe.source_identity(), "normal-source-plan0-test");
    assert_eq!(recipe.retained_source_statement_count(), 1);
    assert!(matches!(
        recipe.recipe().terminal(),
        crate::mir::raw_root_body_recipe::RawScriptTerminalRecipeV1::ValueExpression(_)
    ));
}

#[test]
fn main_zero_only_is_a_scalar_main_root() {
    assert!(matches!(
        seal(program(vec![main_only()])).unwrap(),
        SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(_))
    ));
}

#[test]
fn top_level_or_main_box_helpers_make_callable_modules() {
    let top_level_helper = program(vec![main_only(), function("helper", 1, true)]);
    let main_box_helper = program(vec![main_box(
        vec![
            ("zeta", function("zeta", 0, true)),
            ("main", function("main", 0, true)),
            ("alpha", function("alpha", 1, true)),
        ],
        true,
    )]);
    assert!(matches!(
        seal(top_level_helper).unwrap(),
        SealedNormalSourcePlanV1::CallableModule(_)
    ));
    let SealedNormalSourcePlanV1::CallableModule(module) = seal(main_box_helper).unwrap() else {
        panic!("expected callable module")
    };
    let helper_keys = module
        .additional_callables()
        .iter()
        .filter_map(|site| match site {
            product::NormalAdditionalCallableSiteV1::MainMethod(site) => Some(site.method_key()),
            product::NormalAdditionalCallableSiteV1::TopLevel(_) => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(helper_keys, ["alpha", "zeta"]);
}

#[test]
fn function_only_program_has_no_source_entry() {
    assert_error(
        program(vec![function("helper", 1, true)]),
        NormalSourcePlanErrorV1::MissingSourceEntry,
    );
}

#[test]
fn script_mixed_with_main_or_function_is_rejected_in_either_order() {
    let cases = [
        vec![literal(1), main_only()],
        vec![main_only(), literal(1)],
        vec![literal(1), function("helper", 1, true)],
        vec![function("helper", 1, true), literal(1)],
    ];
    for statements in cases {
        assert_error(
            program(statements),
            NormalSourcePlanErrorV1::MixedSourceFamilies,
        );
    }
}

#[test]
fn duplicate_main_is_rejected_in_either_order() {
    let cases = [
        vec![
            main_only(),
            main_box(vec![("main", function("main", 1, true))], true),
        ],
        vec![
            main_box(vec![("main", function("main", 1, true))], true),
            main_only(),
        ],
    ];
    for statements in cases {
        assert_error(program(statements), NormalSourcePlanErrorV1::DuplicateMain);
    }
}

#[test]
fn main_must_be_static_and_define_static_main_zero() {
    assert_error(
        program(vec![main_box(
            vec![("main", function("main", 0, false))],
            false,
        )]),
        NormalSourcePlanErrorV1::MainMustBeStatic,
    );
    assert_error(
        program(vec![main_box(
            vec![("helper", function("helper", 0, true))],
            true,
        )]),
        NormalSourcePlanErrorV1::MainMethodMissing,
    );
    assert_error(
        program(vec![main_box(
            vec![("main", function("main", 1, true))],
            true,
        )]),
        NormalSourcePlanErrorV1::MainArityMismatch { actual: 1 },
    );
}

#[test]
fn unsupported_declaration_is_rejected_before_family_selection() {
    let unsupported = ASTNode::EnumDeclaration {
        name: "Flag".to_owned(),
        variants: vec![EnumVariantDecl {
            name: "Off".to_owned(),
            payload_type_name: None,
            record_field_decls: Vec::new(),
            tuple_payload_type_names: Vec::new(),
        }],
        type_parameters: Vec::new(),
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    assert_error(
        program(vec![unsupported, main_only()]),
        NormalSourcePlanErrorV1::UnsupportedTopLevelSurface {
            statement_index: 0,
            kind: rejection::NormalUnsupportedTopLevelKindV1::Enum,
        },
    );
}

#[test]
fn main_with_plain_instance_box_seals_module_source() {
    let module = seal_module(program(vec![
        instance_box("Page", vec![("render", function("render", 1, false))]),
        main_only(),
    ]))
    .expect("finite module source");

    assert_eq!(module.source_identity(), "normal-source-plan0-test");
    assert_eq!(module.main_statement_index(), 1);
    assert_eq!(module.main_arity(), 0);
    assert_eq!(module.instance_boxes().len(), 1);
    assert_eq!(module.instance_boxes()[0].statement_index(), 0);
    assert_eq!(module.instance_boxes()[0].name(), "Page");
    assert_eq!(module.callable_catalog().len(), 2);
    assert!(module
        .callable_catalog()
        .declaration_for(
            crate::mir::builder::SameModuleCallableNamespaceV1::InstanceBoxMethod,
            "Page",
            "render",
            1,
        )
        .is_some());
}

#[test]
fn multiple_instance_boxes_preserve_source_order() {
    let module = seal_module(program(vec![
        instance_box("Zeta", Vec::new()),
        main_only(),
        instance_box("Alpha", Vec::new()),
    ]))
    .expect("ordered module source");

    let rows = module
        .instance_boxes()
        .iter()
        .map(|site| (site.statement_index(), site.name()))
        .collect::<Vec<_>>();
    assert_eq!(rows, [(0, "Zeta"), (2, "Alpha")]);
    assert_eq!(module.callable_catalog().len(), 1);
}

#[test]
fn instance_method_catalog_correspondence_is_exact() {
    let module = seal_module(program(vec![
        instance_box(
            "Page",
            vec![
                ("zeta", function("zeta", 0, false)),
                ("alpha", function("alpha", 2, false)),
            ],
        ),
        main_only(),
    ]))
    .expect("exact callable correspondence");

    let keys = module
        .callable_catalog()
        .keys()
        .map(|key| {
            (
                key.namespace(),
                key.owner().to_owned(),
                key.name().to_owned(),
                key.arity(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(keys.len(), 3);
    assert_eq!(keys[0].1, "Main");
    assert_eq!(keys[1].1, "Page");
    assert_eq!(keys[1].2, "alpha");
    assert_eq!(keys[2].2, "zeta");
}

#[test]
fn explicit_constructor_is_rejected_before_builder() {
    let mut page = instance_box("Page", Vec::new());
    let ASTNode::BoxDeclaration { constructors, .. } = &mut page else {
        unreachable!()
    };
    constructors.insert("init/0".to_owned(), function("init", 0, false));

    let rejected = seal_module(program(vec![page, main_only()])).unwrap_err();
    assert_eq!(rejected.stage(), NormalModuleSourceStageV1::BoxShape);
    assert_eq!(
        rejected.error(),
        &NormalModuleSourceErrorV1::BoxShape {
            statement_index: 0,
            cause: NormalModuleBoxSourceErrorV1::ConstructorUnsupported,
        }
    );
    rejected.discard();
}

#[test]
fn static_method_inside_instance_box_is_rejected() {
    let rejected = seal_module(program(vec![
        instance_box("Page", vec![("make", function("make", 0, true))]),
        main_only(),
    ]))
    .unwrap_err();
    assert_eq!(
        rejected.error(),
        &NormalModuleSourceErrorV1::BoxShape {
            statement_index: 0,
            cause: NormalModuleBoxSourceErrorV1::StaticMethod,
        }
    );
    rejected.discard();
}

#[test]
fn top_level_function_or_runtime_statement_is_rejected() {
    for extra in [function("helper", 0, true), literal(1)] {
        let rejected = seal_module(program(vec![
            instance_box("Page", Vec::new()),
            main_only(),
            extra,
        ]))
        .unwrap_err();
        assert_eq!(rejected.stage(), NormalModuleSourceStageV1::Family);
        rejected.discard();
    }
}

#[test]
fn existing_exact_classifier_still_rejects_non_main_box() {
    assert_error(
        program(vec![instance_box("Page", Vec::new()), main_only()]),
        NormalSourcePlanErrorV1::UnsupportedTopLevelSurface {
            statement_index: 0,
            kind: rejection::NormalUnsupportedTopLevelKindV1::Box,
        },
    );
}

#[test]
fn rejection_retains_source_identity_and_has_no_retry() {
    let mut page = instance_box("Page", Vec::new());
    let ASTNode::BoxDeclaration { constructors, .. } = &mut page else {
        unreachable!()
    };
    constructors.insert("init/0".to_owned(), function("init", 0, false));
    let rejected = seal_module(program(vec![page, main_only()])).unwrap_err();
    assert_eq!(rejected.source_identity(), "normal-source-plan0-test");
    rejected.discard();

    assert!(seal_module(program(
        vec![instance_box("Page", Vec::new()), main_only(),]
    ))
    .is_ok());
}

#[test]
fn mixed_instance_function_variants_seal_once_and_bridge_main() {
    let source = seal_module(program(vec![
        instance_box("Zeta", vec![("a", integer_return_function("a", 1))]),
        main_only(),
        instance_box(
            "Alpha",
            vec![(
                "identity",
                i64_parameter_return_function("identity", Some("i64"), "p0"),
            )],
        ),
        instance_box(
            "Beta",
            vec![(
                "cached",
                integer_local_return_function("cached", "value", None, Some(literal(7)), "value"),
            )],
        ),
    ]))
    .unwrap();
    let plans = source.seal_instance_function_plans().unwrap();
    let rows = plans.plans().collect::<Vec<_>>();
    let [(parameter_key, parameter), (local_key, local), (literal_key, literal)] = rows.as_slice()
    else {
        panic!("expected exact mixed cumulative plans")
    };
    let parameter = parameter
        .as_i64_parameter_return()
        .expect("exact i64 parameter variant");
    let local = local
        .as_integer_local_return()
        .expect("integer Local variant");
    let literal = literal
        .as_integer_literal_return()
        .expect("integer literal variant");

    assert_eq!(
        (parameter_key.owner(), parameter_key.name()),
        ("Alpha", "identity")
    );
    assert_eq!(parameter.parameter().source_name(), "p0");
    assert_eq!(parameter.parameter().abi().source_type_name(), "i64");
    assert_eq!(
        parameter.parameter().site(),
        &crate::mir::resolved_semantics::SourceBindingSiteV1::Parameter { index: 0 }
    );
    assert_eq!(parameter.recipe().receiver(), parameter.facts().receiver());
    assert_eq!(
        parameter.recipe().parameter(),
        parameter.parameter().binding()
    );
    assert_eq!(
        parameter.completion().explicit_site(),
        Some(parameter.recipe().return_site())
    );
    assert_eq!((local_key.owner(), local_key.name()), ("Beta", "cached"));
    assert!(matches!(
        local.local().site(),
        crate::mir::resolved_semantics::SourceBindingSiteV1::Local { ordinal: 0, .. }
    ));
    assert_eq!(local.local().source_name(), "value");
    assert_eq!(local.local().binding(), local.recipe().local());
    assert_eq!(local.recipe().receiver(), local.facts().receiver());
    assert_eq!(local.recipe().initializer_value(), 7);
    assert_eq!(
        local.completion().explicit_site(),
        Some(local.recipe().return_site())
    );
    assert_eq!((literal_key.owner(), literal_key.name()), ("Zeta", "a"));
    assert_eq!(literal.recipe().value(), 1);
    assert_eq!(
        literal.completion().explicit_site(),
        Some(literal.recipe().return_site())
    );
    assert!(literal.completion().returns_value());
    assert_ne!(
        literal.recipe().return_site().node(),
        literal.recipe().value_site().node()
    );
    assert_eq!(plans.len(), 3);

    let aggregate = plans.seal_main0_bridge().expect("mixed Main0 bridge");
    assert_eq!(aggregate.instance().len(), 3);
    assert!(!aggregate.main().completion().returns_value());
}

#[test]
fn unsupported_method_rejects_whole_set_and_fresh_local_reuses() {
    let source = seal_module(program(vec![
        main_only(),
        instance_box(
            "Page",
            vec![
                ("good", integer_return_function("good", 1)),
                (
                    "identity",
                    i64_parameter_return_function("identity", Some("i64"), "p0"),
                ),
                (
                    "cached",
                    integer_local_return_function(
                        "cached",
                        "value",
                        None,
                        Some(literal(2)),
                        "value",
                    ),
                ),
                ("bad", function_with_body("bad", Vec::new(), false)),
            ],
        ),
    ]))
    .unwrap();
    let rejected = source.seal_instance_function_plans().unwrap_err();

    assert_eq!(rejected.stage(), GeneralFunctionPlanStageV1::Recipe);
    assert_eq!(rejected.source_identity(), "normal-source-plan0-test");
    assert!(matches!(
        rejected.error(),
        GeneralFunctionPlanErrorV1::UnsupportedBody { key, .. }
            if key.owner() == "Page" && key.name() == "bad"
    ));
    rejected.discard();

    let plans = seal_module(program(vec![
        main_only(),
        instance_box(
            "Page",
            vec![(
                "cached",
                integer_local_return_function("cached", "value", None, Some(literal(7)), "value"),
            )],
        ),
    ]))
    .unwrap()
    .seal_instance_function_plans()
    .unwrap();
    assert_eq!(plans.len(), 1);
}

#[test]
fn empty_instance_boxes_do_not_issue_an_empty_plan_set() {
    let rejected = seal_module(program(vec![main_only(), instance_box("Page", Vec::new())]))
        .unwrap()
        .seal_instance_function_plans()
        .unwrap_err();

    assert_eq!(rejected.stage(), GeneralFunctionPlanStageV1::Inventory);
    assert!(matches!(
        rejected.error(),
        GeneralFunctionPlanErrorV1::NoInstanceMethod
    ));
    rejected.discard();
}

#[test]
fn instance_scalar_variants_reject_widening_without_retry() {
    let mut annotated = integer_return_function("annotated", 1);
    let ASTNode::FunctionDeclaration {
        return_type_name, ..
    } = &mut annotated
    else {
        unreachable!()
    };
    *return_type_name = Some("Integer".to_owned());

    let mut with_uses = integer_return_function("with_uses", 1);
    let ASTNode::FunctionDeclaration { uses, .. } = &mut with_uses else {
        unreachable!()
    };
    uses.push("external".to_owned());

    let mut with_attrs = integer_return_function("with_attrs", 1);
    let ASTNode::FunctionDeclaration { attrs, .. } = &mut with_attrs else {
        unreachable!()
    };
    attrs.runes.push(RuneAttr {
        name: "Public".to_owned(),
        args: Vec::new(),
    });

    let typed = ASTNode::Literal {
        value: LiteralValue::TypedInteger {
            value: 1,
            declared_type_name: "i64".to_owned(),
        },
        span: Span::unknown(),
    };
    let mut parameter_fed = i64_parameter_return_function("parameter_fed", Some("i64"), "p0");
    let ASTNode::FunctionDeclaration { body, .. } = &mut parameter_fed else {
        unreachable!()
    };
    *body = test_support::integer_local_return_body("value", None, Some(variable("p0")), "value");
    let mut cases = vec![
        (
            function("parameter", 1, false),
            GeneralFunctionPlanStageV1::Source,
        ),
        (annotated, GeneralFunctionPlanStageV1::Source),
        (with_uses, GeneralFunctionPlanStageV1::Source),
        (with_attrs, GeneralFunctionPlanStageV1::Source),
        (
            function_with_body("typed", vec![value_return(typed)], false),
            GeneralFunctionPlanStageV1::Recipe,
        ),
        (
            function_with_body("suffix", vec![value_return(literal(1)), literal(2)], false),
            GeneralFunctionPlanStageV1::Recipe,
        ),
        (
            i64_parameter_return_function("wrong_name", Some("i64"), "other"),
            GeneralFunctionPlanStageV1::Recipe,
        ),
        (
            integer_local_return_function("typed_local", "x", Some("i64"), Some(literal(1)), "x"),
            GeneralFunctionPlanStageV1::Recipe,
        ),
        (
            integer_local_return_function("missing", "x", None, None, "x"),
            GeneralFunctionPlanStageV1::Recipe,
        ),
        (
            integer_local_return_function("wrong_local", "x", None, Some(literal(1)), "y"),
            GeneralFunctionPlanStageV1::Recipe,
        ),
        (
            function_with_body(
                "non_integer_local",
                test_support::integer_local_return_body(
                    "x",
                    None,
                    Some(ASTNode::Literal {
                        value: LiteralValue::Bool(true),
                        span: Span::unknown(),
                    }),
                    "x",
                ),
                false,
            ),
            GeneralFunctionPlanStageV1::Recipe,
        ),
        (parameter_fed, GeneralFunctionPlanStageV1::Recipe),
    ];
    for spelling in ["Integer", "int", "IntegerBox", "I64", " i64", "i64 "] {
        cases.push((
            i64_parameter_return_function("bad_type", Some(spelling), "p0"),
            GeneralFunctionPlanStageV1::Source,
        ));
    }
    cases.push((
        i64_parameter_return_function("untyped", None, "p0"),
        GeneralFunctionPlanStageV1::Source,
    ));

    for (method, expected_stage) in cases {
        let name = match &method {
            ASTNode::FunctionDeclaration { name, .. } => name.clone(),
            _ => unreachable!(),
        };
        let rejected = seal_module(program(vec![
            main_only(),
            instance_box("Page", vec![(name.as_str(), method)]),
        ]))
        .unwrap()
        .seal_instance_function_plans()
        .unwrap_err();
        assert_eq!(rejected.stage(), expected_stage);
        rejected.discard();
    }
}

#[test]
fn non_program_root_is_rejected_at_root_surface() {
    let rejected = seal(literal(1)).unwrap_err();
    assert_eq!(rejected.stage(), &NormalSourcePlanStageV1::RootSurface);
    assert_eq!(rejected.error(), &NormalSourcePlanErrorV1::RootNotProgram);
    rejected.discard();
}
