---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: classify the native LLVM builder backend canary after P54.
Related:
  - tools/ROOT_SURFACE.md
  - tools/compat/README.md
  - tools/ny_mir_builder.sh
  - docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md
---

# P55 Native LLVM Builder Capsule Classification

## Goal

Close the root hold queue item for `tools/native_llvm_builder.py` without
turning the native backend canary into a daily compiler route.

This is a BoxShape cleanup slice. It only moves the private canary helper behind
its compat owner and updates the owner docs.

## Owner Reading

- `tools/ny_mir_builder.sh` owns the native route through explicit
  `NYASH_LLVM_BACKEND=native`.
- Native is a replay/canary keep lane, not the default MIR(JSON)->obj/exe
  backend.
- Native reference smokes call `tools/ny_mir_builder.sh`; they do not need the
  helper to remain at tools root.

## Decision

- move `tools/native_llvm_builder.py` to
  `tools/compat/native_llvm_builder.py`
- keep `tools/ny_mir_builder.sh` as the root user/build facade
- update `tools/compat/README.md` so the native builder has an explicit capsule
  owner
- remove the backend canary item from `tools/ROOT_SURFACE.md` root hold queue

## Non-goals

- do not change native LLVM lowering behavior
- do not promote native to the default backend
- do not touch PHI probes, Program(JSON v0) capsules, or smoke shortcut owners
  in this slice

## Acceptance

```bash
python3 -m py_compile tools/compat/native_llvm_builder.py
test ! -e tools/native_llvm_builder.py
rg -n 'tools/compat/native_llvm_builder.py' tools/ny_mir_builder.sh tools/compat/README.md
! rg -g '!docs/development/current/main/phases/phase-29cv/P5*.md' --fixed-strings \
  'tools/native_llvm_builder.py' \
  tools src lang Makefile README.md README.ja.md docs/guides docs/development/current/main
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
