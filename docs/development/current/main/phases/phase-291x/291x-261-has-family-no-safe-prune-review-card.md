---
Status: Active
Date: 2026-04-26
Scope: Close the remaining `has` family cleanup review with exact metadata-absent boundary evidence.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-164-metadata-absent-has-fallback-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-211-runtime-data-has-compat-contract-design-card.md
  - docs/development/current/main/phases/phase-291x/291x-244-mir-call-has-surface-review-card.md
  - docs/development/current/main/phases/phase-291x/291x-259-mir-call-mapbox-has-need-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-260-mir-call-need-dead-surface-fallback-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-261 Has Family No-Safe-Prune Review Card

## Goal

Finish the post-291x-259 `has` family cleanup review and decide whether any
remaining tracked `has` classifier row can be pruned.

## Remaining Rows

```text
classify_generic_method_emit_kind method has
classify_generic_method_has_route box ArrayBox
classify_generic_method_has_route box RuntimeDataBox
classify_mir_call_method_surface method has
```

## Probe Results

Baseline metadata-absent boundaries are still accepted:

```text
ArrayBox.has(index), generic_method_routes=[]
RuntimeDataBox.has(ArrayBox origin), generic_method_routes=[]
```

Both emit objects through the compatibility fallback.

### `mname == "has"` Emit-Kind

Temporary prune:

```text
remove classify_generic_method_emit_kind method has
```

Result for metadata-absent direct `ArrayBox.has`:

```text
bname=ArrayBox mname=has ... map_has:0
lane=none reason=unsupported_pure_shape
```

### `ArrayBox` Has-Route

Temporary prune:

```text
remove classify_generic_method_has_route box ArrayBox
```

Result for metadata-absent direct `ArrayBox.has`:

```text
bname=ArrayBox mname=has ... map_has:0
lane=none reason=unsupported_pure_shape
```

### `RuntimeDataBox` Has-Route

Temporary prune:

```text
remove classify_generic_method_has_route box RuntimeDataBox
```

Result for metadata-absent `RuntimeDataBox.has(ArrayBox origin)`:

```text
bname=RuntimeDataBox mname=has ... map_has:0
lane=none reason=unsupported_pure_shape
```

### MIR-Call Method Surface `has`

Temporary prune:

```text
remove classify_mir_call_method_surface method has
```

Result for metadata-absent direct `MapBox.has`:

```text
bname=MapBox mname=has ... map_has:0
lane=none reason=unsupported_pure_shape
```

## Decision

No safe prune for the remaining `has` family rows.

The only safe code cleanup in this pass was:

- 291x-259: prune redundant `MapBox.has` need-policy branch
- 291x-260: prune the dead need-policy method-surface fallback call

The remaining rows stay pinned until one of these structural changes lands:

- `ArrayHas` CoreMethod contract covers direct `ArrayBox.has` and
  Array-origin `RuntimeDataBox.has`
- metadata-absent direct `MapBox.has` boundaries are retired or covered by a
  non-surface route contract
- the legacy generic `has` fallback path is retired entirely

## Next Work

Proceed to the next task-order family:

```text
len family cleanup
```

## Acceptance

```bash
cargo check -q
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
