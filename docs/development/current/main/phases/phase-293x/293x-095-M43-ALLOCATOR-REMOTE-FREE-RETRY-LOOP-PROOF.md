---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M43 allocator remote-free retry-loop proof
---

# 293x-095 M43 Allocator Remote-Free Retry-Loop Proof

## Decision

`M43 allocator remote-free retry-loop proof` is live-narrow.

M43 adds no new route row. It proves that the M42 same-module policy shape can
own a bounded CAS retry loop:

```text
AllocatorRemoteFreeRetryPolicy.push_retry(head_cell, block_ptr, interferer_ptr)
```

Accepted method-body shape:

```text
loop while not done and retries < bounded_limit:
  old_head = hako_atomic_ptr_load_ordered(head_cell, Acquire)
  hako_atomic_ptr_store_ordered(block, old_head, Release)
  optional injected competing CAS publishes interferer
  observed = hako_atomic_ptr_cas_ordered(head_cell, old_head, block, AcqRel, Acquire)
  if observed == old_head:
    done
  else:
    retries += 1
```

The app forces one failed CAS by injecting an interfering push on the first
attempt, then proves the second attempt succeeds:

```text
block_b -> block_c -> block_a -> null
```

## Owned

- `apps/mimalloc-remote-free-retry-loop-proof/`
- Guard:
  `tools/checks/k2_wide_mimalloc_remote_free_retry_loop_exe_guard.sh`
- Documentation/current pointers for M43.
- Generic no-result consumption for existing pointer store/CAS route facts in
  pure-first EXE emission. This is route-fact driven and not app-specific.

## Not Owned

- New MIR extern route rows.
- New NyRT exports.
- App-specific `.inc` route emit behavior.
- pointer `fetch_add`.
- AtomicCoreBox pointer methods.
- Production allocator retry policy.
- Pointer arithmetic, raw layout source syntax, or native pointer attrs.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_retry_loop_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_remote_free_list_policy_exe_guard.sh
bash tools/checks/k2_wide_pointer_atomic_vocab_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard must verify:

- main uses generic-i64 global routes to reach the retry policy box.
- the retry policy owns pointer load/store/CAS extern route facts.
- pure-first build logs hit global-call and pointer atomic emit traces.
- EXE output proves one retry and the final three-node list shape.
- `.inc` does not branch on the fixture app name.
- no-result pointer store/CAS statements are consumed only through existing
  route facts.
- pointer fetch_add rows remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_mimalloc_remote_free_retry_loop_exe_guard.sh` passes.
