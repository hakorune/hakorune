---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive unreferenced root compat/probe utility helpers.
Related:
  - docs/development/current/main/phases/phase-29cv/P49-ARCHIVE-UNREFERENCED-MANUAL-UTILITY-HELPERS.md
  - tools/archive/manual-tools/README.md
---

# P50 Archive Unreferenced Compat Probe Utilities

## Goal

Continue root surface cleanup by moving zero-reference compat/probe helpers out
of active `tools/`.

## Decision

Move these helpers to `tools/archive/manual-tools/`:

- `tools/compare_harness_on_off.sh`
- `tools/joinir_ab_test.sh`
- `tools/llvmlite_check_deny_direct.sh`
- `tools/python_unit.sh`

These remain available as manual diagnostics, but they are not active gates or
current compat capsules.

## Non-goals

- do not archive documented current probe helpers
- do not change llvmlite or JoinIR behavior
- do not run these historical probes as acceptance

## Acceptance

```bash
bash -n \
  tools/archive/manual-tools/compare_harness_on_off.sh \
  tools/archive/manual-tools/joinir_ab_test.sh \
  tools/archive/manual-tools/llvmlite_check_deny_direct.sh \
  tools/archive/manual-tools/python_unit.sh
! rg -g '!docs/development/current/main/phases/phase-29cv/P50-ARCHIVE-UNREFERENCED-COMPAT-PROBE-UTILITIES.md' --fixed-strings \
  -e 'tools/compare_harness_on_off.sh' \
  -e 'tools/joinir_ab_test.sh' \
  -e 'tools/llvmlite_check_deny_direct.sh' \
  -e 'tools/python_unit.sh' \
  docs/development/current/main docs/development/testing tools src lang Makefile dev README.md README.ja.md docs/reference docs/guides
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
