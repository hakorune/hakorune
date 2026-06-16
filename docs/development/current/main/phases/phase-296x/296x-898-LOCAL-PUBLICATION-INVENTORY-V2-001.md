# 296x-898 LOCAL-PUBLICATION-INVENTORY-V2-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-publication-inventory-v2-v0
source_evidence=296x-897
row_kind=passive_inventory_vocabulary

local_publication_inventory_v2_vocabulary_defined=1
local_publication_inventory_v2_report_only=1
local_publication_inventory_v2_backend_consumable=0
local_publication_inventory_v2_unknown_alias_fallback=1
local_publication_inventory_v2_maybe_published_fallback=1

publication_state_unpublished_fastpath_allowed=1
publication_state_published_fastpath_allowed=0
publication_state_maybe_published_fastpath_allowed=0
fallback_fact_enabled=0
backend_new_lowering_enabled=0
object_storage_plan_execution_enabled=0
next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001
summary=ok
```

## Implementation

`LocalPublicationInventoryRow` is passive report vocabulary in
`src/object_storage_plan.rs`.

It records:

```text
site_id
value_id
alias_class
publication_state
fallback_reason
```

It can tell a later eligibility row whether a site may proceed to fast-path
decision. It is not itself backend-consumable.

## Decision

Inventory rows are observations, not backend proof.

The row may feed later eligibility only when:

```text
alias_class is known
publication_state == Unpublished
fallback_reason == none
```

Unknown alias, `Published`, and `MaybePublished` all remain fallback.

## Tests

```bash
cargo test --lib object_storage_plan -- --nocapture
```

## Stop Lines

- no backend consumption of inventory rows
- no fallback facts
- no full escape engine
- no field-sensitive points-to
- no HostHandle bypass
- no direct storage enablement
- no MIRBuilder representation ownership
