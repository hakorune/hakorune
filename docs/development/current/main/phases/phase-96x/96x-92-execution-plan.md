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
- `boxcall_args_gt1_ported_vm.sh` was not a pure args row; it executed APP-1 (`gate_log_summarizer/main.hako`) and is now retired from active suite/gate ownership
- `mapbox/*` is not phase29y-live; live suite ownership is no longer bridged through `collection_core/`
- `apps/gate_log_summarizer_vm.sh` is now the product owner in `presubmit.txt`
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

Anchor direction:
- `select_emit` -> `phase29y-hako-emit-mir.txt`
- `open_handle_phi` -> `phase29y-hako-emit-mir.txt` / `selfhost-core.txt`
- `boxcall_args_gt1` -> retired from active ownership; no exact non-vm_hako seam replacement exists yet
- `app1_summary_contract_ported_vm.sh` -> retired from active ownership after `presubmit.txt` moved to `apps/gate_log_summarizer_vm.sh`

Required tasks:
1. landed `96xC3a`: keep `select_emit` as a temporary shadow until `96xC3e` adds the exact non-vm_hako emit+exec owner
2. landed `96xC3b`: keep `open_handle_phi` as a temporary non-blocking shadow until `96xC3f` adds the exact non-vm_hako emit+exec owner
3. landed `96xC3c`: remove `boxcall_args_gt1_ported_vm.sh` from `vm-hako-caps.txt`, `phase29y_vm_hako_caps_gate_vm.sh`, and `vm-hako-core.txt`
4. landed `96xC3d`: replace `app1_summary_contract_ported_vm.sh` in `presubmit.txt` with `apps/gate_log_summarizer_vm.sh`
5. landed `96xD1`: demote the APP-1 vm_hako rows from the active vm_hako suite/gate pair
6. landed `96xC3e`: add `phase29y_hako_emit_mir_select_exec_contract_vm.sh` as the exact non-vm_hako emit+exec owner in `phase29y-hako-emit-mir.txt` and `selfhost-core.txt`, then retire `select_emit` from `phase29y_vm_hako_caps_gate_vm.sh`
7. landed `96xC3f`: add `phase29y_hako_emit_mir_open_handle_phi_exec_contract_vm.sh` as the exact non-vm_hako emit+exec owner in `phase29y-hako-emit-mir.txt` and `selfhost-core.txt`, then retire `open_handle_phi` from `vm-hako-core.txt`

### Parallel Track: mapbox re-home

Goal: remove `mapbox` from `vm_hako_caps` ownership without waiting for the product-live waves.

Live rows in `collection-core.txt`:
- none

Fastest ownership move:
1. add `collection_core/mapbox_*` wrapper rows
2. point `collection-core.txt` at those wrappers
3. move the real implementations later
4. replace those bridge rows with dedicated non-vm_hako emit+exec owners

Current state:
- landed: `collection_core/mapbox_*` owner rows carried the live implementations during the bridge step
- landed: `collection-core.txt` was retargeted to `collection_core/mapbox_*` during the bridge step
- landed: archive mirror copies for the 6 non-live rows exist under `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/`
- landed: `MapBox.clear`, `MapBox.delete`, and `MapBox.keys` now have dedicated non-vm_hako emit+exec owners under `phase29y/hako/emit_mir/`
- landed: `MapBox.set`, `MapBox.get`, `MapBox.has`, and `MapBox.size` now also have dedicated non-vm_hako emit+exec owners under `phase29y/hako/emit_mir/`
- landed: the old `collection_core/mapbox_*_ported_vm.sh` bridge scripts are archived under `tools/smokes/v2/profiles/archive/collection_core/`
- landed: the live `vm_hako_caps/mapbox/*` mirror tree has been removed; archive evidence remains under `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/`
- current bridge residue: none

After the wrapper move:
1. landed: archive the non-live `mapbox` rows (`*_bad_key*`, `*_missing*`, `*_getfield*`, `*_setfield*`, `mapbox_newbox_ported_vm.sh`)
2. keep the archive copies only; the live `vm_hako_caps/mapbox/*` tree has already been removed
3. retire the collection-core owner rows after LLVM collection/runtime-data coverage replaces them

Risks:
- no live `collection_core` bridge row remains; all active owners now sit in `phase29y/hako/emit_mir/`
- fixture paths still point at `apps/tests/vm_hako_caps/*`
- the wrapper move is the low-risk first step because it cuts suite ownership before helper surgery
- the mapbox archive copies are the only remaining source of truth for retired rows

## Next Commit Candidates

1. `mirror families`: move the remaining offloaded `vm_hako_caps/**` families (`app1/`, `args/`, `atomic/`, `tls/`, `select_emit/`, `open_handle_phi/`, `file_error_vm.sh`, `filebox_newbox_vm.sh`) into archive or owner-local homes
2. `docs`: phase96x parked closeout sync now that the monitor bundle wording is settled
3. `hold`: keep the frozen `vm-hako-core` 4-row monitor bundle unchanged unless a replacement owner appears
