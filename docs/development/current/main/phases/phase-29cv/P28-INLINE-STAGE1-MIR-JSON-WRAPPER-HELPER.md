---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: inline the single-use mir-json forwarding helper in the retired Stage1 wrapper without changing wrapper behavior.
Related:
  - docs/development/current/main/phases/phase-29cv/P24-KEEPER-DELETE-LAST-ORDER.md
  - docs/development/current/main/phases/phase-29cv/P27-DELETE-DEAD-COMPAT-PROBE-ROUTE-FILES.md
  - tools/selfhost/compat/run_stage1_cli.sh
---

# P28 Inline Stage1 MIR JSON Wrapper Helper

## Goal

Keep the retired Stage1 compatibility wrapper thin.

`tools/selfhost/compat/run_stage1_cli.sh` still had a dedicated
`run_emit_mir_json_from_source()` helper that only forwarded one call to the
Stage1 contract. The helper had a single caller and did not own any behavior.

## Decision

- inline the one forwarding call inside `run_emit_mir_json()`
- delete `run_emit_mir_json_from_source()`
- keep all retirement messages and compat behavior unchanged

## Non-goals

- do not change `emit program-json` retirement handling
- do not change `--from-program-json` retirement handling
- do not change Stage1 contract ownership
- do not change the explicit compat proof route

## Acceptance

```bash
bash -n tools/selfhost/compat/run_stage1_cli.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```
