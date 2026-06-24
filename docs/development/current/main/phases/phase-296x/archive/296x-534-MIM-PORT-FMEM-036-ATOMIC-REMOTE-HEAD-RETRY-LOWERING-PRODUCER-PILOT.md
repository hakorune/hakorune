---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-036.
Related:
  - docs/development/current/main/phases/phase-296x/296x-533-MIM-PORT-FMEM-035-ATOMIC-REMOTE-HEAD-RETRY-POLICY-PREFLIGHT.md
  - docs/development/current/main/phases/phase-296x/296x-531-MIM-PORT-FMEM-033-ATOMIC-REMOTE-HEAD-CAS-LOWERING-PRODUCER-PILOT.md
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-534 MIM-PORT-FMEM-036 AtomicRemoteHead Retry Lowering Producer Pilot

## Purpose

Open the first bounded retry lowering for `AtomicRemoteHeadPush` after MIM-035
made the retry policy visible in report/check evidence.

This row may change the LLVM producer shape for `AtomicRemoteHeadPush`, but only
inside the already verified remote-free publication primitive.

## Candidate Lowering Shape

```text
AtomicRemoteHeadPush(page, block):
  for attempt in 0..retry_attempt_limit:
    old_head = atomic_load(remote_head, acquire)
    block.next = old_head
    result = cmpxchg(remote_head, old_head, block, acq_rel/acquire)
    if result.success:
      break
  expose retry-lowered evidence
```

The first producer pilot can be conservative. It does not need to route the
failure result to source-visible control flow yet, but it must not silently
claim drain/branch behavior.

## Still Closed

```text
AtomicRemoteHead drain/exchange
remote-owner branch routing
same/remote free full body route
TLS backing transfer
owner slot reuse
process allocator replacement
hook installation
global allocator claim
winner claim
full .hako mimalloc algorithm claim
```

## Acceptance

```text
atomic_remote_head_retry_policy_open=1
atomic_remote_head_retry_attempt_limit=3
atomic_remote_head_retry_lowered_count=1
memop_atomic_remote_head_lowered_count=1
atomic_remote_head_drain_open=0
remote_owner_branch_routing_open=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```

## Landed

`AtomicRemoteHeadPush` now carries a verifier-owned
`retry_attempt_limit=3` through the FastMemory access plan and MIR JSON
metadata. The Python LLVM producer consumes that verified plan and emits a
bounded retry loop:

```text
atomic load remote_head
store old head into block.next
cmpxchg remote_head
branch on success to retry-done, otherwise continue to the next bounded attempt
```

The raw remote-head publication remains inside the verified
`AtomicRemoteHeadPush` primitive. It does not open drain/exchange,
remote-owner branch routing, TLS backing transfer, product activation, hook
installation, global allocator claim, winner claim, or full `.hako` mimalloc
algorithm claim.

## Report / Check

Added a dedicated producer profile:

```text
fastmem_atomic_remote_head_retry_producer_pilot=1
replacement_front_next_producer_slice=atomic_remote_head_retry_lowering_producer_pilot
atomic_remote_head_retry_policy_open=1
atomic_remote_head_retry_attempt_limit=3
atomic_remote_head_retry_lowered_count=1
atomic_remote_head_drain_open=0
remote_owner_branch_routing_open=0
```

`remote-free-retry-preflight` remains report-only/static and keeps
`atomic_remote_head_retry_policy_open=0` / `atomic_remote_head_retry_lowered_count=0`.

## Verification

```text
cargo build --release --bin hakorune
cargo test -q --lib atomic_remote_head
.venv/bin/pytest -q src/llvm_py/tests/test_fastmem_memop_layoutref.py
python3 -m py_compile src/llvm_py/instructions/memop.py tools/hako_check/fastmem_mir_to_llvm_producer_report.py tools/hako_check/fastmem_check.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

Open MIM-PORT-FMEM-037 as the `AtomicRemoteHeadDrain` preflight row. That row
should select drain/exchange vocabulary and report/check obligations before any
remote-owner branch routing, TLS transfer, abandoned reclaim behavior, or
product activation opens.
