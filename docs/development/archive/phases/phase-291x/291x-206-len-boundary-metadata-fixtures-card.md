---
Status: Landed
Date: 2026-04-25
Scope: Add CoreMethod len metadata to metadata-absent length/size boundary fixtures before pruning len mirrors.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - apps/tests/mir_shape_guard/string_length_ascii_min_v1.mir.json
  - apps/tests/mir_shape_guard/runtime_data_string_length_ascii_min_v1.mir.json
  - apps/tests/mir_shape_guard/runtime_data_array_length_min_v1.mir.json
  - apps/tests/mir_shape_guard/runtime_data_map_size_min_v1.mir.json
---

# 291x-206 Len Boundary Metadata Fixtures Card

## Goal

Remove metadata-absent `len`/`length`/`size` boundary blockers before pruning
length alias mirrors.

## Boundary

- Add metadata only; do not prune `.inc` rows in this card.
- Reuse existing `ArrayLen`, `MapLen`, and `StringLen` CoreMethod vocabulary.
- Do not change expected return codes or fixture instruction order.

## Implementation

- Add `generic_method.len` metadata to:
  - direct `StringBox.length`
  - `RuntimeDataBox.length(StringBox origin)`
  - `RuntimeDataBox.length(ArrayBox origin)`
  - `RuntimeDataBox.size(MapBox origin)`

## Acceptance

```bash
cargo test -q records_runtime_data_len_from_receiver_origin
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_size_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The four boundary fixtures now carry `generic_method.len` CoreMethod metadata:
direct `StringLen`, RuntimeData `StringLen`, RuntimeData `ArrayLen`, and
RuntimeData `MapLen`. This keeps length boundary smokes green without relying
on metadata-absent method-name fallback.
