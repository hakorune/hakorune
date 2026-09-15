use super::normal_root_execution::ParserNormalRootExecutionTestTerminalV1;
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
fn method_prefixed_member_is_one_direct_method_without_phantom_field() {
    let parsed = parse("static box Api { method run(value) { return value } }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        let ParserStaticBoxParentSourceDispositionV1::Ready(seal) = loan.static_box_parent_source()
        else {
            panic!("method-prefixed member should issue a ready static parent seal");
        };
        assert_eq!(seal.member_count(), 1);
        assert_eq!(
            seal.member_kinds().collect::<Vec<_>>(),
            [ParserStaticBoxMemberKindV1::DirectMethod]
        );
        assert_eq!(seal.direct_method_relations().count(), 1);
        assert!(loan.normal_root_execution().ready().is_some());
    });
}

#[test]
fn method_prefixed_members_mix_with_plain_methods_under_one_seal() {
    let parsed = parse(
        "static box Api { method first() { return 1 } second() { return 2 } method third() { return 3 } }",
    );
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        let ParserStaticBoxParentSourceDispositionV1::Ready(seal) = loan.static_box_parent_source()
        else {
            panic!("mixed method-prefixed members should share one ready seal");
        };
        assert_eq!(seal.member_count(), 3);
        assert_eq!(seal.direct_method_relations().count(), 3);
    });
}

#[test]
fn method_as_method_name_still_parses_as_direct_method() {
    let parsed = parse("static box Api { method() { return 1 } }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        let ParserStaticBoxParentSourceDispositionV1::Ready(seal) = loan.static_box_parent_source()
        else {
            panic!("a method literally named `method` should stay a direct method");
        };
        assert_eq!(seal.member_count(), 1);
        assert_eq!(
            seal.member_kinds().collect::<Vec<_>>(),
            [ParserStaticBoxMemberKindV1::DirectMethod]
        );
    });
}

#[test]
fn lone_method_member_keeps_field_classification() {
    let parsed = parse("static box Api { method }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        assert!(matches!(
            loan.static_box_parent_source(),
            ParserStaticBoxParentSourceDispositionV1::Outside(
                ParserStaticBoxParentOutsideReasonV1::UnsupportedMemberKind
            )
        ));
    });
}

#[test]
fn bounded_static_box_parent_issues_one_parser_owned_ready_seal() {
    let parsed = parse("static box Api { run(value) { return value } }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        let ParserStaticBoxParentSourceDispositionV1::Ready(seal) = loan.static_box_parent_source()
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
    });
}

#[test]
fn static_parent_and_parameter_row_share_the_existing_callable_identity() {
    let parsed = parse("static box Api { run(value) { return value } }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        let super::ParserCallableParameterSourceDispositionV1::Complete(catalog) =
            loan.callable_parameter_source()
        else {
            panic!("parameter catalog should be complete");
        };
        let parameter_identity = catalog.declarations()[0].callable_identity();
        let ParserStaticBoxParentSourceDispositionV1::Ready(seal) = loan.static_box_parent_source()
        else {
            panic!("bounded static Box parent should be source-ready");
        };

        assert!(seal.method_identity().same_as(parameter_identity));
    });
}

#[test]
fn unsupported_static_parent_member_is_explicit_outside() {
    let parsed = parse("static box Api { field run() { return 1 } }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        assert!(matches!(
            loan.static_box_parent_source(),
            ParserStaticBoxParentSourceDispositionV1::Outside(
                ParserStaticBoxParentOutsideReasonV1::UnsupportedMemberKind
            )
        ));
    });
}

#[test]
fn empty_static_parent_is_explicitly_outside_the_direct_method_cohort() {
    let parsed = parse("static box Api {}");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        assert!(matches!(
            loan.static_box_parent_source(),
            ParserStaticBoxParentSourceDispositionV1::Outside(
                ParserStaticBoxParentOutsideReasonV1::DirectMethodCohort
            )
        ));
    });
}

#[test]
fn multiple_static_methods_share_one_parent_seal() {
    let parsed = parse("static box Api { first() { return 1 } second() { return 2 } }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        let ParserStaticBoxParentSourceDispositionV1::Ready(seal) = loan.static_box_parent_source()
        else {
            panic!("multiple direct methods should share one static parent seal");
        };
        assert_eq!(seal.direct_method_relations().count(), 2);
    });
}

#[test]
fn multiple_static_parents_share_one_parser_owned_set() {
    let parsed =
        parse("static box First { one() { return 1 } }\nstatic box Second { two() { return 2 } }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        let ParserStaticBoxParentSourceDispositionV1::Ready(seal) = loan.static_box_parent_source()
        else {
            panic!("same-brand static parents should share one set seal");
        };
        assert_eq!(seal.direct_method_relations().count(), 2);
    });
}

#[test]
fn ordinary_source_path_does_not_reuse_static_parent_seal() {
    let parsed = parse("box Api { run() { return 1 } }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        assert!(matches!(
            loan.static_box_parent_source(),
            ParserStaticBoxParentSourceDispositionV1::SourceAuthorityUnavailable(
                ParserStaticBoxParentSourceUnavailableV1::OrdinarySourcePath
            )
        ));
    });
}

#[test]
fn mixed_program_reuses_the_parser_owned_static_parent_seal() {
    let parsed = parse("box Plain { run() { return 1 } }\nstatic box Api { run() { return 2 } }");
    ParserNormalRootExecutionTestTerminalV1::observe_once(parsed, |loan| {
        let ParserStaticBoxParentSourceDispositionV1::Ready(seal) = loan.static_box_parent_source()
        else {
            panic!("same-brand mixed program should retain the static parent seal")
        };
        assert_eq!(seal.declaration_syntax().name(), "Api");
        assert_eq!(seal.direct_method_relations().count(), 1);
    });
}
