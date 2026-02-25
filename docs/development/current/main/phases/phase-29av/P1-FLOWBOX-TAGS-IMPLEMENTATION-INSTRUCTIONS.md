---
Status: Ready
Scope: Implement FlowBox schema tags (strict/dev only)
Related:
- docs/development/current/main/phases/phase-29av/README.md
- docs/development/current/main/design/flowbox-observability-tags-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29av P1: FlowBox schema tags (implementation)

## Objective

Emit stable FlowBox schema tags in strict/dev only, without changing release
output or behavior.

## Non-goals

- No new env vars.
- No new pattern-name-based tags (keep existing tags for compatibility, but
  introduce schema tags as the preferred observability contract).

## Step 1: Define a single emission helper (SSOT boundary)

Add a small helper module (recommended location):

- `src/mir/builder/control_flow/plan/observability/flowbox_tags.rs`

It should accept:
- strict/dev flag
- `box_kind` (derive from CorePlan variant)
- `features` (from canonical facts / outcome features if available)
- `via` (shadow/release)

Output:
- `eprintln!("[flowbox/adopt ...]")` (strict/dev only)

## Step 2: Emit tags at adopt boundaries (minimal set)

Add schema tags alongside existing tags at these SSOT boundaries:

- JoinIR routing boundary after composer adopt:
  - `src/mir/builder/control_flow/joinir/patterns/router.rs`
- Any strict/dev freeze path where we already emit `[plan/fallback:*]` or freeze tags.

## Step 3: Filter from generic smokes

Update `tools/smokes/v2/lib/test_runner.sh` `filter_noise` to strip:
- `^\[flowbox/adopt `
- `^\[flowbox/freeze `

so existing smoke expectations remain stable.

## Acceptance (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

