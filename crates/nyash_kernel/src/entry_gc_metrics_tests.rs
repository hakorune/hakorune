use super::trial_metrics_json;
use nyash_rust::runtime::{gc_controller::TrialReachability, gc_trace::TraceUnavailable};

#[test]
fn trial_metrics_preserve_unavailable_and_empty_success_distinction() {
    for (observation, status, reason) in [
        (None, "unavailable", Some("controller-unavailable")),
        (Some(TrialReachability::NotRun), "not_run", None),
        (
            Some(TrialReachability::Incomplete(TraceUnavailable::MapStorage)),
            "incomplete",
            Some("map-storage-unavailable"),
        ),
        (
            Some(TrialReachability::Incomplete(TraceUnavailable::ModuleRoots)),
            "incomplete",
            Some("module-roots-unavailable"),
        ),
    ] {
        let fields = trial_metrics_json(observation);
        assert_eq!(fields["status"], status);
        assert!(fields["nodes"].is_null() && fields["edges"].is_null());
        assert_eq!(fields["error"].as_str(), reason);
    }
    let empty = trial_metrics_json(Some(TrialReachability::Complete { nodes: 0, edges: 0 }));
    assert_eq!(empty["status"], "complete");
    assert_eq!(empty["nodes"], 0);
    assert_eq!(empty["edges"], 0);
    assert!(empty["error"].is_null());
}
