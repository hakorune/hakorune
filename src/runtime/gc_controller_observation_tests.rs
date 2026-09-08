use super::*;
use crate::box_trait::IntegerBox;
use crate::box_trait::NyashBox;
use crate::boxes::map_box::MapBox;
use std::sync::Arc;

#[test]
fn gc_native_map_child_projection_keeps_clone_distinct_from_share() {
    let outer = MapBox::new();
    let child = MapBox::new();
    let original = child.share_box();
    outer.insert_key_str("child".into(), Box::new(child));
    let mut visited = 0;
    gc_trace::trace_children(&outer, &mut |projected| {
        let cloned = projected.as_any().downcast_ref::<MapBox>().unwrap();
        cloned.insert_key_str("projected-only".into(), Box::new(IntegerBox::new(1)));
        visited += 1;
    })
    .unwrap();
    assert_eq!(visited, 1);
    assert_eq!(
        original
            .as_any()
            .downcast_ref::<MapBox>()
            .unwrap()
            .entry_count_i64(),
        0
    );
}

struct RootHandle(u64);
impl Drop for RootHandle {
    fn drop(&mut self) {
        crate::runtime::host_handles::drop_handle(self.0);
    }
}

#[test]
fn gc_trial_records_native_graph_then_replaces_success_on_map_failure() {
    let map = Arc::new(MapBox::new());
    map.insert_key_str("value".into(), Box::new(IntegerBox::new(17)));
    let controller = GcController::new(GcMode::RcDiagnostic);
    assert_eq!(
        controller.trial_reachability_last(),
        TrialReachability::NotRun
    );
    let graph = controller.trace_reachability(vec![map.clone()]).unwrap();
    assert_eq!(graph, ReachabilitySummary { nodes: 2, edges: 1 });
    let _root = RootHandle(crate::runtime::host_handles::to_handle_arc(map.clone()));
    controller.run_trial_collection();
    assert!(matches!(
        controller.trial_reachability_last(),
        TrialReachability::Complete { .. }
    ));
    assert_eq!(controller.collection_totals().0, 1);

    // Poison a real Map without poisoning a process-global root registry.
    let poisoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _entries = map.get_data().write().unwrap();
        panic!("test storage failure");
    }));
    assert!(poisoned.is_err());
    let mut callbacks = 0;
    assert_eq!(
        gc_trace::trace_children(map.as_ref(), &mut |_| callbacks += 1),
        Err(gc_trace::TraceUnavailable::MapStorage),
    );
    assert_eq!(callbacks, 0);
    controller.run_trial_collection();
    assert_eq!(
        controller.trial_reachability_last(),
        TrialReachability::Incomplete(gc_trace::TraceUnavailable::MapStorage)
    );
    assert_eq!(
        controller.collection_totals().0,
        2,
        "failed attempts still count"
    );
}

#[test]
fn gc_trial_module_failure_cannot_leave_an_old_successful_summary() {
    let controller = GcController::new(GcMode::RcDiagnostic);
    controller.record_reachability(Ok(ReachabilitySummary {
        nodes: 19,
        edges: 23,
    }));
    controller.record_reachability(Err(gc_trace::TraceUnavailable::ModuleRoots));
    assert_eq!(
        controller.trial_reachability_last(),
        TrialReachability::Incomplete(gc_trace::TraceUnavailable::ModuleRoots)
    );
    controller.record_reachability(Ok(ReachabilitySummary { nodes: 0, edges: 0 }));
    assert_eq!(
        controller.trial_reachability_last(),
        TrialReachability::Complete { nodes: 0, edges: 0 }
    );
}
