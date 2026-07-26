use std::collections::HashMap;

use crate::ast::{ASTNode, DeclarationAttrs, EnumVariantDecl, LiteralValue, ParamDecl, Span};

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

fn seal(source: ASTNode) -> Result<SealedNormalSourcePlanV1, RejectedNormalSourcePlanV1> {
    NormalSourcePlanClassifierV1::seal(input(source))
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
fn non_program_root_is_rejected_at_root_surface() {
    let rejected = seal(literal(1)).unwrap_err();
    assert_eq!(rejected.stage(), &NormalSourcePlanStageV1::RootSurface);
    assert_eq!(rejected.error(), &NormalSourcePlanErrorV1::RootNotProgram);
    rejected.discard();
}
