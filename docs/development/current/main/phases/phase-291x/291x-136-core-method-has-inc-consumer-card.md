---
Status: Landed
Date: 2026-04-24
Scope: Make the generic-method `has` `.inc` consumer prefer CoreMethodOp metadata.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-135-core-method-op-carrier-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
---

# 291x-136 CoreMethod `has` `.inc` Consumer Card

## Goal

Land HCM-5 for one small family: make the generic-method `has` `.inc`
metadata consumer prefer MIR-owned CoreMethodOp metadata when present, while
keeping the existing compatibility route_kind fallback.

First consumed op:

```text
generic_method_routes[*].core_method.op = MapHas
```

## Implementation

- Added `generic_method_has_route_kind_from_core_method_metadata(...)`.
- The `has` metadata matcher now resolves route kind from `core_method` when
  the carrier is present.
- If `core_method` is absent or JSON null, the old `route_kind` metadata path is
  unchanged.
- Malformed or unknown `core_method` metadata is fail-fast invalid metadata.

## Boundary

- Only the `has` metadata consumer moved.
- `ArrayBox.has` / `RuntimeDataBox.has` continue through the compatibility
  route_kind path because they do not have CoreMethodContract rows yet.
- No new hot inline lowering is added.
- No method-name classifier is added to the generic method policy mirror.

## Proof

```bash
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Next

- HCM-6: make LoweringTier metadata consumption explicit enough to separate
  warm direct ABI from later hot inline candidates.
