---
Status: Complete
Scope: return-in-loop minimal (stdlib is_integer)
Related:
- docs/development/current/main/design/return-in-loop-minimal-ssot.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/planfrag-ssot-registry.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/phases/phase-29aq/README.md
---

# Phase 29ar: return-in-loop minimal (stdlib is_integer)

Goal: handle `StringUtils.is_integer` return-in-loop minimal with explicit
strict fail-fast reject + release adopt coverage, without widening general loop semantics.

## Scope

- Target: StringUtils.is_integer minimal shape only.
- No nested loops, no exceptions/unwind, no general return-heavy loops.
- Keep Ok(None) for non-matching shapes; strict/dev uses fail-fast only for
  explicitly gated contradictions.

## Gate / Commands (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ar_string_is_integer_min_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ar_string_is_integer_release_adopt_vm.sh`

## Fixtures / smokes (SSOT)

- `apps/tests/phase29ar_string_is_integer_min.hako`
- `tools/smokes/v2/profiles/integration/joinir/phase29ar_string_is_integer_min_vm.sh`
- `tools/smokes/v2/profiles/integration/joinir/phase29ar_string_is_integer_release_adopt_vm.sh`

## Plan

- P0: docs-first (phase README + return-in-loop minimal SSOT). ✅
- P1: implement `ExitIfReturn` minimal vocabulary + verify/lower + strict/dev
  fail-fast reject contract + fixture/smoke gate. ✅
- P2: switch to release adopt when stable (keep logs unchanged; add a non-strict
  smoke that asserts tag absence in raw output). ✅
- P3: closeout (docs-only): mark complete, fix pointers, and keep gate SSOT green. ✅

## Summary (P0–P2)

- Added `CoreEffectPlan::ExitIfReturn` for loop-body early return.
- Strict/dev fail-fast reject for `is_integer` (`[vm-hako/unimplemented] ... newbox(StringUtils)`).
- Release adopt for `is_integer` with non-strict smoke guarding tag absence.

## Unsupported / Deferred

- General return-heavy loops (non-stdlib).
- Nested return-in-loop and multiple exits.
