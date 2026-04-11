---
Status: Active
Date: 2026-04-11
---

# 96x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `96xA inventory` | completed | lock the current vm_hako gate, the LLVM replacement set, and the monitor canary candidate |
| 2 | `96xB ranking` | completed | choose the smallest LLVM replacement wave and freeze `env/env_get_ported_vm.sh` as the canary |
| 3 | `96xC cutover` | pending | move the first acceptance row(s) to LLVM and shrink vm_hako |
| 4 | `96xD closeout` | pending | prove the new gate shape and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `96xA1` | completed | current vm_hako acceptance inventory lock |
| `96xA2` | completed | LLVM replacement inventory lock |
| `96xB1` | completed | rank the runtime-data / args / collection wave |
| `96xB2` | completed | rank the file / env / compare wave |
| `96xC1` | pending | first LLVM cutover smoke pack for the runtime-data / args wave |
| `96xC2` | completed | freeze `env/env_get_ported_vm.sh` as the single monitor canary |
| `96xD1` | pending | proof / closeout |

## Current Front

| Item | State |
| --- | --- |
| Now | `96xC cutover` |
| Blocker | `none` |
| Next | `96xD closeout` |
| After Next | `none` |

## Acceptance Shape

- the active vm_hako gate is narrowed to monitor-only or a single explicit canary
- the first LLVM replacement wave is green and documented
- no new vm_hako capability rows are added while the cutover runs
- the canary is `tools/smokes/v2/profiles/integration/vm_hako_caps/env/env_get_ported_vm.sh`
- the first cutover wave is the runtime-data / args / collection family, with `mapbox` held for the next wave
