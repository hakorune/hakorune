use super::normal_source_plan_surface::{
    ParserNormalSourcePlanSurfaceDispositionV1, ParserNormalSourcePlanSurfaceV1,
    ParserNormalSourcePlanTopLevelRowV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};

fn parse(source: &str) -> super::ParsedProgramWithCallableParameterSourceV1 {
    NyashParser::parse_from_string_with_callable_parameter_source(
        source,
        ParserBuildConfig::default(),
    )
    .expect("parser product")
}

#[test]
fn empty_program_surface_is_explicit_complete_empty() {
    let parsed = parse("");
    let ParserNormalSourcePlanSurfaceDispositionV1::Ready(bound) =
        parsed.normal_source_plan_surface()
    else {
        panic!("source-backed empty Program must issue a source-plan bound")
    };
    assert!(matches!(
        bound.surface(),
        ParserNormalSourcePlanSurfaceV1::CompleteEmpty
    ));
}

#[test]
fn executable_script_surface_is_issued_once_as_one_complete_row() {
    let parsed = parse("print(1)\n");
    let ParserNormalSourcePlanSurfaceDispositionV1::Ready(bound) =
        parsed.normal_source_plan_surface()
    else {
        panic!("source-backed Script must issue a source-plan bound")
    };
    let ParserNormalSourcePlanSurfaceV1::CompleteRows(rows) = bound.surface() else {
        panic!("one executable statement must not become an empty surface")
    };
    assert_eq!(rows.len(), 1);
    assert!(matches!(
        &rows[0],
        ParserNormalSourcePlanTopLevelRowV1::Executable { .. }
    ));
}

#[test]
fn ordinary_box_surface_keeps_the_parser_owned_non_static_observation() {
    let parsed = parse("box Plain { run() { return 1 } }\n");
    let ParserNormalSourcePlanSurfaceDispositionV1::Ready(bound) =
        parsed.normal_source_plan_surface()
    else {
        panic!("source-backed ordinary program must issue a source-plan bound")
    };
    let ParserNormalSourcePlanSurfaceV1::CompleteRows(rows) = bound.surface() else {
        panic!("ordinary Box must remain an explicit surface row")
    };
    assert_eq!(rows.len(), 1);
    assert!(matches!(
        &rows[0],
        ParserNormalSourcePlanTopLevelRowV1::Unsupported { .. }
    ));
}

#[test]
fn top_level_callable_and_executable_rows_share_one_surface_order() {
    let parsed = parse("function free() {}\nprint(1)\n");
    let ParserNormalSourcePlanSurfaceDispositionV1::Ready(bound) =
        parsed.normal_source_plan_surface()
    else {
        panic!("source-backed mixed Script must issue a source-plan bound")
    };
    let ParserNormalSourcePlanSurfaceV1::CompleteRows(rows) = bound.surface() else {
        panic!("mixed top-level rows must remain explicit")
    };
    assert_eq!(rows.len(), 2);
    assert!(matches!(
        &rows[0],
        ParserNormalSourcePlanTopLevelRowV1::TopLevelCallable { .. }
    ));
    assert!(matches!(
        &rows[1],
        ParserNormalSourcePlanTopLevelRowV1::Executable { .. }
    ));
}

#[test]
fn static_main_surface_keeps_the_nested_parent_relation() {
    let parsed = parse("static box Main { main() { return 1 } helper() {} }\n");
    let ParserNormalSourcePlanSurfaceDispositionV1::Ready(bound) =
        parsed.normal_source_plan_surface()
    else {
        panic!("source-backed static Main must issue a source-plan bound")
    };
    let ParserNormalSourcePlanSurfaceV1::CompleteRows(rows) = bound.surface() else {
        panic!("static Main must remain a non-empty surface")
    };
    assert_eq!(rows.len(), 1);
    let ParserNormalSourcePlanTopLevelRowV1::StaticBox { source, .. } = &rows[0] else {
        panic!("static Main must be retained as a nested static-parent row")
    };
    assert_eq!(source.box_site().path().segments().len(), 1);
}

#[test]
fn compatibility_postpass_cannot_emit_a_source_plan_bound() {
    let parsed = parse("interface box Api { run() }\n");
    assert!(matches!(
        parsed.normal_source_plan_surface(),
        ParserNormalSourcePlanSurfaceDispositionV1::SourceAuthorityUnavailable(_)
            | ParserNormalSourcePlanSurfaceDispositionV1::CompatibilityOutside
    ));
}
