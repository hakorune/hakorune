---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M26 mimalloc TLS cache-slot EXE proof
---

# 293x-078 M26 Mimalloc TLS Cache-Slot EXE Proof

## Decision

`M26 mimalloc TLS cache-slot EXE proof` is live-narrow.

The row adds the first allocator-shaped `hako.tls` cache-slot seam for
pure-first EXE. This is intentionally not a generic TLS cell API.

Accepted shape:

```text
TlsCoreBox.cache_slot_get_i64(slot)
  -> externcall "hako_tls_cache_slot_get_i64"(slot)

TlsCoreBox.cache_slot_set_i64(slot, value)
  -> externcall "hako_tls_cache_slot_set_i64"(slot, value)
```

MIR owns the extern route facts. The pure-first backend reads those facts and
emits the matching runtime calls. The runtime owns per-thread i64 slot storage.

## Owned

- `apps/mimalloc-tls-cache-slot-proof/`
- `TlsCoreBox.cache_slot_get_i64/1`
- `TlsCoreBox.cache_slot_set_i64/2`
- MIR extern route rows for:
  - `hako_tls_cache_slot_get_i64/1`
  - `hako_tls_cache_slot_set_i64/2`
- pure-first `.inc` declaration/need/emit rows for those route ids.
- NyRT exports for the two symbols.
- Guard:
  `tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh`

## Not Owned

- Language-level `TlsCell<T>`.
- Raw numeric TLS slot syntax.
- More than the fixed narrow cache-slot count owned by the runtime export.
- TLS allocator policy.
- Atomic remote-free primitives.
- Native pointer strong attrs.
- Backend-local helper-name inference.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh
```

The guard must verify:

- MIR JSON publishes the two `extern_call_routes` rows.
- `lowering_plan` carries the same route ids/symbols/arity.
- pure-first build logs hit only the route-fact emit consumers.
- the EXE prints deterministic cache-slot get/set output.
- `.inc` does not branch on the fixture app name.

## Result

Result on 2026-05-10: `k2_wide_mimalloc_tls_cache_slot_exe_guard.sh` passes.

## Follow-Up

The next substrate row should add the atomic remote-free primitive as a separate
BoxCount card. Do not fold CAS/load/store/fetch_add into the TLS slot row.
