use super::*;
use crate::ast::{DeclarationAttrs, LiteralValue, Span};
use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use std::collections::HashMap;


fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn function(name: &str) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: Some(Box::new(literal(0))),
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

fn box_declaration(name: &str, is_static: bool) -> ASTNode {
    ASTNode::BoxDeclaration {
        name: name.to_owned(),
        fields: Vec::new(),
        field_decls: Vec::new(),
        public_fields: Vec::new(),
        private_fields: Vec::new(),
        methods: crate::ast::BoxMethodInventoryV1::empty(),
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

fn instance_box_with_birth(name: &str) -> ASTNode {
    let mut declaration = box_declaration(name, false);
    let ASTNode::BoxDeclaration { constructors, .. } = &mut declaration else {
        unreachable!()
    };
    constructors.insert("birth/0".to_owned(), function("birth"));
    declaration
}




#[test]
fn raw_runtime_keeps_the_neutral_statement_carrier() {
    let plan = PreparedProgramRootWorkPlanV1::prepare(
        vec![box_declaration("Helpers", true), literal(7)],
        false,
        ProgramRootWorkPlanAdmissionV1::RawCompatibility,
        None,
    );
    let parts = plan.into_parts();
    assert!(matches!(
        parts.runtime,
        PreparedProgramRootRuntimeWorkV1::RawCompatibility(_)
    ));
}

#[test]
fn selected_top_level_functions_reject_duplicate_physical_projection() {
    let root = ASTNode::Program {
        statements: vec![function("same"), function("same")],
        span: Span::unknown(),
    };
    // The canonical catalog owns the earlier rejection: two top-level
    // declarations projecting to one `free_function` key collide at seal.
    assert_eq!(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap_err(),
        crate::mir::builder::callable_declaration_catalog::SameModuleCallableDeclarationCatalogErrorV1::DuplicateCanonicalKey(
            crate::mir::builder::callable_declaration_catalog::CanonicalSameModuleCallableKeyV1::free_function("same", 0)
        )
    );
}


#[test]
fn raw_top_level_functions_do_not_issue_selected_receipts() {
    let plan = PreparedProgramRootWorkPlanV1::prepare(
        vec![function("same")],
        false,
        ProgramRootWorkPlanAdmissionV1::RawCompatibility,
        None,
    );
    let parts = plan.into_parts();
    assert!(matches!(
        &parts.immediate[0],
        PreparedProgramRootImmediateWorkV1::TopLevelFunction(_)
    ));
}

#[test]
fn selected_normal_without_constructor_cohort_rejects_before_partition() {
    let statements = vec![literal(1)];
    let root = ASTNode::Program {
        statements: statements.clone(),
        span: Span::unknown(),
    };
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("selected callable catalog");
    let error =
        PreparedProgramRootWorkPlanV1::prepare_with_script_root_admission_and_constructor_sources(
            statements,
            true,
            ProgramRootWorkPlanAdmissionV1::SelectedNormal,
            Some(catalog.selected_source_inventory()),
            None,
            None,
        )
        .expect_err("SelectedNormal must not proceed without its constructor cohort");
    assert!(error.contains("instance-constructor-source/cohort-missing"));
}
