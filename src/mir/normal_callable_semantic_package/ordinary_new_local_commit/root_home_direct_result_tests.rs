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

const STORED_SOURCE: &str = "box UserStats { init { name, bytes } birth(name) { me.name = name me.bytes = 0 } add(bytes) { me.bytes = me.bytes + bytes } } box Aggregator { init { stats } birth() { me.stats = new MapBox() } statsFor(user) { if me.stats.has(user) { return me.stats.get(user) } local stats = new UserStats(user) me.stats.set(user, stats) return stats } ingest(user, bytes) { local stats = me.statsFor(user) stats.add(bytes) return 0 } } static box Main { main() { local agg = new Aggregator() return agg.ingest(\"ana\", 10) } }";

fn stored_loan(
    ledger: &OrdinaryNewClaimLedgerV1,
) -> (
    crate::mir::resolved_semantics::FunctionOwnerIdV1,
    crate::mir::normal_callable_semantic_package::StoredMapGetReturnSourceLoanV1,
) {
    let indexed = ledger
        .terminal_relation_index
        .values()
        .flat_map(|index| index.values());
    ledger
        .terminal_relation
        .values()
        .chain(indexed)
        .find_map(|relation| {
            ledger
                .stored_map_get_return_source_v1(relation.owner(), relation.return_site())
                .unwrap()
                .map(|loan| (relation.owner(), loan))
        })
        .expect("original stored Map get Return")
}

#[test]
fn return_source_retention_accepts_each_single_source_and_none() {
    let stored = issue_with_brand_catalog(STORED_SOURCE).unwrap();
    let (_, get) = stored_loan(&stored.ordinary_new_claim_ledger);
    let mut order = RootHomeCleanupOrderV1::ordinary(Vec::new()).unwrap();
    assert!(order.direct.is_none() && order.scalar.is_none() && order.stored_map_get.is_none());
    order.retain_stored_map_get_source(get).unwrap();
    assert!(order.stored_map_get.is_some() && order.direct.is_none() && order.scalar.is_none());
    let direct = package(false, false);
    let mut order = RootHomeCleanupOrderV1::ordinary(Vec::new()).unwrap();
    order.retain_direct_source(direct_source(&direct)).unwrap();
    assert!(order.direct.is_some() && order.scalar.is_none() && order.stored_map_get.is_none());
    let scalar = issue_with_brand_catalog(SCALAR_SOURCE).unwrap();
    let (exit, loan) = scalar_loan(&scalar.ordinary_new_claim_ledger);
    let mut order = RootHomeCleanupOrderV1::ordinary(Vec::new()).unwrap();
    order.retain_scalar_source(exit, loan).unwrap();
    assert!(order.scalar.is_some() && order.direct.is_none() && order.stored_map_get.is_none());
}

#[test]
fn return_source_retention_rejects_pairs_in_both_orders_and_repeats() {
    let stored = issue_with_brand_catalog(STORED_SOURCE).unwrap();
    let (_, get) = stored_loan(&stored.ordinary_new_claim_ledger);
    let direct_package = package(false, false);
    let scalar_package = issue_with_brand_catalog(SCALAR_SOURCE).unwrap();
    let (scalar_exit, scalar) = scalar_loan(&scalar_package.ordinary_new_claim_ledger);
    let fresh = || RootHomeCleanupOrderV1::ordinary(Vec::new()).unwrap();
    // stored first, then each other source
    let mut order = fresh();
    order.retain_stored_map_get_source(get.clone()).unwrap();
    assert!(order
        .retain_direct_source(direct_source(&direct_package))
        .unwrap_err()
        .contains(EXCLUSIVE));
    assert!(order
        .retain_scalar_source(scalar_exit.clone(), scalar.clone())
        .unwrap_err()
        .contains(EXCLUSIVE));
    assert!(order.direct.is_none() && order.scalar.is_none());
    // the same field retained twice is a different rejection from a mixed pair
    assert!(order
        .retain_stored_map_get_source(get.clone())
        .unwrap_err()
        .contains("stored-map-get-source-drift"));
    // each other source first, then stored
    let mut order = fresh();
    order
        .retain_direct_source(direct_source(&direct_package))
        .unwrap();
    assert!(order
        .retain_stored_map_get_source(get.clone())
        .unwrap_err()
        .contains(EXCLUSIVE));
    assert!(order.stored_map_get.is_none());
    let mut order = fresh();
    order
        .retain_scalar_source(scalar_exit.clone(), scalar.clone())
        .unwrap();
    assert!(order
        .retain_stored_map_get_source(get)
        .unwrap_err()
        .contains(EXCLUSIVE));
    assert!(order.stored_map_get.is_none());
    // existing direct/scalar rejections keep their own wording and priority
    let mut order = fresh();
    order
        .retain_direct_source(direct_source(&direct_package))
        .unwrap();
    assert!(order
        .retain_scalar_source(scalar_exit, scalar)
        .unwrap_err()
        .contains("scalar-result-source-drift"));
    assert!(order
        .retain_direct_source(direct_source(&direct_package))
        .unwrap_err()
        .contains("direct-result-source-drift"));
}

#[test]
fn return_source_retention_rejects_a_loan_from_another_exit() {
    let stored = issue_with_brand_catalog(STORED_SOURCE).unwrap();
    let (_, get) = stored_loan(&stored.ordinary_new_claim_ledger);
    let other = package(false, false);
    let ledger = &other.ordinary_new_claim_ledger;
    let (owner, other_exit) = ledger
        .normal_return_dispositions
        .as_ref()
        .unwrap()
        .keys()
        .next()
        .unwrap();
    let view = ledger
        .verified_direct_object_return_source_v1(*owner, other_exit)
        .unwrap()
        .unwrap();
    let origin = RootHomeReleaseOriginV1 {
        subject: RootHomeReleaseSubjectV1::ArgumentMap {
            site: view.target().call_site().clone(),
            ordinal: 0,
        },
        exit: other_exit.clone().into(),
        operation: crate::mir::instruction::InvokeOperation::Map(
            crate::mir::instruction::MapInvokeOperation::End { map: ValueId(1) },
        ),
    };
    let mut order = RootHomeCleanupOrderV1::ordinary(vec![origin]).unwrap();
    assert!(order
        .retain_stored_map_get_source(get)
        .unwrap_err()
        .contains("stored-map-get-source-drift"));
    assert!(order.stored_map_get.is_none());
}

#[test]
fn final_validation_rejects_every_pair_injected_without_retention() {
    // Contract proof the retention tests cannot give: an order built by writing the
    // fields directly (bypassing retain_*) must not pass the final validator, in
    // every pair and both setting orders.
    let stored = issue_with_brand_catalog(STORED_SOURCE).unwrap();
    let ledger = &stored.ordinary_new_claim_ledger;
    let (owner, get) = stored_loan(ledger);
    let exit = get.terminal().return_site().clone();
    let direct_package = package(false, false);
    let scalar_package = issue_with_brand_catalog(SCALAR_SOURCE).unwrap();
    let (scalar_exit, scalar) = scalar_loan(&scalar_package.ordinary_new_claim_ledger);
    let entry = RootHomeExitEntry::Plain {
        local_bindings: Vec::new(),
    };
    let check = |order: &RootHomeCleanupOrderV1| {
        order.validate_direct_entry(ledger, owner, &exit, &entry, &[], None)
    };
    let set_direct =
        |o: &mut RootHomeCleanupOrderV1| o.direct = Some((direct_source(&direct_package), None));
    let set_scalar = |o: &mut RootHomeCleanupOrderV1| {
        o.scalar = Some((scalar_exit.clone(), scalar.clone(), None))
    };
    let set_stored = |o: &mut RootHomeCleanupOrderV1| o.stored_map_get = Some(get.clone());
    let setters: [&dyn Fn(&mut RootHomeCleanupOrderV1); 3] =
        [&set_direct, &set_scalar, &set_stored];
    for (a, first) in setters.iter().enumerate() {
        for (b, second) in setters.iter().enumerate() {
            if a == b {
                continue;
            }
            let mut order = RootHomeCleanupOrderV1::ordinary(Vec::new()).unwrap();
            first(&mut order);
            second(&mut order);
            assert!(
                check(&order).unwrap_err().contains(EXCLUSIVE),
                "pair {a}+{b}"
            );
        }
    }
    let order = RootHomeCleanupOrderV1::ordinary(Vec::new()).unwrap();
    assert!(check(&order).is_ok(), "unselected order stays accepted");
}
