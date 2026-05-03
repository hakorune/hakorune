---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: archive phase29ch Program(JSON) cold compat diagnostics probe
Related:
  - docs/development/current/main/design/frontend-owner-proof-index.md
  - docs/development/current/main/phases/phase-29ch/29ch-20-route-evidence-and-probes.md
  - docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md
  - tools/archive/legacy-selfhost/engineering/phase29ch_program_json_cold_compat_probe.sh
  - tools/dev/phase29ch_program_json_compat_route_probe.sh
---

# P350A: Phase29ch Cold Compat Probe Archive

## Problem

`tools/dev/phase29ch_program_json_cold_compat_probe.sh` was a diagnostics-only
probe for non-authority supplied-Program(JSON) caller shapes. It checked three
facts:

- legacy env supplied-Program(JSON) shape reports `none`
- retired raw wrapper `--from-program-json` reports `none`
- explicit helper still reports `stage1-env-mir-program`

The first two are historical dead-lane evidence. The third is covered by the
live explicit compat route probe and exact Stage1 CLI emit smoke. Keeping the
cold compat probe active made the Stage1 Program(JSON) surface look broader
than the live contract.

## Boundary

Allowed:

- archive the cold compat diagnostics probe
- update active proof indexes to use live compat proof entries
- keep archived evidence runnable with the correct repo-root calculation
- update the phase-29cv delete-last taskboard

Not allowed:

- archive `tools/dev/phase29ch_program_json_compat_route_probe.sh`
- change `stage1_contract_exec_program_json_compat()`
- change `Stage1ProgramJsonCompatBox`
- change `tools/selfhost/compat/run_stage1_cli.sh`
- delete Rust/public Program(JSON v0) delete-last surface

## Implementation

- Moved `tools/dev/phase29ch_program_json_cold_compat_probe.sh` to
  `tools/archive/legacy-selfhost/engineering/`.
- Replaced active canonical proof references with the live explicit compat
  route probe or the exact Stage1 CLI emit contract smoke.
- Updated phase evidence docs and the phase-29cv delete-last taskboard.

Archive metadata:

```text
original_path: tools/dev/phase29ch_program_json_cold_compat_probe.sh
archived_on: 2026-05-03
archived_by_card: P350A-PHASE29CH-COLD-COMPAT-PROBE-ARCHIVE
last_known_owner: phase29ch diagnostics-only cold supplied-Program(JSON) route probe
replacement: tools/dev/phase29ch_program_json_compat_route_probe.sh plus phase29ci_stage1_cli_exact_emit_contract_vm.sh
restore_command: git mv tools/archive/legacy-selfhost/engineering/phase29ch_program_json_cold_compat_probe.sh tools/dev/phase29ch_program_json_cold_compat_probe.sh
```

## Acceptance

```bash
! rg -n "phase29ch_program_json_cold_compat_probe" tools src lang .github --glob '!tools/archive/**'
bash -n tools/archive/legacy-selfhost/engineering/phase29ch_program_json_cold_compat_probe.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
