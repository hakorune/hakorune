# 296x-897 LOCAL-ALIAS-CLASS-MVP-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-alias-class-mvp-v0
source_evidence=296x-896
row_kind=passive_vocabulary

local_alias_class_mvp_vocabulary_defined=1
local_alias_class_mvp_source_count=5
local_alias_source_local_assignment=1
local_alias_source_ssa_copy=1
local_alias_source_phi=1
local_alias_source_select=1
local_alias_source_simple_receiver_alias=1

local_alias_class_heap_graph_enabled=0
local_alias_class_field_sensitive_points_to_enabled=0
local_alias_class_collection_element_alias_enabled=0
local_alias_class_recursive_object_graph_enabled=0
object_storage_plan_execution_enabled=0
backend_new_lowering_enabled=0
next_task=LOCAL-PUBLICATION-INVENTORY-V2-001
summary=ok
```

## Implementation

`src/object_storage_plan.rs` now names the v0 alias sources that a future
classifier may report:

```text
LocalAssignment
SsaCopy
Phi
Select
SimpleReceiverAlias
```

`LocalAliasClassObservation` is passive vocabulary only. It does not run a
classifier and does not authorize backend lowering.

## Decision

The alias MVP is deliberately smaller than a general escape / points-to engine.
It may group local assignment, SSA copy, PHI, select, and simple receiver alias
observations. Heap graph traversal, field-sensitive points-to, collection
element aliasing, and recursive object graph aliasing remain out of scope.

## Tests

```bash
cargo test --lib object_storage_plan -- --nocapture
```

## Stop Lines

- no heap graph traversal
- no field-sensitive points-to
- no collection element aliasing
- no recursive object graph aliasing
- no backend lowering enablement
- no MIRBuilder representation ownership
