---
Status: Ready
Scope: Minimal `match` (BranchN) subset - Return-only (no join) in strict/dev
Related:
- docs/development/current/main/phases/phase-29at/README.md
- docs/development/current/main/design/match-branchn-skeleton-ssot.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29at P3: `match` minimal subset (Return-only; no join)

## Objective

Prove that `match` is representable as `CorePlan::BranchN` end-to-end **without**
introducing post-phi join payload yet.

We intentionally pick a subset that needs no join:
- each arm ends with `return <const>` (or return of a simple expression),
- so there is no “merge value” after the match.

This phase is strict/dev first:
- strict/dev: CorePlan adopt is enabled + a stable tag is emitted.
- non-strict/release: behavior must remain unchanged.

## Non-goals

- No `match` value-join (that is a later phase once BranchN exists).
- No `match` inside loop-body effect-only constraints.
- No new env vars.
- No by-name dispatch.

## Step 1: Facts extraction (tiny, conservative)

Add a new facts module that detects:

- Function-level return of match expression:
  - `Return(MatchExpr { scrutinee, arms, else_expr })`

Constraints (subset):
- `scrutinee` is a simple variable or integer literal.
- Each arm and else yields a returnable literal (e.g., `Integer`, `Bool`).
- Arms are >= 2.

Recommended files:
- `src/mir/builder/control_flow/plan/facts/pattern_match_return_facts.rs`
- Register in `src/mir/builder/control_flow/plan/facts/mod.rs`
- Wire into the existing facts aggregation entry used by `single_planner` outcome.

Return policy:
- shape mismatch => `Ok(None)`
- looks like match but violates subset => strict/dev `Freeze::unsupported` (or `Freeze::contract`), release `Ok(None)`

## Step 2: Build CorePlan::BranchN (Return-only)

Implement a composer helper that builds:

- `CorePlan::BranchN` where each arm plans are:
  - `CorePlan::Exit(CoreExitPlan::Return(Some(value_id)))`

No block_params needed (no join).

Where to plug (recommended):
- `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`
  - strict/dev only `try_shadow_adopt_match_return(...) -> Option<ShadowAdoptOutcome>`
  - stable tag: `[coreplan/shadow_adopt:match_return]`

Alternative (also acceptable for P3):
- A tiny strict/dev-only fast-path in `src/mir/builder/stmts.rs` inside
  `build_return_statement`, only when `return (MatchExpr ...)` matches the subset.
  In this variant, add a minimal “standalone CorePlan lowering” entry in
  `src/mir/builder/control_flow/plan/lowerer.rs` that does not require loop context.

## Step 3: Wiring (strict/dev only)

Route/adopt only when:
- planner selected a plan for the region (or outcome contains the match facts),
- and match-return facts are present.

If facts are present but composer rejects:
- strict/dev fail-fast with a stable reason tag (follow Phase 29as policy).

## Step 4: Fixture + smoke

Add a minimal fixture:
- `apps/tests/phase29at_match_return_min.hako`
  - match on an integer with >=2 arms + else
  - each branch returns a constant

Add integration smoke (strict):
- `tools/smokes/v2/profiles/integration/joinir/match_return_strict_shadow_vm.sh`
  - require tag `[coreplan/shadow_adopt:match_return]`
  - assert no `[plan/fallback:` tags in raw output

Wire into:
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `docs/development/current/main/phases/phase-29ae/README.md`

## Acceptance (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
