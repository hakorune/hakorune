//! BORROW-P0-ROOT-P0b: collector-wide root-batch transaction proof.

use super::main_pending_draft::{
    MainCompletionRequestV1, MainDraftIdentityV1, MainHeaderLoanV1, MainHeaderSourceV1,
    PendingMainDraftV1,
};
use super::module_draft_collector::{
    CompletedDraftSignatureViewV1, DraftPublicationPolicyV1, FunctionDraftKeyV1,
    ModuleDraftAdmissionErrorV1, ModuleDraftCollectorV1, RootCollectorBatchPrepareErrorV1,
};
use super::module_invocation_drain::ConditionFnPolicyV1;
use super::root_body_completion::{RootBodyCompletionTrackerV1, RootBodyResultV1};
use super::root_draft_batch::PreparedRootDraftBatchV1;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType};

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

fn pending_main() -> PendingMainDraftV1 {
    let root = RootBodyCompletionTrackerV1::new()
        .complete(RootBodyResultV1::NoValue)
        .unwrap();
    let request = MainCompletionRequestV1::new(MainDraftIdentityV1::root(), root, false);
    let headers = MirModule::new("headers".into());
    request
        .finish(
            draft("main", 0),
            MainHeaderLoanV1::new(&headers, MainHeaderSourceV1::InvocationCollector),
        )
        .unwrap()
}

fn root_batch() -> PreparedRootDraftBatchV1 {
    PreparedRootDraftBatchV1::prepare(
        pending_main(),
        Some(draft("condition_fn", 1)),
        ConditionFnPolicyV1::Required,
    )
    .unwrap()
}

fn collect(
    collector: &mut ModuleDraftCollectorV1,
    key: FunctionDraftKeyV1,
    symbol: &str,
    arity: usize,
    policy: DraftPublicationPolicyV1,
) {
    collector
        .prepare_admission(key, symbol.to_owned(), arity, policy)
        .unwrap()
        .seal(draft(symbol, arity))
        .unwrap()
        .collect();
}

fn symbols(collector: &ModuleDraftCollectorV1) -> Vec<String> {
    let mut symbols = Vec::new();
    collector.visit_symbols(&mut |symbol| symbols.push(symbol.to_owned()));
    symbols
}

#[test]
fn second_root_admission_failure_preserves_exact_collector_prefix() {
    let mut collector = ModuleDraftCollectorV1::default();
    collect(
        &mut collector,
        FunctionDraftKeyV1::LegacySymbol("prefix/0".into()),
        "prefix/0",
        0,
        DraftPublicationPolicyV1::LegacyReplaceWholePair,
    );
    collect(
        &mut collector,
        FunctionDraftKeyV1::SyntheticConditionFn,
        "condition_fn",
        1,
        DraftPublicationPolicyV1::CanonicalRejectDuplicate,
    );
    let before = symbols(&collector);

    let rejected = collector.prepare_root_batch(root_batch()).unwrap_err();
    assert_eq!(symbols(rejected.collector()), before);
    assert!(!rejected.collector().contains_symbol("main"));
    assert!(matches!(
        rejected.error(),
        RootCollectorBatchPrepareErrorV1::Admission {
            ordinal: 1,
            source: ModuleDraftAdmissionErrorV1::DuplicateKey(
                FunctionDraftKeyV1::SyntheticConditionFn
            ),
            ..
        }
    ));
}

#[test]
fn prepared_root_batch_commits_main_and_condition_once_after_full_preflight() {
    let mut collector = ModuleDraftCollectorV1::default();
    collect(
        &mut collector,
        FunctionDraftKeyV1::LegacySymbol("prefix/0".into()),
        "prefix/0",
        0,
        DraftPublicationPolicyV1::LegacyReplaceWholePair,
    );

    let prepared = collector.prepare_root_batch(root_batch()).unwrap();
    let (collector, receipt) = prepared.commit();
    assert_eq!(receipt.admissions().len(), 2);
    assert_eq!(
        symbols(&collector),
        vec!["condition_fn", "main", "prefix/0"]
    );
    assert_eq!(collector.signature("main").unwrap().params.len(), 0);
    assert_eq!(collector.signature("condition_fn").unwrap().params.len(), 1);
}

#[test]
fn legacy_main_replacement_is_prepared_with_the_whole_root_batch() {
    let mut collector = ModuleDraftCollectorV1::default();
    collect(
        &mut collector,
        FunctionDraftKeyV1::Main,
        "main",
        0,
        DraftPublicationPolicyV1::LegacyReplaceWholePair,
    );

    let prepared = collector.prepare_root_batch(root_batch()).unwrap();
    let (collector, receipt) = prepared.commit();
    assert_eq!(receipt.admissions().len(), 2);
    assert_eq!(symbols(&collector), vec!["condition_fn", "main"]);
    assert_eq!(collector.symbol_count(), 2);
}
