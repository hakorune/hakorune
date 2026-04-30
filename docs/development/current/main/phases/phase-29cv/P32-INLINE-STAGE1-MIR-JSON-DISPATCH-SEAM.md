---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: inline the wrapper-local emit mir-json dispatch seam in the retired Stage1 CLI wrapper.
Related:
  - docs/development/current/main/phases/phase-29cv/P24-KEEPER-DELETE-LAST-ORDER.md
  - docs/development/current/main/phases/phase-29cv/P29-INLINE-RETIRED-STAGE1-WRAPPER-EXIT.md
  - tools/selfhost/compat/run_stage1_cli.sh
---

# P32 Inline Stage1 MIR JSON Dispatch Seam

## Goal

Finish another small wrapper-local cleanup in the retired Stage1 CLI wrapper.

After P28 and P29, `tools/selfhost/compat/run_stage1_cli.sh` still kept
`run_emit_mir_json()` as a single-use dispatch seam. The function only owned
wrapper-local argument validation for `emit mir-json` and then forwarded to the
existing Stage1 contract helper.

## Decision

- inline the `emit mir-json` wrapper-local argument validation into the dispatch
  branch
- delete `run_emit_mir_json()`
- keep the retired `emit program-json` redirect unchanged
- keep the eventual `stage1_contract_exec_direct_emit_mode ... emit-mir` call
  unchanged

## Non-goals

- do not change the Stage1 contract helper
- do not revive `--from-program-json` in this wrapper
- do not change the retired `emit program-json` redirect
- do not change proof routes or route ownership

## Acceptance

```bash
bash -n tools/selfhost/compat/run_stage1_cli.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```
