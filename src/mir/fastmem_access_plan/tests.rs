use super::*;
mod support;
mod table;

use crate::mir::function::{
    FastMemBlockNextFact, FastMemBlockNextProofKind, FastMemFreeHeadNonEmptyFact,
    FastMemFreeHeadNonEmptyProofKind, FastMemLocalFreeNonEmptyFact,
    FastMemLocalFreeNonEmptyProofKind, FastMemRemoteOwnerFact, FastMemRemoteOwnerProofKind,
    FastMemSameOwnerFact, FastMemSameOwnerProofKind,
};
use support::*;

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
        remote_head.remote_head_layout_id.as_deref(),
        Some("PageMetaLayoutV0")
    );
    assert_eq!(
        remote_head.remote_head_field_id.as_deref(),
        Some("remote_head")
    );
    assert_eq!(
        remote_head.remote_head_field_class.as_deref(),
        Some("atomic_remote_head")
    );
    assert_eq!(remote_head.remote_head_byte_offset, Some(32));
    assert_eq!(remote_head.remote_head_field_size, Some(8));
    assert_eq!(remote_head.remote_head_field_type.as_deref(), Some("usize"));
    assert_eq!(remote_head.remote_head_alignment, Some(8));
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
        remote_head.remote_head_layout_id.as_deref(),
        Some("PageMetaLayoutV0")
    );
    assert_eq!(
        remote_head.remote_head_field_id.as_deref(),
        Some("remote_head")
    );
    assert_eq!(remote_head.remote_head_byte_offset, Some(32));
    assert!(!remote_head.remote_owner_required);
    assert!(!remote_head.remote_owner_proof_valid);
    assert!(!remote_head.block_next_required);
    assert!(!remote_head.block_next_proof_valid);
    assert_eq!(remote_head.memory_order_policy, "acquire_exchange");
    assert_eq!(remote_head.retry_attempt_limit, 0);
    assert_eq!(
        remote_head.remote_head_field_class.as_deref(),
        Some("atomic_remote_head")
    );
    assert_eq!(remote_head.remote_head_field_size, Some(8));
    assert_eq!(remote_head.remote_head_field_type.as_deref(), Some("usize"));
    assert_eq!(remote_head.remote_head_alignment, Some(8));
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
        drain.local_free_head_layout_id.as_deref(),
        Some("PageMetaLayoutV0")
    );
    assert_eq!(
        drain.local_free_head_field_id.as_deref(),
        Some("local_free_head")
    );
    assert_eq!(
        drain.local_free_head_field_class.as_deref(),
        Some("local_free_head")
    );
    assert_eq!(drain.local_free_head_byte_offset, Some(24));
    assert_eq!(drain.local_free_head_field_size, Some(8));
    assert_eq!(drain.local_free_head_field_type.as_deref(), Some("usize"));
    assert_eq!(drain.local_free_head_alignment, Some(8));
    assert_eq!(
        drain.block_next_layout_id.as_deref(),
        Some("FreeBlockNodeLayoutV0")
    );
    assert_eq!(drain.block_next_field_id.as_deref(), Some("next"));
    assert_eq!(
        drain.block_next_field_class.as_deref(),
        Some("local_free_block_next")
    );
    assert_eq!(drain.block_next_byte_offset, Some(0));
    assert_eq!(drain.block_next_field_size, Some(8));
    assert_eq!(drain.block_next_field_type.as_deref(), Some("usize"));
    assert_eq!(drain.block_next_alignment, Some(8));
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
        remote_head.block_next_layout_id.as_deref(),
        Some("FreeBlockNodeLayoutV0")
    );
    assert_eq!(remote_head.block_next_field_id.as_deref(), Some("next"));
    assert_eq!(
        remote_head.block_next_field_class.as_deref(),
        Some("local_free_block_next")
    );
    assert_eq!(remote_head.block_next_byte_offset, Some(0));
    assert_eq!(remote_head.block_next_field_size, Some(8));
    assert_eq!(remote_head.block_next_field_type.as_deref(), Some("usize"));
    assert_eq!(remote_head.block_next_alignment, Some(8));
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

#[test]
fn refresh_adds_nonlowerable_local_free_list_plans() {
    let mut function = make_function(vec![
        memop(
            MemOpKind::LocalFreePush,
            None,
            vec![ValueId::new(10), ValueId::new(11)],
            None,
        ),
        memop(
            MemOpKind::LocalFreePop,
            Some(ValueId::new(12)),
            vec![ValueId::new(10)],
            None,
        ),
    ]);

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 2);
    for plan in &function.metadata.fastmem_access_plans {
        assert_eq!(plan.status, FastMemAccessPlanStatus::Rejected);
        assert_eq!(
            plan.failure_reason.as_deref(),
            Some("local-free-same-owner-proof-missing")
        );
        let FastMemAccessPlanPayload::LocalFree(local_free) = &plan.payload else {
            panic!("expected local free-list plan");
        };
        assert_eq!(
            local_free.local_free_head_field_id.as_deref(),
            Some("local_free_head")
        );
        assert_eq!(
            local_free.local_free_head_field_class.as_deref(),
            Some("local_free_head")
        );
        assert_eq!(local_free.local_free_head_byte_offset, Some(24));
        assert_eq!(local_free.local_free_head_field_size, Some(8));
        assert_eq!(
            local_free.local_free_head_field_type.as_deref(),
            Some("usize")
        );
        assert_eq!(local_free.local_free_head_alignment, Some(8));
        assert!(!local_free.same_owner_proof_valid);
        assert!(!local_free.block_next_proof_valid);
        assert!(!local_free.non_empty_proof_valid);
        assert!(!local_free.remote_owner_rejected);
        assert!(!local_free.lowerable);
    }
}

#[test]
fn refresh_verifies_local_free_push_when_precondition_facts_exist() {
    let mut function = make_function(vec![
        memop(
            MemOpKind::LocalFreePush,
            None,
            vec![ValueId::new(10), ValueId::new(11)],
            None,
        ),
        memop(
            MemOpKind::LocalFreePop,
            Some(ValueId::new(12)),
            vec![ValueId::new(10)],
            None,
        ),
    ]);
    function
        .metadata
        .fastmem_same_owner_facts
        .push(FastMemSameOwnerFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            page_value: ValueId::new(10),
            proof_value: ValueId::new(20),
            proof_kind: FastMemSameOwnerProofKind::SourceAssumeOwnerEq,
            remote_owner_rejected: true,
        });
    function
        .metadata
        .fastmem_block_next_facts
        .push(FastMemBlockNextFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            block_value: ValueId::new(11),
            next_field_id: "next".to_string(),
            proof_kind: FastMemBlockNextProofKind::SourceAssumeLocalFreeBlockNext,
            writable: true,
            provenance_valid: true,
        });

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 2);
    let push_plan = &function.metadata.fastmem_access_plans[0];
    assert_eq!(push_plan.kind, FastMemAccessPlanKind::LocalFreePush);
    assert_eq!(push_plan.status, FastMemAccessPlanStatus::Verified);
    assert_eq!(push_plan.failure_reason, None);
    let FastMemAccessPlanPayload::LocalFree(push) = &push_plan.payload else {
        panic!("expected local free-list push plan");
    };
    assert!(push.same_owner_proof_valid);
    assert!(push.block_next_proof_valid);
    assert!(push.remote_owner_rejected);
    assert!(push.lowerable);
    assert_eq!(
        push.local_free_head_layout_id.as_deref(),
        Some("PageMetaLayoutV0")
    );
    assert_eq!(
        push.local_free_head_field_id.as_deref(),
        Some("local_free_head")
    );
    assert_eq!(push.local_free_head_byte_offset, Some(24));
    assert_eq!(push.local_free_head_field_size, Some(8));
    assert_eq!(push.local_free_head_field_type.as_deref(), Some("usize"));
    assert_eq!(push.local_free_head_alignment, Some(8));
    assert!(!push.non_empty_proof_valid);
    assert_eq!(
        push.block_next_layout_id.as_deref(),
        Some("FreeBlockNodeLayoutV0")
    );
    assert_eq!(push.block_next_field_id.as_deref(), Some("next"));
    assert_eq!(
        push.block_next_field_class.as_deref(),
        Some("local_free_block_next")
    );
    assert_eq!(push.block_next_byte_offset, Some(0));
    assert_eq!(push.block_next_field_size, Some(8));
    assert_eq!(push.block_next_field_type.as_deref(), Some("usize"));
    assert_eq!(push.block_next_alignment, Some(8));

    let pop_plan = &function.metadata.fastmem_access_plans[1];
    assert_eq!(pop_plan.kind, FastMemAccessPlanKind::LocalFreePop);
    assert_eq!(pop_plan.status, FastMemAccessPlanStatus::Rejected);
    assert_eq!(
        pop_plan.failure_reason.as_deref(),
        Some("local-free-non-empty-proof-missing")
    );
    let FastMemAccessPlanPayload::LocalFree(pop) = &pop_plan.payload else {
        panic!("expected local free-list pop plan");
    };
    assert!(pop.same_owner_proof_valid);
    assert!(!pop.block_next_proof_valid);
    assert!(!pop.non_empty_proof_valid);
    assert!(pop.remote_owner_rejected);
    assert!(!pop.lowerable);
}

#[test]
fn refresh_verifies_local_free_pop_preconditions_without_lowering() {
    let mut function = make_function(vec![memop(
        MemOpKind::LocalFreePop,
        Some(ValueId::new(12)),
        vec![ValueId::new(10)],
        None,
    )]);
    function
        .metadata
        .fastmem_same_owner_facts
        .push(FastMemSameOwnerFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            page_value: ValueId::new(10),
            proof_value: ValueId::new(20),
            proof_kind: FastMemSameOwnerProofKind::SourceAssumeOwnerEq,
            remote_owner_rejected: true,
        });
    function
        .metadata
        .fastmem_local_free_non_empty_facts
        .push(FastMemLocalFreeNonEmptyFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            page_value: ValueId::new(10),
            proof_kind: FastMemLocalFreeNonEmptyProofKind::SourceAssumeLocalFreeNonEmpty,
            non_empty: true,
        });

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 1);
    let pop_plan = &function.metadata.fastmem_access_plans[0];
    assert_eq!(pop_plan.kind, FastMemAccessPlanKind::LocalFreePop);
    assert_eq!(pop_plan.status, FastMemAccessPlanStatus::Verified);
    assert_eq!(pop_plan.failure_reason, None);
    let FastMemAccessPlanPayload::LocalFree(pop) = &pop_plan.payload else {
        panic!("expected local free-list pop plan");
    };
    assert!(pop.same_owner_proof_valid);
    assert!(pop.non_empty_proof_valid);
    assert!(!pop.block_next_proof_valid);
    assert!(pop.remote_owner_rejected);
    assert!(pop.lowerable);
    assert_eq!(
        pop.local_free_head_layout_id.as_deref(),
        Some("PageMetaLayoutV0")
    );
    assert_eq!(
        pop.local_free_head_field_id.as_deref(),
        Some("local_free_head")
    );
    assert_eq!(pop.local_free_head_byte_offset, Some(24));
    assert_eq!(pop.local_free_head_field_size, Some(8));
    assert_eq!(pop.local_free_head_field_type.as_deref(), Some("usize"));
    assert_eq!(pop.local_free_head_alignment, Some(8));
    assert_eq!(
        pop.block_next_layout_id.as_deref(),
        Some("FreeBlockNodeLayoutV0")
    );
    assert_eq!(pop.block_next_field_id.as_deref(), Some("next"));
    assert_eq!(
        pop.block_next_field_class.as_deref(),
        Some("local_free_block_next")
    );
    assert_eq!(pop.block_next_byte_offset, Some(0));
    assert_eq!(pop.block_next_field_size, Some(8));
    assert_eq!(pop.block_next_field_type.as_deref(), Some("usize"));
    assert_eq!(pop.block_next_alignment, Some(8));
}

#[test]
fn refresh_verifies_free_head_pop_preconditions_without_lowering() {
    let mut function = make_function(vec![memop(
        MemOpKind::FreeHeadPop,
        Some(ValueId::new(12)),
        vec![ValueId::new(10)],
        None,
    )]);
    function
        .metadata
        .fastmem_same_owner_facts
        .push(FastMemSameOwnerFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            page_value: ValueId::new(10),
            proof_value: ValueId::new(20),
            proof_kind: FastMemSameOwnerProofKind::SourceAssumeOwnerEq,
            remote_owner_rejected: true,
        });
    function
        .metadata
        .fastmem_free_head_non_empty_facts
        .push(FastMemFreeHeadNonEmptyFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            page_value: ValueId::new(10),
            proof_kind: FastMemFreeHeadNonEmptyProofKind::SourceAssumeFreeHeadNonEmpty,
            non_empty: true,
        });

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 1);
    let pop_plan = &function.metadata.fastmem_access_plans[0];
    assert_eq!(pop_plan.kind, FastMemAccessPlanKind::FreeHeadPop);
    assert_eq!(pop_plan.status, FastMemAccessPlanStatus::Verified);
    assert_eq!(pop_plan.failure_reason, None);
    let FastMemAccessPlanPayload::FreeHead(pop) = &pop_plan.payload else {
        panic!("expected free-head pop plan");
    };
    assert!(pop.same_owner_proof_valid);
    assert!(pop.non_empty_proof_valid);
    assert!(pop.remote_owner_rejected);
    assert!(pop.lowerable);
    assert_eq!(pop.free_head_layout_id.as_deref(), Some("PageMetaLayoutV0"));
    assert_eq!(pop.free_head_field_id.as_deref(), Some("free_head"));
    assert_eq!(pop.free_head_field_class.as_deref(), Some("plain_pointer"));
    assert_eq!(pop.free_head_byte_offset, Some(16));
    assert_eq!(pop.free_head_field_size, Some(8));
    assert_eq!(pop.free_head_field_type.as_deref(), Some("usize"));
    assert_eq!(pop.free_head_alignment, Some(8));
    assert_eq!(
        pop.block_next_layout_id.as_deref(),
        Some("FreeBlockNodeLayoutV0")
    );
    assert_eq!(pop.block_next_field_id.as_deref(), Some("next"));
    assert_eq!(
        pop.block_next_field_class.as_deref(),
        Some("local_free_block_next")
    );
    assert_eq!(pop.block_next_byte_offset, Some(0));
    assert_eq!(pop.block_next_field_size, Some(8));
    assert_eq!(pop.block_next_field_type.as_deref(), Some("usize"));
    assert_eq!(pop.block_next_alignment, Some(8));
}

#[test]
fn refresh_verifies_free_head_push_preconditions_without_lowering() {
    let mut function = make_function(vec![memop(
        MemOpKind::FreeHeadPush,
        None,
        vec![ValueId::new(10), ValueId::new(11)],
        None,
    )]);
    function
        .metadata
        .fastmem_same_owner_facts
        .push(FastMemSameOwnerFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            page_value: ValueId::new(10),
            proof_value: ValueId::new(20),
            proof_kind: FastMemSameOwnerProofKind::SourceAssumeOwnerEq,
            remote_owner_rejected: true,
        });
    function
        .metadata
        .fastmem_block_next_facts
        .push(FastMemBlockNextFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            block_value: ValueId::new(11),
            next_field_id: "next".to_string(),
            proof_kind: FastMemBlockNextProofKind::SourceAssumeFreeHeadBlockNext,
            writable: true,
            provenance_valid: true,
        });

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 1);
    let push_plan = &function.metadata.fastmem_access_plans[0];
    assert_eq!(push_plan.kind, FastMemAccessPlanKind::FreeHeadPush);
    assert_eq!(push_plan.status, FastMemAccessPlanStatus::Verified);
    assert_eq!(push_plan.failure_reason, None);
    let FastMemAccessPlanPayload::FreeHead(push) = &push_plan.payload else {
        panic!("expected free-head push plan");
    };
    assert_eq!(push.block, Some(ValueId::new(11)));
    assert_eq!(push.result, None);
    assert!(push.same_owner_proof_valid);
    assert!(push.block_next_proof_valid);
    assert!(!push.non_empty_proof_valid);
    assert!(push.remote_owner_rejected);
    assert!(push.lowerable);
    assert_eq!(
        push.free_head_layout_id.as_deref(),
        Some("PageMetaLayoutV0")
    );
    assert_eq!(push.free_head_field_id.as_deref(), Some("free_head"));
    assert_eq!(push.free_head_field_class.as_deref(), Some("plain_pointer"));
    assert_eq!(push.free_head_byte_offset, Some(16));
    assert_eq!(push.free_head_field_size, Some(8));
    assert_eq!(push.free_head_field_type.as_deref(), Some("usize"));
    assert_eq!(push.free_head_alignment, Some(8));
    assert_eq!(
        push.block_next_layout_id.as_deref(),
        Some("FreeBlockNodeLayoutV0")
    );
    assert_eq!(push.block_next_field_id.as_deref(), Some("next"));
    assert_eq!(
        push.block_next_field_class.as_deref(),
        Some("local_free_block_next")
    );
    assert_eq!(push.block_next_byte_offset, Some(0));
    assert_eq!(push.block_next_field_size, Some(8));
    assert_eq!(push.block_next_field_type.as_deref(), Some("usize"));
    assert_eq!(push.block_next_alignment, Some(8));
}

#[test]
fn refresh_derives_free_head_non_empty_after_verified_push_for_later_pop() {
    let mut function = make_function(vec![
        memop(
            MemOpKind::FreeHeadPush,
            None,
            vec![ValueId::new(10), ValueId::new(11)],
            None,
        ),
        memop(
            MemOpKind::FreeHeadPop,
            Some(ValueId::new(12)),
            vec![ValueId::new(10)],
            None,
        ),
    ]);
    function
        .metadata
        .fastmem_same_owner_facts
        .push(FastMemSameOwnerFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            page_value: ValueId::new(10),
            proof_value: ValueId::new(20),
            proof_kind: FastMemSameOwnerProofKind::SourceAssumeOwnerEq,
            remote_owner_rejected: true,
        });
    function
        .metadata
        .fastmem_block_next_facts
        .push(FastMemBlockNextFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            block_value: ValueId::new(11),
            next_field_id: "next".to_string(),
            proof_kind: FastMemBlockNextProofKind::SourceAssumeFreeHeadBlockNext,
            writable: true,
            provenance_valid: true,
        });

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 2);
    assert_eq!(function.metadata.fastmem_free_head_non_empty_facts.len(), 1);
    assert_eq!(
        function.metadata.fastmem_free_head_non_empty_facts[0].proof_kind,
        FastMemFreeHeadNonEmptyProofKind::DerivedFromFreeHeadPush
    );
    assert_eq!(
        function.metadata.fastmem_free_head_non_empty_facts[0].page_value,
        ValueId::new(10)
    );

    let push_plan = &function.metadata.fastmem_access_plans[0];
    let pop_plan = &function.metadata.fastmem_access_plans[1];
    assert_eq!(push_plan.kind, FastMemAccessPlanKind::FreeHeadPush);
    assert_eq!(push_plan.status, FastMemAccessPlanStatus::Verified);
    assert_eq!(pop_plan.kind, FastMemAccessPlanKind::FreeHeadPop);
    assert_eq!(pop_plan.status, FastMemAccessPlanStatus::Verified);
    let FastMemAccessPlanPayload::FreeHead(pop) = &pop_plan.payload else {
        panic!("expected free-head pop plan");
    };
    assert!(pop.non_empty_proof_valid);
    assert!(pop.lowerable);

    refresh_function_fastmem_access_plans(&mut function);
    assert_eq!(function.metadata.fastmem_free_head_non_empty_facts.len(), 1);
}

#[test]
fn refresh_does_not_derive_free_head_non_empty_before_push() {
    let mut function = make_function(vec![
        memop(
            MemOpKind::FreeHeadPop,
            Some(ValueId::new(12)),
            vec![ValueId::new(10)],
            None,
        ),
        memop(
            MemOpKind::FreeHeadPush,
            None,
            vec![ValueId::new(10), ValueId::new(11)],
            None,
        ),
    ]);
    function
        .metadata
        .fastmem_same_owner_facts
        .push(FastMemSameOwnerFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            page_value: ValueId::new(10),
            proof_value: ValueId::new(20),
            proof_kind: FastMemSameOwnerProofKind::SourceAssumeOwnerEq,
            remote_owner_rejected: true,
        });
    function
        .metadata
        .fastmem_block_next_facts
        .push(FastMemBlockNextFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            block_value: ValueId::new(11),
            next_field_id: "next".to_string(),
            proof_kind: FastMemBlockNextProofKind::SourceAssumeFreeHeadBlockNext,
            writable: true,
            provenance_valid: true,
        });

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 2);
    let pop_plan = &function.metadata.fastmem_access_plans[0];
    let push_plan = &function.metadata.fastmem_access_plans[1];
    assert_eq!(pop_plan.kind, FastMemAccessPlanKind::FreeHeadPop);
    assert_eq!(pop_plan.status, FastMemAccessPlanStatus::Rejected);
    assert_eq!(
        pop_plan.failure_reason.as_deref(),
        Some("free-head-non-empty-proof-missing")
    );
    assert_eq!(push_plan.kind, FastMemAccessPlanKind::FreeHeadPush);
    assert_eq!(push_plan.status, FastMemAccessPlanStatus::Verified);
    assert_eq!(function.metadata.fastmem_free_head_non_empty_facts.len(), 1);
}

#[test]
fn refresh_ignores_layout_table_memops_without_symbolic_ids() {
    let mut function = make_function(vec![memop(
        MemOpKind::FieldLoad,
        Some(ValueId::new(1)),
        vec![ValueId::new(0)],
        None,
    )]);

    refresh_function_fastmem_access_plans(&mut function);

    assert!(function.metadata.fastmem_access_plans.is_empty());
}
