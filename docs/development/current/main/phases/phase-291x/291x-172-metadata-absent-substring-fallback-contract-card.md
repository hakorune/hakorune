---
Status: Landed
Date: 2026-04-24
Scope: Pin the metadata-absent `substring` fallback contract after the rejected substring mirror-row prune.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-171-core-method-substring-emit-kind-metadata-card.md
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-172 Metadata-Absent Substring Fallback Contract Card

## Goal

Turn the post-H171 prune probe into an explicit cleanup contract:

```text
generic_method_policy.inc mname == "substring"
  -> may not be deleted by "CoreMethod StringSubstring metadata exists" alone
  -> deletion also requires metadata-absent substring boundary coverage
```

This is a docs/guard contract card. It does not change codegen behavior or
string corridor/window lowering.

## Boundary

- Do not remove the `substring` allowlist row.
- Do not add new classifiers.
- Do not change substring helper symbols or corridor/window policy.
- Do not update fixtures to hide metadata-absent fallback dependencies.

## Probe

Temporary removal of the legacy `substring` emit-kind classifier kept the daily
owner smoke green:

```bash
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_substring_concat_loop_min.sh
```

But the metadata-absent pure boundary fixture failed with
`unsupported pure shape for current backend recipe`:

```bash
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_substring_concat_loop_min.sh
```

## Implementation

- Keep the legacy `substring` classifier in place.
- Tighten the `substring` allowlist deletion condition so future prune attempts
  must account for metadata-absent substring boundary fixtures.

## Result

The `classify_generic_method_emit_kind` `substring` allowlist row now requires:

```text
replace-with-core-method-op-id-and-metadata-absent-substring-boundary-contract
```

This preserves the current 27-row baseline and prevents a CoreMethod-only prune
from breaking metadata-absent substring fixtures.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_substring_concat_loop_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
