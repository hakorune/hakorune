use super::*;

fn package() -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1
{
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Token {} box Maker { make(size: i64) { return new Token() } relay(size: i64) { return me.make(size) } } static box Main { main() { return 0 } }"
    ).unwrap()
}
fn loan(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
) -> crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1 {
    let claims = &package.ordinary_new_claim_ledger.callable_result_classes;
    let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Maker", "relay", 1,
    );
    claims
        .object_return_qualification(claims.outcomes(&key).unwrap()[0].site())
        .unwrap()
}

#[test]
fn object_argument_request_rejects_foreign_facts_before_missing_target() {
    let package = package();
    let foreign = self::package();
    let own = loan(&package);
    let other = loan(&foreign);
    let ledger = &package.ordinary_new_claim_ledger;
    let mut pending = ledger.borrowed_formal_actuals.clone();
    let original = pending.clone();
    for (qualification, accepted) in [(&own, true), (&other, false)] {
        let result = borrowed_call_arguments_callback_v1(
            &Ok(Vec::new()),
            &package.parameter_contracts,
            &[],
            None,
            ledger.borrowed_formal_source.as_ref().unwrap(),
            &Ok(Default::default()),
            &mut pending,
            &ledger.borrowed_i64_results,
            &mut |_| None,
            qualification.call(),
            BorrowedCallActualRequestV1::ObjectArguments(qualification, None),
            &ledger.callable_result_classes,
            &ledger.receiver_call_observations,
        );
        if accepted {
            assert!(result.unwrap().is_none());
        } else {
            assert!(format!("{:?}", result.unwrap_err()).contains("foreign-object-qualification"));
        }
        assert_eq!(
            pending, original,
            "a demand never stages a second authority"
        );
    }
}
