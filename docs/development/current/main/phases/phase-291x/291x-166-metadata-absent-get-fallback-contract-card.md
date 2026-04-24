---
Status: Landed
Date: 2026-04-24
Scope: Pin the metadata-absent `get` fallback contract after the rejected MapGet mirror-row prune.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-165-core-method-mapget-emit-kind-metadata-card.md
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-166 Metadata-Absent Get Fallback Contract Card

## Goal

Turn the post-H165 prune probe into an explicit cleanup contract:

```text
generic_method_policy.inc mname == "get"
  -> may not be deleted by "CoreMethod MapGet metadata exists" alone
  -> deletion also requires metadata-absent RuntimeData.get contract coverage
```

This is a docs/guard contract card. It does not change codegen behavior,
helper selection, or MapGet return-shape policy.

## Boundary

- Do not remove the `get` allowlist row.
- Do not add new classifiers.
- Do not change `nyash.runtime_data.get_hh`, map get, or array get helper
  selection.
- Do not update fixtures to hide the metadata-absent fallback dependency.

## Probe

Temporary removal of the legacy `mname == "get"` emit-kind classifier showed
that the row is still required:

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q generic_method_routes
env NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_vm.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_get_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_get_min.sh
```

The build and route unit passed, but the focused smokes failed with
`unsupported pure shape for current backend recipe`. The metadata-absent
boundary fixtures still rely on the legacy `get` fallback.

## Implementation

- Keep the legacy `mname == "get"` classifier in place.
- Tighten the `get` allowlist row deletion condition so future prune attempts
  must account for metadata-absent `RuntimeDataBox.get` boundary fixtures.

## Result

The `classify_generic_method_emit_kind` `get` allowlist row now requires:

```text
replace-with-core-method-op-id-and-metadata-absent-runtime-data-get-contract
```

This preserves the current 27-row baseline and prevents a CoreMethod-only
prune from breaking metadata-absent `RuntimeDataBox.get` and array get boundary
fixtures.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_get_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_get_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
