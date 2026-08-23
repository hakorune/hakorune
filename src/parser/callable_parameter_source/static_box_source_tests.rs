use super::static_box_source::{
    ParserStaticBoxMemberKindV1, ParserStaticBoxParentOutsideReasonV1,
    ParserStaticBoxParentSourceDispositionV1, ParserStaticBoxParentSourceUnavailableV1,
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
fn bounded_static_box_parent_issues_one_parser_owned_ready_seal() {
    let parsed = parse("static box Api { run(value) { return value } }");
    let ParserStaticBoxParentSourceDispositionV1::Ready(seal) = parsed.static_box_parent_source()
    else {
        panic!("bounded static Box parent should be source-ready");
    };

    assert_eq!(seal.declaration_syntax().name(), "Api");
    assert!(!seal.declaration_syntax().is_sync());
    assert_eq!(seal.member_count(), 1);
    assert_eq!(
        seal.member_kinds().collect::<Vec<_>>(),
        [ParserStaticBoxMemberKindV1::DirectMethod]
    );
}

#[test]
fn static_parent_and_parameter_row_share_the_existing_callable_identity() {
    let parsed = parse("static box Api { run(value) { return value } }");
    let super::ParserCallableParameterSourceDispositionV1::Complete(catalog) =
        parsed.callable_parameter_source()
    else {
        panic!("parameter catalog should be complete");
    };
    let parameter_identity = catalog.declarations()[0].callable_identity();
    let ParserStaticBoxParentSourceDispositionV1::Ready(seal) = parsed.static_box_parent_source()
    else {
        panic!("bounded static Box parent should be source-ready");
    };

    assert!(seal.method_identity().same_as(parameter_identity));
}

#[test]
fn unsupported_static_parent_member_is_explicit_outside() {
    let parsed = parse("static box Api { field run() { return 1 } }");
    assert!(matches!(
        parsed.static_box_parent_source(),
        ParserStaticBoxParentSourceDispositionV1::Outside(
            ParserStaticBoxParentOutsideReasonV1::UnsupportedMemberKind
        )
    ));
}

#[test]
fn multiple_static_methods_are_outside_the_first_cohort() {
    let parsed = parse("static box Api { first() { return 1 } second() { return 2 } }");
    assert!(matches!(
        parsed.static_box_parent_source(),
        ParserStaticBoxParentSourceDispositionV1::Outside(
            ParserStaticBoxParentOutsideReasonV1::DirectMethodCohort
        )
    ));
}

#[test]
fn ordinary_source_path_does_not_reuse_static_parent_seal() {
    let parsed = parse("box Api { run() { return 1 } }");
    assert!(matches!(
        parsed.static_box_parent_source(),
        ParserStaticBoxParentSourceDispositionV1::SourceAuthorityUnavailable(
            ParserStaticBoxParentSourceUnavailableV1::OrdinarySourcePath
        )
    ));
}

#[test]
fn mixed_program_is_outside_without_static_parent_repair() {
    let parsed = parse("box Plain { run() { return 1 } }\nstatic box Api { run() { return 2 } }");
    assert!(matches!(
        parsed.static_box_parent_source(),
        ParserStaticBoxParentSourceDispositionV1::Outside(
            ParserStaticBoxParentOutsideReasonV1::ProgramCohort
        )
    ));
}
