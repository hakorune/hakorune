use super::support::*;
use crate::mir::fastmem_access_plan::*;
use crate::mir::instruction::{MemOpAccess, MemOpKind};
use crate::mir::{MirInstruction, ValueId};

#[test]
fn refresh_verifies_page_meta_field_sites_and_rejects_unbounded_table() {
    let mut function = make_function(vec![
        memop(
            MemOpKind::TableIndex,
            Some(ValueId::new(10)),
            vec![ValueId::new(1), ValueId::new(2)],
            Some(MemOpAccess::table("page_table")),
        ),
        memop(
            MemOpKind::FieldLoad,
            Some(ValueId::new(11)),
            vec![ValueId::new(10)],
            Some(MemOpAccess::field("owner_id")),
        ),
        memop(
            MemOpKind::FieldStore,
            None,
            vec![ValueId::new(10), ValueId::new(3)],
            Some(MemOpAccess::field("local_free_head")),
        ),
    ]);

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 3);
    assert_eq!(
        function.metadata.fastmem_access_plans[0].status,
        FastMemAccessPlanStatus::Rejected
    );
    assert_eq!(
        function.metadata.fastmem_access_plans[0]
            .failure_reason
            .as_deref(),
        Some("table-length-unresolved")
    );
    assert_eq!(
        function.metadata.fastmem_access_plans[1].status,
        FastMemAccessPlanStatus::Verified
    );
    assert_eq!(
        function.metadata.fastmem_access_plans[2].status,
        FastMemAccessPlanStatus::Verified
    );
    let FastMemAccessPlanPayload::Field(field) = &function.metadata.fastmem_access_plans[1].payload
    else {
        panic!("expected owner field plan");
    };
    assert_eq!(field.layout_id.as_deref(), Some("PageMetaLayoutV0"));
    assert_eq!(field.field_id, "owner_worker_id");
    assert_eq!(field.byte_offset, Some(0));
    assert_eq!(field.field_class.as_deref(), Some("plain_scalar"));
    let FastMemAccessPlanPayload::Table(table) = &function.metadata.fastmem_access_plans[0].payload
    else {
        panic!("expected table plan");
    };
    assert_eq!(table.element_layout_id.as_deref(), Some("PageMetaLayoutV0"));
    assert_eq!(table.element_repr.as_deref(), Some("pointer_to_element"));
    assert!(!table.proof.is_lowerable());
    assert!(!table.proof.table_length_resolved);
    assert!(!table.proof.bounds_proof_valid);
    assert!(table.proof.stride_resolved);
    assert!(table.proof.field_offset_resolved);
    assert!(!table.proof.overflow_proof_valid);
    assert!(table.proof.alignment_valid);
    assert!(table.proof.element_layout_verified);
    assert_eq!(
        table.proof.failure_reason.as_deref(),
        Some("table-length-unresolved")
    );
    assert_eq!(function.metadata.fastmem_table_field_access_links.len(), 2);
    let owner_link = &function.metadata.fastmem_table_field_access_links[0];
    assert_eq!(owner_link.table_instruction_index, 0);
    assert_eq!(owner_link.field_instruction_index, 1);
    assert_eq!(owner_link.table_result, ValueId::new(10));
    assert_eq!(owner_link.field_base, ValueId::new(10));
    assert_eq!(owner_link.field_id, "owner_worker_id");
    assert_eq!(owner_link.byte_offset, 0);
    assert_eq!(owner_link.field_size, 8);
    assert_eq!(owner_link.proof, "table_field_link:0:1");
}

#[test]
fn refresh_links_table_index_to_field_sites_through_copy_aliases() {
    let mut function = make_function(vec![
        memop(
            MemOpKind::TableIndex,
            Some(ValueId::new(10)),
            vec![ValueId::new(1), ValueId::new(2)],
            Some(MemOpAccess::table("page_table")),
        ),
        MirInstruction::Copy {
            dst: ValueId::new(11),
            src: ValueId::new(10),
        },
        MirInstruction::Copy {
            dst: ValueId::new(12),
            src: ValueId::new(11),
        },
        memop(
            MemOpKind::FieldLoad,
            Some(ValueId::new(13)),
            vec![ValueId::new(12)],
            Some(MemOpAccess::field("block_size")),
        ),
    ]);

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 2);
    assert_eq!(function.metadata.fastmem_table_field_access_links.len(), 1);
    let link = &function.metadata.fastmem_table_field_access_links[0];
    assert_eq!(link.table_result, ValueId::new(10));
    assert_eq!(link.field_base, ValueId::new(12));
    assert_eq!(link.field_id, "block_size");
}

#[test]
fn refresh_consumes_explicit_table_length_fact_without_making_table_lowerable() {
    let mut function = make_function(vec![memop(
        MemOpKind::TableIndex,
        Some(ValueId::new(10)),
        vec![ValueId::new(1), ValueId::new(2)],
        Some(MemOpAccess::table("page_table")),
    )]);
    function
        .metadata
        .fastmem_table_length_facts
        .push(table_length_fact());

    refresh_function_fastmem_access_plans(&mut function);

    assert_eq!(function.metadata.fastmem_access_plans.len(), 1);
    let FastMemAccessPlanPayload::Table(table) = &function.metadata.fastmem_access_plans[0].payload
    else {
        panic!("expected table plan");
    };
    assert_eq!(table.length, Some(64));
    assert!(table.proof.table_length_resolved);
    assert_eq!(
        table.proof.table_length_policy.as_deref(),
        Some("explicit_const_len")
    );
    assert!(!table.proof.bounds_proof_valid);
    assert!(!table.proof.overflow_proof_valid);
    assert!(!table.proof.is_lowerable());
    assert_eq!(
        function.metadata.fastmem_access_plans[0]
            .failure_reason
            .as_deref(),
        Some("verified-table-access-proof-incomplete")
    );
}

#[test]
fn refresh_consumes_range_index_fact_as_bounds_proof_after_length_fact() {
    let mut function = make_function(vec![memop(
        MemOpKind::TableIndex,
        Some(ValueId::new(10)),
        vec![ValueId::new(1), ValueId::new(2)],
        Some(MemOpAccess::table("page_table")),
    )]);
    function
        .metadata
        .fastmem_table_length_facts
        .push(table_length_fact());
    function
        .metadata
        .range_index_facts
        .push(range_index_fact(7, ValueId::new(2)));

    refresh_function_fastmem_access_plans(&mut function);

    let FastMemAccessPlanPayload::Table(table) = &function.metadata.fastmem_access_plans[0].payload
    else {
        panic!("expected table plan");
    };
    assert!(table.proof.table_length_resolved);
    assert!(table.proof.bounds_proof_valid);
    assert_eq!(table.proof.bounds_proof.as_deref(), Some("range_fact:7"));
    assert!(!table.proof.overflow_proof_valid);
    assert!(!table.proof.is_lowerable());
    assert_eq!(
        function.metadata.fastmem_access_plans[0]
            .failure_reason
            .as_deref(),
        Some("verified-table-access-proof-incomplete")
    );
}

#[test]
fn refresh_sets_overflow_proof_from_length_bounds_and_table_field_link() {
    let mut function = make_function(vec![
        memop(
            MemOpKind::TableIndex,
            Some(ValueId::new(10)),
            vec![ValueId::new(1), ValueId::new(2)],
            Some(MemOpAccess::table("page_table")),
        ),
        memop(
            MemOpKind::FieldLoad,
            Some(ValueId::new(11)),
            vec![ValueId::new(10)],
            Some(MemOpAccess::field("capacity")),
        ),
    ]);
    function
        .metadata
        .fastmem_table_length_facts
        .push(table_length_fact());
    function
        .metadata
        .range_index_facts
        .push(range_index_fact(7, ValueId::new(2)));

    refresh_function_fastmem_access_plans(&mut function);

    let FastMemAccessPlanPayload::Table(table) = &function.metadata.fastmem_access_plans[0].payload
    else {
        panic!("expected table plan");
    };
    assert!(table.proof.table_length_resolved);
    assert!(table.proof.bounds_proof_valid);
    assert!(table.proof.field_offset_resolved);
    assert!(table.proof.overflow_proof_valid);
    assert!(table.proof.is_lowerable());
    assert!(table
        .proof
        .overflow_proof
        .as_deref()
        .unwrap_or_default()
        .contains("usize_mul_add_no_overflow+offset_within_object"));
    assert_eq!(
        function.metadata.fastmem_access_plans[0].status,
        FastMemAccessPlanStatus::Verified
    );
    assert_eq!(
        function.metadata.fastmem_access_plans[0].failure_reason,
        None
    );
    assert_eq!(function.metadata.fastmem_table_field_access_links.len(), 1);
    let link = &function.metadata.fastmem_table_field_access_links[0];
    assert_eq!(link.field_id, "capacity");
    assert!(link.byte_offset > 0);
    assert_eq!(link.field_size, 8);
    assert_eq!(link.field_access, FastMemFieldAccessMode::Load);
    assert_eq!(link.proof, "table_field_link:0:1");
}

#[test]
fn refresh_keeps_overflow_proof_closed_without_bounds_proof() {
    let mut function = make_function(vec![
        memop(
            MemOpKind::TableIndex,
            Some(ValueId::new(10)),
            vec![ValueId::new(1), ValueId::new(2)],
            Some(MemOpAccess::table("page_table")),
        ),
        memop(
            MemOpKind::FieldLoad,
            Some(ValueId::new(11)),
            vec![ValueId::new(10)],
            Some(MemOpAccess::field("capacity")),
        ),
    ]);
    function
        .metadata
        .fastmem_table_length_facts
        .push(table_length_fact());

    refresh_function_fastmem_access_plans(&mut function);

    let FastMemAccessPlanPayload::Table(table) = &function.metadata.fastmem_access_plans[0].payload
    else {
        panic!("expected table plan");
    };
    assert!(table.proof.table_length_resolved);
    assert!(!table.proof.bounds_proof_valid);
    assert!(table.proof.field_offset_resolved);
    assert!(!table.proof.overflow_proof_valid);
    assert!(!table.proof.is_lowerable());
    assert_eq!(
        function.metadata.fastmem_access_plans[0]
            .failure_reason
            .as_deref(),
        Some("verified-table-access-proof-incomplete")
    );
}

#[test]
fn refresh_does_not_link_field_access_before_table_index() {
    let mut function = make_function(vec![
        memop(
            MemOpKind::FieldLoad,
            Some(ValueId::new(11)),
            vec![ValueId::new(10)],
            Some(MemOpAccess::field("capacity")),
        ),
        memop(
            MemOpKind::TableIndex,
            Some(ValueId::new(10)),
            vec![ValueId::new(1), ValueId::new(2)],
            Some(MemOpAccess::table("page_table")),
        ),
    ]);

    refresh_function_fastmem_access_plans(&mut function);

    let FastMemAccessPlanPayload::Table(table) = &function.metadata.fastmem_access_plans[1].payload
    else {
        panic!("expected table plan");
    };
    assert!(!table.proof.field_offset_resolved);
    assert!(function
        .metadata
        .fastmem_table_field_access_links
        .is_empty());
}

#[test]
fn refresh_does_not_consume_range_index_fact_without_matching_length_fact() {
    let mut function = make_function(vec![memop(
        MemOpKind::TableIndex,
        Some(ValueId::new(10)),
        vec![ValueId::new(1), ValueId::new(2)],
        Some(MemOpAccess::table("page_table")),
    )]);
    function
        .metadata
        .range_index_facts
        .push(range_index_fact(7, ValueId::new(2)));

    refresh_function_fastmem_access_plans(&mut function);

    let FastMemAccessPlanPayload::Table(table) = &function.metadata.fastmem_access_plans[0].payload
    else {
        panic!("expected table plan");
    };
    assert!(!table.proof.table_length_resolved);
    assert!(!table.proof.bounds_proof_valid);
    assert_eq!(table.proof.bounds_proof, None);
    assert_eq!(
        function.metadata.fastmem_access_plans[0]
            .failure_reason
            .as_deref(),
        Some("table-length-unresolved")
    );
}

#[test]
fn refresh_rejects_range_index_fact_when_upper_does_not_match_length_value() {
    let mut function = make_function(vec![memop(
        MemOpKind::TableIndex,
        Some(ValueId::new(10)),
        vec![ValueId::new(1), ValueId::new(2)],
        Some(MemOpAccess::table("page_table")),
    )]);
    let mut range = range_index_fact(7, ValueId::new(2));
    range.upper_exclusive_value = ValueId::new(51);
    function
        .metadata
        .fastmem_table_length_facts
        .push(table_length_fact());
    function.metadata.range_index_facts.push(range);

    refresh_function_fastmem_access_plans(&mut function);

    let FastMemAccessPlanPayload::Table(table) = &function.metadata.fastmem_access_plans[0].payload
    else {
        panic!("expected table plan");
    };
    assert!(table.proof.table_length_resolved);
    assert!(!table.proof.bounds_proof_valid);
    assert_eq!(table.proof.bounds_proof, None);
    assert!(!table.proof.is_lowerable());
}
