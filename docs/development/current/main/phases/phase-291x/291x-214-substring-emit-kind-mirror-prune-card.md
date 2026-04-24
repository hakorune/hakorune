---
Status: Landed
Date: 2026-04-25
Scope: Prune the generic-method substring emit-kind method-name mirror after boundary metadata landed.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-213-substring-boundary-metadata-fixtures-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-214 Substring Emit-Kind Mirror Prune Card

## Goal

Remove the legacy emit-kind classifier:

```c
if (mname && !strcmp(mname, "substring") && route && route->runtime_string) {
  return HAKO_LLVMC_GENERIC_METHOD_EMIT_SUBSTRING;
}
```

Substring emit-kind selection should now come from
`generic_method_routes[].core_method.op = StringSubstring`.

## Boundary

- Prune only the generic emit-kind `method substring` row.
- Do not prune MIR-call route-surface `substring` in this card.
- Do not change substring lowering or helper symbols.
- Do not remove `EMIT_SUBSTRING` itself.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_substring_concat_loop_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_substring_concat_loop_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The legacy generic-method `substring` emit-kind method-name classifier is gone.
Substring emit-kind selection now depends on `StringSubstring` CoreMethod
metadata. The MIR-call route-surface `substring` row remains for the next card.
The no-growth baseline dropped from `classifiers=14 rows=14` to
`classifiers=13 rows=13`.
