---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive unreferenced root manual utility/debug helpers outside the active tools surface.
Related:
  - docs/development/current/main/phases/phase-29cv/P48-ARCHIVE-HAKO-CHECK-DEADCODE-SHIM.md
  - tools/archive/manual-tools/README.md
---

# P49 Archive Unreferenced Manual Utility Helpers

## Goal

Move zero-reference root utilities that are manual, historical, or potentially
destructive out of the active `tools/` surface.

## Decision

Create `tools/archive/manual-tools/` and move these helpers there:

- `tools/archive_rust_llvm.sh`
- `tools/clean_root_artifacts.sh`
- `tools/codex-keep-two.sh`
- `tools/dep_tree.sh`
- `tools/parallel-refactor-nyash.sh`
- `tools/trace_last_fn_from_log.sh`
- `tools/vm_stats_diff.sh`

These are not active gates, not current build helpers, and not compat capsules.

## Non-goals

- do not archive active build helpers
- do not archive current documented probe helpers
- do not execute these manual utilities as acceptance

## Acceptance

```bash
bash -n \
  tools/archive/manual-tools/archive_rust_llvm.sh \
  tools/archive/manual-tools/clean_root_artifacts.sh \
  tools/archive/manual-tools/codex-keep-two.sh \
  tools/archive/manual-tools/dep_tree.sh \
  tools/archive/manual-tools/parallel-refactor-nyash.sh \
  tools/archive/manual-tools/trace_last_fn_from_log.sh \
  tools/archive/manual-tools/vm_stats_diff.sh
! rg -g '!docs/development/current/main/phases/phase-29cv/P49-ARCHIVE-UNREFERENCED-MANUAL-UTILITY-HELPERS.md' --fixed-strings \
  -e 'tools/archive_rust_llvm.sh' \
  -e 'tools/clean_root_artifacts.sh' \
  -e 'tools/codex-keep-two.sh' \
  -e 'tools/dep_tree.sh' \
  -e 'tools/parallel-refactor-nyash.sh' \
  -e 'tools/trace_last_fn_from_log.sh' \
  -e 'tools/vm_stats_diff.sh' \
  docs/development/current/main docs/development/testing tools src lang Makefile dev README.md README.ja.md docs/reference docs/guides
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
