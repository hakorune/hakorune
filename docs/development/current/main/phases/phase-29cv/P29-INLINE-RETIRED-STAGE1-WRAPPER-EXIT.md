---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: inline the single-use retired emit-program-json exit helper in the Stage1 compatibility wrapper.
Related:
  - docs/development/current/main/phases/phase-29cv/P24-KEEPER-DELETE-LAST-ORDER.md
  - docs/development/current/main/phases/phase-29cv/P28-INLINE-STAGE1-MIR-JSON-WRAPPER-HELPER.md
  - tools/selfhost/compat/run_stage1_cli.sh
---

# P29 Inline Retired Stage1 Wrapper Exit

## Goal

Keep the retired Stage1 compatibility wrapper thin and local.

`tools/selfhost/compat/run_stage1_cli.sh` still owned a dedicated
`exit_emit_program_json_wrapper_retired()` helper even though the helper had a
single callsite and only printed the wrapper retirement message before exiting.

## Decision

- inline the existing retirement message at the `emit program-json` case
- delete `exit_emit_program_json_wrapper_retired()`
- keep the message text and exit code unchanged

## Non-goals

- do not revive `emit program-json`
- do not change the explicit compat proof route
- do not change the Stage1 contract
- do not change any probe expectations

## Acceptance

```bash
bash -n tools/selfhost/compat/run_stage1_cli.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```
