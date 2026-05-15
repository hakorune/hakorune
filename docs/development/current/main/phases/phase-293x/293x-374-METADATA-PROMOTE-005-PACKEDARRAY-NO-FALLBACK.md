# 293x-374 METADATA-PROMOTE-005 PackedArray No-Fallback

Status: landed
Date: 2026-05-15

## Decision

`METADATA-PROMOTE-005` documents and guards the PackedArray no-fallback
contract before any packed-record backend lowering flag is enabled.

This is a BoxShape docs / guard row only. It does not change MIR JSON shape,
Rust metadata structs, verifier behavior, backend lowering, runtime behavior,
or source PackedArray acceptance.

## Responsibility

Canonical wording lives in:

```text
docs/reference/mir/metadata-facts-ssot.md
```

Guard owner:

```text
tools/checks/mir_metadata_catalog_guard.sh
```

Existing row-local guards remain the behavioral anchors:

```text
tools/checks/k2_wide_source_packed_array_autouse_pilot_guard.sh
tools/checks/k2_wide_packed_record_backend_failfast_guard.sh
```

## Guarded Contract

- A source `PackedArray<T>` declaration is a storage requirement, not an
  optimizer hint.
- Unsupported packed routes must fail fast; they must not silently fall back to
  ordinary `Array<T>` / `ArrayBox` materialization.
- Source PackedArray pilot and direct-read rows must keep
  `boxed_fallback_enabled=false`, `public_array_get_materialization_enabled=false`,
  and `backend_lowering_enabled=false` until a proof-bearing backend row lands.
- Enabling `backend_lowering_enabled=true` later requires a direct-read proof,
  the shared packed-record backend capability gate, and
  `silent_fallback_allowed=false`.
- hako_alloc packed-store pilot rows remain verifier-active only and keep live
  scalar columns as the storage truth until a real storage owner exists.

## Stop Lines

- Do not use ordinary `ArrayBox` storage as an implicit fallback for declared
  `PackedArray<T>`.
- Do not enable packed backend lowering from source text alone.
- Do not combine this row with allocator behavior changes.

## Evidence

```text
bash tools/checks/mir_metadata_catalog_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
