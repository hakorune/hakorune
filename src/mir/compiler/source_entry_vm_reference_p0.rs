use super::raw_root_environment_manifest::RawRootEnvironmentManifestV1;
use super::raw_root_source_facts::RawRootSourceRouteV1;
use super::source_entry_result::{
    CanonicalProcessExitV1, ProcessExitCodeV1, ProcessExitProfileV1, ProcessFaultV1,
    SealedObjectResultV1, SealedSourceFaultV1, SourceEntryResultKindV1, SourceEntryResultV1,
    UnitOriginV1,
};
use super::source_entry_selection::{select_source_entry, SelectedSourceEntryRouteV1};
use super::source_entry_vm_reference::VmReferenceProcessOutcomeV1;

fn consume(
    route: RawRootSourceRouteV1,
    result: SourceEntryResultV1,
) -> VmReferenceProcessOutcomeV1 {
    select_source_entry(RawRootEnvironmentManifestV1::from_test(route))
        .begin_thunk()
        .complete(result)
        .into_physical()
        .prepare_process_projection(ProcessExitProfileV1::Canonical(CanonicalProcessExitV1::V1))
        .expect("canonical profile")
        .project()
        .consume_vm_reference()
}

#[test]
fn script_unit_and_app_byte_status_are_exact() {
    let script = consume(
        RawRootSourceRouteV1::Script,
        SourceEntryResultV1::Unit(UnitOriginV1::EmptyBody),
    );
    assert_eq!(script.status(), ProcessExitCodeV1::zero());
    assert_eq!(script.fault(), None);
    assert_eq!(script.route_for_test(), SelectedSourceEntryRouteV1::Script);
    script.discard();

    for value in [0_i64, 255] {
        let app = consume(
            RawRootSourceRouteV1::App,
            SourceEntryResultV1::Integer(value),
        );
        assert_eq!(app.status().normalized_i64(), value);
        assert_eq!(app.fault(), None);
        assert_eq!(app.route_for_test(), SelectedSourceEntryRouteV1::AppMain0);
        app.discard();
    }
}

#[test]
fn out_of_range_status_is_reserved_and_keeps_exact_value() {
    for value in [-1_i64, 256] {
        let outcome = consume(
            RawRootSourceRouteV1::App,
            SourceEntryResultV1::Integer(value),
        );
        assert_eq!(outcome.status(), ProcessExitCodeV1::reserved_fault());
        assert_eq!(
            outcome.fault(),
            Some(&ProcessFaultV1::ExitCodeOutOfRange { value })
        );
        outcome.discard();
    }
}

#[test]
fn unsupported_results_keep_exact_kind_without_success_fallback() {
    for (result, kind) in [
        (
            SourceEntryResultV1::Bool(true),
            SourceEntryResultKindV1::Bool,
        ),
        (
            SourceEntryResultV1::Float(1.5),
            SourceEntryResultKindV1::Float,
        ),
        (
            SourceEntryResultV1::String("text".into()),
            SourceEntryResultKindV1::String,
        ),
        (
            SourceEntryResultV1::Object(SealedObjectResultV1::new("ArrayBox".into())),
            SourceEntryResultKindV1::Object,
        ),
    ] {
        let outcome = consume(RawRootSourceRouteV1::App, result);
        assert_eq!(outcome.status(), ProcessExitCodeV1::reserved_fault());
        assert_eq!(
            outcome.fault(),
            Some(&ProcessFaultV1::UnsupportedProcessResult { kind })
        );
        outcome.discard();
    }
}

#[test]
fn source_fault_keeps_code_and_detail_with_reserved_status() {
    let outcome = consume(
        RawRootSourceRouteV1::Script,
        SourceEntryResultV1::Fault(SealedSourceFaultV1::new(
            "body-fault",
            "exact detail".into(),
        )),
    );
    assert_eq!(outcome.status(), ProcessExitCodeV1::reserved_fault());
    assert_eq!(
        outcome.fault(),
        Some(&ProcessFaultV1::SourceFault {
            code: "body-fault",
            detail: "exact detail".into(),
        })
    );
    assert_eq!(outcome.route_for_test(), SelectedSourceEntryRouteV1::Script);
    outcome.discard();
}
