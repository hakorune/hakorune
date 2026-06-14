---
Status: Active
Date: 2026-06-15
Task: CROSS-BLOCK-FIELD-GET-ALIAS-FORWARDING-DESIGN-001
Scope: Design the safe keeper boundary for cross-block field_get-origin copy
  chains selected by 296x-670 before touching LocalSSA or MIRBuilder code.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-670-FIELD-GET-DIRECT-CONSUMER-FORWARDING-REFRESH-002.md
  - docs/development/current/main/phases/phase-296x/296x-182-FIELD-GET-DIRECT-CONSUMER-FORWARDING-KEEPER-DESIGN.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# CROSS-BLOCK-FIELD-GET-ALIAS-FORWARDING-DESIGN-001

## Purpose

296x-670 showed that the current remaining owner is not the historical
same-block `FieldGet -> consumer` case:

```text
forwarding_candidate_copy_count=4
same_block_candidate_count=1
cross_block_candidate_count=3
covered_by_existing_rule_count=0
selected_owner=cross_block_field_get_alias_copy_chain
```

This row designs the narrow keeper boundary for that owner. It must not reopen
the rejected param forwarding path and must not broaden LocalSSA copy
coalescing.

## Required Design Questions

```text
1. dominance:
   Does the root field_get value dominate the candidate consumer block?

2. mutation safety:
   Is there any intervening field_set to the same receiver/field along the
   candidate path?

3. SSA visibility:
   Can the consumer legally use the root value id directly, or is a block-local
   materialization required by current MIR/LocalSSA contracts?

4. row182 relation:
   Is the same-block remaining candidate a copy-chain gap after row182, or a
   case that should stay materialized?

5. implementation seam:
   Is the keeper owned by LocalSSA materialization, field_get expression
   lowering, or a small dominance-aware alias map?
```

## Non-Goals

```text
do not implement forwarding in this row
do not coalesce arbitrary copy chains
do not forward across possible field mutation
do not forward without dominance proof
do not touch .hako source
do not touch allocator provider activation
do not change product NyRT startup
```

## Expected Output

```text
output_contract=hako-mimalloc-cross-block-field-get-alias-forwarding-design-v0
selected_owner=cross_block_field_get_alias_copy_chain
keeper_shape=<dominance_alias|same_block_chain_only|no_keeper>
dominance_required=1
same_field_mutation_guard_required=1
arbitrary_copy_coalescing_allowed=0
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

## Acceptance

```text
cross_block_field_get_alias_forwarding_design_active=1
source_evidence=296x-670
keeper_shape=0
dominance_required=1
same_field_mutation_guard_required=1
implementation_started=0
optimization_open=0
summary=pending
```
