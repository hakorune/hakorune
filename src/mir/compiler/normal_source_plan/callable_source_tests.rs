use super::super::{
    NormalCallableCatalogSourceErrorV1, NormalCallableCatalogSourceStageV1,
    NormalSourcePlanClassifierV1, PreparedNormalSourcePlanInputV1, SealedNormalSourcePlanV1,
};
use super::*;
use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
use std::collections::HashMap;

fn function(name: &str, arity: usize) -> ASTNode {
    let params = (0..arity)
        .map(|index| format!("p{index}"))
        .collect::<Vec<_>>();
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        param_decls: params
            .iter()
            .map(|name| ParamDecl {
                name: name.clone(),
                declared_type_name: Some("i64".to_owned()),
            })
            .collect(),
        params,
        return_type_name: Some("i64".to_owned()),
        body: Vec::new(),
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn main_box(helper: Option<&str>) -> ASTNode {
    let mut methods = HashMap::new();
    methods.insert("main".to_owned(), function("main", 0));
    if let Some(helper) = helper {
        methods.insert(helper.to_owned(), function(helper, 1));
    }
    ASTNode::BoxDeclaration {
        name: "Main".to_owned(),
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

fn callable_source(
    source: ASTNode,
) -> Result<VerifiedNormalCallableSourceUnitV1, RejectedNormalCallableSourceV1> {
    let plan = NormalSourcePlanClassifierV1::seal(PreparedNormalSourcePlanInputV1::new(
        source,
        "normal-callable-source0-test",
    ))
    .expect("callable source family");
    let SealedNormalSourcePlanV1::CallableModule(source) = plan else {
        panic!("expected CallableModule")
    };
    source.prepare_callable_source()
}

#[test]
fn one_program_owner_exposes_exact_top_level_helper_sites() {
    let unit = callable_source(program(vec![
        main_box(None),
        function("zeta", 1),
        function("alpha", 1),
    ]))
    .unwrap();

    assert_eq!(
        unit.helper_sites()
            .iter()
            .map(|site| site.statement_index())
            .collect::<Vec<_>>(),
        [1, 2]
    );
    assert_eq!(unit.main_statement_index(), 0);
    assert_eq!(unit.main_method_key(), "main");
    let headers = unit
        .helper_sites()
        .iter()
        .map(|&site| {
            unit.helper_source()
                .located_header(site)
                .expect("sealed helper")
                .header()
                .name()
                .to_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(headers, ["zeta", "alpha"]);
}

#[test]
fn helper_declaration_reorder_preserves_exact_selected_meaning() {
    for statements in [
        vec![main_box(None), function("first", 1), function("second", 1)],
        vec![function("second", 1), main_box(None), function("first", 1)],
    ] {
        let unit = callable_source(program(statements)).unwrap();
        let mut names = unit
            .helper_sites()
            .iter()
            .map(|&site| {
                unit.helper_source()
                    .located_header(site)
                    .unwrap()
                    .header()
                    .name()
                    .to_owned()
            })
            .collect::<Vec<_>>();
        names.sort();
        assert_eq!(names, ["first", "second"]);
    }
}

#[test]
fn main_box_helpers_reject_before_catalog_sealing() {
    let rejected = callable_source(program(vec![main_box(Some("helper"))])).unwrap_err();
    assert_eq!(
        rejected.stage(),
        NormalCallableSourceStageV1::HelperSiteProjection
    );
    assert_eq!(
        rejected.error(),
        &NormalCallableSourceErrorV1::MainMethodHelperUnsupported {
            method_key: "helper".into()
        }
    );
    rejected.discard();
}

#[test]
fn one_program_owner_commits_one_complete_helper_catalog() {
    let unit = callable_source(program(vec![
        main_box(None),
        function("first", 1),
        function("second", 2),
    ]))
    .unwrap()
    .prepare_helper_catalog(17)
    .unwrap();

    assert_eq!(unit.source_identity(), "normal-callable-source0-test");
    assert_eq!(unit.main_statement_index(), 0);
    assert_eq!(unit.main_method_key(), "main");
}

#[test]
fn helper_catalog_rejection_retains_the_complete_source_owner() {
    let rejected = callable_source(program(vec![
        main_box(None),
        function("same", 1),
        function("same", 1),
    ]))
    .unwrap()
    .prepare_helper_catalog(0)
    .unwrap_err();

    assert_eq!(
        rejected.stage(),
        NormalCallableCatalogSourceStageV1::OwnerFreeCandidates
    );
    assert!(matches!(
        rejected.error(),
        NormalCallableCatalogSourceErrorV1::OwnerFreeCandidates(
            crate::mir::resolved_semantics::CallableCatalogCandidateSealErrorV1::DuplicateSourceKey {
                ..
            }
        )
    ));
    rejected.discard();
}
