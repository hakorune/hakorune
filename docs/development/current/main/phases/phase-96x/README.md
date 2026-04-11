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

- the active reference gate is still `tools/smokes/v2/profiles/integration/vm_hako_caps/gate/phase29y_vm_hako_caps_gate_vm.sh`
- the live `vm_hako_caps` family still covers `app1`, `args`, `atomic`, `compare`, `env`, `file`, `mapbox`, `misc`, `open_handle_phi`, `select_emit`, `tls`
- the current pressure point is the remaining product-visible live row set: `file`
- LLVM replacement anchors already exist in the current `phase29ck_boundary/runtime_data/*`, `phase29ck_llvm_backend_*`, and `phase163x_boundary_*` proof families
- recommended single monitor canary while the replacement matrix is moving:
  - `tools/smokes/v2/profiles/integration/vm_hako_caps/env/env_get_ported_vm.sh`
  - it is the smallest stable signal that still exercises extern routing
- recommended non-blocking semantic shadow canary while compiler/backend seam cutover is still moving:
  - `tools/smokes/v2/profiles/integration/vm_hako_caps/open_handle_phi/open_handle_phi_ported_vm.sh`
  - it is the narrowest live seam probe for PHI/open-handle drift
- freeze-first vm_hako families:
  - `mapbox/`
  - `misc/`
  - `atomic/`
  - `tls/`
  - `compare/`
  - these are either indirect-live (`mapbox`) or narrow single-purpose rows that should not grow during cutover
- `mapbox/` is now bridged through `collection_core/mapbox_*` rather than referenced directly by `collection-core.txt`, so it belongs to a parallel re-home track instead of the first acceptance cutover wave
- exact row-to-row mapping is locked by the inventory now; `96xC` is the execution wave for:
  - landed: `96xC1a` retired `args_vm.sh` against `apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
  - landed: `96xC1b` moved env product ownership to `presubmit.txt` via `core/phase2035/v1_extern_env_get_canary_vm.sh` and kept `vm_hako_caps/env/env_get_ported_vm.sh` as monitor-only in `vm-hako-core.txt`
  - wave `1a`: `file`
  - wave `1b`: `compare` + `misc` + `atomic` + `tls`
  - wave `2`: `select_emit` + `open_handle_phi` + `boxcall_args_gt1` + `app1`
  - parallel track: `mapbox -> collection-core` ownership move
- `app1_summary_contract_ported_vm.sh` is also still referenced by `tools/smokes/v2/suites/integration/presubmit.txt`, so `app1` remains a late demotion/retire family rather than an early cutover target
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
