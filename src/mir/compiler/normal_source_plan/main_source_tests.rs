use super::super::product::{
    NormalMainMethodSiteV1, NormalTopLevelSiteV1, PreparedNormalSourcePlanInputV1,
    SealedNormalMainSourceV1, SealedNormalScalarRootV1, SealedNormalSourcePlanV1,
};
use super::super::NormalSourcePlanClassifierV1;
use super::*;
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use std::collections::HashMap;

fn function(
    name: &str,
    is_static: bool,
    arity: usize,
    return_type_name: Option<&str>,
    body: Vec<ASTNode>,
) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: (0..arity).map(|index| format!("p{index}")).collect(),
        param_decls: Vec::new(),
        return_type_name: return_type_name.map(str::to_owned),
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn main_box(methods: HashMap<String, ASTNode>, fields: Vec<String>) -> ASTNode {
    ASTNode::BoxDeclaration {
        name: "Main".to_owned(),
        fields,
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
        is_sync: false,
        is_record: false,
        type_parameters: Vec::new(),
        extends: Vec::new(),
        implements: Vec::new(),
        is_static: true,
        static_init: None,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn program(statements: Vec<ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements,
        span: Span::unknown(),
    }
}

fn valid_main(body: Vec<ASTNode>, annotation: Option<&str>) -> ASTNode {
    let mut methods = HashMap::new();
    methods.insert(
        "main".to_owned(),
        function("main", true, 0, annotation, body),
    );
    program(vec![main_box(methods, vec!["retained".to_owned()])])
}

fn classified_main(source: ASTNode) -> SealedNormalMainSourceV1 {
    let input = PreparedNormalSourcePlanInputV1::new(source, "main-source-test");
    let plan = NormalSourcePlanClassifierV1::seal(input).expect("valid Main0 plan");
    let SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(main)) = plan else {
        panic!("expected Main0 plan");
    };
    main
}

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn sealed_fixture(
    source: ASTNode,
    statement_index: usize,
    method_key: &str,
    arity: usize,
    is_static: bool,
) -> SealedNormalMainSourceV1 {
    sealed_fixture_with_sites(
        source,
        statement_index,
        statement_index,
        method_key,
        arity,
        is_static,
    )
}

fn sealed_fixture_with_sites(
    source: ASTNode,
    main_statement_index: usize,
    method_statement_index: usize,
    method_key: &str,
    arity: usize,
    is_static: bool,
) -> SealedNormalMainSourceV1 {
    SealedNormalMainSourceV1::seal(
        PreparedNormalSourcePlanInputV1::new(source, "drift-fixture"),
        NormalTopLevelSiteV1::new(main_statement_index),
        NormalMainMethodSiteV1::new(
            method_statement_index,
            method_key.to_owned().into_boxed_str(),
            arity,
            is_static,
        ),
    )
}

#[test]
fn exact_private_site_does_not_reclassify_unrelated_program_statements() {
    let mut methods = HashMap::new();
    methods.insert(
        "main".to_owned(),
        function("main", true, 0, None, Vec::new()),
    );
    let source = program(vec![literal(1), main_box(methods, Vec::new())]);
    let unit = sealed_fixture(source, 1, "main", 0, true)
        .prepare_function_source()
        .expect("SOURCE0 verifies the sealed site without family reclassification");
    assert_eq!(unit.borrow_exact_function().main_statement_index(), 1);
}

#[test]
fn main_zero_seals_one_borrowed_exact_function_without_clone() {
    let unit = classified_main(valid_main(Vec::new(), None))
        .prepare_function_source()
        .expect("exact Main relation");
    let first = unit.borrow_exact_function();
    let first_ptr = first.function() as *const ASTNode;
    assert_eq!(first.main_statement_index(), 0);
    assert_eq!(first.method_key(), "main");
    let second = unit.borrow_exact_function();
    assert_eq!(first_ptr, second.function() as *const ASTNode);
}

#[test]
fn main_body_annotation_and_program_owned_box_fields_survive_source_sealing() {
    let body = vec![ASTNode::Return {
        value: Some(Box::new(literal(7))),
        span: Span::unknown(),
    }];
    let unit = classified_main(valid_main(body, Some("i64")))
        .prepare_function_source()
        .expect("exact Main relation");
    let view = unit.borrow_exact_function();
    let ASTNode::FunctionDeclaration {
        body,
        return_type_name,
        ..
    } = view.function()
    else {
        panic!("verified view must remain a function");
    };
    assert_eq!(return_type_name.as_deref(), Some("i64"));
    assert!(matches!(body.as_slice(), [ASTNode::Return { .. }]));
    let ASTNode::Program { statements, .. } = unit.owned_program_for_test() else {
        panic!("source unit must retain its Program owner");
    };
    let [ASTNode::BoxDeclaration { fields, .. }] = statements.as_slice() else {
        panic!("source unit must retain its Main declaration");
    };
    assert_eq!(fields, &["retained".to_owned()]);
}

#[test]
fn missing_or_drifted_main_statement_is_typed_and_retained() {
    let missing = sealed_fixture(valid_main(Vec::new(), None), 1, "main", 0, true)
        .prepare_function_source()
        .expect_err("missing statement rejects");
    assert_eq!(
        missing.error(),
        &NormalMainFunctionSourceErrorV1::MainStatementMissing
    );
    missing.discard();

    let drifted = sealed_fixture(program(vec![literal(1)]), 0, "main", 0, true)
        .prepare_function_source()
        .expect_err("statement drift rejects");
    assert_eq!(
        drifted.error(),
        &NormalMainFunctionSourceErrorV1::MainStatementDrift
    );
    drifted.discard();

    let mismatched_site =
        sealed_fixture_with_sites(valid_main(Vec::new(), None), 0, 1, "main", 0, true)
            .prepare_function_source()
            .expect_err("method-to-box site drift rejects");
    assert_eq!(
        mismatched_site.error(),
        &NormalMainFunctionSourceErrorV1::MainStatementDrift
    );
    mismatched_site.discard();
}

#[test]
fn root_and_missing_method_are_typed_and_retained() {
    let root = sealed_fixture(literal(1), 0, "main", 0, true)
        .prepare_function_source()
        .expect_err("non-Program root rejects");
    assert_eq!(
        root.error(),
        &NormalMainFunctionSourceErrorV1::RootNotProgram
    );
    root.discard();

    let missing = sealed_fixture(
        program(vec![main_box(HashMap::new(), Vec::new())]),
        0,
        "main",
        0,
        true,
    )
    .prepare_function_source()
    .expect_err("missing exact method rejects");
    assert_eq!(
        missing.error(),
        &NormalMainFunctionSourceErrorV1::MainMethodMissing
    );
    missing.discard();
}

#[test]
fn method_key_name_shape_static_and_arity_drift_are_typed() {
    let key = sealed_fixture(valid_main(Vec::new(), None), 0, "other", 0, true)
        .prepare_function_source()
        .expect_err("method key drift rejects");
    assert_eq!(
        key.error(),
        &NormalMainFunctionSourceErrorV1::MainMethodNameDrift
    );

    let mut wrong_name_methods = HashMap::new();
    wrong_name_methods.insert(
        "main".to_owned(),
        function("different", true, 0, None, Vec::new()),
    );
    let name = sealed_fixture(
        program(vec![main_box(wrong_name_methods, Vec::new())]),
        0,
        "main",
        0,
        true,
    )
    .prepare_function_source()
    .expect_err("declaration name drift rejects");
    assert_eq!(
        name.error(),
        &NormalMainFunctionSourceErrorV1::MainMethodNameDrift
    );

    let mut shape_methods = HashMap::new();
    shape_methods.insert("main".to_owned(), literal(1));
    let shape = sealed_fixture(
        program(vec![main_box(shape_methods, Vec::new())]),
        0,
        "main",
        0,
        true,
    )
    .prepare_function_source()
    .expect_err("method shape drift rejects");
    assert_eq!(
        shape.error(),
        &NormalMainFunctionSourceErrorV1::MainMethodShapeDrift
    );

    let mut instance_methods = HashMap::new();
    instance_methods.insert(
        "main".to_owned(),
        function("main", false, 0, None, Vec::new()),
    );
    let static_drift = sealed_fixture(
        program(vec![main_box(instance_methods, Vec::new())]),
        0,
        "main",
        0,
        true,
    )
    .prepare_function_source()
    .expect_err("static drift rejects");
    assert_eq!(
        static_drift.error(),
        &NormalMainFunctionSourceErrorV1::MainMethodStaticDrift
    );

    let mut arity_methods = HashMap::new();
    arity_methods.insert(
        "main".to_owned(),
        function("main", true, 1, None, Vec::new()),
    );
    let arity = sealed_fixture(
        program(vec![main_box(arity_methods, Vec::new())]),
        0,
        "main",
        0,
        true,
    )
    .prepare_function_source()
    .expect_err("arity drift rejects");
    assert_eq!(
        arity.error(),
        &NormalMainFunctionSourceErrorV1::MainMethodArityDrift
    );
}
