use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonKind;

pub(super) fn coreloop_base_gate(facts: &CanonicalLoopFacts) -> bool {
    matches!(facts.skeleton_kind, SkeletonKind::Loop) && facts.cleanup_kinds_present.is_empty()
}

pub(super) fn exit_kinds_empty(facts: &CanonicalLoopFacts) -> bool {
    facts.exit_kinds_present.is_empty()
}
