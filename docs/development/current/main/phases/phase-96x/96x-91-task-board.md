---
Status: Active
Date: 2026-04-11
---

# 96x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `96xA inventory` | completed | lock the current vm_hako gate, the LLVM replacement set, and the monitor canary candidate |
| 2 | `96xB ranking` | completed | choose the smallest LLVM replacement wave and freeze `env/env_get_ported_vm.sh` as the canary |
| 3 | `96xC cutover` | pending | execute the split cutover waves and the separate mapbox re-home track |
| 4 | `96xD closeout` | pending | prove the new gate shape and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `96xA1` | completed | current vm_hako acceptance inventory lock |
| `96xA2` | completed | LLVM replacement inventory lock |
| `96xB1` | completed | rank the runtime-data / args / collection wave |
| `96xB2` | completed | rank the file / env / compare wave |
| `96xB3` | completed | freeze `env/env_get_ported_vm.sh` as the blocking canary and `open_handle_phi` as the semantic shadow |
| `96xC1` | in_progress | wave 1a: LLVM cutover pack for `env` + `file` after the narrow `args_vm` cut |
| `96xC2` | pending | wave 1b: LLVM cutover pack for `compare` + `misc` + `atomic` + `tls` |
| `96xC3` | pending | wave 2: seam shadow replacement for `select_emit` + `open_handle_phi` + `boxcall_args_gt1` |
| `96xC4` | in_progress | parallel `mapbox -> collection-core` re-home track; wrapper bridge + suite retarget landed |
| `96xD1` | pending | `app1` late demotion and proof / closeout |

### Wave 1a Substeps

| Task | Status | Read as |
| --- | --- | --- |
| `96xC1a` | completed | retire `args_vm.sh` against `phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` and remove its live vm_hako refs |
| `96xC1b` | completed | split `env_get_ported_vm.sh` into canary-vs-product ownership |
| `96xC1c` | pending | retire `filebox_newbox_vm.sh` against the PLG-04 FileBox pilot anchor |
| `96xC1d` | pending | retire `file_read_ported_vm.sh` and `file_close_ported_vm.sh` against the PLG-07 FileBox anchors |
| `96xC1e` | pending | resolve `file_error_vm.sh` with a dedicated non-vm_hako open-error witness or leave it as the last wave 1a survivor |

## Execution Anchor

- exact step order is fixed in `96x-92-execution-plan.md`

## Current Front

| Item | State |
| --- | --- |
| Now | `96xC1c file newbox` |
| Blocker | `none` |
| Next | `96xC1d file read/close` |
| After Next | `96xC1e file error gap` |

## Acceptance Shape

- the active vm_hako gate is narrowed to monitor-only or a single explicit canary
- the first LLVM replacement wave is split and documented
- no new vm_hako capability rows are added while the cutover runs
- the blocking canary is `tools/smokes/v2/profiles/integration/vm_hako_caps/env/env_get_ported_vm.sh`, and it now lives only in `vm-hako-core.txt`
- the non-blocking semantic shadow is `tools/smokes/v2/profiles/integration/vm_hako_caps/open_handle_phi/open_handle_phi_ported_vm.sh`
- `96xC1a` is landed: `args_vm.sh` is retired from `vm-hako-caps.txt`, `vm-hako-core.txt`, and `phase29y_vm_hako_caps_gate_vm.sh`
- `96xC1b` is landed: `env_get_ported_vm.sh` is retired from `vm-hako-caps.txt` and `phase29y_vm_hako_caps_gate_vm.sh`, while `core/phase2035/v1_extern_env_get_canary_vm.sh` is added to `presubmit.txt`
- wave `1a` is now `file`
- wave `1b` is `compare` + `misc` + `atomic` + `tls`
- wave `2` is `select_emit` + `open_handle_phi` + `boxcall_args_gt1`
- `mapbox` is a separate `collection-core` re-home track, not part of wave `1a`
- current landed substep: `collection-core.txt` now points at `collection_core/mapbox_*` wrappers instead of `vm_hako_caps/mapbox/*`
