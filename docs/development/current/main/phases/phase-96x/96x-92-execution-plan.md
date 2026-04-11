---
Status: Active
Date: 2026-04-11
Scope: turn phase-96x into an execution-order document with explicit cutover waves, anchor gaps, and late-demotion rules.
Related:
  - docs/development/current/main/phases/phase-96x/README.md
  - docs/development/current/main/phases/phase-96x/96x-90-vm-hako-llvm-cutover-ssot.md
  - docs/development/current/main/phases/phase-96x/96x-91-task-board.md
  - tools/smokes/v2/suites/integration/vm-hako-caps.txt
  - tools/smokes/v2/profiles/integration/vm_hako_caps/gate/phase29y_vm_hako_caps_gate_vm.sh
  - tools/smokes/v2/suites/integration/collection-core.txt
  - tools/smokes/v2/suites/integration/presubmit.txt
---

# 96x-92 Execution Plan

## Rule

- cut faster by separating product-visible rows, seam sentinels, and indirect-live bridge rows
- do not let `mapbox` or APP-1 seam rows slow the first LLVM product cutover wave
- keep `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh` as one retirement object

## Immediate Read

- `args_vm.sh` is retired from the live vm_hako suite/gate pair; `phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` is the explicit green replacement anchor
- `boxcall_args_gt1_ported_vm.sh` is not a pure args row; it executes APP-1 (`gate_log_summarizer/main.hako`) and belongs to the late APP/seam track
- `mapbox/*` is not phase29y-live; live suite ownership is now bridged through `collection_core/mapbox_*`
- `app1_summary_contract_ported_vm.sh` is still referenced by `presubmit.txt`
- `argv_multiline_roundtrip.sh` remains a narrow future candidate, but it is not the current keeper because it is red in this tree

## Fastest Order

### Wave 1a: product-visible live rows

Goal: remove the narrowest daily product-facing vm_hako rows first.

Landed:
- `96xC1a`: `args_vm.sh` retired from `phase29y_vm_hako_caps_gate_vm.sh`, `vm-hako-caps.txt`, and `vm-hako-core.txt`
- replacement anchor: `tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
- `96xC1b`: `env_get_ported_vm.sh` retired from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh`
- product owner: `tools/smokes/v2/profiles/integration/core/phase2035/v1_extern_env_get_canary_vm.sh` via `tools/smokes/v2/suites/integration/presubmit.txt`
- monitor keep: `tools/smokes/v2/profiles/integration/vm_hako_caps/env/env_get_ported_vm.sh` remains only in `tools/smokes/v2/suites/integration/vm-hako-core.txt`
- `96xC1c`: `filebox_newbox_vm.sh` retired from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh`
- replacement anchor: `tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg04_filebox_pilot_vm.sh` (green standalone anchor; suite promotion can stay separate)
- `96xC1d`: `file_read_ported_vm.sh` and `file_close_ported_vm.sh` retired from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh`
- replacement anchors:
  - `tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_hako_route_vm.sh`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_retire_execution_lock_vm.sh`
- monitor keep:
  - `tools/smokes/v2/profiles/integration/vm_hako_caps/file/file_read_ported_vm.sh`
  - `tools/smokes/v2/profiles/integration/vm_hako_caps/file/file_close_ported_vm.sh`
  - both remain only in `tools/smokes/v2/suites/integration/vm-hako-core.txt`
- `96xC1e`: `file_error_vm.sh` retired from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh`
- replacement anchor: `tools/smokes/v2/profiles/integration/apps/phase96x_filebox_missing_open_vm.sh`

Rows:
- none; wave `1a` owner rows are complete

Exact order:
1. complete

Current live refs:
- `vm-hako-core.txt` still owns the monitor canary `env_get_ported_vm.sh` plus the FileBox monitor rows `file_read_ported_vm.sh` and `file_close_ported_vm.sh`
- `phase29y_vm_hako_caps_gate_vm.sh` no longer runs any wave `1a` rows
- `presubmit.txt` now owns `core/phase2035/v1_extern_env_get_canary_vm.sh`

Exact replacement anchors:
- `file_read_ported_vm.sh` -> `tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_hako_route_vm.sh`
- `file_close_ported_vm.sh` -> `tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_retire_execution_lock_vm.sh`
- `file_error_vm.sh` -> `tools/smokes/v2/profiles/integration/apps/phase96x_filebox_missing_open_vm.sh`

Cutover rule:
- every wave 1a retirement commit must update `phase29y_vm_hako_caps_gate_vm.sh`, `vm-hako-caps.txt`, and `vm-hako-core.txt` together when the row is still present in those artifacts

Required tasks:
1. wave `1a` complete

### Wave 1b: narrow single-purpose witnesses

Goal: remove the narrow rows that are not the product face, but still consume gate budget.

Rows:
- none; wave `1b` owner rows are complete
- landed `96xC2a`:
  - `vm_hako_caps/compare/compare_ported_vm.sh` -> live owner retired from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh`; monitor keep remains in `vm-hako-core.txt`
  - `vm_hako_caps/atomic/atomic_fence_ported_vm.sh` -> owner anchor is `tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh` via `tools/smokes/v2/suites/integration/presubmit.txt`
  - `vm_hako_caps/tls/tls_last_error_ported_vm.sh` -> owner anchor is `tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh` via `tools/smokes/v2/suites/integration/presubmit.txt`
- landed `96xC2b`:
  - `vm_hako_caps/compare/compare_ge_ported_vm.sh` -> explicit archive-only evidence at `tools/smokes/v2/profiles/archive/vm_hako_caps/compare/compare_ge_ported_vm.sh`
- landed `96xC2c`:
  - `vm_hako_caps/misc/const_void_ported_vm.sh` -> explicit archive-only evidence at `tools/smokes/v2/profiles/archive/vm_hako_caps/misc/const_void_ported_vm.sh`

Exact order:
1. complete

Required tasks:
1. landed `96xC2a`: retire the rows with explicit non-vm_hako anchors already present
2. landed `96xC2b`: archive-only closure for `compare_ge_ported_vm.sh` because no concrete non-vm_hako live owner exists
3. landed `96xC2c`: archive-only closure for `const_void_ported_vm.sh` because no concrete non-vm_hako live owner exists
4. wave `1b` complete

### Wave 2: seam shadow and APP-1 late lane

Goal: keep seam-sensitive rows out of wave 1 so product retirement does not stall on compiler/backend coupling.

Rows:
- `vm_hako_caps/select_emit/select_emit_block_vm.sh`
- `vm_hako_caps/open_handle_phi/open_handle_phi_ported_vm.sh`
- `vm_hako_caps/args/boxcall_args_gt1_ported_vm.sh`
- `vm_hako_caps/app1/app1_stack_overflow_after_open_ported_vm.sh`
- `vm_hako_caps/app1/app1_summary_contract_ported_vm.sh`

Anchor direction:
- `select_emit` -> `phase29y-hako-emit-mir.txt`
- `open_handle_phi` -> `joinir-bq.txt` or `selfhost-core.txt`
- `boxcall_args_gt1` -> APP-1/open-handle seam lane, not runtime-data wave 1a
- `app1_summary_contract_ported_vm.sh` -> late demotion only after `presubmit.txt` stops depending on it

Required tasks:
1. landed `96xC3a`: keep `select_emit` as shadow because `phase29y-hako-emit-mir.txt` does not yet pin the exact `select` emit+exec contract
2. landed `96xC3b`: keep `open_handle_phi` as shadow because `joinir-bq.txt` / `selfhost-core.txt` do not yet pin the exact `FileBox.open` handle-propagation seam contract
3. split `boxcall_args_gt1` out of the generic args retirement narrative
4. replace `app1_summary_contract_ported_vm.sh` in `presubmit.txt`
5. only then demote the APP-1 rows

### Parallel Track: mapbox re-home

Goal: remove `mapbox` from `vm_hako_caps` ownership without waiting for the product-live waves.

Live rows in `collection-core.txt`:
- `mapbox_set_ported_vm.sh`
- `mapbox_get_ported_vm.sh`
- `mapbox_has_ported_vm.sh`
- `mapbox_delete_ported_vm.sh`
- `mapbox_keys_ported_vm.sh`
- `mapbox_clear_ported_vm.sh`
- `mapbox_size_ported_vm.sh`

Fastest ownership move:
1. add `collection_core/mapbox_*` wrapper rows
2. point `collection-core.txt` at those wrappers
3. move the real implementations later

Current state:
- landed: `collection_core/mapbox_*` owner rows now carry the live implementations
- landed: `collection-core.txt` retarget to `collection_core/mapbox_*`
- landed: archive mirror copies for the 6 non-live rows exist under `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/`
- next: leave the old `vm_hako_caps/mapbox/*` tree untouched until the eventual final cleanup, then retire the `collection_core/` owner rows after LLVM coverage exists

After the wrapper move:
1. landed: archive the non-live `mapbox` rows (`*_bad_key*`, `*_missing*`, `*_getfield*`, `*_setfield*`, `mapbox_newbox_ported_vm.sh`)
2. leave the old `vm_hako_caps/mapbox/*` tree untouched as temporary mirror/source until the eventual cleanup step
3. retire the collection-core owner rows after LLVM collection/runtime-data coverage replaces them

Risks:
- the 7 live rows depend on `vm_hako_caps_common.sh`
- fixture paths still point at `apps/tests/vm_hako_caps/*`
- the wrapper move is the low-risk first step because it cuts suite ownership before helper surgery
- the current `vm_hako_caps/mapbox/*` live rows already carry active uncommitted edits, so the physical move must preserve that content rather than overwrite it with stale HEAD copies

## Next Commit Candidates

1. `96xC3c`: close `boxcall_args_gt1_ported_vm.sh` through the APP-1/open-handle seam lane instead of treating it as a generic args row
2. `96xD1`: resolve the late `app1` demotion blocker in `presubmit.txt`
3. `mapbox`: retire the `collection_core/` owner rows after LLVM collection/runtime-data coverage replaces them
