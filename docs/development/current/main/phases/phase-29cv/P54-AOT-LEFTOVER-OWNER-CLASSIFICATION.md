---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: classify the remaining root-level AOT leftovers after P53.
Related:
  - tools/ROOT_SURFACE.md
  - tools/archive/manual-tools/README.md
  - docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md
  - docs/guides/cranelift_aot_egui_hello.md
---

# P54 AOT Leftover Owner Classification

## Goal

Resolve the AOT hold queue without treating every old AOT helper as archiveable.

This slice keeps current user-facing AOT entrypoints in root and archives only the
historical wrapper that no longer has a current owner.

## Owner Reading

- `tools/build_aot.sh` is still owned by the current root READMEs and the current
  Cranelift AOT guide.
- `tools/build_aot.ps1` is still owned by the same current docs as the explicit
  Windows entrypoint.
- `tools/build_python_aot.sh` is only a thin wrapper around `build_aot.sh` plus a
  Python plugin build step, and its remaining non-self reference is the
  historical `tools/smokes/jit-migration-plan.md` note.

## Decision

- keep `tools/build_aot.sh` in root as a protected build helper
- keep `tools/build_aot.ps1` in root as a protected platform helper
- move `tools/build_python_aot.sh` to
  `tools/archive/manual-tools/build_python_aot.sh`
- update `tools/ROOT_SURFACE.md` so the AOT hold queue is closed

Archive metadata for `build_python_aot.sh`:

- `original_path`: `tools/build_python_aot.sh`
- `archived_on`: `2026-05-01`
- `archived_by_card`: `phase-29cv/P54-AOT-LEFTOVER-OWNER-CLASSIFICATION.md`
- `last_known_owner`: `tools/smokes/jit-migration-plan.md` historical note
- `delete_after`: after two cleanup batches or 30-60 days with no active refs
- `restore_command`: `git checkout HEAD^ -- tools/build_python_aot.sh`
- `delete_blocker`: any new current README/guide/gate owner

## Non-goals

- do not change AOT build behavior
- do not archive `tools/build_aot.sh` or `tools/build_aot.ps1`
- do not touch native LLVM canary or PHI probe owners in this slice

## Acceptance

```bash
bash -n tools/archive/manual-tools/build_python_aot.sh
! test -e tools/build_python_aot.sh
rg -n 'tools/build_aot.sh|tools/build_aot.ps1' README.md README.ja.md docs/guides/cranelift_aot_egui_hello.md tools/ROOT_SURFACE.md
! rg -g '!docs/development/current/main/phases/phase-29cv/P5*.md' --fixed-strings \
  'tools/build_python_aot.sh' \
  docs/development/current/main docs/guides src lang Makefile README.md README.ja.md
! rg -g '!docs/development/current/main/phases/phase-29cv/P5*.md' --fixed-strings \
  'bash tools/build_python_aot.sh' \
  tools src lang README.md README.ja.md Makefile docs/guides docs/development/current/main
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
