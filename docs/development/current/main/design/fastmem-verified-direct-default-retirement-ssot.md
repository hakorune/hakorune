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
  fastmem region -> OwnerEq condition gate + ordinary if CFG lowering
  ordinary route  -> ordinary if lowering
```

This duplication is allowed only as a transition path for the existing
mimalloc FastMemory producer work.

## Current Status After MIRBUILDER-FMEM-007

The retirement lane is partially landed. The direction is correct, but
`src/mir/builder/fastmem.rs` is still a transitional source lowerer.

Current score:

```text
docs / policy:
  pass
  transitional lowering and verified-direct default are SSOT-backed here

inventory / guard:
  mostly pass
  dedicated lowerer fields and route miss counters exist in report/check

implementation separation:
  partial
  metadata/plans/facts are split, but fastmem.rs still reads broad AST shapes

legacy retirement:
  partial
  branch CFG ownership moved to ordinary if lowering in 007, but field/index/
  numeric/assignment/source wrapper logic still has dedicated interpretation

remaining work clarity:
  open
  008+ rows must retire broad AST interpretation without adding product,
  allocator activation, or AtomicRemoteHead behavior
```

Important correction after 007:

```text
branch dedicated CFG lowering:
  retired

branch condition gate:
  still fastmem-owned
  records FastMemBranchConditionFact and requires region-local OwnerEq
```

This means branch is no longer a bespoke CFG producer, but it is not yet an
ordinary `if` plus generic route fact all the way from the ordinary MIRBuilder
entry point.

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

## Residual Work Map

The MIRBuilder retirement lane is closed through `MIRBUILDER-FMEM-015`.
Remaining work now lives in the proof/contract lane, not in a broader AST
lowerer retirement slice.

```text
closed:
  MIRBUILDER-FMEM-001..015

open:
  FMEM-TABLE-001..005

parked:
  DIRECTARRAY-FMEM-AUTO-LOWERING-LATER
```

What the remaining proof lane owns:

```text
TableIndex bounds proof
OverflowProof
FieldLoad / FieldStore verified plan
JSON/report/check rejection of incomplete TableIndex
VerifiedTableAccess lowering
```

What it does not own:

```text
product activation
hook / global allocator claims
AtomicRemoteHead
new broad AST semantic duplication
```

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

source inventory:
  fastmem_numeric_verified_direct_count tracks supported Add/Sub/Shr/BitAnd
  fastmem_numeric_required_route_miss_count stays at 0
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

Then choose exactly one row from the active retirement rows below.

Rules:

```text
do not mix BoxShape cleanup with a new accepted route
do not add product/allocator activation evidence
do not extend fastmem.rs for a new broad AST shape
prefer report/check inventory before behavior changes
```

Completed restart rows:

```text
MIRBUILDER-FMEM-001..007
```

Current next row:

```text
phase-296x next lane selection pending
```

## Remaining Retirement Rows

### MIRBUILDER-FMEM-008: Post-007 Debt Inventory

Freeze the exact remaining broad AST duplication after branch CFG retirement.

Acceptance:

```text
fastmem_dedicated_local_lowering_count is visible
fastmem_dedicated_assignment_lowering_count is visible
fastmem_dedicated_literal_lowering_count is visible
fastmem_dedicated_variable_lowering_count is visible
fastmem_dedicated_call_lowering_count is visible
fastmem_dedicated_method_call_lowering_count is visible
fastmem_branch_condition_gate_count is visible
fastmem_dedicated_branch_lowering_count=0 remains true
behavior change: none
```

Landed:

```text
FastMemory inventory now exposes the remaining dedicated AST-shape debt as
explicit counts, so the next row can retire shared statement handling.
```

### MIRBUILDER-FMEM-009: Shared Statement Shell

Landed: the fastmem statement shell now routes local, print, return, and
variable assignment through shared builder helpers where safe, while keeping
fastmem expression lowering and verified-direct obligations intact.

The shared shell now owns the mechanical statement handling, while the
verified-direct fastmem expression route remains in place for the actual
memory-profile semantics.

### MIRBUILDER-FMEM-010: Field Route Retirement

Retire direct `FieldAccess -> MemOp(FieldLoad/FieldStore)` interpretation from
the fastmem source lowerer.

Acceptance:

```text
fastmem field source emits ordinary FieldGet/FieldSet plus FieldAccessSite
verified field route remains required inside fastmem regions
fastmem_dedicated_field_access_lowering_count=0
field_access_required_verified_direct_miss_count=0
remote_head plain FieldStore remains rejected
```

Landed:

```text
FastMemory field accesses now share the ordinary FieldGet/FieldSet builder
path while keeping verified-direct field evidence and rejecting remote-head
plain FieldStore.
```

### MIRBUILDER-FMEM-010 Status

Landed: the field route now uses the shared `FieldGet` / `FieldSet` builder
path, with field access sites and verified-direct field evidence preserved in
inventory and check output.

### MIRBUILDER-FMEM-011: Index Route Retirement

Retire direct `Index -> MemOp(TableIndex)` interpretation from the fastmem
source lowerer.

Acceptance:

```text
fastmem index source emits ordinary index origin plus IndexAccessSite
verified table route remains required inside fastmem regions
fastmem_dedicated_index_lowering_count=0
index_access_required_verified_table_miss_count=0
TableIndex without bounds/overflow proof remains non-lowerable
```

### MIRBUILDER-FMEM-011 Status

Landed: the fastmem index route now uses the shared index helper path while
keeping verified-table evidence and index access sites visible in inventory
and check output.

### MIRBUILDER-FMEM-012: Numeric Route Retirement

Retire direct `BinaryOp -> MemOp(Add/Sub/Shr/BitAnd)` interpretation from the
fastmem source lowerer.

Acceptance:

```text
fastmem numeric source emits ordinary BinOp plus numeric route fact
fastmem_numeric_verified_direct_count > 0
fastmem_numeric_required_route_miss_count=0
fastmem_dedicated_binary_op_lowering_count=0
ordinary short-circuit remains ordinary lowering-owned
```

### MIRBUILDER-FMEM-012 Status

Landed: the fastmem numeric route now uses the shared binary helper path while
keeping verified-direct numeric evidence and ordinary `BinOp` shape visible in
inventory and check output.

### MIRBUILDER-FMEM-013: Intrinsic Registry Cleanup

Keep `mem.*` as FastMemory intrinsic vocabulary, but move hardcoded call
matching behind a small registry descriptor.

Acceptance:

```text
mem.currentAllocOwnerId / mem.ownerEq / free-list / remote-head intrinsics
  are registry-backed
arity and unsupported-intrinsic errors are stable
no new intrinsic behavior
no product activation / hook / global allocator claim
```

### MIRBUILDER-FMEM-013 Status

Landed: the fastmem `mem.*` call lowering now uses a small intrinsic registry
descriptor with stable arity and unsupported-intrinsic handling, while keeping
intrinsic behavior stable and centralized.

### MIRBUILDER-FMEM-014: Branch Condition Gate Generalization

Move the remaining branch condition gate from fastmem-only AST handling toward
ordinary condition route facts.

Acceptance:

```text
ordinary if condition can carry required ownerEq route fact
fastmem branch condition still requires ownerEq
fastmem_branch_condition_required_owner_eq_count > 0
fastmem_branch_condition_owner_eq_miss_count=0
fastmem branch wrapper no longer owns condition AST interpretation
```

### MIRBUILDER-FMEM-014 Status

Landed: the remaining fastmem branch gate now carries ownerEq route facts
through the shared if-form path, so the fastmem branch wrapper no longer owns
AST-specific condition interpretation.

### MIRBUILDER-FMEM-015: Dedicated Lowerer Closeout

Close the transitional source lowerer once field/index/numeric/branch-condition
routes are ordinary-entry plus verified-direct obligations.

Acceptance:

```text
fastmem.rs keeps only region entry / obligation shell
fastmem_source_dedicated_lowerer_retirement_required=0
all fastmem_dedicated_*_lowering_count=0
verified-direct report/check gates remain positive where fixtures exercise them
no allocator/product activation claims opened
```

### MIRBUILDER-FMEM-015 Status

Landed: the transitional fastmem source lowerer is now a thin region-entry and
obligation shell. Field, index, numeric, and branch-condition handling stay on
the shared builder paths while verified-direct evidence remains intact.

### Current Next Row

`phase-296x next lane selection pending`

## Source Surface Fate Gate (queued, not an execution permission)

The public `fastmem Contract { ... }` surface, `mem.assume*` proof calls, and
the internal `MemOp` dialect have different fates. Do not delete the source
surface or the internal dialect from a textual grep result. The source surface
is a retirement candidate; the internal `MemOp`, access plans, verifier facts,
and LLVM lowering remain kept unless a separate producer/consumer census
proves otherwise.

### MIR-FMEM-SOURCE-REGION-ORDINARY-OWNER-CENSUS-D0

```text
Decision:
  Run one read-only, finite census before opening any source-retirement row.
  Treat FastMemRegion source syntax, mem.assume* proof vocabulary, and the
  internal MemOp dialect as separate fate decisions.

Source authority + canonical issuer:
  Existing parser/AST FastMemRegion, region registration in fastmem.rs, and
  MemOp issuance in fastmem/ops.rs are the inventory anchors. The census must
  name the one ordinary-source/planner owner, if one exists, that can issue the
  same region obligation without a second source authority.

Non-authority:
  JSON/report text, mem.* strings outside the region contract, fixture names,
  LLVM symbols, VM/CArtifact support, registry lookups, and product-activation
  flags do not prove a production caller or a replacement owner.

Fail-fast boundary:
  The census starts at the FastMemRegion parser/source entry and ends at every
  source/wire/MemOp/verified-plan/backend consumer. Missing proof, region
  escape, unsupported backend shape, or an unclassified use blocks deletion.
  No source syntax is removed before an equal-or-stronger ordinary owner and
  a finite successor/delete set are named.

Smallest next slice:
  Inventory every FastMem source file/fixture row, all 17 MemOp kinds, and
  their producer/consumer classes. Classify each use as Product,
  InternalCompiler, Proof, Test, or Historical; record the exact product
  caller count and the ordinary owner candidate. Code, fixtures, and guards
  remain unchanged in D0.

Non-claims:
  No immediate syntax deletion, MemOp deletion, allocator activation,
  generic ContractRegion execution, VM/CArtifact enablement, or legacy test /
  guard purge.
```

The census boundary includes parser/AST, the Hako parser mirror, source/wire
projection, region registration, `mem.*` source admission, MemOp producers,
verified access plans, JSON/report consumers, and LLVM consumers. It excludes
ordinary external `mem.get/set/alloc/free` calls that are not part of the
FastMem region contract.

The required outcome is one of these finite states:

```text
Product caller = 0 and one ordinary owner + finite successor exists:
  eligible for a later source-surface retirement row

Product caller > 0:
  name the migration owner and keep the source surface until the ordinary
  route has source/proof evidence; do not delete in D0

ordinary owner = 0, multiple, or unknown:
  ParkedSealed__NoOrdinaryRegionOwner; do not add another receipt, adapter,
  fixture, or guard to search for a different answer
```

`mem.assume*` must be counted separately from operational `mem.*` calls. It
is a proof/contract injection vocabulary, not a normal runtime call. A future
replacement must be compiler-issued or an explicitly bounded internal
contract, not an unrestricted user-level trust-me path. Operational source
forms may only be retired after their ordinary source shape, access-site facts,
planner proof, and existing MemOp lowering have one named owner.

After this D0, a source-retirement row may remove the parser/AST/source
dispatcher and migrate proof fixtures only when its caller-zero and successor
evidence are complete. A separate later row owns the seven
transport/proof-centered MemOps that currently lack a full execution backend;
that row must not be merged with public syntax retirement.

### Conditional follow-up tasks (named, not opened)

These are routing outcomes of the D0, not simultaneous execution permission:

```text
MIR-FMEM-SOURCE-ORDINARY-ROUTE-MIGRATE-I0:
  open only when Product caller > 0 and exactly one ordinary
  source/planner owner plus one production caller is identified.
  Migrate one semantic family to region obligation -> verified plan -> the
  existing MemOp path, and delete that family's old source edge in the same
  slice.  No new source dialect or second planner is allowed.

MIR-FMEM-SOURCE-SURFACE-RETIRE-R0:
  open only when Product caller = 0, the ordinary successor is proven, and
  the finite delete set is closed.  Retire the public parser/AST/source
  dispatcher and source-only smoke/proof fixtures that have a successor;
  keep MIR MemOp, plans, verifier, LLVM lowering, and their contract tests.

MIR-FMEM-MEMOP-TRANSPORT-CENSUS-D0:
  separately classify all 17 MemOp kinds by producer, plan/verifier,
  transport, backend, and test/reference use.  Do not infer deletion from
  LLVM/VM support alone.

MIR-FMEM-UNUSED-MEMOP-RETIRE-R0:
  open only for a MemOp kind whose producer, consumer, and required evidence
  are all zero or have one equal-or-stronger successor.  Remove its
  transport/proof-only fixtures and guards together; retain shared vocabulary
  and tests for still-live kinds.

MIR-FMEM-SOURCE-FIXTURE-GUARD-PRUNE-R0:
  run only after the source fate or MemOp fate row reaches caller-zero.
  Delete duplicate/obsolete source fixtures and guards with that family's
  old edge, while preserving verifier/plan negative coverage and recording
  the exact changed-test names.
```

If the D0 returns zero, multiple, or unknown ordinary owners, none of the
conditional implementation/retirement rows opens.  The result is recorded as
`ParkedSealed__NoOrdinaryRegionOwner`; cleanup remains limited to unrelated
historical documentation whose own lifecycle owner and references are closed.
