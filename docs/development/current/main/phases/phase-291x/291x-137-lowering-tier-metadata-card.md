---
Status: Landed
Date: 2026-04-24
Scope: Make CoreMethod LoweringTier explicit contract metadata without adding hot lowering.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - lang/src/runtime/meta/core_method_contract_box.hako
  - lang/src/runtime/meta/generated/core_method_contract_manifest.json
  - lang/c-abi/shims/hako_llvmc_ffi_core_method_metadata.inc
  - src/mir/core_method_op.rs
---

# 291x-137 LoweringTier Metadata Card

## Goal

Land HCM-6: make LoweringTier a first-class metadata field that later
optimization cards can point at, without changing backend behavior or adding
hot inline lowering.

## Implementation

- Renamed the CoreMethodContract manifest field from `hot_lowering` to
  `lowering_tier`.
- Regenerated `core_method_contract_manifest.json`.
- Added MIR-side parsing helpers for `CoreMethodLoweringTier` manifest tokens.
- Added a shared `.inc` metadata reader for `core_method.proof` and
  `core_method.lowering_tier`.
- Updated the generic-method `has` consumer to use the shared tier reader
  instead of local string checks.

## Boundary

- No `hot_inline` tier is introduced.
- No helper call is inlined.
- No new CoreBox method surface is added.
- `ArrayBox.has` / `RuntimeDataBox.has` compatibility fallback is unchanged.

## Proof

```bash
python3 tools/core_method_contract_manifest_codegen.py --write
bash tools/checks/core_method_contract_manifest_guard.sh
cargo test -q core_method
cargo test -q build_mir_json_root_emits_generic_method_routes
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Next

- HCM-7 remains evidence-gated: run owner-first perf/asm before any hot inline
  lowering.
