use super::normal_root_source::ParserNormalRootSourceDispositionV1;
use super::script_source_rows::CanonicalScriptSourceRowsDispositionV1;
use crate::parser::{NyashParser, ParserBuildConfig};

fn parse(source: &str) -> super::ParsedProgramWithCallableParameterSourceV1 {
    NyashParser::parse_from_string_with_callable_parameter_source(
        source,
        ParserBuildConfig::default(),
    )
    .expect("parser product")
}

#[test]
fn pure_script_has_one_same_invocation_root_witness() {
    let parsed = parse("print(1)\n");
    assert!(matches!(
        parsed.normal_root_source(),
        ParserNormalRootSourceDispositionV1::ScriptReady(_)
    ));
}

#[test]
fn app_ready_cannot_be_discarded_into_script_a() {
    let parsed = parse("static box Main { main() { return 1 } }\n");
    let (disposition, _) = parsed.into_source_disposition_with_script_rows();
    assert!(matches!(
        disposition.discard_root_before_a(),
        Err(super::product::ParserCallableSourceRootRouteRejectV1::AppReadyRequiresNormalRootConsumer)
    ));
}

#[test]
fn script_a_discard_is_explicit_and_keeps_rows_separate() {
    let parsed = parse("print(1)\n");
    let (disposition, rows) = parsed.into_source_disposition_with_script_rows();
    let disposition = disposition
        .discard_root_before_a()
        .expect("Script root may be explicitly discarded before A");
    assert!(disposition.root_is_discarded_before_a());
    assert!(matches!(
        rows,
        CanonicalScriptSourceRowsDispositionV1::HandoffReady(_)
    ));
}

#[test]
fn nonzero_main_arity_stays_outside_instead_of_becoming_script() {
    let parsed = parse("static box Main { main(argument) { return argument } }\n");
    assert!(matches!(
        parsed.normal_root_source(),
        ParserNormalRootSourceDispositionV1::Outside(_)
    ));
}
