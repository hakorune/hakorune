---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M37 allocator remote-free policy integration proof
---

# 293x-089 M37 Allocator Remote-Free Policy Integration Proof

## Decision

`M37 allocator remote-free policy integration proof` is live-narrow.

M37 adds no new runtime or backend route. It connects the M36 mailbox seam to a
small same-module allocator policy box:

```text
AllocatorRemoteFreePolicy.install_mailbox(slot, mailbox_ptr)
AllocatorRemoteFreePolicy.publish_remote_free(slot, block_ptr)
AllocatorRemoteFreePolicy.release_mailbox(slot)
```

The policy box uses existing route facts only:

- M26 TLS cache-slot get/set.
- M35 direct native pointer atomic store.
- hako.mem alloc/free.
- same-module generic-i64 body routing.

The accepted policy body shape is intentionally straight-line. M37 does not
widen same-module generic-i64 body acceptance for branchy policy methods.

The only C seam change is metadata-classification cleanup: same-module prepass
uses `return_shape=scalar_i64` as the scalar-result truth and no longer treats
`value_demand=runtime_i64` as part of the return type. This lets the existing
M35 pointer-store route (`value_demand=native_ptr_nullable`) participate in a
same-module generic-i64 body without adding a new route or symbol matcher.

## Owned

- `apps/mimalloc-remote-free-policy-proof/`
- Guard:
  `tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh`
- Docs/taskboard update for the M37 policy integration proof.

## Not Owned

- New MIR route rows.
- New NyRT exports.
- `ptr_load_ordered`.
- `ptr_cas_ordered`.
- pointer `fetch_add`.
- Full mimalloc remote-free list policy.
- Native pointer attrs or noalias/nonnull widening.
- App-specific `.inc` matchers.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh
```

The guard must verify:

- policy methods route as same-module generic-i64 bodies.
- TLS helper functions still publish MIR-owned extern route facts.
- `hako_atomic_ptr_store_ordered` still publishes the M35 pointer-store route
  fact.
- pure-first build logs hit policy, TLS, pointer-store, and hako.mem emit
  traces.
- the EXE publishes through the policy box and exits `0`.
- pointer load/CAS/fetch_add rows remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_mimalloc_remote_free_policy_exe_guard.sh` passes.
