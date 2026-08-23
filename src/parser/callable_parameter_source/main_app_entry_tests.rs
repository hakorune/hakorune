use super::main_app_entry::{
    ParserMainAppEntryDispositionV1, ParserMainAppEntryOutsideReasonV1,
};
use super::model::ParserCallableDeclarationKindV1;
use super::ParserCallableParameterSourceDispositionV1;
use crate::parser::{NyashParser, ParserBuildConfig};

fn parse(source: &str) -> super::ParsedProgramWithCallableParameterSourceV1 {
    NyashParser::parse_from_string_with_callable_parameter_source(
        source,
        ParserBuildConfig::default(),
    )
    .expect("parser product")
}

#[test]
fn one_static_main_zero_is_parser_ready_and_relation_bound() {
    let parsed = parse("static box Main { main() { return 1 } }");
    let ParserMainAppEntryDispositionV1::AppMainReady(seal) = parsed.main_app_entry() else {
        panic!("one static Main/main/0 should be parser-ready");
    };
    assert_eq!(seal.method_site().source_member_ordinal(), 0);

    let ParserCallableParameterSourceDispositionV1::Complete(catalog) =
        parsed.callable_parameter_source()
    else {
        panic!("static Main should keep a complete parameter catalog");
    };
    let row = &catalog.declarations()[0];
    assert_eq!(row.kind(), ParserCallableDeclarationKindV1::StaticBoxMethod);
    assert_eq!(row.diagnostic_name(), "main");
    assert!(row.parameters().is_empty());
    assert!(row.callable_identity().same_as(seal.callable_identity()));
    assert_eq!(row.source_site(), seal.method_site());
}

#[test]
fn ordinary_program_is_not_app_main() {
    let parsed = parse("box Main { main() { return 1 } }");
    assert!(matches!(
        parsed.main_app_entry(),
        ParserMainAppEntryDispositionV1::Outside(
            ParserMainAppEntryOutsideReasonV1::ProgramCohort
        )
    ));
}

#[test]
fn non_main_static_box_is_explicit_outside() {
    let parsed = parse("static box Utility { run() { return 1 } }");
    assert!(matches!(
        parsed.main_app_entry(),
        ParserMainAppEntryDispositionV1::Outside(
            ParserMainAppEntryOutsideReasonV1::NonMainStaticBox
        )
    ));
}

#[test]
fn nonzero_main_arity_is_explicit_outside() {
    let parsed = parse("static box Main { main(value) { return value } }");
    assert!(matches!(
        parsed.main_app_entry(),
        ParserMainAppEntryDispositionV1::Outside(
            ParserMainAppEntryOutsideReasonV1::NonZeroMainArity
        )
    ));
}

#[test]
fn mixed_program_is_outside_without_main_reselection() {
    let parsed = parse(
        "box Plain { run() { return 1 } }\nstatic box Main { main() { return 2 } }",
    );
    assert!(matches!(
        parsed.main_app_entry(),
        ParserMainAppEntryDispositionV1::Outside(
            ParserMainAppEntryOutsideReasonV1::ProgramCohort
        )
    ));
}

#[test]
fn multiple_static_parents_remain_explicit_outside() {
    let parsed = parse(
        "static box Main { main() { return 1 } }\nstatic box Other { run() { return 2 } }",
    );
    assert!(matches!(
        parsed.main_app_entry(),
        ParserMainAppEntryDispositionV1::Outside(
            ParserMainAppEntryOutsideReasonV1::StaticParent(
                super::static_box_source::ParserStaticBoxParentOutsideReasonV1::MultipleParentRows
            )
        )
    ));
}

#[test]
fn unsupported_main_member_remains_explicit_outside() {
    let parsed = parse("static box Main { field main() { return 1 } }");
    assert!(matches!(
        parsed.main_app_entry(),
        ParserMainAppEntryDispositionV1::Outside(
            ParserMainAppEntryOutsideReasonV1::StaticParent(
                super::static_box_source::ParserStaticBoxParentOutsideReasonV1::UnsupportedMemberKind
            )
        )
    ));
}
