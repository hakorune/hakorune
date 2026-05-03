---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv Stage1 diagnostics-only Program(JSON) probe archive
Related:
  - docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md
  - docs/development/current/main/phases/phase-29ch/29ch-20-route-evidence-and-probes.md
  - tools/archive/legacy-selfhost/engineering/phase29ch_program_json_explicit_mode_gate_probe.sh
  - tools/archive/legacy-selfhost/engineering/phase29ch_program_json_helper_exec_probe.sh
  - docs/development/current/main/CURRENT_STATE.toml
---

# P342A: Phase29ch Diagnostic Probe Archive

## Problem

The phase29ch Program(JSON) diagnostics-only probes below no longer had active
tool, smoke, source, or CI callers:

- `tools/dev/phase29ch_program_json_explicit_mode_gate_probe.sh`
- `tools/dev/phase29ch_program_json_helper_exec_probe.sh`

Leaving them in active `tools/dev/` made Stage1 Program(JSON) compatibility
look broader than the live contract surface. The active keeper is still the
explicit compat route proof, not these diagnostics-only probes.

## Boundary

Do not archive `tools/dev/phase29ch_program_json_compat_route_probe.sh`.

Do not archive `tools/dev/phase29ch_program_json_cold_compat_probe.sh`.

Do not change `tools/selfhost/lib/stage1_contract.sh` or
`tools/selfhost/compat/run_stage1_cli.sh`.

Do not delete Rust/public Program(JSON v0) delete-last surface.

This is a BoxShape cleanup only: move dead active dev probes to archived
engineering evidence.

## Implementation

- move the two diagnostics-only probes to
  `tools/archive/legacy-selfhost/engineering/`
- update the explicit-mode probe root calculation for the archive depth
- update archive README, phase-29cv keeper docs, P33, and phase-29ch evidence
  wording

Archive metadata:

```text
original_path: tools/dev/phase29ch_program_json_explicit_mode_gate_probe.sh
archived_on: 2026-05-03
archived_by_card: P342A-PHASE29CH-DIAGNOSTIC-PROBE-ARCHIVE
last_known_owner: phase29ch diagnostics-only explicit Program(JSON) mode gate
replacement: tools/dev/phase29ch_program_json_compat_route_probe.sh remains the live explicit compat proof
restore_command: git mv tools/archive/legacy-selfhost/engineering/phase29ch_program_json_explicit_mode_gate_probe.sh tools/dev/phase29ch_program_json_explicit_mode_gate_probe.sh
```

```text
original_path: tools/dev/phase29ch_program_json_helper_exec_probe.sh
archived_on: 2026-05-03
archived_by_card: P342A-PHASE29CH-DIAGNOSTIC-PROBE-ARCHIVE
last_known_owner: phase29ch diagnostics-only raw helper exec absence probe
replacement: no active replacement; this was negative diagnostic evidence only
restore_command: git mv tools/archive/legacy-selfhost/engineering/phase29ch_program_json_helper_exec_probe.sh tools/dev/phase29ch_program_json_helper_exec_probe.sh
```

## Acceptance

```text
rg -n "phase29ch_program_json_explicit_mode_gate_probe|phase29ch_program_json_helper_exec_probe" \
  tools src lang .github
-> only archived paths, no active callers
```

```text
bash -n \
  tools/archive/legacy-selfhost/engineering/phase29ch_program_json_explicit_mode_gate_probe.sh \
  tools/archive/legacy-selfhost/engineering/phase29ch_program_json_helper_exec_probe.sh
-> ok
```

```text
bash tools/checks/current_state_pointer_guard.sh
-> ok

git diff --check
-> ok
```
