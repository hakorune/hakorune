---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-044.
Related:
  - docs/development/current/main/phases/phase-296x/296x-541-MIM-PORT-FMEM-043-ATOMIC-REMOTE-HEAD-DRAIN-LOCAL-LIST-MUTATION-PROOF.md
  - src/mir/fastmem_access_plan.rs
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-542 MIM-PORT-FMEM-044 AtomicRemoteHead Drain Local List Mutation Vocabulary Preflight

## Purpose

Add source/MIR vocabulary for the dedicated owner-local list mutation operation
that will consume the `remote_free_list_token` produced by
`AtomicRemoteHeadDrain(page)`.

## Candidate Shape

```text
remote = mem.atomicRemoteHeadDrain(page)
mem.drainRemoteListToLocal(page, remote)
```

The row should prove the vocabulary is visible in source/MIR metadata and that
the operation is rejected before verifier-owned mutation plans exist.

## Required Boundaries

```text
local list mutation lowering remains closed
remote-owner branch routing remains closed
same/remote free full body route remains closed
TLS backing transfer remains closed
owner slot reuse remains closed
abandoned reclaim behavior remains closed
process allocator replacement remains closed
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
```

## Acceptance Sketch

```text
fastmem_atomic_remote_head_drain_local_list_mutation_vocabulary_preflight=1
fastmem_memop_drain_remote_list_to_local_count=1
atomic_remote_head_drain_local_list_mutation_selected=1
atomic_remote_head_drain_local_list_mutation_open=0
atomic_remote_head_drain_local_list_mutation_lowerable_count=0
remote_owner_branch_routing_open=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```

If the operation name needs adjustment, keep the card scoped to vocabulary and
source/MIR observation only. Producer lowering should be split into a later row.

## Landed

```text
mem.drainRemoteListToLocal(page, remote_free_list_token)
```

is now visible as source/MIR vocabulary through
`MemOpKind::DrainRemoteListToLocal` with JSON kind
`drain_remote_list_to_local`.

The operation is transport-visible only:

```text
fastmem_memop_drain_remote_list_to_local_count=1
atomic_remote_head_drain_local_list_mutation_lowerable_count=0
atomic_remote_head_drain_local_list_mutation_open=0
remote_owner_branch_routing_open=0
```

LLVM lowering remains closed and fails fast on the unsupported MemOp kind.

## Evidence

```bash
cargo build --release --bin hakorune
cargo test -q --lib fastmem_source_emits_drain_remote_list_to_local_memop
cargo test -q --lib fastmem_v0_memop_kind_count_is_intentional
python3 -m py_compile tools/hako_check/fastmem_check.py tools/hako_check/fastmem_mir_to_llvm_producer_report.py tools/hako_check/fastmem_capability_inventory_common.py tools/hako_check/fastmem_capability_inventory_impl.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

MIM-PORT-FMEM-045 should add verifier-owned preconditions for
`DrainRemoteListToLocal(page, token)` without opening mutation lowering,
remote-owner branch routing, TLS transfer, product activation, or allocator
claims.
