---
Status: Ready
Scope: Pattern6_NestedLoopMinimal strict/dev observability (no release change)
---

# Phase 29ap P9: Pattern6_NestedLoopMinimal — strict/dev shadow gate (facts-only)

## Goal

- Detect nested-loop shape via Plan Facts (observability only).
- In strict/dev, fail-fast with a freeze tag so legacy routing is not silently used.
- Release/default behavior stays unchanged.

## Nested minimal skeleton (SSOT)

- Outer loop: `loop(i < N)`
- Body contains an inner loop: `loop(j < M)`
- Inner body is effect-only (e.g., `sum = sum + 1; j = j + 1`).
- No breaks/continues/returns in the outer loop body.

Reference fixture:
- `apps/tests/phase1883_nested_minimal.hako`

## Implementation (facts-only; no new lowering)

1. Add a feature flag for nested loops
   - `src/mir/builder/control_flow/plan/facts/feature_facts.rs`
   - Add `nested_loop: bool` to `LoopFeatureFacts` and a helper detector.
   - Project to `CanonicalLoopFacts` so composer can read it.

2. Strict/dev fail-fast guard
   - `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`
   - Add a strict/dev guard that emits a freeze tag on `nested_loop == true`.
   - Use `Freeze::unstructured` with a short message.

3. Router uses the strict/dev guard before legacy fallback
   - `src/mir/builder/control_flow/joinir/patterns/router.rs`
   - In strict/dev, return the freeze error string (do not fall through to legacy).

## Gate / Smoke

- New smoke (strict/dev):
  - `tools/smokes/v2/profiles/integration/joinir/phase29ap_pattern6_nested_strict_shadow_vm.sh`
  - Requires tag `[plan/freeze:unstructured]` and non-zero exit.
- Gate: add to `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`.

## Acceptance

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git commit -m "phase29ap(p9): add nested minimal facts + strict shadow gate"`
