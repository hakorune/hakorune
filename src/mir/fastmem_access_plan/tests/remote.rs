use super::support::*;
use crate::mir::fastmem_access_plan::*;
use crate::mir::function::{
    FastMemBlockNextFact, FastMemBlockNextProofKind, FastMemRemoteOwnerFact,
    FastMemRemoteOwnerProofKind,
};
use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};
use crate::mir::{BasicBlockId, ValueId};

#[test]
fn refresh_rejects_plain_store_to_atomic_remote_head() {
    let mut function = make_function(vec![memop(
        MemOpKind::FieldStore,
        None,
        vec![ValueId::new(10), ValueId::new(3)],
        Some(MemOpAccess::field("remote_head")),
    )]);

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 1);
    let plan = &function.metadata.fastmem_access_plans[0];
    assert_eq!(plan.status, FastMemAccessPlanStatus::Rejected);
    assert_eq!(
        plan.failure_reason.as_deref(),
        Some("atomic-field-plain-store:remote_head")
    );
}

#[test]
fn refresh_adds_nonlowerable_atomic_remote_head_push_plan() {
    let mut function = make_function(vec![memop(
        MemOpKind::AtomicRemoteHeadPush,
        None,
        vec![ValueId::new(10), ValueId::new(11)],
        None,
    )]);

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 1);
    let plan = &function.metadata.fastmem_access_plans[0];
    assert_eq!(plan.kind, FastMemAccessPlanKind::AtomicRemoteHeadPush);
    assert_eq!(plan.status, FastMemAccessPlanStatus::Rejected);
    assert_eq!(
        plan.failure_reason.as_deref(),
        Some("atomic-remote-head-remote-owner-proof-missing")
    );
    let FastMemAccessPlanPayload::AtomicRemoteHead(remote_head) = &plan.payload else {
        panic!("expected atomic remote-head plan");
    };
    assert_eq!(remote_head.page, ValueId::new(10));
    assert_eq!(remote_head.block, Some(ValueId::new(11)));
    assert_eq!(remote_head.result, None);
    assert_eq!(
        remote_head.remote_head.layout_id.as_deref(),
        Some("PageMetaLayoutV0")
    );
    assert_eq!(
        remote_head.remote_head.field_id.as_deref(),
        Some("remote_head")
    );
    assert_eq!(
        remote_head.remote_head.field_class.as_deref(),
        Some("atomic_remote_head")
    );
    assert_eq!(remote_head.remote_head.byte_offset, Some(32));
    assert_eq!(remote_head.remote_head.field_size, Some(8));
    assert_eq!(remote_head.remote_head.field_type.as_deref(), Some("usize"));
    assert_eq!(remote_head.remote_head.alignment, Some(8));
    assert!(remote_head.remote_owner_required);
    assert!(!remote_head.remote_owner_proof_valid);
    assert!(remote_head.block_next_required);
    assert!(!remote_head.block_next_proof_valid);
    assert_eq!(remote_head.memory_order_policy, "acq_rel");
    assert_eq!(remote_head.retry_attempt_limit, 3);
    assert!(!remote_head.lowerable);
}

#[test]
fn refresh_adds_lowerable_atomic_remote_head_drain_plan() {
    let mut function = make_function(vec![memop(
        MemOpKind::AtomicRemoteHeadDrain,
        Some(ValueId::new(12)),
        vec![ValueId::new(10)],
        None,
    )]);

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 1);
    let plan = &function.metadata.fastmem_access_plans[0];
    assert_eq!(plan.kind, FastMemAccessPlanKind::AtomicRemoteHeadDrain);
    assert_eq!(plan.status, FastMemAccessPlanStatus::Verified);
    assert_eq!(plan.failure_reason.as_deref(), None);
    let FastMemAccessPlanPayload::AtomicRemoteHead(remote_head) = &plan.payload else {
        panic!("expected atomic remote-head plan");
    };
    assert_eq!(remote_head.page, ValueId::new(10));
    assert_eq!(remote_head.block, None);
    assert_eq!(remote_head.result, Some(ValueId::new(12)));
    assert_eq!(
        remote_head.remote_head.layout_id.as_deref(),
        Some("PageMetaLayoutV0")
    );
    assert_eq!(
        remote_head.remote_head.field_id.as_deref(),
        Some("remote_head")
    );
    assert_eq!(remote_head.remote_head.byte_offset, Some(32));
    assert!(!remote_head.remote_owner_required);
    assert!(!remote_head.remote_owner_proof_valid);
    assert!(!remote_head.block_next_required);
    assert!(!remote_head.block_next_proof_valid);
    assert_eq!(remote_head.memory_order_policy, "acquire_exchange");
    assert_eq!(remote_head.retry_attempt_limit, 0);
    assert_eq!(
        remote_head.remote_head.field_class.as_deref(),
        Some("atomic_remote_head")
    );
    assert_eq!(remote_head.remote_head.field_size, Some(8));
    assert_eq!(remote_head.remote_head.field_type.as_deref(), Some("usize"));
    assert_eq!(remote_head.remote_head.alignment, Some(8));
    assert!(remote_head.lowerable);
}

#[test]
fn refresh_adds_drain_remote_list_to_local_precondition_plan() {
    let mut function = make_function(vec![
        memop(
            MemOpKind::AtomicRemoteHeadDrain,
            Some(ValueId::new(12)),
            vec![ValueId::new(10)],
            None,
        ),
        memop(
            MemOpKind::DrainRemoteListToLocal,
            None,
            vec![ValueId::new(10), ValueId::new(12)],
            None,
        ),
    ]);

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 2);
    let drain_plan = &function.metadata.fastmem_access_plans[1];
    assert_eq!(
        drain_plan.kind,
        FastMemAccessPlanKind::DrainRemoteListToLocal
    );
    assert_eq!(drain_plan.status, FastMemAccessPlanStatus::Verified);
    assert_eq!(drain_plan.failure_reason.as_deref(), None);
    let FastMemAccessPlanPayload::DrainRemoteListToLocal(drain) = &drain_plan.payload else {
        panic!("expected drain remote-list to local plan");
    };
    assert_eq!(drain.page, ValueId::new(10));
    assert_eq!(drain.token, ValueId::new(12));
    assert_eq!(drain.token_source_block, Some(BasicBlockId::new(0)));
    assert_eq!(drain.token_source_instruction_index, Some(0));
    assert!(drain.token_provenance_valid);
    assert!(drain.page_operand_valid);
    assert!(drain.head_class_resolved);
    assert_eq!(
        drain.local_list_head_class.as_deref(),
        Some("owner_local_free_or_free_head")
    );
    assert_eq!(
        drain.local_free_head.layout_id.as_deref(),
        Some("PageMetaLayoutV0")
    );
    assert_eq!(
        drain.local_free_head.field_id.as_deref(),
        Some("local_free_head")
    );
    assert_eq!(
        drain.local_free_head.field_class.as_deref(),
        Some("local_free_head")
    );
    assert_eq!(drain.local_free_head.byte_offset, Some(24));
    assert_eq!(drain.local_free_head.field_size, Some(8));
    assert_eq!(drain.local_free_head.field_type.as_deref(), Some("usize"));
    assert_eq!(drain.local_free_head.alignment, Some(8));
    assert_eq!(
        drain.block_next.layout_id.as_deref(),
        Some("FreeBlockNodeLayoutV0")
    );
    assert_eq!(drain.block_next.field_id.as_deref(), Some("next"));
    assert_eq!(
        drain.block_next.field_class.as_deref(),
        Some("local_free_block_next")
    );
    assert_eq!(drain.block_next.byte_offset, Some(0));
    assert_eq!(drain.block_next.field_size, Some(8));
    assert_eq!(drain.block_next.field_type.as_deref(), Some("usize"));
    assert_eq!(drain.block_next.alignment, Some(8));
    assert!(drain.block_next_access_resolved);
    assert_eq!(
        drain.publication_order,
        "verifier_owned_acquire_then_owner_local"
    );
    assert!(drain.lowerable);
}

#[test]
fn refresh_observes_atomic_remote_head_block_next_proof_but_keeps_lowering_closed() {
    let mut function = make_function(vec![memop(
        MemOpKind::AtomicRemoteHeadPush,
        None,
        vec![ValueId::new(10), ValueId::new(11)],
        None,
    )]);
    function
        .metadata
        .fastmem_block_next_facts
        .push(FastMemBlockNextFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            block_value: ValueId::new(11),
            next_field_id: "next".to_string(),
            proof_kind: FastMemBlockNextProofKind::SourceAssumeRemoteFreeBlockNext,
            writable: true,
            provenance_valid: true,
        });

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 1);
    let plan = &function.metadata.fastmem_access_plans[0];
    assert_eq!(plan.kind, FastMemAccessPlanKind::AtomicRemoteHeadPush);
    assert_eq!(plan.status, FastMemAccessPlanStatus::Rejected);
    let FastMemAccessPlanPayload::AtomicRemoteHead(remote_head) = &plan.payload else {
        panic!("expected atomic remote-head plan");
    };
    assert!(remote_head.block_next_proof_valid);
    assert_eq!(
        remote_head.block_next.layout_id.as_deref(),
        Some("FreeBlockNodeLayoutV0")
    );
    assert_eq!(remote_head.block_next.field_id.as_deref(), Some("next"));
    assert_eq!(
        remote_head.block_next.field_class.as_deref(),
        Some("local_free_block_next")
    );
    assert_eq!(remote_head.block_next.byte_offset, Some(0));
    assert_eq!(remote_head.block_next.field_size, Some(8));
    assert_eq!(remote_head.block_next.field_type.as_deref(), Some("usize"));
    assert_eq!(remote_head.block_next.alignment, Some(8));
    assert!(remote_head.remote_owner_required);
    assert!(!remote_head.remote_owner_proof_valid);
    assert!(!remote_head.lowerable);
}

#[test]
fn refresh_observes_atomic_remote_head_proofs_and_verifies_cas_lowering_plan() {
    let mut function = make_function(vec![memop(
        MemOpKind::AtomicRemoteHeadPush,
        None,
        vec![ValueId::new(10), ValueId::new(11)],
        None,
    )]);
    function
        .metadata
        .fastmem_remote_owner_facts
        .push(FastMemRemoteOwnerFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            page_value: ValueId::new(10),
            proof_kind: FastMemRemoteOwnerProofKind::SourceAssumeRemoteOwner,
            same_owner_rejected: true,
        });
    function
        .metadata
        .fastmem_block_next_facts
        .push(FastMemBlockNextFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            block_value: ValueId::new(11),
            next_field_id: "next".to_string(),
            proof_kind: FastMemBlockNextProofKind::SourceAssumeRemoteFreeBlockNext,
            writable: true,
            provenance_valid: true,
        });

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 1);
    let plan = &function.metadata.fastmem_access_plans[0];
    assert_eq!(plan.kind, FastMemAccessPlanKind::AtomicRemoteHeadPush);
    assert_eq!(plan.status, FastMemAccessPlanStatus::Verified);
    assert_eq!(plan.failure_reason, None);
    let FastMemAccessPlanPayload::AtomicRemoteHead(remote_head) = &plan.payload else {
        panic!("expected atomic remote-head plan");
    };
    assert!(remote_head.remote_owner_required);
    assert!(remote_head.remote_owner_proof_valid);
    assert!(remote_head.block_next_required);
    assert!(remote_head.block_next_proof_valid);
    assert_eq!(remote_head.memory_order_policy, "acq_rel");
    assert_eq!(remote_head.retry_attempt_limit, 3);
    assert!(remote_head.lowerable);
}
