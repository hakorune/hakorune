use super::super::{RootHomeReleaseOriginV1, RootHomeReleaseSubjectV1};
use super::*;
use crate::mir::instruction::{InvokeOperation, MapInvokeOperation};
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, FunctionOwnerIssuerV1, OwnedExprSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1,
};
use crate::mir::ValueId;

fn origin(index: u32) -> RootHomeReleaseOriginV1 {
    static OWNER: std::sync::OnceLock<FunctionOwnerIdV1> = std::sync::OnceLock::new();
    let owner = *OWNER.get_or_init(|| {
        FunctionOwnerIssuerV1::new_for_compilation()
            .unwrap()
            .issue()
            .unwrap()
    });
    RootHomeReleaseOriginV1 {
        subject: RootHomeReleaseSubjectV1::ArgumentMap {
            site: OwnedExprSiteV1::new(
                owner,
                SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                    SourcePathSegmentV1::Body(index),
                ])),
            ),
            ordinal: index,
        },
        exit: SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(8),
        ])),
        operation: InvokeOperation::Map(MapInvokeOperation::End {
            map: ValueId(index),
        }),
    }
}
#[test]
fn root_cleanup_order_retains_full_normal_and_each_fault_residual() {
    let full = vec![origin(1), origin(2), origin(3)];
    let order = RootHomeCleanupOrderV1::ordinary(full.clone()).unwrap();
    assert_eq!(order.full(), full);
    assert_eq!(order.normal(), full);
    assert_eq!(order.acquisition_fault(), full);
    for step in 0..full.len() {
        assert_eq!(
            order.fault_after_normal_step(step).unwrap(),
            full[step + 1..]
        );
    }
    assert!(order.fault_after_normal_step(full.len()).is_err());
}
#[test]
fn root_cleanup_order_fault_retains_normal_excluded_obligation_in_source_order() {
    let full = vec![origin(1), origin(2), origin(3)];
    let normal = vec![full[0].clone(), full[2].clone()];
    let order = RootHomeCleanupOrderV1::from_sequences(full.clone(), &normal, &full).unwrap();
    assert_eq!(order.normal(), normal);
    assert_eq!(order.fault_after_normal_step(0).unwrap(), full[1..]);
    assert_eq!(
        order.fault_after_normal_step(1).unwrap(),
        vec![full[1].clone()]
    );
    assert_eq!(order.acquisition_fault(), full);
}
#[test]
fn root_cleanup_order_rejects_duplicate_foreign_and_reordered_origins() {
    let full = vec![origin(1), origin(2)];
    assert!(
        RootHomeCleanupOrderV1::ordinary(vec![full[0].clone(), full[0].clone()])
            .unwrap_err()
            .contains("duplicate-subject")
    );
    for normal in [
        vec![origin(3)],
        vec![full[1].clone(), full[0].clone()],
        vec![full[0].clone(), full[0].clone()],
    ] {
        assert!(RootHomeCleanupOrderV1::from_sequences(full.clone(), &normal, &full).is_err());
    }
    let mut changed = full[0].clone();
    changed.operation = InvokeOperation::Map(MapInvokeOperation::End { map: ValueId(99) });
    assert!(
        RootHomeCleanupOrderV1::from_sequences(full.clone(), &[changed], &full)
            .unwrap_err()
            .contains("foreign-origin")
    );
    let mut foreign_exit = full[1].clone();
    foreign_exit.exit = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(9),
    ]));
    assert!(
        RootHomeCleanupOrderV1::ordinary(vec![full[0].clone(), foreign_exit])
            .unwrap_err()
            .contains("foreign-exit")
    );
}
#[test]
fn root_cleanup_order_argument_prefix_preserves_both_original_sequences_atomically() {
    let full = vec![origin(2), origin(3)];
    let normal = vec![full[1].clone()];
    let mut order = RootHomeCleanupOrderV1::from_sequences(full.clone(), &normal, &full).unwrap();
    let prefix = origin(1);
    order.prepend_argument_maps(vec![prefix.clone()]).unwrap();
    assert_eq!(
        order.full(),
        vec![prefix.clone(), full[0].clone(), full[1].clone()]
    );
    assert_eq!(order.normal(), vec![prefix.clone(), full[1].clone()]);
    assert_eq!(order.fault_after_normal_step(0).unwrap(), full);
    let before = order.full().to_vec();
    assert!(order.prepend_argument_maps(vec![prefix]).is_err());
    assert_eq!(order.full(), before);
}
