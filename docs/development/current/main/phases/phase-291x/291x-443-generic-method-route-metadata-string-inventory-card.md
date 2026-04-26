---
Status: Landed
Date: 2026-04-27
Scope: GenericMethodRoute metadata string inventory
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-442-next-lane-selection-card.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
---

# 291x-443: Generic Method Route Metadata String Inventory

## Goal

Inventory the remaining method-string-derived metadata on
`GenericMethodRoute` before code edits.

This is a BoxShape inventory. No behavior changed.

## Current Read

`GenericMethodRoute` still stores compatibility fields:

- `box_name`
- `method`

Those fields are still useful as JSON/debug compatibility and as a bridge for
non-migrated readers.

The cleaner issue is narrower: exported metadata helpers derive tokens from
`method`:

- `route_id()`
- `emit_kind()`
- `effect_tags()`

These tokens are consumed by JSON emission and `.inc` readers. Their spelling
must not change in this lane.

## Consumer Constraints

| Token | Current consumer | Constraint |
| --- | --- | --- |
| `route_id` | JSON tests and `.inc` route readers | keep existing strings |
| `emit_kind` | `.inc` generic method dispatch | keep existing strings |
| `effects` | JSON metadata/debug consumers | keep existing strings |
| `method` | compatibility/debug field | keep for now |
| `box_name` | compatibility/debug field | keep for now |

## Decision

The next code slice should move token derivation from `method` to
`route_kind`.

Why `route_kind` first:

- every current generic route already has a decided `GenericMethodRouteKind`
- `route_kind` maps directly to helper/route semantics without reading raw
  method spelling
- the slice keeps JSON spellings unchanged
- it avoids `.inc` behavior changes while reducing MIR-side policy mirror
  pressure

`core_method` remains the long-term semantic owner for generated consumers, but
not every compatibility route has a `core_method` carrier yet. Use
`route_kind` for this bounded cleanup and leave `core_method` consumer
migration to later cards.

## Next Cleanup Slice

Use `291x-444-generic-method-route-kind-metadata-helper`:

- add `route_id()`, `emit_kind()`, and `effect_tags()` helpers on
  `GenericMethodRouteKind`
- make `GenericMethodRoute` delegate to `route_kind`
- keep all JSON strings unchanged
- add a small test that proves these exported tokens do not depend on the raw
  `method` field
- do not touch `.inc`

## Guards

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
