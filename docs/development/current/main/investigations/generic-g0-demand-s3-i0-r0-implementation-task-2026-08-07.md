# GENERIC-G0-DEMAND-S3-I0-R0

Status: next implementation row; caller-zero only.
Date: 2026-08-07
Design authority: `generic-g0-demand-s3-design-task-2026-08-07.md`

## Objective

Add one consuming, test-only handoff:

```text
VerifiedLoopFamilyAdmissionWindowV1
  -> CanonicalLoopFamilySelectionV1
  -> VerifiedGenericRecipeDemandG0
```

The row must consume a selected Generic candidate exactly once, preserve the
canonical selector lease, and expose an AST-free source capability to the
future Generic Recipe producer. It must not create Recipe keys or touch
Builder/MIR.

## Allowed implementation

- add narrow move-out methods to the selector, candidate, observation, and
  handoff owners;
- remove the duplicate resolver lease from the test-only Generic handoff;
  derive its private brand from a borrow of the canonical selector lease;
- add the neutral `VerifiedGenericRecipeDemandG0` product and one issuer;
- add natural positive and typed negative tests through the five-row Ready /
  Selected(Generic) boundary;
- keep every production caller at zero and every test-only adapter explicit.

## Demand invariants

The demand owns exactly one canonical `VerifiedLoopFamilyWindowLeaseV1`, the
borrowed handoff brand, the moved typed source bundle, the moved post-loop
read, and selector profile/mode/coverage. It does not duplicate target or
role rows, and it does not issue `LoopBindingKeyV1`.

The issuer rejects other-family selections, foreign or mismatched lease
brands, frame/site/forest/BindingRef/tail conflicts, duplicate/uncovered
roles, and contradictory provenance. Out-of-window or opaque/incomplete
inputs remain typed `Unresolved`. S3 never issues `NoCandidate`.

## Explicit non-goals

No `RecipeBody`/`RecipeBlock`, AST or source-view retention, route ID/schedule,
legacy demand, retry/fallback, `LoopRecipeV1`, `LoopJoinSigV1`,
`LoopBindingKeyV1`, `ValueId`, `PHI`, `Builder`, `MIR`, physical preflight,
production selection, or legacy deletion.

## Acceptance gates

```text
cargo test --lib generic_g0 --features plugins
cargo test --lib generic_resolved_carrier_source_lease --features plugins
cargo check --lib --features plugins
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
bash tools/checks/lib/loop_family_observation_contract.sh
git diff --check
```

The implementation commit must update the exact `docs/reference/**` receipt,
Generic/Loop SSOTs, module READMEs, workstream, `CURRENT_STATE.toml`, and
current mirrors in the same commit. Public language activation remains zero.
Keep source/check files below 800 lines and this task below 1000 lines.
