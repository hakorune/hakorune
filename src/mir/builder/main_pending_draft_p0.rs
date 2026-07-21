//! HEADERPORT0-I0-MAINPENDING0-P0: disconnected root completion matrix.

use super::main_pending_draft::{
    MainCompletionRequestV1, MainDraftIdentityV1, MainHeaderLoanV1, MainHeaderSourceV1,
};
use super::root_body_completion::{RootBodyCompletionTrackerV1, RootBodyResultV1};
use crate::mir::{
    BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType, ValueId,
};

fn draft(symbol: &str, arity: usize) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol.to_owned(),
            params: vec![MirType::Integer; arity],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn no_value_body() -> super::root_body_completion::CompletedRootBodyV1 {
    RootBodyCompletionTrackerV1::new()
        .complete(RootBodyResultV1::NoValue)
        .unwrap()
}

#[test]
fn header_source_matrix_preserves_the_selected_route_without_fallback() {
    let invocation_headers = MirModule::new("invocation".into());
    let compatibility_headers = MirModule::new("compatibility".into());

    let invocation_request =
        MainCompletionRequestV1::new(MainDraftIdentityV1::root(), no_value_body(), false);
    let invocation_loan =
        MainHeaderLoanV1::new(&invocation_headers, MainHeaderSourceV1::InvocationCollector);
    assert_eq!(invocation_loan.signature("main"), None);
    let invocation_pending = invocation_request
        .finish(draft("main", 0), invocation_loan)
        .unwrap();
    assert_eq!(
        invocation_pending.header_source(),
        MainHeaderSourceV1::InvocationCollector
    );

    let compatibility_request =
        MainCompletionRequestV1::new(MainDraftIdentityV1::root(), no_value_body(), false);
    let compatibility_loan = MainHeaderLoanV1::new(
        &compatibility_headers,
        MainHeaderSourceV1::ModuleCompatibility,
    );
    let compatibility_pending = compatibility_request
        .finish(draft("main", 0), compatibility_loan)
        .unwrap();
    assert_eq!(
        compatibility_pending.header_source(),
        MainHeaderSourceV1::ModuleCompatibility
    );
}

#[test]
fn root_value_and_no_value_dispositions_are_preserved() {
    let headers = MirModule::new("headers".into());
    let value_request = MainCompletionRequestV1::new(
        MainDraftIdentityV1::root(),
        RootBodyCompletionTrackerV1::new()
            .complete(RootBodyResultV1::Value(ValueId::new(17)))
            .unwrap(),
        true,
    );
    let value_pending = value_request
        .finish(
            draft("main", 0),
            MainHeaderLoanV1::new(&headers, MainHeaderSourceV1::InvocationCollector),
        )
        .unwrap();
    assert_eq!(
        value_pending.root_body().result(),
        RootBodyResultV1::Value(ValueId::new(17))
    );
    assert!(value_pending.returns_value());

    let no_value_request =
        MainCompletionRequestV1::new(MainDraftIdentityV1::root(), no_value_body(), false);
    let no_value_pending = no_value_request
        .finish(
            draft("main", 0),
            MainHeaderLoanV1::new(&headers, MainHeaderSourceV1::ModuleCompatibility),
        )
        .unwrap();
    assert_eq!(
        no_value_pending.root_body().result(),
        RootBodyResultV1::NoValue
    );
    assert!(!no_value_pending.returns_value());
}
