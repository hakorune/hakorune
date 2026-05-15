# 293x-372 METADATA-PROMOTE-003 Function Contract Rows

Status: landed
Date: 2026-05-15

## Decision

`METADATA-PROMOTE-003` documents and guards the active function-level metadata
rows that already behave as verifier contracts.

This is a BoxShape docs / guard row only. It does not change MIR JSON shape,
Rust metadata structs, verifier behavior, backend lowering, or runtime
behavior.

## Responsibility

Canonical wording lives in:

```text
docs/reference/mir/metadata-facts-ssot.md
```

Guard owner:

```text
tools/checks/mir_metadata_catalog_guard.sh
```

## Guarded Contract Rows

- `effect_plans`: verifier source for live `Contract(no_alloc/no_safepoint)`
  obligations. Raw `runes` remain transport/provenance after refresh.
- `inline_plans`: contract-active only for `request=required`; accepted rows
  require `verified=true`, required contracts, narrow leaf shape, and
  `fallback=fail_fast`.
- `string_kernel_plans`: verifier-visible borrow/publication/carrier/text
  consumer/stable-view facts before emitters may trust a direct-kernel row.
- `exact_numeric_runtime_check_contracts`: dynamic exact numeric range-check
  obligations tied to backend capability fail-fast gates.

## Stop Lines

- Do not make `capability_plans` verifier-active in this row.
- Do not let backends consume `Profile(...)` names.
- Do not treat advisory inline hints as backend inline mandates.
- Do not promote seed routes; legality stays with source-family plans or
  generic route rows.

## Evidence

```text
bash tools/checks/mir_metadata_catalog_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
