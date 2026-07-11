---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: archive redundant phase29ch Program(JSON) text-only diagnostics probe
Related:
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - docs/development/current/main/phases/phase-29ch/29ch-20-route-evidence-and-probes.md
  - tools/archive/legacy-selfhost/engineering/phase29ch_program_json_text_only_probe.sh
  - tools/dev/phase29ch_program_json_compat_route_probe.sh
---

# P347A: Phase29ch Text-Only Probe Archive

## Problem

`tools/dev/phase29ch_program_json_text_only_probe.sh` was a diagnostics-only
probe for the explicit supplied-Program(JSON) compat route. It checked that
the remaining compat resolver accepts text transport without a separate path
lane.

That proof is now covered by the live explicit compat route probe:

```text
tools/dev/phase29ch_program_json_compat_route_probe.sh
  -> stage1_contract_exec_program_json_compat()
  -> emit-mir-program
  -> STAGE1_SOURCE_TEXT transport
```

Keeping a second active probe made the Stage1 Program(JSON) surface look wider
than the live contract.

## Boundary

Allowed:

- move the text-only probe to archived engineering evidence
- update active docs so only the explicit compat route probe remains a live
  caller
- keep the archived script runnable with the correct repo-root calculation

Not allowed:

- archive `tools/dev/phase29ch_program_json_compat_route_probe.sh`
- archive `tools/dev/phase29ch_program_json_cold_compat_probe.sh`
- change `stage1_contract_exec_program_json_compat()`
- change `Stage1ProgramJsonCompatBox`
- delete Rust/public Program(JSON v0) delete-last surface

## Implementation

- Moved `tools/dev/phase29ch_program_json_text_only_probe.sh` to
  `tools/archive/legacy-selfhost/engineering/`.
- Updated archive inventory and current design/evidence docs.
- Kept the live explicit supplied-Program(JSON) proof on
  `tools/dev/phase29ch_program_json_compat_route_probe.sh`.

Archive metadata:

```text
original_path: tools/dev/phase29ch_program_json_text_only_probe.sh
archived_on: 2026-05-03
archived_by_card: P347A-PHASE29CH-TEXT-ONLY-PROBE-ARCHIVE
last_known_owner: phase29ch diagnostics-only Program(JSON) text transport proof
replacement: tools/dev/phase29ch_program_json_compat_route_probe.sh exercises stage1_contract_exec_program_json_compat() text transport
restore_command: git mv tools/archive/legacy-selfhost/engineering/phase29ch_program_json_text_only_probe.sh tools/dev/phase29ch_program_json_text_only_probe.sh
```

## Acceptance

```bash
! rg -n "phase29ch_program_json_text_only_probe" tools src lang .github --glob '!tools/archive/**'
bash -n tools/archive/legacy-selfhost/engineering/phase29ch_program_json_text_only_probe.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- no active tool/source/CI caller remains
- archived probe syntax is valid
- current-state pointer guard reports `ok`
- diff check reports no whitespace errors
