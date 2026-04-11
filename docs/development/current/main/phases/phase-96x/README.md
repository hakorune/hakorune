---
Status: Active
Decision: provisional
Date: 2026-04-11
Scope: cut vm_hako acceptance over to LLVM-line replacement families and shrink vm_hako to monitor-only keep.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-96x/96x-92-execution-plan.md
  - docs/development/current/main/phases/phase-29y/README.md
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
  - docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md
  - docs/development/current/main/phases/phase-29y/82-VM-HAKO-BOXCALL-CONTRACT-SSOT.md
  - docs/development/current/main/phases/phase-29x/29x-98-legacy-route-retirement-investigation-ssot.md
  - tools/smokes/v2/profiles/integration/vm_hako_caps/README.md
  - tools/smokes/v2/profiles/integration/phase29ck_boundary/README.md
---

# Phase 96x: vm_hako LLVM Acceptance Cutover

## Goal

- phase-cut the current vm_hako acceptance family into LLVM-line replacement families
- keep vm_hako as reference/monitor-only while the replacement smoke matrix is pinned
- retire the active vm_hako capability gate without adding new shim debt

## Why This Phase

- the current vm_hako capability gate is explicit and stable, but it is also the highest-maintenance reference surface
- LLVM boundary/proof families already exist for the same semantic space
- the right move is to cut acceptance over, not to extend vm_hako coverage

## Fixed Policy

1. LLVM is the product acceptance line.
2. vm_hako stays reference/conformance only.
3. Do not add new vm_hako capability rows.
4. Keep one explicit monitor canary until the LLVM replacement set is green.
5. Do not mix this corridor with rust-vm residual keep or current optimization work.

## Big Tasks

1. `96xA1` vm_hako acceptance inventory lock
2. `96xA2` LLVM replacement inventory lock
3. `96xB1` replacement ranking and canary freeze
4. `96xB2` first cutover wave split
5. `96xC1` first product-visible LLVM cutover wave
6. `96xC2` vm_hako semantic seam shadow policy
7. `96xC3` mapbox re-home track
8. `96xD1` proof / closeout

## Current Read

- the phase29y vm_hako gate is now a retired compatibility stub; `select_emit`, `open_handle_phi`, and all `mapbox` rows moved to non-vm_hako owners, and the remaining vm_hako references are frozen as the 4-row monitor pack `tools/smokes/v2/suites/integration/vm-hako-core.txt` (`compare`, `env`, `file_close`, `file_read`)
- the `mapbox` bridge is fully retired from `collection-core.txt`; all 7 `MapBox.*` owner rows now live in dedicated non-vm_hako emit+exec smokes under `phase29y/hako/emit_mir/`
- LLVM replacement anchors already exist in the current `phase29ck_boundary/runtime_data/*`, `phase29ck_llvm_backend_*`, and `phase163x_boundary_*` proof families
- recommended single monitor canary while the replacement matrix is moving:
  - `tools/smokes/v2/profiles/integration/vm_hako_caps/env/env_get_ported_vm.sh`
  - it is the smallest stable signal that still exercises extern routing
- freeze-first vm_hako families:
  - `mapbox/`
  - `misc/`
  - `atomic/`
  - `tls/`
  - `compare/`
  - these are either indirect-live (`mapbox`) or narrow single-purpose rows that should not grow during cutover
- `mapbox/` live ownership no longer executes from `collection_core/`; all 7 rows now live in dedicated non-vm_hako emit+exec owners, while `vm_hako_caps/mapbox/*` remains as the temporary mirror/archive source
- exact row-to-row mapping is locked by the inventory now; `96xC` is the execution wave for:
  - landed: `96xC1a` retired `args_vm.sh` against `apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
  - landed: `96xC1b` moved env product ownership to `presubmit.txt` via `core/phase2035/v1_extern_env_get_canary_vm.sh` and kept `vm_hako_caps/env/env_get_ported_vm.sh` as monitor-only in `vm-hako-core.txt`
  - landed: `96xC1c` retired `filebox_newbox_vm.sh` from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh` with `apps/archive/phase29cc_plg04_filebox_pilot_vm.sh` as the explicit green anchor
  - landed: `96xC1d` retired `file_read_ported_vm.sh` and `file_close_ported_vm.sh` from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh` with the PLG-07 FileBox anchors as explicit green anchors; both vm_hako rows remain only in `vm-hako-core.txt`
  - landed: `96xC1e` retired `file_error_vm.sh` from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh` with `apps/phase96x_filebox_missing_open_vm.sh` as the dedicated green witness
  - wave `1a`: complete
  - landed: `96xC2a` retired `compare_ported_vm.sh`, `atomic_fence_ported_vm.sh`, and `tls_last_error_ported_vm.sh` from the live vm_hako gate/suite pair; `compare_ported_vm.sh` stays only in `vm-hako-core.txt`, `proof/native-reference/native_backend_compare_eq_canary_vm.sh` + `native_backend_compare_lt_canary_vm.sh` are the explicit compare proof anchors, and `apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh` via `presubmit.txt` is the shared atomic/tls owner anchor
  - landed: `96xC2b` archived `compare_ge_ported_vm.sh` as explicit retired evidence and removed it from the live vm_hako gate/suite pair because no concrete non-vm_hako live owner exists yet
  - landed: `96xC2c` archived `const_void_ported_vm.sh` as explicit retired evidence and removed it from `vm-hako-caps.txt`, `phase29y_vm_hako_caps_gate_vm.sh`, and `vm-hako-core.txt`
  - wave `1b`: complete
  - landed: `96xC4a` copied the 6 non-live `mapbox` rows into `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/` as archive mirrors, while leaving the current dirty `vm_hako_caps/mapbox/*` worktree content untouched
  - landed: `96xC4b/C4c` moved all 7 `MapBox.*` rows to dedicated non-vm_hako emit+exec owners under `phase29y/hako/emit_mir/`, removed every `collection_core/mapbox_*` row from `collection-core.txt`, and archived the bridge scripts under `tools/smokes/v2/profiles/archive/collection_core/`
  - landed: `96xC3a/C3e/C3f` moved `select_emit` and `open_handle_phi` to dedicated non-vm_hako emit+exec owners under `phase29y/hako/emit_mir/`
  - landed: `96xC3c` retired `boxcall_args_gt1_ported_vm.sh` from `vm-hako-caps.txt`, `phase29y_vm_hako_caps_gate_vm.sh`, and `vm-hako-core.txt`
  - landed: `96xC3d` / `96xD1` moved APP-1 product ownership to `apps/gate_log_summarizer_vm.sh` via `presubmit.txt` and removed the APP-1 vm_hako rows from the active vm_hako suite/gate pair
  - landed: `96xC3e` moved `select_emit` from the phase29y vm_hako gate to `phase29y_hako_emit_mir_select_exec_contract_vm.sh` via `phase29y-hako-emit-mir.txt` and `selfhost-core.txt`; the phase29y gate is now a retired compatibility stub
  - landed: `96xD2` froze `vm-hako-core.txt` as the final 4-row monitor bundle after `tools/smokes/v2/run.sh --profile integration --suite vm-hako-core` passed `4/4`
  - wave `2`: complete; the phase29y gate no longer owns any active vm_hako row
  - parallel track: `mapbox -> collection-core` ownership move
- detailed execution order is fixed in `96x-92-execution-plan.md`

## Scope and Non-Goals

In scope:

- `tools/smokes/v2/profiles/integration/vm_hako_caps/**`
- `tools/smokes/v2/suites/integration/vm-hako-caps.txt`
- LLVM replacement proof families under `tools/smokes/v2/profiles/integration/phase29ck_boundary/**`
- LLVM replacement proof families under `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_*`
- current-doc pointer sync for this corridor

Out of scope:

- `phase-163x` implementation work
- new vm_hako feature shims
- rust-vm keep/fallback retirement
- llvmlite monitor-only keep reduction
- archive/delete sweeps for still-live proof or reference surfaces

## Success Criteria

- the vm_hako capability gate is reduced to monitor-only status
- the LLVM replacement matrix is explicit and green
- the current docs point at the same retirement corridor
- no new vm_hako acceptance row lands while the cutover is in progress
