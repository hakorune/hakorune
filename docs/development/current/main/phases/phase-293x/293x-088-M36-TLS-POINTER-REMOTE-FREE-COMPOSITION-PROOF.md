---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M36 TLS + pointer remote-free composition proof
---

# 293x-088 M36 TLS Pointer Remote-Free Composition Proof

## Decision

`M36 TLS + pointer remote-free composition proof` is live-narrow.

M36 does not add a new route row. It composes existing M26 TLS cache-slot rows
with the M35 direct native pointer atomic store row:

```text
TlsCoreBox.cache_slot_set_i64(slot, mailbox_ptr)
TlsCoreBox.cache_slot_get_i64(slot)
externcall "hako_atomic_ptr_store_ordered"(mailbox_ptr, block_ptr, Release)
```

This proves the allocator remote-free mailbox seam can pass a native pointer
cell through TLS and publish a native block pointer through MIR-owned route
facts under pure-first EXE.

## Owned

- `apps/mimalloc-tls-ptr-remote-free-proof/`
- Guard:
  `tools/checks/k2_wide_mimalloc_tls_ptr_remote_free_exe_guard.sh`
- Docs/taskboard update for the M36 composition proof.

## Not Owned

- New MIR route rows.
- New NyRT exports.
- `ptr_load_ordered`.
- `ptr_cas_ordered`.
- pointer `fetch_add`.
- Production remote-free allocator policy.
- Native pointer attrs or noalias/nonnull widening.
- App-specific `.inc` matchers.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_tls_ptr_remote_free_exe_guard.sh
```

The guard must verify:

- TLS helper functions still publish MIR-owned extern route facts.
- `hako_atomic_ptr_store_ordered` still publishes the M35 pointer-store route
  fact.
- pure-first build logs hit the TLS get/set and pointer-store emit traces.
- the EXE publishes through the TLS mailbox and exits `0`.
- pointer load/CAS/fetch_add rows remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_mimalloc_tls_ptr_remote_free_exe_guard.sh` passes.
