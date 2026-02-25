---
Status: Ready
Scope: return-in-loop minimal (stdlib `StringUtils.is_integer`)
Related:
- docs/development/current/main/phases/phase-29ar/README.md
- docs/development/current/main/design/return-in-loop-minimal-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md
---

# Phase 29ar P2: release adopt for is_integer

Goal: adopt the CorePlan path for the `is_integer` minimal subset in non-strict
builds, while keeping default logs unchanged.

## Non-goals

- No widening beyond the exact `is_integer` subset.
- No new env vars.
- No release tag output (strict/dev-only tags remain as-is).

## Implementation

1. Composer/router: enable non-strict adopt when the facts subset matches.
   - If the subset does not match, keep the existing behavior (no “silent”
     rerouting with new semantics).
2. Add a non-strict smoke that ensures:
   - exit code is unchanged (expected 0)
   - **raw output does not contain** `[coreplan/shadow_adopt:` tags
3. Wire the new smoke into the JoinIR regression pack.

## Files (expected touchpoints)

- `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`
- `src/mir/builder/control_flow/joinir/patterns/router.rs`
- `tools/smokes/v2/profiles/integration/joinir/phase29ar_string_is_integer_release_adopt_vm.sh` (new)
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `docs/development/current/main/phases/phase-29ar/README.md`
- `docs/development/current/main/phases/phase-29ae/README.md`

## Verification (acceptance)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

