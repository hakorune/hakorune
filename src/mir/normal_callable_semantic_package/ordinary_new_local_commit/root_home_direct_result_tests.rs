use super::*;
use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog;
use crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;

fn package(nullable: bool, owned: bool) -> VerifiedNormalCallableSemanticPackageV1 {
    let fields = if owned {
        "items: ArrayBox = new ArrayBox() tail: ArrayBox = new ArrayBox() birth() {}"
    } else {
        ""
    };
    let body = if nullable {
        "if size > 0 { return new Token() } return null"
    } else {
        "return new Token()"
    };
    issue_with_brand_catalog(&format!("box Token {{ {fields} }} box Maker {{ make(size: i64) {{ {body} }} relay() {{ return me.make(7) }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap()
}
#[test]
fn direct_result_order_binds_original_source_once_without_home_or_binding() {
    for nullable in [false, true] {
        for owned in [false, true] {
            let package = package(nullable, owned);
            let ledger = &package.ordinary_new_claim_ledger;
            let (owner, exit) = ledger
                .normal_return_dispositions
                .as_ref()
                .unwrap()
                .keys()
                .next()
                .unwrap();
            let view = ledger
                .verified_direct_object_return_source_v1(*owner, exit)
                .unwrap()
                .unwrap();
            let Some(source) = view.cleanup_source() else {
                assert!(nullable && owned, "only nullable children lack an envelope");
                continue;
            };
            assert!(ledger
                .normal_exit_projection_v1(*owner, exit)
                .unwrap()
                .unwrap()
                .homes()
                .is_empty());
            let mut order = RootHomeCleanupOrderV1::ordinary(Vec::new()).unwrap();
            order.retain_direct_source(source).unwrap();
            let value = crate::mir::MirBuilder::new().next_value_id();
            let kind = view.result();
            assert!(order
                .bind_direct_result(value, InvokeCallResultKind::I64)
                .is_err());
            assert!(order.full().is_empty(), "failed bind must not change order");
            order.bind_direct_result(value, kind).unwrap();
            assert!(order.normal().is_empty());
            assert!(order.acquisition_fault().is_empty());
            assert!(
                matches!(order.full().last().unwrap().subject(), RootHomeReleaseSubjectV1::DirectResult { site } if site == view.target().call_site())
            );
            assert!(order
                .full()
                .iter()
                .all(|origin| origin.binding().is_none() && origin.exit() == exit));
            if owned {
                let ordinals: Vec<_> = order
                    .full()
                    .iter()
                    .filter_map(|origin| match origin.subject() {
                        RootHomeReleaseSubjectV1::DirectResultField { field, .. } => {
                            Some(field.declaration_ordinal())
                        }
                        _ => None,
                    })
                    .collect();
                assert_eq!(ordinals, [1, 0]);
            }
            let count = order.full().len();
            assert!(order
                .bind_direct_result(value, kind)
                .unwrap_err()
                .contains("duplicate"));
            assert_eq!(order.full().len(), count);
            assert!(ledger.validate_no_pending_object_returns_v1().is_err());
        }
    }
}
#[test]
fn direct_result_fault_residual_keeps_result_after_normal_attempts() {
    let package = package(false, false);
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, exit) = ledger
        .normal_return_dispositions
        .as_ref()
        .unwrap()
        .keys()
        .next()
        .unwrap();
    let view = ledger
        .verified_direct_object_return_source_v1(*owner, exit)
        .unwrap()
        .unwrap();
    let mut order = RootHomeCleanupOrderV1::ordinary(Vec::new()).unwrap();
    order
        .retain_direct_source(view.cleanup_source().unwrap())
        .unwrap();
    // Graph/order model only: no installed Home or fake Binding is issued.
    let map = RootHomeReleaseOriginV1 {
        subject: RootHomeReleaseSubjectV1::ArgumentMap {
            site: view.target().call_site().clone(),
            ordinal: 0,
        },
        exit: exit.clone(),
        operation: crate::mir::instruction::InvokeOperation::Map(
            crate::mir::instruction::MapInvokeOperation::End { map: ValueId(77) },
        ),
    };
    order.prepend_argument_maps(vec![map.clone()]).unwrap();
    order
        .bind_direct_result(ValueId(78), view.result())
        .unwrap();
    assert_eq!(order.normal(), [map.clone()]);
    assert_eq!(order.acquisition_fault(), [map]);
    assert_eq!(order.full().len(), 2);
    assert_eq!(order.fault_after_normal_step(0).unwrap(), order.full()[..1]);
    assert!(order.prepend_argument_maps(Vec::new()).is_err());
}
