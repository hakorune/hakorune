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
- the runtime-data / args / collection slice is the first obvious migration target because it maps cleanly to the LLVM boundary runtime-data proof family
- file / env / compare / app1 are still live contract slices, but they should follow after the first runtime-data replacement wave lands

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
| `args/` | entry routing / `RuntimeDataBox` pressure | `phase29ck_boundary/runtime_data/*` | first migration wave because it is the closest semantic match |
| `mapbox/` | collection semantics / handle-presence pressure | `phase29ck_boundary/runtime_data/*` | live via `collection-core`; keep as the next wave after `args` / `runtime_data` is pinned |
| `file/` | file handle lifecycle / open-read-close | `phase29ck_llvm_backend_*` | migrate only after the runtime-data wave stays green |
| `compare/` and `env/` | control-flow and env routing | `phase29ck_llvm_backend_*` and current LLVM boundary proofs | migration should reuse existing proof families instead of adding vm_hako rows |
| `app1/` | wide end-to-end summary parity | `phase29ck_llvm_backend_*` plus product LLVM boundary proofs | treat as the last wave, not the first |

## Decision Rule

- rank the first cutover wave by leverage and by direct semantic replacement coverage
- keep exactly one monitor canary while the replacement matrix is still moving
  - recommended canary: `tools/smokes/v2/profiles/integration/vm_hako_caps/env/env_get_ported_vm.sh`
  - it stays small, stable, and still exercises extern routing
- freeze `args/` and `mapbox/` first; they overlap the first LLVM replacement wave most directly
- do not solve the cutover by adding new vm_hako shims or by widening the gate

## Update Rule

- if a vm_hako smoke is moved to LLVM acceptance, update this file, the task board, and the current pointer docs in the same commit
- if a vm_hako smoke remains as monitor-only keep, name it explicitly here so the reduction does not drift back into a hidden owner surface
- do not touch rust-vm keep / fallback corridors in this lane
