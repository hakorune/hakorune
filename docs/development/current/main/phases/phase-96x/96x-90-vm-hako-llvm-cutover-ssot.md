---
Status: Active
Decision: provisional
Date: 2026-04-11
Scope: lock the vm_hako -> LLVM acceptance cutover corridor and define the first migration wave.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-96x/README.md
  - docs/development/current/main/phases/phase-96x/96x-92-execution-plan.md
  - docs/development/current/main/phases/phase-29y/README.md
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
  - docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md
  - docs/development/current/main/phases/phase-29y/82-VM-HAKO-BOXCALL-CONTRACT-SSOT.md
  - docs/development/current/main/phases/phase-29x/29x-98-legacy-route-retirement-investigation-ssot.md
  - tools/smokes/v2/profiles/integration/vm_hako_caps/README.md
  - tools/smokes/v2/profiles/integration/phase29ck_boundary/README.md
---

# 96x-90 vm_hako LLVM Acceptance Cutover SSOT

## Intent

- move the active acceptance surface from `vm_hako` to LLVM-line replacement families
- keep `vm_hako` as reference/monitor only while the replacement matrix is pinned
- do not add new vm_hako capability coverage as a way to solve the cutover

## Starting Read

- the active `vm_hako` gate is still `tools/smokes/v2/profiles/integration/vm_hako_caps/gate/phase29y_vm_hako_caps_gate_vm.sh`
- the `vm_hako_caps` family is now best read as a monitor bucket, not as a growth bucket
- the runtime-data / collection slice is the first obvious migration target because it maps cleanly to the LLVM boundary/runtime-data proof family
- the narrow `args_vm.sh` row is already cut over; the remaining `args/` owner row is the late seam-side `boxcall_args_gt1`
- file / compare / app1 are still live contract slices, but they should follow after the first runtime-data replacement wave lands

## Current Inventory

### Live vm_hako acceptance bucket

- `tools/smokes/v2/profiles/integration/vm_hako_caps/gate/phase29y_vm_hako_caps_gate_vm.sh`
- `tools/smokes/v2/profiles/integration/vm_hako_caps/README.md`
- `tools/smokes/v2/suites/integration/vm-hako-caps.txt`

### Current vm_hako smoke families

- `app1/`
- `args/`
- `atomic/`
- `compare/`
- `env/`
- `file/`
- `mapbox/`
- `misc/`
- `open_handle_phi/`
- `select_emit/`
- `tls/`

- `mapbox/` is not in the phase29y vm_hako gate, but it remains live via
  `tools/smokes/v2/suites/integration/collection-core.txt`.

### Family State Table

| Family | Current role | phase29y-live count | other live count | LLVM replacement anchor | Status target | Retirement condition |
| --- | --- | --- | --- | --- | --- | --- |
| `env/` | environment routing monitor canary | `0` | `1` via `vm-hako-core.txt` | `core/phase2035/v1_extern_env_get_canary_vm.sh` via `presubmit.txt` | cutover -> monitor keep | product ownership is outside `vm-hako-caps`; `env_get` stays only as the explicit vm_hako monitor canary |
| `file/` | file handle lifecycle monitor residue | `0` | `2` via `vm-hako-core.txt` | `apps/archive/phase29cc_plg04_filebox_pilot_vm.sh` for `newbox`, PLG-07 FileBox anchors for `read/close`, and `apps/phase96x_filebox_missing_open_vm.sh` for missing-open | cutover -> monitor keep | product-facing file rows are no longer gate-owned; only explicit vm_hako monitor rows remain |
| `args/` | mixed lane: narrow args row retired, seam row remains | `1` | `0` | `apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` for the narrow row; APP/seam pack for `boxcall_args_gt1` | cutover -> seam keep | `args_vm.sh` is no longer gate-owned and the remaining seam row moves in wave `2` |
| `compare/` | compare-op pin | `2` | `0` | product LLVM acceptance pack | freeze -> cutover | LLVM-line compare coverage is explicit and green |
| `misc/` | one-off capability pin | `1` | `0` | product LLVM acceptance pack | freeze -> archive | replacement row exists and vm_hako no longer owns the witness |
| `atomic/` | atomic fence pin | `1` | `0` | product LLVM acceptance pack | freeze -> archive | LLVM-line atomic witness exists or the row is intentionally dropped |
| `tls/` | TLS last-error pin | `1` | `0` | product LLVM acceptance pack | freeze -> archive | LLVM-line TLS/error witness exists |
| `select_emit/` | compiler/backend emission seam | `1` | `0` | `phase29y-hako-emit-mir.txt` or sibling selfhost seam pack | keep -> shadow -> retire | LLVM/JoinIR seam pack replaces vm_hako ownership |
| `open_handle_phi/` | PHI/open-handle seam | `1` | `0` | JoinIR/selfhost seam pack | keep -> shadow -> retire | LLVM/JoinIR seam pack covers the same propagation truth |
| `app1/` | broad summary contract | `2` | `1` via `presubmit.txt` | product LLVM acceptance pack | keep -> late demote | leaf families are retired and `presubmit` no longer needs the vm_hako witness |
| `mapbox/` | collection/runtime-data bridge | `0` | `7` via `collection_core/mapbox_*` in `collection-core.txt` | `collection-core.txt` plus runtime-data LLVM pack | freeze -> re-home -> retire | collection-core runs the live rows from `collection_core/` and the old `vm_hako_caps/mapbox/*` copy is reduced to archive source only |

### LLVM replacement anchors

- `tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_get_min.sh`
- `tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh`
- `tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_get_min.sh`
- `tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh`
- `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_map_set_size_runtime_proof.sh`
- `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_box_capi_link_min.sh`
- `tools/smokes/v2/profiles/integration/phase163x_boundary_user_box_metadata_keep_min.sh`
- `tools/smokes/v2/profiles/integration/phase163x_boundary_user_box_method_known_receiver_min.sh`

## Replacement Gap

| Current vm_hako family | Current role | LLVM replacement anchor | Notes |
| --- | --- | --- | --- |
| `file/` | product-visible live rows | `apps/archive/phase29cc_plg04_filebox_pilot_vm.sh`, PLG-07 FileBox anchors, and `apps/phase96x_filebox_missing_open_vm.sh` | wave `1a` is complete; remaining file rows are monitor-only in `vm-hako-core.txt` |
| `compare/` + `misc/` + `atomic/` + `tls/` | narrow single-purpose witnesses | product LLVM acceptance pack | second wave after the product-visible wave is stable |
| `select_emit/` + `open_handle_phi/` + `boxcall_args_gt1` | compiler/backend seam sentinels | `phase29y-hako-emit-mir.txt`, `joinir-bq.txt`, `selfhost-core.txt` | keep as seam shadow rows until LLVM/JoinIR proofs are explicit |
| `app1/` | wide end-to-end summary parity | `presubmit.txt` plus product LLVM acceptance pack | late demotion only after leaf families stop owning the contract |
| `mapbox/` | collection semantics / handle-presence pressure | `collection-core.txt` plus runtime-data LLVM pack | live rows are moved into `collection_core/`; non-live archive remains a parallel re-home substep |

## Gate Artifact Pairing

- `tools/smokes/v2/suites/integration/vm-hako-caps.txt` and `tools/smokes/v2/profiles/integration/vm_hako_caps/gate/phase29y_vm_hako_caps_gate_vm.sh` are a coupled artifact pair.
- The suite file already states they must stay in exact sync; phase-96x must treat them as one retirement object, not as independent cleanup tasks.
- Any row removal or demotion must update both files in the same commit.

## Monitor-Only Policy

- blocking daily health canary:
  - `tools/smokes/v2/profiles/integration/vm_hako_caps/env/env_get_ported_vm.sh`
  - purpose: cheap liveness plus extern-routing sanity
- product-facing replacement owner:
  - `tools/smokes/v2/profiles/integration/core/phase2035/v1_extern_env_get_canary_vm.sh`
  - suite: `tools/smokes/v2/suites/integration/presubmit.txt`
- non-blocking semantic shadow canary:
  - `tools/smokes/v2/profiles/integration/vm_hako_caps/open_handle_phi/open_handle_phi_ported_vm.sh`
  - purpose: PHI/open-handle seam drift detection during LLVM/JoinIR cutover
- retire the shadow canary only after the LLVM/JoinIR seam packs are explicit and green.

## Decision Rule

- rank the first cutover wave by leverage and by direct semantic replacement coverage
- keep exactly one monitor canary while the replacement matrix is still moving
  - recommended canary: `tools/smokes/v2/profiles/integration/vm_hako_caps/env/env_get_ported_vm.sh`
  - it stays small, stable, and still exercises extern routing
- keep `open_handle_phi` as the non-blocking semantic shadow while compiler/backend seam replacement is still moving
- freeze `mapbox`, `misc`, `atomic`, `tls`, and `compare` first so cutover work does not keep widening vm_hako ownership
- do not solve the cutover by adding new vm_hako shims or by widening the gate

## Update Rule

- if a vm_hako smoke is moved to LLVM acceptance, update this file, the task board, and the current pointer docs in the same commit
- if a vm_hako smoke remains as monitor-only keep, name it explicitly here so the reduction does not drift back into a hidden owner surface
- keep `mapbox` on its own re-home track until the old `vm_hako_caps/mapbox/*` mirror/archive source is gone and LLVM coverage replaces the `collection_core/` owner rows
- do not touch rust-vm keep / fallback corridors in this lane
