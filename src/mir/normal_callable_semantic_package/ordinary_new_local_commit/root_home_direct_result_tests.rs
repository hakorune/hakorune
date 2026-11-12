use super::*;
use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog;
use crate::mir::normal_callable_semantic_package::{
    OrdinaryNewClaimLedgerV1, VerifiedNormalCallableSemanticPackageV1,
};

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
        exit: exit.clone().into(),
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

#[test]
fn scalar_terminal_order_retains_same_source_without_result_home() {
    let package = issue_with_brand_catalog(
        "box Item { value: i64 birth() { me.value = 5 } } box Leaf { flag: i64 birth() { me.flag = 0 } read(p: Item) { if p == null { return 7 } return p.value } } box Parent { child: Leaf birth() { me.child = new Leaf() } run(p: Item) { return me.child.read(p) } } static box Main { main() { local parent = new Parent() local item = new Item() return parent.run(item) } }"
    ).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let terminal = ledger
        .borrowed_i64_results
        .keys()
        .find_map(|owner| {
            ledger
                .call_relations_for_owner(*owner)
                .into_iter()
                .find(|terminal| terminal.source_scalar().is_some())
        })
        .expect("original scalar terminal");
    let loan = terminal.source_scalar().unwrap();
    let super::super::super::prepare::RootHomePreparationV1::Prepared {
        mut order,
        direct_object,
    } = ledger
        .preview_root_home_exit_v1(terminal.owner(), terminal.return_site().node())
        .unwrap()
    else {
        panic!("original terminal Home flow");
    };
    assert!(!direct_object);
    assert_eq!(order.scalar.as_ref().unwrap().1, *loan);
    assert_eq!(order.ingress_result_kind(), InvokeCallResultKind::I64);
    let original = order.full().to_vec();
    order.prepend_argument_maps(Vec::new()).unwrap();
    assert_eq!(order.scalar.as_ref().unwrap().1, *loan);
    assert!(order
        .retain_scalar_source(terminal.return_site().clone(), loan.clone())
        .is_err());
    assert!(order
        .bind_direct_result(ValueId(77), InvokeCallResultKind::Bool)
        .is_err());
    order
        .bind_direct_result(ValueId(77), InvokeCallResultKind::I64)
        .unwrap();
    assert_eq!(order.full(), original);
    assert!(order.full().iter().all(|origin| !matches!(
        origin.subject(),
        RootHomeReleaseSubjectV1::DirectResult { .. }
            | RootHomeReleaseSubjectV1::DirectResultField { .. }
    )));
    assert!(order
        .bind_direct_result(ValueId(78), InvokeCallResultKind::I64)
        .is_err());
    assert!(order.prepend_argument_maps(Vec::new()).is_err());
}

const EXCLUSIVE: &str = "return-source-exclusive";
const SCALAR_SOURCE: &str = "box Item { value: i64 birth() { me.value = 5 } } box Leaf { flag: i64 birth() { me.flag = 0 } read(p: Item) { if p == null { return 7 } return p.value } } box Parent { child: Leaf birth() { me.child = new Leaf() } run(p: Item) { return me.child.read(p) } } static box Main { main() { local parent = new Parent() local item = new Item() return parent.run(item) } }";

fn direct_source(
    package: &VerifiedNormalCallableSemanticPackageV1,
) -> crate::mir::normal_callable_semantic_package::ordinary_new_coseal::completion_index::DirectRootCleanupSourceV1
{
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, exit) = ledger
        .normal_return_dispositions
        .as_ref()
        .unwrap()
        .keys()
        .next()
        .unwrap();
    ledger
        .verified_direct_object_return_source_v1(*owner, exit)
        .unwrap()
        .unwrap()
        .cleanup_source()
        .unwrap()
}
fn scalar_loan(
    ledger: &OrdinaryNewClaimLedgerV1,
) -> (
    crate::mir::resolved_semantics::SourceStmtSiteV1,
    crate::mir::normal_callable_semantic_package::BorrowedScalarResultSourceLoanV1,
) {
    let terminal = ledger
        .borrowed_i64_results
        .keys()
        .find_map(|owner| {
            ledger
                .call_relations_for_owner(*owner)
                .into_iter()
                .find(|terminal| terminal.source_scalar().is_some())
        })
        .expect("original scalar terminal");
    (
        terminal.return_site().clone(),
        terminal.source_scalar().unwrap().clone(),
    )
}

#[test]
fn final_validation_rejects_direct_and_scalar_injected_without_retention() {
    // Contract proof the retention tests cannot give: an order built by writing the
    // fields directly (bypassing retain_*) must not pass the final validator, in
    // either setting order. A stored_map_get loan needs a selected stored Return
    // owner that no unit-test source reaches, so that pair is not covered here.
    let direct_package = package(false, false);
    let ledger = &direct_package.ordinary_new_claim_ledger;
    let scalar_package = issue_with_brand_catalog(SCALAR_SOURCE).unwrap();
    let (scalar_exit, scalar) = scalar_loan(&scalar_package.ordinary_new_claim_ledger);
    let (owner, exit) = ledger
        .normal_return_dispositions
        .as_ref()
        .unwrap()
        .keys()
        .next()
        .unwrap();
    let entry = RootHomeExitEntry::Plain {
        local_bindings: Vec::new(),
    };
    let check = |order: &RootHomeCleanupOrderV1| {
        order.validate_direct_entry(ledger, *owner, exit, &entry, &[], None)
    };
    for direct_first in [true, false] {
        let mut order = RootHomeCleanupOrderV1::ordinary(Vec::new()).unwrap();
        let set_direct = |o: &mut RootHomeCleanupOrderV1| {
            o.direct = Some((direct_source(&direct_package), None))
        };
        let set_scalar = |o: &mut RootHomeCleanupOrderV1| {
            o.scalar = Some((scalar_exit.clone(), scalar.clone(), None))
        };
        if direct_first {
            set_direct(&mut order);
            set_scalar(&mut order);
        } else {
            set_scalar(&mut order);
            set_direct(&mut order);
        }
        assert!(check(&order).unwrap_err().contains(EXCLUSIVE));
    }
    let order = RootHomeCleanupOrderV1::ordinary(Vec::new()).unwrap();
    assert!(check(&order).is_ok(), "unselected order stays accepted");
}
