# 293x-229 C212 Packed Record Backend Fail-Fast Hardening

Status: Complete

## Purpose

C212 adds the backend capability gate for packed record routes. It prevents
future packed record lowering rows from silently falling back to boxed record or
legacy `ArrayBox` behavior on unsupported backends.

## Decision

Decision: accepted.

Add a MIR-owned backend capability wrapper:

```text
mir::backend_capability::enforce_mir_backend_supported(...)
```

The wrapper runs existing exact numeric backend checks and the new packed record
backend check from one entry point. Backend call sites should use this wrapper
instead of calling the exact numeric checker directly.

C212 does not enable packed record backend lowering. C209-C211 still emit
metadata with `backend_lowering_enabled = false`, so existing backend behavior
stays unchanged. The fail-fast gate is active for future rows that explicitly
set packed record backend lowering required.

## Row Contract

C212 defines:

```text
unsupported_tag = [freeze:backend][array-record/packed-route-unsupported]
consumer_capability = arraybox.inline_record_columns_v0
silent_fallback_allowed = false
vm_only_completion_allowed = false
```

Fail-fast applies only to packed record plans that explicitly require backend
lowering:

```text
backend_lowering_enabled = true
```

## Stop Lines

- Do not implement packed record backend lowering in this row.
- Do not turn C209-C211 metadata into required backend routes.
- Do not allow silent boxed fallback.
- Do not claim VM-only success as completion.
- Do not add `.inc` allocator/provider/mimalloc name matching.
- Do not touch provider activation, hooks, or process allocator replacement.

## Acceptance

- Backend gate call sites use `enforce_mir_backend_supported(...)`.
- The packed record checker reports zero required routes for C209-C211 default
  plans.
- A test-only required packed route fails on unsupported backend with the C212
  stable tag.
- A supported reference backend accepts the same required route.
- C212 guard stays local-run / index-listed and is not added to quick/dev gates.

## Verification

```bash
bash tools/checks/k2_wide_packed_record_backend_failfast_guard.sh
cargo test -q mir::array_record_backend_capability
cargo test -q mir::backend_capability
```
