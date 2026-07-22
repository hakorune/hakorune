//! CUT0-S0-COMPAT0-P0: compatibility policy and typed failure fixtures.

use super::calls::CanonicalFunctionSessionErrorV1;
use super::decls::CallableMainCompatibilityLoweringErrorV1;

#[test]
fn selected_callable_failure_keeps_session_error_kind() {
    let error = CallableMainCompatibilityLoweringErrorV1::from(
        CanonicalFunctionSessionErrorV1::DuringCleanup {
            primary: "primary".to_owned(),
            cleanup: "cleanup".to_owned(),
        },
    );
    assert!(matches!(
        error,
        CallableMainCompatibilityLoweringErrorV1::Session(
            CanonicalFunctionSessionErrorV1::DuringCleanup { .. }
        )
    ));
}

#[test]
fn selected_callable_lowering_error_is_not_a_missing_receipt() {
    let error = CallableMainCompatibilityLoweringErrorV1::Lowering("lowering".to_owned());
    let rendered = error.to_string();
    assert!(rendered.contains("callable-main/lowering"));
    assert!(!rendered.contains("MissingCallableMainCompatibility"));
}
