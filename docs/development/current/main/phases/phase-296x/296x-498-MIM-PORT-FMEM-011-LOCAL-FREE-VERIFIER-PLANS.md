---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-011.
Related:
  - docs/development/current/main/phases/phase-296x/296x-497-MIM-PORT-FMEM-010-LOCAL-FREE-MEMOP-VOCABULARY.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - src/mir/fastmem_access_plan.rs
  - src/runner/mir_json_emit/metadata.rs
  - tools/hako_check/fastmem_capability_inventory_impl.py
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-498 MIM-PORT-FMEM-011 Local Free Verifier Plans

## Decision

Add verifier-owned local free-list access-plan rows for `LocalFreePush` and
`LocalFreePop`, but keep them non-lowerable.

This row turns local free-list MemOps from pure vocabulary into explicit plan
evidence. It still does not open LLVM lowering or mutate `local_free_head`.

## Implemented

```text
src/mir/fastmem_access_plan.rs:
  adds FastMemAccessPlanKind::LocalFreePush / LocalFreePop and a
  FastMemLocalFreeListPlan payload.

src/runner/mir_json_emit/metadata.rs:
  emits local-free plan rows into MIR JSON metadata.

tools/hako_check/fastmem_capability_inventory_common.py:
tools/hako_check/fastmem_capability_inventory_impl.py:
  report local-free plan counts and missing proof counters.

tools/hako_check/fastmem_source_syntax_smoke.sh:
  verifies that the hako_alloc local-free vocabulary pilot now has two
  non-lowerable local-free plan rows.
```

## Plan Shape

Current local-free plans resolve the `local_free_head` field contract but remain
rejected:

```text
kind:
  local_free_push | local_free_pop

status:
  rejected

failure_reason:
  local-free-same-owner-proof-missing

proof flags:
  same_owner_proof_valid=0
  block_next_proof_valid=0
  lowerable=0
```

This is intentional. The next lowering row must first provide a same-owner
route proof and block-next layout/provenance proof.

## Evidence Shape

Expected inventory fields for the pilot:

```text
fastmem_memop_local_free_push_count=1
fastmem_memop_local_free_pop_count=1
fastmem_local_free_list_plan=1
fastmem_local_free_push_plan_count=1
fastmem_local_free_pop_plan_count=1
fastmem_local_free_nonlowerable_count=2
fastmem_local_free_same_owner_required=1
fastmem_local_free_same_owner_missing_count=2
fastmem_local_free_block_next_proof_missing_count=2
```

Expected MIR-to-LLVM producer boundary remains:

```text
[llvm/fastmem:unsupported-kind] local_free_push
```

## Still Closed

```text
LocalFreePush lowering
LocalFreePop lowering
local_free_head ordinary FieldLoad lowering
local_free_head ordinary FieldStore lowering
free_head FieldStore as a mutation shortcut
remote_head / AtomicRemoteHead lowering
remote-owner free routing
TLS backing transfer
owner slot reuse
Python-template C diagnostic payload deletion/archive
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Acceptance

```bash
cargo check -q --lib
cargo test -q --lib refresh_adds_nonlowerable_local_free_list_plans
cargo build --release --bin hakorune
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

```text
MIM-PORT-FMEM-012:
  add the first LocalFreePush LLVM producer pilot only after a verifier-owned
  same-owner proof and block-next layout/provenance proof exist. LocalFreePop
  may stay closed if the push path is the safer first lowering slice.
```
