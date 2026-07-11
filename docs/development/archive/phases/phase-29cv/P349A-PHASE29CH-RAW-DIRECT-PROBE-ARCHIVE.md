---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: archive phase29ch raw direct stage1-cli absence diagnostics probe
Related:
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/phases/phase-29ch/29ch-20-route-evidence-and-probes.md
  - tools/archive/legacy-selfhost/engineering/phase29ch_raw_direct_stage1_cli_probe.sh
---

# P349A: Phase29ch Raw Direct Probe Archive

## Problem

`tools/dev/phase29ch_raw_direct_stage1_cli_probe.sh` was a diagnostics-only
absence probe. It proved that compiled `stage1-cli` artifacts do not expose a
raw direct authority lane for:

- `<bin> <source>`
- `<bin> emit program-json <source>`
- `<bin> emit mir-json <source>`

All three forms were expected to return `rc=97`. This is historical route
evidence, not a live Program(JSON v0) keeper or mainline proof.

## Boundary

Allowed:

- archive the raw direct absence probe as engineering evidence
- update active docs to point at the archived evidence
- keep the archived script runnable with the existing repo-root calculation

Not allowed:

- change `tools/selfhost/compat/run_stage1_cli.sh`
- change Stage1 CLI raw/subcmd behavior
- change `stage1_contract.sh`
- weaken the explicit Program(JSON) compat proof

## Implementation

- Moved `tools/dev/phase29ch_raw_direct_stage1_cli_probe.sh` to
  `tools/archive/legacy-selfhost/engineering/`.
- Updated route/evidence docs so the raw direct absence proof is read as
  archived evidence.

Archive metadata:

```text
original_path: tools/dev/phase29ch_raw_direct_stage1_cli_probe.sh
archived_on: 2026-05-03
archived_by_card: P349A-PHASE29CH-RAW-DIRECT-PROBE-ARCHIVE
last_known_owner: phase29ch diagnostics-only raw direct stage1-cli absence proof
replacement: no live replacement; raw direct stage1-cli authority remains outside the current route contract
restore_command: git mv tools/archive/legacy-selfhost/engineering/phase29ch_raw_direct_stage1_cli_probe.sh tools/dev/phase29ch_raw_direct_stage1_cli_probe.sh
```

## Acceptance

```bash
! rg -n "phase29ch_raw_direct_stage1_cli_probe" tools src lang .github --glob '!tools/archive/**'
bash -n tools/archive/legacy-selfhost/engineering/phase29ch_raw_direct_stage1_cli_probe.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
