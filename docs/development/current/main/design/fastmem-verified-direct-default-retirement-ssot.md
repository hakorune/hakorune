---
Status: SSOT
Decision: accepted
Date: 2026-06-08
Scope: phased retirement plan for dedicated FastMemory AST lowering in MIRBuilder.
Related:
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - docs/development/current/main/design/contract-region-v0-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - src/mir/builder/README.md
  - src/mir/builder/fastmem.rs
  - src/mir/builder/exprs.rs
  - src/mir/builder/fields.rs
---

# FastMemory Verified-Direct Default / Dedicated Lowering Retirement

## Decision

Do not make `fastmem` a default language surface.

Do make verified-direct routing the default optimization/check path when the
planner/verifier can prove a direct memory route.

Short form:

```text
fastmem syntax:
  contract/check region

verified-direct route:
  default optimization target when proof exists

fallback inside fastmem region:
  forbidden
```

The current dedicated FastMemory AST lowering in `src/mir/builder/fastmem.rs`
is accepted as Phase-1 compatibility. It is not the long-term architecture.

Long-term target:

```text
ordinary MIRBuilder:
  emits canonical source shape plus access-site metadata

Planner / verifier:
  selects and proves verified-direct route

Lowering:
  consumes verified plans only
```

## Current Transitional Shape

Today, FastMemory source lowering and ordinary source lowering interpret the
same AST shapes through separate paths:

```text
ASTNode::FieldAccess:
  fastmem region -> MemOp(FieldLoad)
  ordinary route  -> FieldGet

field assignment:
  fastmem region -> MemOp(FieldStore)
  ordinary route  -> FieldSet

ASTNode::Index:
  fastmem region -> MemOp(TableIndex)
  ordinary route  -> box/plugin get/set call

ASTNode::BinaryOp:
  fastmem region -> MemOp(LogicalShr / BitAnd / Add / Sub)
  ordinary route  -> BinOp / Compare / short-circuit lowering

fastmem branch:
  fastmem region -> narrow OwnerEq/branch route owner
  ordinary route  -> ordinary if lowering
```

This duplication is allowed only as a transition path for the existing
mimalloc FastMemory producer work.

## Non-Goals

Do not do these in this retirement lane:

```text
delete src/mir/builder/fastmem.rs immediately
rewrite all ordinary Index lowering into TableIndex
add generic IndexGet/IndexSet before an accepted design row
make MemOp a generic all-purpose route vocabulary
merge FastMemAccessPlan and DirectArrayAccessPlan payloads
open CurrentAllocOwnerId / OwnerEq in this cleanup unless its row says so
open AtomicRemoteHead
open TLS backing transfer
open provider/product allocator activation
install hooks
claim global allocator replacement
claim a benchmark winner
```

## Invariants

MIRBuilder is one source-to-MIR builder. FastMemory is not a second language.

```text
MIRBuilder owns:
  source span
  region/contract metadata
  access-site origin
  symbolic ids
  malformed-source fail-fast

MIRBuilder does not own:
  fast-vs-slow route selection
  page-map strategy
  layout offset truth
  table stride truth
  backend selection
  Type ABI hot route
  Provider ABI hot route
```

Planner/verifier own:

```text
required route
fallback policy
layout/table/index/alignment proof
no-escape proof
verified access plan
```

Lowering owns:

```text
read verified plan
emit backend code
report producer evidence
```

Lowering must not recompute or infer layout/table facts.

## Phase-1 Guard

Until retired, `src/mir/builder/fastmem.rs` may continue to lower the current
explicit fastmem source vocabulary.

New rule:

```text
no new broad AST semantic duplication in fastmem.rs
```

If a new AST shape appears necessary, add an access-site / planner-route design
first. Do not extend the dedicated lowerer as the default answer.

## Task Order

### MIRBUILDER-FMEM-000: Retirement SSOT

This document.

Acceptance:

```text
dedicated FastMemory lowering is documented as transitional
verified-direct default wording is documented
no immediate fastmem.rs deletion is required
next tasks are ordered
```

### MIRBUILDER-FMEM-001: Dedicated Lowering Inventory

Add report/check inventory for the transitional lowerer.

Suggested report fields:

```text
fastmem_source_dedicated_lowerer_enabled=1
fastmem_source_dedicated_lowerer_transitional=1
fastmem_source_dedicated_lowerer_retirement_required=1

fastmem_dedicated_field_access_lowering_count
fastmem_dedicated_index_lowering_count
fastmem_dedicated_binary_op_lowering_count
fastmem_dedicated_assignment_lowering_count
fastmem_dedicated_branch_lowering_count
```

This row observes the debt. It does not need to reduce the counts.

Acceptance:

```text
inventory is report-visible
fastmem-check can assert the transitional flags
no backend behavior changes
no new smoke script unless an existing FastMemory smoke cannot be extended
```

### MIRBUILDER-FMEM-002: FieldAccessSite Metadata

Introduce field access-site metadata usable by ordinary lowering and
FastMemory-required routes.

Minimum shape:

```text
FieldAccessSite:
  site_id
  source_span
  base_value
  field_name
  region_id optional
  required_route optional
  fallback_policy
```

Policy:

```text
ordinary code:
  required_route=none
  fallback_policy=allow_dynamic

inside fastmem contract region:
  required_route=verified_layout_field
  fallback_policy=forbidden
```

Acceptance:

```text
ordinary FieldGet/FieldSet can carry or reference a FieldAccessSite
fastmem region FieldAccessSite marks verified-direct as required
no dedicated field lowering is retired yet
```

### MIRBUILDER-FMEM-003: Field Route Unification

Retire dedicated FastMemory field load/store AST lowering first.

Target:

```text
fastmem page.used:
  FieldGet + FieldAccessSite
  -> planner/verifier verified field route
  -> VerifiedMemAccessPlan
  -> MemOp/LLVM lowering
```

Acceptance:

```text
fastmem_dedicated_field_access_lowering_count=0
field_access_required_verified_direct_count > 0
field_access_required_verified_direct_miss_count=0
memop_field_load_lowered_count > 0
FieldStore remains restricted to allowed mutable non-atomic fields
remote_head plain FieldStore remains rejected
```

### MIRBUILDER-FMEM-004: IndexAccessSite Metadata

Add index access-site metadata before unifying TableIndex.

Start with metadata on the current ordinary route. Do not add generic
`IndexGet` / `IndexSet` unless a separate accepted row chooses that vocabulary.

Minimum shape:

```text
IndexAccessSite:
  site_id
  source_span
  target_value
  index_value
  region_id optional
  table_id optional
  required_route optional
  fallback_policy
```

Acceptance:

```text
ordinary index get/set preserves origin through IndexAccessSite
fastmem region index sites can require verified_table_index
dedicated TableIndex lowering is not retired yet
```

### MIRBUILDER-FMEM-005: Index Route Unification

Retire dedicated FastMemory index AST lowering.

Target:

```text
ordinary IndexAccessSite
  -> planner/verifier verified table route
  -> TableIndex / LayoutRef plan
  -> LLVM private LayoutRef carrier
```

Acceptance:

```text
fastmem_dedicated_index_lowering_count=0
index_access_required_verified_table_count > 0
index_access_required_verified_table_miss_count=0
TableIndex without bounds/overflow proof remains non-lowerable
raw metadata pointer is not inserted into ordinary vmap
```

### MIRBUILDER-FMEM-006: Numeric Route Unification

Retire dedicated FastMemory binary-op AST lowering.

Target:

```text
ordinary BinOp
  -> exact numeric/address/page-key route facts
  -> planner/verifier required FastMemory numeric route when inside region
```

Acceptance:

```text
fastmem_dedicated_binary_op_lowering_count=0
fastmem_numeric_verified_direct_count > 0
fastmem_numeric_required_route_miss_count=0
ordinary short-circuit semantics remain ordinary lowering owned
```

### MIRBUILDER-FMEM-007: Branch Route Unification

Retire dedicated FastMemory branch lowering last.

Target:

```text
ordinary if lowering
  + condition route fact
  + fastmem branch verifier
```

Acceptance:

```text
fastmem_dedicated_branch_lowering_count=0
fastmem_branch_condition_required_owner_eq_count > 0
fastmem_branch_condition_owner_eq_miss_count=0
ordinary CFG ownership remains in the ordinary builder
```

## Smaller-Model Restart Recipe

When continuing this lane, read in this order:

```text
1. docs/development/current/main/CURRENT_STATE.toml
2. CURRENT_TASK.md
3. docs/development/current/main/workstreams/mimalloc-current.md
4. this document
5. src/mir/builder/fastmem.rs
6. src/mir/builder/exprs.rs
7. src/mir/builder/fields.rs
```

Then choose exactly one row from `MIRBUILDER-FMEM-001..007`.

Rules:

```text
do not mix BoxShape cleanup with a new accepted route
do not add product/allocator activation evidence
do not extend fastmem.rs for a new broad AST shape
prefer report/check inventory before behavior changes
```

First implementation row:

```text
MIRBUILDER-FMEM-001
```

Reason:

```text
it makes the transitional lowerer visible before any retirement work starts
```
