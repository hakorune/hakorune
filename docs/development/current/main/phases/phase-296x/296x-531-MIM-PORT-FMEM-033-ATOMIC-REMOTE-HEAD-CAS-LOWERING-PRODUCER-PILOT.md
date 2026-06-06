---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-033.
Related:
  - docs/development/current/main/phases/phase-296x/296x-530-MIM-PORT-FMEM-032-ATOMIC-REMOTE-HEAD-CAS-LOWERING-REPORT-CHECK-PREFLIGHT.md
  - src/llvm_py/llvm_builder.py
  - src/llvm_py/fastmem_metadata.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-531 MIM-PORT-FMEM-033 AtomicRemoteHead CAS Lowering Producer Pilot

## Purpose

Open the first LLVM producer implementation for `AtomicRemoteHeadPush` after
MIM-032 made the proof-consuming report/check contract visible.

This row should consume only verified AtomicRemoteHead access plans:

```text
remote_head field metadata resolved
remote-owner proof valid
remote-free block-next proof valid
memory_order_policy selected by this row
```

## Candidate Lowering Shape

```text
AtomicRemoteHeadPush(page, block):
  remote_head_ptr = verified PageMeta.remote_head address
  old_head = atomic_load(remote_head_ptr)
  block.next = old_head
  cmpxchg remote_head_ptr old_head block
  retry policy = deferred
```

The exact LLVM helper shape is still implementation-owned by this row, but it
must not recompute field offsets or route policy in the lowerer. Lowering reads
the verified plan only.

## Landed

The first producer pilot is intentionally narrow:

```text
AtomicRemoteHeadPush(page, block):
  page is consumed as a backend-private LayoutRef
  block is consumed as a pointer-sized scalar
  remote_head_ptr comes from the verified PageMeta.remote_head plan
  old_head = atomic load acquire
  block.next = old_head
  cmpxchg remote_head old_head block with acq_rel / acquire
```

This is a single-attempt CAS evidence row. Retry loops, CAS result routing,
remote-owner branch routing, and drain/exchange stay closed for the next row.

## Still Closed

```text
remote owner branch routing
AtomicRemoteHead drain/exchange
TLS backing transfer
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Acceptance

```text
memop_atomic_remote_head_lowered_count=1
atomic_remote_head_cas_lowering_open=1
atomic_remote_head_push_lowerable_count=1
fastmem_lowering_used_verified_plan=1
fastmem_lowering_recomputed_layout_offset_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```

## Verification

```text
cargo build --release --bin hakorune
cargo test -q --lib atomic_remote_head
.venv/bin/pytest -q src/llvm_py/tests/test_fastmem_memop_layoutref.py
python3 -m py_compile src/llvm_py/instructions/memop.py tools/hako_check/fastmem_mir_to_llvm_producer_report.py tools/hako_check/fastmem_check.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-034:
  select the next AtomicRemoteHead route slice after the single-attempt CAS
  producer pilot. Candidate work: retry/drain/remote-owner routing selection
  with TLS transfer, product activation, hooks, global allocator claim, and
  winner claim still closed.
```
