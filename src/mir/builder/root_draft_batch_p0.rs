//! HEADERPORT0-I0-ROOTBATCH0-P0: disconnected policy/failure matrix.

use super::main_pending_draft::{
    MainCompletionRequestV1, MainDraftIdentityV1, MainHeaderLoanV1, MainHeaderSourceV1,
    PendingMainDraftV1,
};
use super::root_body_completion::{RootBodyCompletionTrackerV1, RootBodyResultV1};
use super::root_draft_batch::{PreparedRootDraftBatchV1, RootDraftBatchErrorV1};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType};

fn main_draft() -> PendingMainDraftV1 {
    let body = RootBodyCompletionTrackerV1::new()
        .complete(RootBodyResultV1::NoValue)
        .unwrap();
    let request = MainCompletionRequestV1::new(MainDraftIdentityV1::root(), body, false);
    let headers = MirModule::new("headers".into());
    request
        .finish(
            MirFunction::new(
                FunctionSignature {
                    name: "main".into(),
                    params: Vec::new(),
                    return_type: MirType::Void,
                    effects: EffectMask::PURE,
                },
                BasicBlockId::new(0),
            ),
            MainHeaderLoanV1::new(&headers, MainHeaderSourceV1::InvocationCollector),
        )
        .unwrap()
}

fn condition_fn(symbol: &str, arity: usize) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol.to_owned(),
            params: vec![MirType::Integer; arity],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

#[test]
fn condition_policy_matrix_has_one_primary_result() {
    let optional_missing = PreparedRootDraftBatchV1::prepare(
        main_draft(),
        None,
        super::module_invocation_drain::ConditionFnPolicyV1::Optional,
    )
    .unwrap();
    assert_eq!(optional_missing.admissions().len(), 1);

    let optional_present = PreparedRootDraftBatchV1::prepare(
        main_draft(),
        Some(condition_fn("condition_fn", 1)),
        super::module_invocation_drain::ConditionFnPolicyV1::Optional,
    )
    .unwrap();
    assert_eq!(optional_present.admissions().len(), 2);

    assert_eq!(
        PreparedRootDraftBatchV1::prepare(
            main_draft(),
            None,
            super::module_invocation_drain::ConditionFnPolicyV1::Required,
        )
        .unwrap_err(),
        RootDraftBatchErrorV1::MissingConditionFn
    );
    assert_eq!(
        PreparedRootDraftBatchV1::prepare(
            main_draft(),
            Some(condition_fn("condition_fn", 1)),
            super::module_invocation_drain::ConditionFnPolicyV1::Forbidden,
        )
        .unwrap_err(),
        RootDraftBatchErrorV1::UnexpectedConditionFn
    );
}

#[test]
fn condition_identity_failures_are_typed_before_batch_creation() {
    assert_eq!(
        PreparedRootDraftBatchV1::prepare(
            main_draft(),
            Some(condition_fn("other", 1)),
            super::module_invocation_drain::ConditionFnPolicyV1::Required,
        )
        .unwrap_err(),
        RootDraftBatchErrorV1::ConditionSymbolMismatch {
            actual: "other".to_owned(),
        }
    );
    assert_eq!(
        PreparedRootDraftBatchV1::prepare(
            main_draft(),
            Some(condition_fn("condition_fn", 0)),
            super::module_invocation_drain::ConditionFnPolicyV1::Required,
        )
        .unwrap_err(),
        RootDraftBatchErrorV1::ConditionArityMismatch { actual: 0 }
    );
}
