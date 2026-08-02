use super::*;
use crate::ast::{DeclarationAttrs, LiteralValue, Span};
use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::parser::NyashParser;

fn selected_plan(statements: Vec<ASTNode>, is_app_mode: bool) -> PreparedProgramRootWorkPlanV1 {
    let root = ASTNode::Program {
        statements: statements.clone(),
        span: Span::unknown(),
    };
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("selected callable catalog");
    PreparedProgramRootWorkPlanV1::prepare(
        statements,
        is_app_mode,
        ProgramRootWorkPlanAdmissionV1::SelectedNormal,
        Some(catalog.selected_source_inventory()),
    )
}

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
        methods: HashMap::new(),
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
fn app_partition_preserves_source_order_and_runtime_retention() {
    let plan = selected_plan(
        vec![
            box_declaration("Page", false),
            function("helper"),
            box_declaration("Helpers", true),
            literal(7),
            box_declaration("Main", true),
        ],
        true,
    );
    let parts = plan.into_parts();
    assert_eq!(
        parts.terminal,
        ProgramRootTerminalScheduleV1::VerifiedAppMain
    );
    assert_eq!(parts.immediate.len(), 2);
    assert!(matches!(
        &parts.immediate[0],
        PreparedProgramRootImmediateWorkV1::InstanceBox(work) if work.name == "Page"
    ));
    assert!(matches!(
        &parts.immediate[1],
        PreparedProgramRootImmediateWorkV1::TopLevelFunction(work) if work.name() == "helper"
    ));
    assert_eq!(parts.deferred_static.len(), 1);
    assert_eq!(parts.deferred_static[0].name, "Helpers");
    assert_eq!(parts.runtime.len(), 4);
    assert!(
        matches!(parts.runtime.statement_at(0), ASTNode::BoxDeclaration { name, .. } if name == "Page")
    );
    assert!(
        matches!(parts.runtime.statement_at(3), ASTNode::BoxDeclaration { name, .. } if name == "Main")
    );
}

#[test]
fn script_partition_keeps_static_boxes_out_of_deferred_work() {
    let ASTNode::Program { statements, .. } = NyashParser::parse_from_string(
        "static box Helpers { value() { return 1 } }\nfunction helper() { return 2 }\nprint(3)",
    )
    .expect("parsed Script partition fixture")
    else {
        panic!("expected Program root")
    };
    let plan = selected_plan(statements, false);
    let parts = plan.into_parts();
    assert_eq!(parts.terminal, ProgramRootTerminalScheduleV1::ScriptRuntime);
    assert_eq!(parts.deferred_static.len(), 0);
    assert_eq!(parts.immediate.len(), 1);
    assert_eq!(parts.runtime.len(), 2);
    assert!(
        matches!(parts.runtime.statement_at(0), ASTNode::BoxDeclaration { name, .. } if name == "Helpers")
    );
    assert!(matches!(parts.runtime.statement_at(1), ASTNode::Print { .. }));
    let PreparedProgramRootRuntimeWorkV1::SelectedNormal(runtime) = &parts.runtime else {
        panic!("expected selected Script runtime work")
    };
    assert_eq!(
        (
            runtime.source_statement_index_at(0),
            runtime.source_statement_index_at(1)
        ),
        (0, 2)
    );
}

#[test]
fn selected_script_transports_one_constructor_source_to_its_second_demand() {
    let plan = selected_plan(vec![instance_box_with_birth("Page")], false);
    let parts = plan.into_parts();
    let PreparedProgramRootImmediateWorkV1::InstanceBox(immediate) = &parts.immediate[0] else {
        panic!("expected immediate instance Box")
    };
    let immediate_sources = immediate
        .normal_constructor_sources
        .as_ref()
        .expect("selected immediate source");
    assert_eq!(immediate_sources.sources()[0].statement_index(), 0);
    assert_eq!(
        immediate_sources.sources()[0].parser_constructor_key(),
        "birth/0"
    );
    let PreparedProgramRootRuntimeWorkV1::SelectedNormal(runtime) = &parts.runtime else {
        panic!("expected selected Script runtime work")
    };
    let (runtime_sources, _) = runtime
        .constructor_admission_at(0)
        .expect("selected Script second demand source");
    assert_eq!(runtime_sources.sources(), immediate_sources.sources());
}

#[test]
fn selected_nonplain_script_retains_constructor_source_for_full_runtime_lifecycle() {
    let mut nonplain = instance_box_with_birth("RecordPage");
    let ASTNode::BoxDeclaration { is_record, .. } = &mut nonplain else {
        unreachable!()
    };
    *is_record = true;
    let plan = selected_plan(vec![nonplain], false);
    let parts = plan.into_parts();
    let PreparedProgramRootImmediateWorkV1::InstanceBox(immediate) = &parts.immediate[0] else {
        panic!("expected immediate instance Box")
    };
    assert!(immediate.normal_constructor_sources.is_some());
    let PreparedProgramRootRuntimeWorkV1::SelectedNormal(runtime) = &parts.runtime else {
        panic!("expected selected Script runtime work")
    };
    assert!(matches!(
        runtime.admission_at(0),
        NormalScriptRuntimeStatementAdmissionV1::NonPlainInstanceFullLifecycle { .. }
    ));
    assert!(runtime.constructor_admission_at(0).is_some());
}

#[test]
fn selected_constructor_sources_keep_parser_key_order_and_skip_nonfunctions() {
    let mut declaration = box_declaration("Page", false);
    let ASTNode::BoxDeclaration { constructors, .. } = &mut declaration else {
        unreachable!()
    };
    constructors.insert("init/0".to_owned(), function("init"));
    constructors.insert("birth/1".to_owned(), function("birth"));
    constructors.insert("not-a-function".to_owned(), literal(0));
    let plan = selected_plan(vec![declaration], true);
    let parts = plan.into_parts();
    let PreparedProgramRootImmediateWorkV1::InstanceBox(immediate) = &parts.immediate[0] else {
        panic!("expected immediate instance Box")
    };
    let keys = immediate
        .normal_constructor_sources
        .as_ref()
        .expect("selected source batch")
        .sources()
        .iter()
        .map(|source| source.parser_constructor_key())
        .collect::<Vec<_>>();
    assert_eq!(keys, ["birth/1", "init/0"]);
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
fn selected_top_level_functions_keep_distinct_source_occurrences() {
    let plan = selected_plan(vec![function("same"), function("same")], false);
    let parts = plan.into_parts();
    let admissions = parts
        .immediate
        .iter()
        .map(|work| match work {
            PreparedProgramRootImmediateWorkV1::TopLevelFunction(
                PreparedProgramRootTopLevelFunctionWorkV1::SelectedNormal { admission, .. },
            ) => admission,
            other => panic!("expected selected top-level work, got {other:?}"),
        })
        .collect::<Vec<_>>();
    assert_eq!(admissions.len(), 2);
    assert_eq!(admissions[0].source_key().statement_index(), 0);
    assert_eq!(admissions[1].source_key().statement_index(), 1);
    assert_eq!(admissions[0].physical_symbol(), "same/0");
    assert_eq!(admissions[1].physical_symbol(), "same/0");
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
        PreparedProgramRootImmediateWorkV1::TopLevelFunction(
            PreparedProgramRootTopLevelFunctionWorkV1::RawCompatibility(_)
        )
    ));
}
