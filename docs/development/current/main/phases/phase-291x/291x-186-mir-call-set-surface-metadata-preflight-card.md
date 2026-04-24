---
Status: Landed
Date: 2026-04-25
Scope: Preflight pruning the MIR-call route-policy `set` method surface mirror row.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-182-core-method-set-storage-route-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-185-runtime-data-set-fallback-contract-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-186 MIR-Call Set Surface Metadata Preflight Card

## Goal

Prepare a focused probe for this route-policy mirror row:

```c
if (!strcmp(mname, "set")) {
  return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SET;
}
```

The set emit-kind and set storage-route consumers now read
`generic_method_routes` CoreMethod metadata for direct ArraySet/MapSet. This
makes the MIR-call route-policy `set` surface row a deletion candidate, but it
must be tested separately from the generic-method fallback rows.

## Boundary

- Do not remove `classify_generic_method_emit_kind(... method == "set")`.
- Do not remove `classify_generic_method_set_route(...)` MapBox or
  RuntimeDataBox fallback branches.
- Do not change `RuntimeDataBox.set` metadata-absent behavior.
- Do not change helper symbols or lowering.
- Keep ArrayBox value-shape discrimination in the generic-method set route
  consumer.
- Treat this as a route-policy surface cleanup only; no semantic expansion.

## Probe Shape

The next probe may remove only:

```text
classify_mir_call_method_surface method set
```

Acceptance must include direct ArrayBox, direct MapBox, and RuntimeData set
boundary checks. If any metadata-absent set boundary reaches this row, reject
the prune and pin the fallback condition.

## Result

The next implementation target is a one-row prune probe for the MIR-call
route-policy `set` method surface mirror.

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
