---
Status: Landed
Date: 2026-04-25
Scope: Repair active array-string indexOf boundary smokes by restoring MIR-owned observer metadata in their fixtures.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-237-array-string-get-metadata-fixture-sweep-card.md
  - src/mir/array_text_observer_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_indexof_observer_lowering.inc
---

# 291x-238 Array-String IndexOf Observer Fixture Repair Card

## Goal

Restore the active `phase29ck_boundary_pure_array_string_indexof_*` smokes
after the 291x-237 push/get metadata sweep exposed their next missing
contract: the fixtures still lack MIR-owned `array_text_observer_routes`.

The backend lowering already consumes `array_text_observer_routes` to defer
`array.get(i).indexOf(needle)` without re-scanning method names. The broken
smokes therefore should be repaired by making the fixtures explicit and by
checking the current stable lowering output, not by reintroducing legacy
exact-window classifiers.

## Boundary

- Add fixture metadata only for the active array-string `indexOf` boundary
  seeds and the same-slot `RuntimeDataBox.set(ArrayBox)` consumer already
  present in the `branch_live_after_get` fixture.
- Do not change `.inc` route classification or helper selection.
- Do not add method-name fallback rows.
- Do not touch `array_text_state_residence_route` or the larger
  `indexof_line_pure_min_v1` seed.

## Contract

Each repaired fixture carries one observer route:

```text
metadata.array_text_observer_routes[0]
  get_block = 0
  get_instruction_index = 6
  observer_kind = indexof
  observer_arg0_repr = const_utf8
  observer_arg0_text = line
  proof_region = array_get_receiver_indexof
  publication_boundary = none
  result_repr = scalar_i64
  keep_get_live = false
```

The smoke contract is:

```text
compile rc = 0
IR contains nyash.array.string_indexof_hih
IR does not contain nyash.array.slot_load_hi
```

The `branch_live_after_get` fixture also carries:

```text
value_consumer_facts[16].direct_set_consumer = true
generic_method.set
  core_method.op = ArraySet
  route_kind = array_store_any
  lowering_tier = cold_fallback
```

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_branch_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_select_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_cross_block_select_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_interleaved_branch_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_interleaved_select_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed in 2026-04-25 fixture/smoke repair slice.

- Added `array_text_observer_routes` metadata to six active array-string
  `indexOf` boundary fixtures.
- Added the missing direct-set consumer facts and `generic_method.set` /
  `ArraySet` metadata to the live-after-get suffix-store fixture.
- Updated the active boundary smokes to check the current metadata route
  evidence (`mir_call_method reason=indexOf ... string_indexof:1`) instead of
  retired exact-window trace tags.
- Updated the AI handoff trace contract so future repairs do not chase the
  retired exact-window tags again.
