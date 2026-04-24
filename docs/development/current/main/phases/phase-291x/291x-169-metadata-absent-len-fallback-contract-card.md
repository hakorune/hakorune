---
Status: Landed
Date: 2026-04-24
Scope: Pin the metadata-absent `len`/`length`/`size` fallback contract after the rejected length mirror-row prune.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-168-core-method-len-emit-kind-metadata-card.md
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-169 Metadata-Absent Len Fallback Contract Card

## Goal

Turn the post-H168 prune probe into an explicit cleanup contract:

```text
generic_method_policy.inc mname in {len,length,size}
  -> may not be deleted by "CoreMethod Len metadata exists" alone
  -> deletion also requires metadata-absent length boundary coverage
```

This is a docs/guard contract card. It does not change codegen behavior,
helper selection, or length lowering.

## Boundary

- Do not remove the `len`/`length`/`size` allowlist rows.
- Do not add new classifiers.
- Do not change `nyash.array.slot_len_h`, `nyash.map.entry_count_i64`, or
  `nyash.string.len_h` selection.
- Do not update fixtures to hide metadata-absent fallback dependencies.

## Probe

Temporary removal of the legacy length alias emit-kind classifier kept the
daily owner smokes green:

```bash
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_size_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_length_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_length_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_length_min.sh
```

But metadata-absent pure boundary fixtures failed with
`unsupported pure shape for current backend recipe`:

```bash
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_size_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
```

## Implementation

- Keep the legacy `len`/`length`/`size` classifier in place.
- Tighten the three length alias allowlist deletion conditions so future prune
  attempts must account for metadata-absent length boundary fixtures.

## Result

The `classify_generic_method_emit_kind` `len`/`length`/`size` allowlist rows now
require:

```text
replace-with-core-method-op-id-and-metadata-absent-len-boundary-contract
```

This preserves the current 27-row baseline and prevents a CoreMethod-only prune
from breaking metadata-absent length fixtures.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_size_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
