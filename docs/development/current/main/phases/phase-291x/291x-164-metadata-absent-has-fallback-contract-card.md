---
Status: Landed
Date: 2026-04-24
Scope: Pin the metadata-absent `has` fallback contract after the rejected MapHas mirror-row prune.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-163-maphas-emit-kind-mirror-prune-card.md
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-164 Metadata-Absent Has Fallback Contract Card

## Goal

Turn the H163 rejection into an explicit cleanup contract:

```text
generic_method_policy.inc mname == "has"
  -> may not be deleted by "CoreMethod metadata exists" alone
  -> deletion also requires metadata-absent RuntimeData.has contract coverage
```

This is a docs/guard contract card. It does not change codegen behavior or
helper selection.

## Boundary

- Do not remove the `has` allowlist row.
- Do not add new classifiers.
- Do not change MapHas lowering or route metadata.
- Do not update fixtures to hide the metadata-absent fallback dependency.

## Implementation

- Tighten the `has` allowlist row deletion condition so future prune attempts
  must account for metadata-absent `RuntimeDataBox.has` boundary fixtures.

## Result

The `classify_generic_method_emit_kind` `has` allowlist row now requires:

```text
replace-with-core-method-op-id-and-metadata-absent-runtime-data-has-contract
```

This keeps the current 27-row baseline explicit while preventing a future
CoreMethod-only prune from silently breaking metadata-absent boundary fixtures.

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
