---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: move root PHI trace probes behind a debug capsule owner.
Related:
  - tools/ROOT_SURFACE.md
  - tools/debug/phi/README.md
  - docs/guides/phi-off-troubleshooting.md
  - docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md
---

# P56 PHI Trace Probes Debug Capsule

## Goal

Close the root hold queue item for the PHI trace probes without deleting the
diagnostic workflow.

This is a BoxShape cleanup slice. It narrows ownership and keeps the probes
manual/debug-only.

## Owner Reading

- `phi_trace_run.sh` is a one-shot PHI trace runner for `.hako` apps.
- `phi_trace_bridge_try.sh` is a legacy bridge probe for explicit
  Program(JSON v0) investigations.
- `phi_trace_check.py` is shared validator infrastructure for those probes and
  `tools/smokes/phi_trace_local.sh`.
- None of these files is a daily compiler proof route.

## Decision

- move the PHI trace probe cluster to `tools/debug/phi/`
- add `tools/debug/phi/README.md` as the capsule owner
- update the active troubleshooting guide and local PHI trace smoke wrapper
- remove PHI probes from the root tool surface hold queue

## Non-goals

- do not change PHI trace semantics
- do not run the heavy LLVM PHI trace workflow as part of this cleanup
- do not touch native LLVM, Program(JSON v0) capsules, or smoke shortcut owners
  in this slice

## Acceptance

```bash
bash -n tools/debug/phi/phi_trace_bridge_try.sh
bash -n tools/debug/phi/phi_trace_run.sh
bash -n tools/smokes/phi_trace_local.sh
python3 -m py_compile tools/debug/phi/phi_trace_check.py
test ! -e tools/phi_trace_bridge_try.sh
test ! -e tools/phi_trace_run.sh
test ! -e tools/phi_trace_check.py
rg -n 'tools/debug/phi/phi_trace_(bridge_try.sh|run.sh|check.py)' \
  tools/debug/phi/README.md docs/guides/phi-off-troubleshooting.md tools/smokes/phi_trace_local.sh
! rg -g '!docs/development/current/main/phases/phase-29cv/P5*.md' --fixed-strings \
  'tools/phi_trace_run.sh' \
  tools src lang Makefile README.md README.ja.md docs/guides docs/development/current/main
! rg -g '!docs/development/current/main/phases/phase-29cv/P5*.md' --fixed-strings \
  'tools/phi_trace_bridge_try.sh' \
  tools src lang Makefile README.md README.ja.md docs/guides docs/development/current/main
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
