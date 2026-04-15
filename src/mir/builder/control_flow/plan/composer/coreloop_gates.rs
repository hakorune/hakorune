use crate::mir::builder::control_flow::plan::facts::feature_facts::ExitKindFacts;
use crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonKind;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;

pub(super) fn coreloop_base_gate(facts: &CanonicalLoopFacts) -> bool {
    matches!(facts.skeleton_kind, SkeletonKind::Loop) && facts.cleanup_kinds_present.is_empty()
}

pub(super) fn coreloop_value_join_gate(facts: &CanonicalLoopFacts) -> bool {
    coreloop_base_gate(facts) && facts.value_join_needed
}

pub(super) fn exit_kinds_allow_return_only(facts: &CanonicalLoopFacts) -> bool {
    facts.exit_kinds_present.is_empty()
        || (facts.exit_kinds_present.len() == 1
            && facts.exit_kinds_present.contains(&ExitKindFacts::Return))
}

pub(super) fn exit_kinds_empty(facts: &CanonicalLoopFacts) -> bool {
    facts.exit_kinds_present.is_empty()
}

pub(super) fn loop_break_value_join_gate(facts: &CanonicalLoopFacts) -> bool {
    coreloop_value_join_gate(facts) && exit_kinds_allow_return_only(facts)
}

pub(super) fn loop_true_early_exit_value_join_gate(facts: &CanonicalLoopFacts) -> bool {
    coreloop_value_join_gate(facts) && exit_kinds_allow_return_only(facts)
}

pub(super) fn if_phi_join_value_join_gate(facts: &CanonicalLoopFacts) -> bool {
    coreloop_value_join_gate(facts) && exit_kinds_allow_return_only(facts)
}
