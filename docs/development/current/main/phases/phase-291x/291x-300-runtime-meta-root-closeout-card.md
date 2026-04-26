---
Status: Landed
Date: 2026-04-26
Scope: runtime/meta root closeout
Related:
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/meta/hako_module.toml
  - lang/src/runtime/meta/core_method_contract_box.hako
  - lang/src/runtime/meta/generated/core_method_contract_manifest.json
  - lang/src/runtime/meta/support/json_shape_parser.hako
---

# 291x-300: runtime/meta Root Closeout

## Goal

Close the `runtime/meta` cleanup series with a local invariant:

```text
runtime/meta root = semantic contract owner entry + generated artifact pointer
runtime/meta/support = active compatibility / fixture support exports
```

This is BoxShape cleanup only. It does not change exports or behavior.

## Root Inventory

Current root files:

```text
lang/src/runtime/meta/README.md
lang/src/runtime/meta/hako_module.toml
lang/src/runtime/meta/core_method_contract_box.hako
lang/src/runtime/meta/generated/core_method_contract_manifest.json
lang/src/runtime/meta/support/json_shape_parser.hako
```

## Decision

The only live compiler semantic contract table under `runtime/meta` is:

```text
CoreMethodContractBox
```

The generated manifest is derived data, not an owner:

```text
generated/core_method_contract_manifest.json
```

`JsonShapeToMap` is intentionally kept as an active support export under:

```text
support/json_shape_parser.hako
```

Retired stale exports:

```text
MirCallRoutePolicy
MirCallNeedPolicy
MirCallSurfacePolicy
UsingResolver
UsingDecision
```

## Invariant

New root-level `.hako` files in `lang/src/runtime/meta/` must be semantic
contract tables or explicit root facades. Support utilities belong under
`support/` and need an owner-audit card plus a caller-count retirement
condition.

## Acceptance

```bash
find lang/src/runtime/meta -maxdepth 2 -type f | sort
bash tools/checks/module_registry_hygiene_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
