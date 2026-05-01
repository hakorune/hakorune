---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: protected-category policy and first weak-ref helper archive.
Related:
  - docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md
  - docs/development/refactoring/refactor-roadmap.md
  - tools/archive/manual-smokes/README.md
  - tools/archive/manual-tools/README.md
---

# P52 Root Helper Protected Category Policy

## Goal

Turn the post-P51 external review into a repo-local cleanup rule:

```text
no active refs
+ no current PASS gate
+ no compat capsule owner
+ no protected category owner
= archive/delete candidate
```

This keeps root helper cleanup fast while protecting real platform/build/CI
entrypoints.

## Protected Category Inventory

| Helper | Owner evidence | Category | Decision |
| --- | --- | --- | --- |
| `tools/core_ci.sh` | `CLI_TESTING_GUIDE.md` | CI helper | keep; owns the golden MIR chain |
| `tools/ci_check_golden.sh` | called by `core_ci.sh` | CI helper | keep |
| `tools/compare_mir.sh` | called by `ci_check_golden.sh` | CI helper | keep |
| `tools/snapshot_mir.sh` | called by `compare_mir.sh` | CI helper | keep |
| `tools/build_plugins_all.sh` | P51 non-goal | build helper | keep until plugin-build owner retires it |
| `tools/build_llvm.ps1` | Windows LLVM wrappers | platform build helper | keep until Windows LLVM owner retires it |
| `tools/build_aot.ps1` | README / guide refs | legacy/platform build helper | hold; classify before archive |
| `tools/build_python_aot.sh` | example / migration-plan refs | legacy build helper | hold; depends on old AOT route |
| `tools/native_llvm_builder.py` | called by `ny_mir_builder.sh` | backend canary | hold; capsule-classify before archive |
| `tools/phi_trace_run.sh` | PHI troubleshooting guide | debug probe | hold; move only with PHI trace owner decision |
| `tools/mir13-migration-helper.sh` | old refactor roadmap mention | historical migration helper | archive now |

## Decision

- Add `tool-entrypoint-lifecycle-ssot.md` as the durable policy.
- Add delete policy text to manual archive READMEs.
- Move `tools/mir13-migration-helper.sh` to
  `tools/archive/manual-tools/mir13-migration-helper.sh`.
- Repoint the remaining refactoring roadmap mention to the archived helper.

## Non-goals

- Do not archive the CI/golden MIR chain.
- Do not archive Windows LLVM or AOT build helpers in this slice.
- Do not archive `tools/native_llvm_builder.py`; it is still called by
  `tools/ny_mir_builder.sh` under the native LLVM backend route.
- Do not change compiler or backend behavior.

## Acceptance

```bash
bash -n tools/archive/manual-tools/mir13-migration-helper.sh
! test -e tools/mir13-migration-helper.sh
! rg -g '!docs/development/current/main/phases/phase-29cv/P52-ROOT-HELPER-PROTECTED-CATEGORY-POLICY.md' --fixed-strings \
  '`tools/mir13-migration-helper.sh`' \
  docs/development/current/main docs/development/refactoring tools src lang Makefile README.md README.ja.md
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
