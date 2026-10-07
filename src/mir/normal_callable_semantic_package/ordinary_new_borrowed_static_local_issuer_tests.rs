//! The local issuer corroborates exact source identity at the callback seam.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{
    issue_static_source_local_for_test, BorrowedCallActualRequestV1, BorrowedCallArgumentsV1,
};

#[test]
fn static_local_issuer_rejects_same_arity_corrupted_source_arguments() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "static box Layout { pick(p) { return 0 } } box Heap { lookup(size) { local k = Layout.pick(size) return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7) return 0 } }"
    ).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let fact = source.static_arguments.values().next().unwrap();
    let original = ledger
        .local_call_for_owner(fact.call().owner(), fact.call().site())
        .unwrap();
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == original.owner())
        .unwrap();
    let (claim, _) = package
        .source_static_claims_for_test
        .claim_target(fact.call_source().caller(), fact.call().site())
        .unwrap();
    package
        .batch()
        .with_lowering_input(contract.batch_slot, |input| {
            for argument in [
                LocalCallArgumentV1::BorrowedActual {
                    ordinal: 1,
                    site: fact.use_site().site().clone(),
                },
                LocalCallArgumentV1::BorrowedActual {
                    ordinal: 0,
                    site: fact.call().site().clone(),
                },
                LocalCallArgumentV1::Scalar(fact.target_formal()),
                LocalCallArgumentV1::Bool(true),
            ] {
                let result = issue_static_source_local_for_test(
                    input,
                    original,
                    claim,
                    &mut |_, request| {
                        assert!(matches!(
                            request,
                            BorrowedCallActualRequestV1::QualifiedStaticSourceArguments(_)
                        ));
                        Ok(Some(BorrowedCallArgumentsV1::StaticSource(
                            vec![argument.clone()].into_boxed_slice(),
                        )))
                    },
                )
                .unwrap();
                assert!(result.is_none());
            }
            let mut demands = 0;
            let rejected =
                issue_static_source_local_for_test(input, original, claim, &mut |_, _| {
                    demands += 1;
                    Err("selected-refusal".into())
                });
            assert_eq!(rejected.unwrap_err(), "selected-refusal");
            assert_eq!(
                demands, 1,
                "a selected failure cannot retry strict arguments"
            );
            let exact = issue_static_source_local_for_test(input, original, claim, &mut |_, _| {
                Ok(Some(BorrowedCallArgumentsV1::StaticSource(
                    original.arguments().to_vec().into_boxed_slice(),
                )))
            })
            .unwrap()
            .unwrap();
            assert_eq!(exact.owner(), original.owner());
            assert_eq!(exact.site(), original.site());
            assert_eq!(exact.local_binding(), original.local_binding());
            assert_eq!(exact.prior_homes(), original.prior_homes());
            let wrong_role =
                issue_static_source_local_for_test(input, original, claim, &mut |_, _| {
                    Ok(Some(BorrowedCallArgumentsV1::Scalar(
                        original.arguments().to_vec().into_boxed_slice(),
                    )))
                })
                .unwrap();
            assert!(wrong_role.is_none());
        })
        .unwrap();
}
