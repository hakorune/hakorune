use super::super::normal_source_plan_surface::{
    ParserNormalSourcePlanSurfaceDispositionV1, ParserNormalSourcePlanSurfaceIncompleteV1,
    ParserNormalSourcePlanSurfaceIntegrityIssueV1,
};
use super::model::{
    ParserNormalRootExecutionIncompleteV1, ParserNormalRootExecutionIntegrityIssueV1,
};
use super::*;
use crate::parser::{NyashParser, ParserBuildConfig};

fn parse(source: &str) -> super::super::ParsedProgramWithCallableParameterSourceV1 {
    NyashParser::parse_from_string_with_callable_parameter_source(
        source,
        ParserBuildConfig::default(),
    )
    .expect("parser product")
}

fn role(source: &str) -> Option<ParserNormalRootExecutionRoleV1> {
    ParserNormalRootExecutionTestTerminalV1::observe_once(parse(source), |loan| {
        loan.normal_root_execution_role()
    })
}

#[test]
fn total_root_distinguishes_program_runtime_and_app() {
    assert_eq!(
        role("print(1)"),
        Some(ParserNormalRootExecutionRoleV1::ProgramRuntime)
    );
    assert_eq!(
        role("static box Main { helper() {} main(x) {} }"),
        Some(ParserNormalRootExecutionRoleV1::App)
    );
    assert_eq!(
        role("box Main { main() {} }"),
        Some(ParserNormalRootExecutionRoleV1::App),
        "staticness is a compiler-policy fact, not a parser root classifier"
    );
}

#[test]
fn empty_and_non_main_provider_remain_program_runtime() {
    for source in ["", "static box Provider { run() {} }"] {
        assert_eq!(
            role(source),
            Some(ParserNormalRootExecutionRoleV1::ProgramRuntime)
        );
    }
}

#[test]
fn app_relation_retains_main_and_static_children() {
    let parsed = parse("static box Main { helper() {} main() {} }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        let source = loan.normal_root_execution().ready().expect("App relation");
        let relation = source.app_relation().expect("App role");
        assert_eq!(relation.main_statement(), 0);
        assert!(relation.main_box_is_static());
        assert_eq!(relation.static_children().len(), 1);
    });
}

#[test]
fn non_static_main_remains_one_app_relation_with_a_policy_fact() {
    let parsed = parse("box Main { main() {} }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        let source = loan
            .normal_root_execution()
            .ready()
            .expect("non-static Main remains a complete App observation");
        let relation = source.app_relation().expect("App role");
        assert!(!relation.main_box_is_static());
    });
}

#[test]
fn missing_main_method_is_a_typed_incomplete_terminal() {
    let parsed = parse("static box Main { helper() {} }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        assert!(matches!(
            loan.normal_root_execution(),
            ParserNormalRootExecutionSourceDispositionV1::Incomplete(
                ParserNormalRootExecutionIncompleteV1::MainMethodMissing
            )
        ));
    });
}

#[test]
fn app_relation_keeps_top_level_siblings_in_the_same_surface() {
    let parsed = parse("function sibling() {}\nstatic box Main { main() {} helper() {} }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        let source = loan
            .normal_root_execution()
            .ready()
            .expect("complete App relation");
        let relation = source.app_relation().expect("App role");
        assert_eq!(relation.main_statement(), 1);
        assert_eq!(relation.static_children().len(), 1);
        assert!(matches!(
            source.bound().surface(),
            super::super::normal_source_plan_surface::ParserNormalSourcePlanSurfaceV1::CompleteRows(rows)
                if rows.rows().len() == 2
        ));
    });
}

#[test]
fn duplicate_main_is_a_typed_integrity_terminal() {
    let duplicate_main = parse("static box Main { main() {} }\nstatic box Main { main() {} }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(duplicate_main, |loan| {
        assert!(matches!(
            loan.normal_root_execution(),
            ParserNormalRootExecutionSourceDispositionV1::IntegrityInvalid(
                ParserNormalRootExecutionIntegrityIssueV1::DuplicateMain
            )
        ));
    });
}

#[test]
fn surface_missing_and_integrity_terminals_propagate_without_reclassification() {
    let missing = ParserNormalRootExecutionIssuerV1::issue_once(
        ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
            ParserNormalSourcePlanSurfaceIncompleteV1::CallableSourceMissing,
        ),
    );
    assert!(matches!(
        missing,
        ParserNormalRootExecutionSourceDispositionV1::Incomplete(
            ParserNormalRootExecutionIncompleteV1::Surface(
                ParserNormalSourcePlanSurfaceIncompleteV1::CallableSourceMissing
            )
        )
    ));

    for issue in [
        ParserNormalSourcePlanSurfaceIntegrityIssueV1::ForeignParserRelation,
        ParserNormalSourcePlanSurfaceIntegrityIssueV1::DuplicateCallableSource,
    ] {
        let invalid = ParserNormalRootExecutionIssuerV1::issue_once(
            ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(issue),
        );
        assert!(matches!(
            invalid,
            ParserNormalRootExecutionSourceDispositionV1::IntegrityInvalid(
                ParserNormalRootExecutionIntegrityIssueV1::Surface(actual)
            ) if actual == issue
        ));
    }
}
