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
| `96xC1` | pending | wave 1a: LLVM cutover pack for `env` + `file` + narrow `args_vm` |
| `96xC2` | pending | wave 1b: LLVM cutover pack for `compare` + `misc` + `atomic` + `tls` |
| `96xC3` | pending | wave 2: seam shadow replacement for `select_emit` + `open_handle_phi` + `boxcall_args_gt1` |
| `96xC4` | pending | parallel `mapbox -> collection-core` re-home track |
| `96xD1` | pending | `app1` late demotion and proof / closeout |

## Execution Anchor

- exact step order is fixed in `96x-92-execution-plan.md`

## Current Front

| Item | State |
| --- | --- |
| Now | `96xC cutover` |
| Blocker | `none` |
| Next | `96xC1 wave 1a` |
| After Next | `96xC2 wave 1b` |

## Acceptance Shape

- the active vm_hako gate is narrowed to monitor-only or a single explicit canary
- the first LLVM replacement wave is split and documented
- no new vm_hako capability rows are added while the cutover runs
- the blocking canary is `tools/smokes/v2/profiles/integration/vm_hako_caps/env/env_get_ported_vm.sh`
- the non-blocking semantic shadow is `tools/smokes/v2/profiles/integration/vm_hako_caps/open_handle_phi/open_handle_phi_ported_vm.sh`
- wave `1a` is `env` + `file` + narrow `args_vm`
- wave `1b` is `compare` + `misc` + `atomic` + `tls`
- wave `2` is `select_emit` + `open_handle_phi` + `boxcall_args_gt1`
- `mapbox` is a separate `collection-core` re-home track, not part of wave `1a`
