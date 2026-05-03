---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: archive phase29ch selfhost Program(JSON) helper diagnostics probe
Related:
  - docs/development/current/main/phases/phase-29ch/29ch-20-route-evidence-and-probes.md
  - docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md
  - tools/archive/legacy-selfhost/engineering/phase29ch_selfhost_program_json_helper_probe.sh
---

# P351A: Phase29ch Selfhost Program(JSON) Helper Probe Archive

## Problem

`tools/dev/phase29ch_selfhost_program_json_helper_probe.sh` was a
diagnostics-only probe. It built a temporary helper that calls
`MirBuilderBox.emit_from_program_json_v0(...)`, then checked Stage1/Stage2 MIR
parity and runtime flags.

That is useful historical evidence, but it is not a live Program(JSON v0)
keeper and has no active smoke/tool caller. Keeping it under `tools/dev/`
made the active Stage1 Program(JSON) surface look larger than the live
contract.

## Boundary

Allowed:

- archive the selfhost Program(JSON) helper probe as engineering evidence
- keep the archived script runnable with repo-root stable path calculation
- update phase evidence docs and the phase-29cv taskboard

Not allowed:

- change `MirBuilderBox.emit_from_program_json_v0(...)`
- change Stage1/Stage2 artifact contracts
- change `stage1_contract.sh`
- change `tools/ny_mir_builder.sh`
- delete Rust/public Program(JSON v0) delete-last surface

## Implementation

- Moved `tools/dev/phase29ch_selfhost_program_json_helper_probe.sh` to
  `tools/archive/legacy-selfhost/engineering/`.
- Updated phase route/evidence docs and archive inventory.
- Kept live proof ownership on current Stage1 contract and explicit compat
  route probes.

Archive metadata:

```text
original_path: tools/dev/phase29ch_selfhost_program_json_helper_probe.sh
archived_on: 2026-05-03
archived_by_card: P351A-PHASE29CH-SELFHOST-PROGRAM-JSON-HELPER-PROBE-ARCHIVE
last_known_owner: phase29ch diagnostics-only Stage1/Stage2 Program(JSON)->MIR helper parity proof
replacement: no live replacement; parity evidence is archived while active proof stays on Stage1 contract/explicit compat route
restore_command: git mv tools/archive/legacy-selfhost/engineering/phase29ch_selfhost_program_json_helper_probe.sh tools/dev/phase29ch_selfhost_program_json_helper_probe.sh
```

## Acceptance

```bash
! rg -n "phase29ch_selfhost_program_json_helper_probe" tools src lang .github --glob '!tools/archive/**'
bash -n tools/archive/legacy-selfhost/engineering/phase29ch_selfhost_program_json_helper_probe.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
