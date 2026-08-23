use super::normal_root_execution::ParserNormalRootExecutionTestTerminalV1;
use super::normal_source_plan_surface::{
    ParserNormalSourcePlanSurfaceV1, ParserNormalSourcePlanTopLevelRowV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};

fn with_surface<R>(
    source: &str,
    callback: impl for<'surface> FnOnce(Option<&'surface ParserNormalSourcePlanSurfaceV1>) -> R,
) -> R {
    let parsed = NyashParser::parse_from_string_with_callable_parameter_source(
        source,
        ParserBuildConfig::default(),
    )
    .expect("parser product");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        callback(
            loan.normal_root_execution()
                .ready()
                .map(|root| root.bound().surface()),
        )
    })
}

#[test]
fn empty_program_surface_is_explicit_complete_empty() {
    with_surface("", |surface| {
        assert!(matches!(
            surface,
            Some(ParserNormalSourcePlanSurfaceV1::CompleteEmpty)
        ));
    });
}

#[test]
fn executable_script_surface_is_issued_once_as_one_complete_row() {
    with_surface("print(1)\n", |surface| {
        let Some(ParserNormalSourcePlanSurfaceV1::CompleteRows(rows)) = surface else {
            panic!("one executable statement must not become an empty surface")
        };
        let rows = rows.rows();
        assert_eq!(rows.len(), 1);
        assert!(matches!(
            &rows[0],
            ParserNormalSourcePlanTopLevelRowV1::Executable { .. }
        ));
    });
}

#[test]
fn ordinary_box_surface_keeps_the_parser_owned_non_static_observation() {
    with_surface("box Plain { run() { return 1 } }\n", |surface| {
        let Some(ParserNormalSourcePlanSurfaceV1::CompleteRows(rows)) = surface else {
            panic!("ordinary Box must remain an explicit surface row")
        };
        let rows = rows.rows();
        assert_eq!(rows.len(), 1);
        let ParserNormalSourcePlanTopLevelRowV1::OrdinaryBox { source, .. } = &rows[0] else {
            panic!("ordinary Box must keep its parser-sealed source row")
        };
        assert_eq!(source.diagnostic_name(), "Plain");
        assert!(!source.is_sync());
        assert_eq!(source.direct_method_relations().len(), 1);
        assert_eq!(source.box_site().statement_ordinal(), 0);
    });
}

#[test]
fn non_static_main_surface_is_observed_without_becoming_program_runtime() {
    with_surface("box Main { main() { return 1 } }\n", |surface| {
        let Some(ParserNormalSourcePlanSurfaceV1::CompleteRows(rows)) = surface else {
            panic!("non-static Main must remain a complete observed Box row")
        };
        let ParserNormalSourcePlanTopLevelRowV1::OrdinaryBox { source, .. } = &rows.rows()[0]
        else {
            panic!("non-static Main must not collapse into Unsupported(Box)")
        };
        assert_eq!(source.diagnostic_name(), "Main");
        assert_eq!(source.direct_method_relations().len(), 1);
    });
}

#[test]
fn top_level_callable_and_executable_rows_share_one_surface_order() {
    with_surface("function free() {}\nprint(1)\n", |surface| {
        let Some(ParserNormalSourcePlanSurfaceV1::CompleteRows(rows)) = surface else {
            panic!("mixed top-level rows must remain explicit")
        };
        let rows = rows.rows();
        assert_eq!(rows.len(), 2);
        assert!(matches!(
            &rows[0],
            ParserNormalSourcePlanTopLevelRowV1::TopLevelCallable { .. }
        ));
        assert!(matches!(
            &rows[1],
            ParserNormalSourcePlanTopLevelRowV1::Executable { .. }
        ));
    });
}

#[test]
fn static_main_surface_keeps_the_nested_parent_relation() {
    with_surface(
        "static box Main { main() { return 1 } helper() {} }\n",
        |surface| {
            let Some(ParserNormalSourcePlanSurfaceV1::CompleteRows(rows)) = surface else {
                panic!("static Main must remain a non-empty surface")
            };
            let rows = rows.rows();
            assert_eq!(rows.len(), 1);
            let ParserNormalSourcePlanTopLevelRowV1::StaticBox { source, .. } = &rows[0] else {
                panic!("static Main must be retained as a nested static-parent row")
            };
            assert_eq!(source.box_site().path().segments().len(), 1);
        },
    );
}

#[test]
fn compatibility_postpass_cannot_emit_a_source_plan_bound() {
    let parsed = NyashParser::parse_from_string_with_callable_parameter_source(
        "interface box Api { run() }\n",
        ParserBuildConfig::default(),
    )
    .expect("parser product");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        assert!(matches!(
            loan.normal_root_execution(),
            super::normal_root_execution::ParserNormalRootExecutionSourceDispositionV1::
                SourceAuthorityUnavailable(_)
        ));
    });
}
