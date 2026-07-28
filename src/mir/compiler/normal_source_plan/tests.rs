use std::collections::HashMap;

use crate::ast::{
    ASTNode, DeclarationAttrs, EnumVariantDecl, FieldDecl, LiteralValue, ParamDecl, RuneAttr, Span,
};

use super::*;

fn input(source: ASTNode) -> PreparedNormalSourcePlanInputV1 {
    PreparedNormalSourcePlanInputV1::new(source, "normal-source-plan0-test")
}

fn program(statements: Vec<ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements,
        span: Span::unknown(),
    }
}

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn function(name: &str, arity: usize, is_static: bool) -> ASTNode {
    let params = (0..arity)
        .map(|index| format!("p{index}"))
        .collect::<Vec<_>>();
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        param_decls: params
            .iter()
            .map(|name| ParamDecl {
                name: name.to_owned(),
                declared_type_name: None,
            })
            .collect(),
        params,
        return_type_name: None,
        body: Vec::new(),
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn function_with_body(name: &str, body: Vec<ASTNode>, is_static: bool) -> ASTNode {
    let mut function = function(name, 0, is_static);
    let ASTNode::FunctionDeclaration {
        body: function_body,
        ..
    } = &mut function
    else {
        unreachable!()
    };
    *function_body = body;
    function
}

fn value_return(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    }
}

fn integer_return_function(name: &str, value: i64) -> ASTNode {
    function_with_body(name, vec![value_return(literal(value))], false)
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
fn all_instance_integer_return_methods_seal_in_catalog_order() {
    let source = seal_module(program(vec![
        instance_box("Zeta", vec![("a", integer_return_function("a", 1))]),
        main_only(),
        instance_box("Alpha", vec![("b", integer_return_function("b", 2))]),
    ]))
    .unwrap();
    let plans = source.seal_instance_integer_return_plans().unwrap();
    let rows = plans
        .plans()
        .map(|(key, plan)| {
            (
                key.owner().to_owned(),
                key.name().to_owned(),
                plan.recipe().value(),
                plan.facts().owner_count(),
                plan.recipe().receiver() == plan.facts().receiver(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        rows,
        [
            ("Alpha".to_owned(), "b".to_owned(), 2, 1, true),
            ("Zeta".to_owned(), "a".to_owned(), 1, 1, true),
        ]
    );
    assert_eq!(plans.len(), 2);
    assert_eq!(plans.source_identity(), "normal-source-plan0-test");
}

#[test]
fn integer_return_recipe_pairs_exact_completion_without_claiming_main() {
    let main = main_box(
        vec![(
            "main",
            function_with_body("main", vec![literal(99), literal(100)], true),
        )],
        true,
    );
    let plans = seal_module(program(vec![
        main,
        instance_box(
            "Page",
            vec![("render", integer_return_function("render", 42))],
        ),
    ]))
    .unwrap()
    .seal_instance_integer_return_plans()
    .unwrap();
    let rows = plans.plans().collect::<Vec<_>>();
    let [(key, plan)] = rows.as_slice() else {
        panic!("expected one instance plan")
    };

    assert_eq!(
        key.namespace(),
        crate::mir::builder::SameModuleCallableNamespaceV1::InstanceBoxMethod
    );
    assert_eq!(
        plan.completion().explicit_site(),
        Some(plan.recipe().return_site())
    );
    assert!(plan.completion().returns_value());
    assert_eq!(plan.completion().unreachable_suffix_count(), 0);
    assert_ne!(
        plan.recipe().return_site().node(),
        plan.recipe().value_site().node()
    );
}

#[test]
fn one_unsupported_method_rejects_the_whole_plan_set() {
    let source = seal_module(program(vec![
        main_only(),
        instance_box(
            "Page",
            vec![
                ("good", integer_return_function("good", 1)),
                ("bad", function_with_body("bad", Vec::new(), false)),
            ],
        ),
    ]))
    .unwrap();
    let rejected = source.seal_instance_integer_return_plans().unwrap_err();

    assert_eq!(rejected.stage(), GeneralFunctionPlanStageV1::Recipe);
    assert_eq!(rejected.source_identity(), "normal-source-plan0-test");
    assert!(matches!(
        rejected.error(),
        GeneralFunctionPlanErrorV1::UnsupportedBody { key, .. }
            if key.owner() == "Page" && key.name() == "bad"
    ));
    rejected.discard();
}

#[test]
fn empty_instance_boxes_do_not_issue_an_empty_plan_set() {
    let rejected = seal_module(program(vec![main_only(), instance_box("Page", Vec::new())]))
        .unwrap()
        .seal_instance_integer_return_plans()
        .unwrap_err();

    assert_eq!(rejected.stage(), GeneralFunctionPlanStageV1::Inventory);
    assert!(matches!(
        rejected.error(),
        GeneralFunctionPlanErrorV1::NoInstanceMethod
    ));
    rejected.discard();
}

#[test]
fn instance_integer_return_rejects_signature_and_body_widening() {
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
    let cases = [
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
    ];

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
        .seal_instance_integer_return_plans()
        .unwrap_err();
        assert_eq!(rejected.stage(), expected_stage);
        rejected.discard();
    }
}

#[test]
fn rejection_discards_without_retry_and_fresh_source_reuses() {
    let rejected = seal_module(program(vec![
        main_only(),
        instance_box(
            "Page",
            vec![("render", function_with_body("render", Vec::new(), false))],
        ),
    ]))
    .unwrap()
    .seal_instance_integer_return_plans()
    .unwrap_err();
    rejected.discard();

    let plans = seal_module(program(vec![
        main_only(),
        instance_box(
            "Page",
            vec![("render", integer_return_function("render", 7))],
        ),
    ]))
    .unwrap()
    .seal_instance_integer_return_plans()
    .unwrap();
    assert_eq!(plans.len(), 1);
}

#[test]
fn non_program_root_is_rejected_at_root_surface() {
    let rejected = seal(literal(1)).unwrap_err();
    assert_eq!(rejected.stage(), &NormalSourcePlanStageV1::RootSurface);
    assert_eq!(rejected.error(), &NormalSourcePlanErrorV1::RootNotProgram);
    rejected.discard();
}
