# Callable Loop Production Logical Issuer S0

Status: `closed` — implementation receipt landed 2026-08-08 after accepted
`CALLABLE-LOOP-PRODUCTION-LOGICAL-ISSUER-D0`.

## Change

Promote the bounded callable single-loop source-map -> Recipe/JoinSig/After
co-seal from test-only to production scope. Reuse the existing Recipe verifier,
JoinSig elaborator, `require_after_binding`, and source-bound Core co-seal.
Remove no physical or legacy route in this row; there is no production caller
switch yet.

## Contract

```text
resolver SourceMap (move)
  -> exact seven-operation source relation DTO
  -> LoopRecipeArtifact + LoopRecipeVerifierV1
  -> LoopJoinSigElaboratorV1
  -> require_after_binding(root, B0, I64)
  -> issue_source_bound_core_v1
  -> move-only logical product
```

The exact mapping is owned by the D0 design SSOT. `CallableSingleLoopV1` is
diagnostic provenance only. `callable_recipe()` and `issue_*_for_test` remain
test-only. The issuer must not inspect AST, reconstruct names/ordinals, select
a route, create Builder/MIR/CFG/SSA/PHI state, or use retry/fallback.

## Implementation receipt

The logical issuer is now production-scoped but remains caller-zero: it is
compiled and available to a future ingress, while no production selector or
caller invokes it. It consumes the resolver-branded SourceMap exactly once,
constructs the fixed seven-operation profile Recipe through the profile
adapter, and delegates verification, JoinSig elaboration, After binding, and
source-bound Core sealing to their existing owners. `CallableSingleLoopV1`
remains diagnostics-only provenance.

The profile Recipe shape now lives in
`src/mir/compiler/callable_single_loop_recipe.rs`. The old
`callable_single_loop_recipe_shape.rs` is a test-only parity fixture wrapper;
the production issuer does not import a test-only builder. No new semantic
owner was introduced.

## Done

- production logical issuer compiles without `cfg(test)`;
- the production issuer has no AST/name re-walk and no Builder/MIR/physical
  imports or effects;
- source-map ownership, exact owner/frame/Scope relation, and seven operation
  source relations are consumed once;
- Recipe, JoinSig, After, binding/effect coverage, and prefix/Tail separation
  are verified by existing owners;
- missing/duplicate/foreign/unconsumed/unsupported evidence rejects before a
  function session;
- source unit drop and fresh request reuse remain valid;
- production issuer caller census remains zero; Builder/ValueId/BasicBlockId
  imports/effects remain zero;
- implementation updates `src/mir/compiler/README.md`,
  `docs/reference/mir/loop-recipe-contract.md`,
  `docs/reference/mir/generic-loop-stage-matrix.md`, current pointers, and
  migration/diagnostic notes in the same closeout commit;
- focused tests, `git diff --check`, rustfmt check, current-state guard, and
  MirBuilder replacement guard are green.

Focused receipt: `callable_single_loop_recipe_coseal` 4 tests,
`callable_single_loop_syntax_facts` 14 tests, and
`callable_single_loop_source_map` 6 tests pass; `cargo check --lib` is green.

## Stop

Do not open Prepared physicalization, ABI/Completion handoff, CFG/SSA/PHI,
selector/admission, production caller switch, Generic G0, retry/fallback, or
legacy deletion. If a relation cannot be represented by the existing Recipe,
JoinSig, or source-bound Core owner, return typed `NoSafeSlice` and revise D0.

The next design stop is
`CALLABLE-LOOP-PRODUCTION-PREPARED-INGRESS-D0`; this S0 does not authorize
physical implementation.
