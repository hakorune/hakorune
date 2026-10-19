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
        exit: (SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(8),
        ]))).into(),
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
    for exit in [
        SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![SourcePathSegmentV1::Body(9)])).into(),
        super::RootHomeExitKeyV1::ImplicitBodyEnd,
    ] {
        let mut foreign_exit = full[1].clone();
        foreign_exit.exit = exit;
        assert!(RootHomeCleanupOrderV1::ordinary(vec![full[0].clone(), foreign_exit])
            .unwrap_err().contains("foreign-exit"));
    }
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

#[test]
fn root_cleanup_order_expands_original_received_homes_and_preserves_fault_only_home() {
    use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog;
    use crate::mir::resolved_semantics::SourceBindingSiteV1;
    for owned in [false, true] {
        let fields = if owned {
            "items: ArrayBox = new ArrayBox() children: ArrayBox = new ArrayBox() birth() {}"
        } else {
            ""
        };
        let package = issue_with_brand_catalog(&format!("box Token {{ {fields} }} box Maker {{ make(size: i64) {{ return new Token() }} relay() {{ local first = me.make(7) local item = me.make(8) return item }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let (owner, exit) = ledger
            .normal_return_dispositions
            .as_ref()
            .unwrap()
            .keys()
            .next()
            .unwrap()
            .clone();
        let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
            "Maker", "make", 1,
        );
        let callee = ledger.callable_result_classes.outcomes(&key).unwrap()[0]
            .site()
            .owner();
        let flow = ledger.completion_index[&owner]
            .as_ref()
            .unwrap()
            .cleanup()
            .root_flow()
            .unwrap();
        for (index, call) in flow.local_calls().iter().enumerate() {
            let (declaration, binding) = call.local_binding().unwrap();
            let SourceBindingSiteV1::Local { statement, ordinal } = declaration else {
                panic!("local")
            };
            let value = ValueId(10 + index as u32);
            ledger
                .begin_handle_call_emission(call.site(), callee)
                .unwrap();
            // Installed-value fixture only; no claim of final physical verification.
            ledger
                .record_handle_call_emission(
                    call.site(),
                    value,
                    vec![(
                        crate::mir::BasicBlockId(0),
                        crate::mir::MirInstruction::Copy {
                            dst: value,
                            src: value,
                        },
                    )],
                )
                .unwrap();
            ledger
                .complete_local_installation(
                    owner,
                    statement.node(),
                    &[(binding, *ordinal, value, value)],
                )
                .unwrap();
        }
        let projection = ledger
            .normal_exit_projection_v1(owner, &exit)
            .unwrap()
            .unwrap();
        let rows = ledger.local_commits.borrow();
        let order = RootHomeCleanupOrderV1::from_home_end_plans(
            &rows,
            &exit,
            projection.fault_homes(),
            projection.homes(),
        )
        .unwrap()
        .unwrap();
        let (selected_full, selected_normal) = source_home_end_selection_v2(
            &rows, projection.fault_homes(), projection.homes()).unwrap().unwrap();
        assert_eq!(selected_full.iter().map(|(_, subject, operation)| (subject, operation)).collect::<Vec<_>>(),
            order.full().iter().map(|origin| (origin.subject(), origin.operation())).collect::<Vec<_>>());
        for step in 0..selected_normal.len() {
            assert_eq!(fault_indices_after_normal_step_v2(selected_full.len(), &selected_normal, step).unwrap(),
                order.fault_indices_after_normal_step(step).unwrap());
        }
        let plan_len = if owned { 3 } else { 1 };
        assert_eq!(order.full().len(), 2 * plan_len);
        assert_eq!(order.normal().len(), plan_len);
        assert_eq!(order.full()[0].binding(), Some(projection.fault_homes()[0].binding()));
        assert_eq!(order.normal()[0].binding(), Some(projection.homes()[0].binding()));
        assert_eq!(order.acquisition_fault(), order.full());
        assert_eq!(
            order.fault_after_normal_step(plan_len - 1).unwrap(),
            order.full()[..plan_len]
        );
        for origin in &order.full()[..plan_len] {
            assert_eq!(origin.binding(), Some(projection.fault_homes()[0].binding()));
            assert!(!order.normal().contains(origin));
        }
        if owned {
            let ordinals: Vec<_> = order.full()[..plan_len]
                .iter()
                .filter_map(|origin| match origin.subject() {
                    RootHomeReleaseSubjectV1::FieldResidence { field, .. } => {
                        Some(field.declaration_ordinal())
                    }
                    _ => None,
                })
                .collect();
            assert_eq!(ordinals, [1, 0]);
        }
        // This source Call Fault has no root exit key. The shared expansion
        // follows exactly the supplied order, including each complete child End.
        let input = [projection.fault_homes()[1].clone(), projection.fault_homes()[0].clone()];
        let call_fault = source_home_end_plans_v1(&rows, &input).unwrap().unwrap();
        assert_eq!(call_fault.len(), 2 * plan_len);
        for (index, binding) in input.iter().enumerate() {
            let part = &call_fault[index * plan_len..(index + 1) * plan_len];
            assert!(part.iter().all(|(home, subject, _)| home == binding && match subject {
                RootHomeReleaseSubjectV1::Binding(actual)
                | RootHomeReleaseSubjectV1::FieldResidence { binding: actual, .. } => *actual == binding.binding(),
                _ => false,
            }));
            if owned {
                let fields: Vec<_> = part.iter().filter_map(|(_, subject, _)| match subject {
                    RootHomeReleaseSubjectV1::FieldResidence { field, .. } => Some(field.declaration_ordinal()), _ => None,
                }).collect();
                assert_eq!(fields, [1, 0], "complete child order inside each source Home");
            }
        }
        let full = projection.fault_homes();
        for (f, n, error) in [
            (vec![full[0].clone(), full[0].clone()], vec![], "duplicate-home"),
            (vec![full[0].clone()], vec![full[1].clone()], "foreign-home"),
            (full.to_vec(), vec![full[1].clone(), full[0].clone()], "home-order"),
            (full.to_vec(), vec![full[0].clone(), full[0].clone()], "home-order"),
        ] {
            assert!(
                RootHomeCleanupOrderV1::from_home_end_plans(&rows, &exit, &f, &n)
                    .unwrap_err()
                    .contains(error)
            );
        }
        drop(rows);
        let mut rows = ledger.local_commits.borrow_mut();
        let unavailable_site = rows
            .iter()
            .find(|(_, row)| row.installs(full[0].binding()))
            .unwrap()
            .0
            .clone();
        let missing_site = rows
            .iter()
            .find(|(_, row)| row.installs(full[1].binding()))
            .unwrap()
            .0
            .clone();
        // A retained child cannot outlive an anchor selected for Normal End.
        // Both root exits and Loop ports use this same source-only selection.
        let super::super::super::LocalCommitV1::CallReceived(child) =
            rows.get_mut(&unavailable_site).unwrap() else { panic!("received"); };
        let saved_parents = child.field_support_parents.replace(vec![full[1].binding()].into_boxed_slice());
        for normal in [vec![], vec![full[1].clone()], full.to_vec()] {
            let selected = source_home_end_selection_v2(&rows, full, &normal);
            let root = RootHomeCleanupOrderV1::from_home_end_plans(&rows, &exit, full, &normal);
            if normal == vec![full[1].clone()] {
                assert!(selected.unwrap_err().contains("escaping-local-anchor"));
                assert!(root.unwrap_err().contains("escaping-local-anchor"));
            } else {
                assert!(selected.unwrap().is_none(), "fixture has no retained physical Call packet");
                assert!(root.unwrap().is_none());
            }
        }
        let super::super::super::LocalCommitV1::CallReceived(child) =
            rows.get_mut(&unavailable_site).unwrap() else { panic!("received"); };
        child.field_support_parents = saved_parents;
        let super::super::super::LocalCommitV1::CallReceived(home) =
            rows.get_mut(&unavailable_site).unwrap()
        else {
            panic!("received")
        };
        // Structural End-plan check only: this installed fixture does not
        // issue a mixed source/Call seal. Even with child residences, the
        // conditional outcome guards the whole End rather than its outer
        // release alone.
        let original_release = home.release;
        home.release = super::super::super::call_received::CallReceivedReleaseV1::Mixed { formal_ordinal: 0 };
        let conditional = home.end_plan();
        let [(RootHomeReleaseSubjectV1::Binding(binding),
            crate::mir::instruction::InvokeOperation::ObjectEndIfOwned { object, value, end })] = conditional.as_ref()
            else { panic!("one conditional envelope for the entire End"); };
        assert_eq!((*binding, *object, *value), (home.binding, home.object, home.local().unwrap()));
        use crate::mir::instruction::ObjectEndStepV1 as Step;
        if owned {
            assert!(matches!(end.as_ref(), [Step::ArrayField { field: second }, Step::ArrayField { field: first }, Step::ReleaseOuter]
                if second.object() == home.object && first.object() == home.object
                    && second.declaration_ordinal() == 1 && first.declaration_ordinal() == 0));
        } else {
            assert_eq!(end.as_ref(), &[Step::ReleaseOuter]);
        }
        home.release = original_release;
        home.end_children = Some(None);
        assert!(RootHomeCleanupOrderV1::from_home_end_plans(
            &rows,
            &exit,
            full,
            projection.homes()
        )
        .unwrap()
        .is_none());
        let removed = rows.remove(&missing_site).unwrap();
        assert!(RootHomeCleanupOrderV1::from_home_end_plans(
            &rows,
            &exit,
            full,
            projection.homes()
        )
        .unwrap_err()
        .contains("root-home-not-installed"));
        rows.insert(missing_site.clone(), removed);
        let super::super::super::LocalCommitV1::CallReceived(original) = &rows[&missing_site]
        else {
            panic!("received")
        };
        use super::super::super::call_received::{
            CallReceivedCommitV1, CallReceivedPhase, CallReceivedProgress,
        };
        let duplicate = CallReceivedCommitV1 {
            owner: original.owner,
            binding: original.binding,
            declaration: original.declaration.clone(),
            object: original.object,
            release: original.release,
            end_children: original.end_children.clone(),
            field_support_parents: original.field_support_parents.clone(),
            progress: CallReceivedProgress::Emitted {
                result: original.local().unwrap(),
                bindings: vec![],
                packet: None,
                phase: CallReceivedPhase::Installed,
            },
        };
        let original_result_site = ledger.callable_result_classes.outcomes(&key).unwrap()[0]
            .site()
            .clone();
        rows.insert(
            original_result_site,
            super::super::super::LocalCommitV1::CallReceived(duplicate),
        );
        assert!(RootHomeCleanupOrderV1::from_home_end_plans(
            &rows,
            &exit,
            full,
            projection.homes()
        )
        .unwrap_err()
        .contains("duplicate-root-home"));
    }
}

#[path = "root_cleanup_order_path_tests.rs"]
mod path_tests;
