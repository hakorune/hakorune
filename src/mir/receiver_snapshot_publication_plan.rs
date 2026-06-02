/*!
 * Metadata-only narrow mixed-base publication recipes.
 *
 * This plan consumes `EffectSummary` rows. v0 accepts scalar snapshot
 * publication from one foreign base into receiver fields. Foreign handle
 * publication is accepted only as metadata when the current user-box field
 * store model requires no runtime barrier and the helper has no hidden effects.
 */

use crate::mir::{effect_summary::EffectSummary, MirFunction, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiverSnapshotPublicationPlan {
    pub method: String,
    pub receiver_value: Option<ValueId>,
    pub foreign_base_count: usize,
    pub receiver_reads: usize,
    pub receiver_writes: usize,
    pub foreign_reads: usize,
    pub handle_publications: usize,
    pub publication_kind: &'static str,
    pub barrier_policy: &'static str,
    pub handle_publication_proof_kind: Option<&'static str>,
    pub lifetime_policy: &'static str,
    pub lowering_consumer_enabled: bool,
    pub summary: &'static str,
    pub failure_reason: Option<&'static str>,
}

pub fn refresh_function_receiver_snapshot_publication_plans(function: &mut MirFunction) {
    function
        .metadata
        .receiver_snapshot_publication_plans
        .clear();

    for summary in &function.metadata.effect_summaries {
        let Some(plan) = plan_from_effect_summary(summary) else {
            continue;
        };
        function
            .metadata
            .receiver_snapshot_publication_plans
            .push(plan);
    }
}

fn plan_from_effect_summary(summary: &EffectSummary) -> Option<ReceiverSnapshotPublicationPlan> {
    match summary.candidate_kind {
        "mixed_base_scalar_snapshot_candidate" | "mixed_base_publication_candidate" => {}
        _ => return None,
    }

    let (
        publication_kind,
        barrier_policy,
        handle_publication_proof_kind,
        lifetime_policy,
        status,
        failure_reason,
    ) = if summary.summary != "ok" {
        (
            "unsupported_effect_shape",
            "unresolved",
            None,
            "unresolved",
            "rejected",
            summary.failure_reason.or(Some("effect_summary_rejected")),
        )
    } else if summary.handle_publications == 0 {
        ("scalar_snapshot", "none", None, "scalar_value", "ok", None)
    } else {
        (
            "foreign_handle_publication",
            "none",
            Some("single_foreign_base_no_hidden_effects"),
            "caller_visible_handle",
            "ok",
            None,
        )
    };

    Some(ReceiverSnapshotPublicationPlan {
        method: summary.method.clone(),
        receiver_value: summary.receiver_value,
        foreign_base_count: summary.foreign_base_count,
        receiver_reads: summary.receiver_reads,
        receiver_writes: summary.receiver_writes,
        foreign_reads: summary.foreign_reads,
        handle_publications: summary.handle_publications,
        publication_kind,
        barrier_policy,
        handle_publication_proof_kind,
        lifetime_policy,
        lowering_consumer_enabled: false,
        summary: status,
        failure_reason,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        effect_summary::EffectSummary, BasicBlockId, EffectMask, FunctionSignature, MirFunction,
        MirType,
    };

    fn function_with_summary(summary: EffectSummary) -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: summary.method.clone(),
                params: vec![
                    MirType::Box("ProofQueue".to_string()),
                    MirType::Box("ProofPage".to_string()),
                ],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.metadata.effect_summaries.push(summary);
        function
    }

    fn base_summary(candidate_kind: &'static str, handle_publications: usize) -> EffectSummary {
        EffectSummary {
            method: "ProofQueue.recordSelectedScalar/3".to_string(),
            receiver_value: Some(ValueId::new(0)),
            receiver_reads: 1,
            receiver_writes: 4,
            foreign_reads: 1,
            foreign_writes: 0,
            handle_publications,
            nested_call_count: 0,
            allocation_count: 0,
            safepoint_count: 0,
            branch_count: 0,
            loop_like_count: 0,
            foreign_base_count: 1,
            candidate_kind,
            summary: "ok",
            failure_reason: None,
        }
    }

    #[test]
    fn accepts_scalar_snapshot_metadata_only() {
        let mut function =
            function_with_summary(base_summary("mixed_base_scalar_snapshot_candidate", 0));

        refresh_function_receiver_snapshot_publication_plans(&mut function);

        let plan = &function.metadata.receiver_snapshot_publication_plans[0];
        assert_eq!(plan.publication_kind, "scalar_snapshot");
        assert_eq!(plan.barrier_policy, "none");
        assert_eq!(plan.lowering_consumer_enabled, false);
        assert_eq!(plan.summary, "ok");
        assert_eq!(plan.failure_reason, None);
    }

    #[test]
    fn accepts_foreign_handle_publication_as_metadata_when_barrier_is_proven_none() {
        let mut function =
            function_with_summary(base_summary("mixed_base_publication_candidate", 1));

        refresh_function_receiver_snapshot_publication_plans(&mut function);

        let plan = &function.metadata.receiver_snapshot_publication_plans[0];
        assert_eq!(plan.publication_kind, "foreign_handle_publication");
        assert_eq!(plan.barrier_policy, "none");
        assert_eq!(
            plan.handle_publication_proof_kind,
            Some("single_foreign_base_no_hidden_effects")
        );
        assert_eq!(plan.lifetime_policy, "caller_visible_handle");
        assert_eq!(plan.summary, "ok");
        assert_eq!(plan.failure_reason, None);
    }
}
