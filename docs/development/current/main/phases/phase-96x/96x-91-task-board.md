---
Status: Active
Date: 2026-04-11
---

# 96x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `96xA inventory` | completed | lock the current vm_hako gate, the LLVM replacement set, and the monitor canary candidate |
| 2 | `96xB ranking` | completed | choose the smallest LLVM replacement wave and freeze `env/env_get_ported_vm.sh` as the canary |
| 3 | `96xC cutover` | completed | execute the split cutover waves and the separate mapbox re-home track |
| 4 | `96xD closeout` | in_progress | prove the new gate shape and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `96xA1` | completed | current vm_hako acceptance inventory lock |
| `96xA2` | completed | LLVM replacement inventory lock |
| `96xB1` | completed | rank the runtime-data / args / collection wave |
| `96xB2` | completed | rank the file / env / compare wave |
| `96xB3` | completed | freeze `env/env_get_ported_vm.sh` as the blocking canary and `open_handle_phi` as the semantic shadow |
| `96xC1` | completed | wave 1a: LLVM cutover pack for `env` + `file` after the narrow `args_vm` cut |
| `96xC2` | completed | wave 1b: LLVM cutover pack for `compare` + `misc` + `atomic` + `tls` |
| `96xC3` | completed | wave 2: seam shadow replacement for `select_emit` + `open_handle_phi` + `boxcall_args_gt1` |
| `96xC4` | completed | parallel `mapbox -> collection-core` re-home track; all 7 `MapBox.*` rows are retired to non-vm_hako owners and the bridge is archive-only |
| `96xD1` | completed | `app1` late demotion and proof / closeout |

### Wave 1a Substeps

| Task | Status | Read as |
| --- | --- | --- |
| `96xC1a` | completed | retire `args_vm.sh` against `phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` and remove its live vm_hako refs |
| `96xC1b` | completed | split `env_get_ported_vm.sh` into canary-vs-product ownership |
| `96xC1c` | completed | retire `filebox_newbox_vm.sh` against the PLG-04 FileBox pilot anchor |
| `96xC1d` | completed | retire `file_read_ported_vm.sh` and `file_close_ported_vm.sh` against the PLG-07 FileBox anchors |
| `96xC1e` | completed | resolve `file_error_vm.sh` with a dedicated non-vm_hako open-error witness |

### Wave 1b Substeps

| Task | Status | Read as |
| --- | --- | --- |
| `96xC2a` | completed | retire `compare_ported_vm.sh`, `atomic_fence_ported_vm.sh`, and `tls_last_error_ported_vm.sh` from the live vm_hako gate/suite pair |
| `96xC2b` | completed | close `compare_ge_ported_vm.sh` as explicit archive-only retired evidence |
| `96xC2c` | completed | close `const_void_ported_vm.sh` as explicit archive-only retired evidence |

### Wave 2 Substeps

| Task | Status | Read as |
| --- | --- | --- |
| `96xC3a` | completed | keep `select_emit` as a shadow row; no exact non-vm_hako replacement anchor exists yet |
| `96xC3b` | completed | keep `open_handle_phi` as a non-blocking shadow row in `vm-hako-core.txt`; no exact non-vm_hako replacement anchor exists yet |
| `96xC3c` | completed | remove `boxcall_args_gt1_ported_vm.sh` from active vm_hako suite/gate ownership and treat it as retired APP-1 seam evidence |
| `96xC3d` | completed | replace `app1_summary_contract_ported_vm.sh` in `presubmit.txt` and demote the APP-1 rows |
| `96xC3e` | completed | replace `select_emit` with a dedicated non-vm_hako emit+exec owner and retire the phase29y vm_hako gate to a compatibility stub |
| `96xC4a` | completed | move the 7 live MapBox rows under `collection_core/` and archive the 6 non-live mirrors |
| `96xC4b` | completed | add non-vm_hako emit+exec owners for `MapBox.clear`, `MapBox.delete`, and `MapBox.keys`, then remove those rows from `collection-core.txt` |
| `96xC4c` | completed | retire the remaining `collection_core/mapbox_set|get|has|size` bridge rows to non-vm_hako emit+exec owners |

## Execution Anchor

- exact step order is fixed in `96x-92-execution-plan.md`

## Current Front

| Item | State |
| --- | --- |
| Now | `vm-hako-core shadow closeout sync` |
| Blocker | `none` |
| Next | `open_handle_phi shadow retirement or archive decision` |
| After Next | `phase96x closeout sync` |

## Acceptance Shape

- the active vm_hako gate is retired to a compatibility stub
- the first LLVM replacement wave is split and documented
- no new vm_hako capability rows are added while the cutover runs
- the blocking canary is `tools/smokes/v2/profiles/integration/vm_hako_caps/env/env_get_ported_vm.sh`, and it now lives only in `vm-hako-core.txt`
- the non-blocking semantic shadow is `tools/smokes/v2/profiles/integration/vm_hako_caps/open_handle_phi/open_handle_phi_ported_vm.sh`, and it now lives only in `vm-hako-core.txt`
- `96xC1a` is landed: `args_vm.sh` is retired from `vm-hako-caps.txt`, `vm-hako-core.txt`, and `phase29y_vm_hako_caps_gate_vm.sh`
- `96xC1b` is landed: `env_get_ported_vm.sh` is retired from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh`, while `core/phase2035/v1_extern_env_get_canary_vm.sh` is added to `presubmit.txt`
- `96xC1c` is landed: `filebox_newbox_vm.sh` is retired from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh`
- `96xC1d` is landed: `file_read_ported_vm.sh` and `file_close_ported_vm.sh` are retired from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh`
- `96xC1e` is landed: `file_error_vm.sh` is retired from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh`
- wave `1a` is complete
- `96xC2a` is landed: `compare_ported_vm.sh`, `atomic_fence_ported_vm.sh`, and `tls_last_error_ported_vm.sh` are retired from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh`
- `96xC2b` is landed: `compare_ge_ported_vm.sh` is retired from the live vm_hako gate/suite pair and preserved only as archive evidence
- `96xC2c` is landed: `const_void_ported_vm.sh` is retired from the live vm_hako gate/suite pair and from `vm-hako-core.txt`, and preserved only as archive evidence
- wave `1b` is complete
- `96xC3a` / `96xC3e` are landed: `select_emit` is retired from the active vm_hako gate after `phase29y_hako_emit_mir_select_exec_contract_vm.sh` was added as the exact non-vm_hako emit+exec owner in `phase29y-hako-emit-mir.txt` and `selfhost-core.txt`
- `96xC3b` is landed: `open_handle_phi` stays shadow because the current `joinir-bq` / `selfhost-core` packs do not yet cover the exact `FileBox.open` handle-propagation seam contract; it no longer blocks the phase29y gate
- `96xC3c` is landed: `boxcall_args_gt1_ported_vm.sh` is removed from `vm-hako-caps.txt`, `phase29y_vm_hako_caps_gate_vm.sh`, and `vm-hako-core.txt`
- `96xC3d` / `96xD1` are landed: `presubmit.txt` now owns `apps/gate_log_summarizer_vm.sh`, and the APP-1 vm_hako rows are removed from the active vm_hako suite/gate pair
- wave `2` is complete; the phase29y gate is a compatibility stub, `select_emit` is owned by a non-vm_hako smoke, and the remaining vm_hako shadows live in `vm-hako-core.txt`
- `mapbox` is a separate `collection-core` re-home track, not part of wave `1a`
- current landed substeps:
  - `collection-core.txt` no longer points at any `collection_core/mapbox_*` row
  - all 7 `MapBox.*` rows now live in dedicated non-vm_hako emit+exec owners under `phase29y-hako-emit-mir.txt` and `selfhost-core.txt`
  - the 6 non-live `vm_hako_caps/mapbox/*` rows are copied into `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/`
  - the 7 retired `collection_core/mapbox_*_ported_vm.sh` bridge scripts are archived under `tools/smokes/v2/profiles/archive/collection_core/`
  - `96xC4` is complete; only archive/mirror cleanup remains for `mapbox`
