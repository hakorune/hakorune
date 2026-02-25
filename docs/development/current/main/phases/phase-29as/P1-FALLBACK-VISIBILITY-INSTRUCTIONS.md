---
Status: Ready
Scope: CorePlan purity Stage-1 (strict/dev fallback visibility)
Related:
- docs/development/current/main/phases/phase-29as/README.md
- docs/development/current/main/design/coreplan-purity-stage1-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29as P1: strict/dev fallback visibility

Goal: for gate-target shapes, ensure strict/dev never silently falls back by
surfacing fallback via a stable tag or via Freeze, while keeping release logs
unchanged.

## Non-goals

- No new env vars.
- No by-name dispatch hacks.
- No release tag output (strict/dev only).

## Implementation outline

1. Pick a single “fallback boundary” as SSOT (one file/function).
2. Add stable raw tag lines (strict/dev only), e.g.:
   - `[plan/fallback:planner-none]`
   - `[plan/fallback:composer-reject]`
3. For gate-target candidate shapes only, prefer Freeze over “tag and continue”.

## Acceptance (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
