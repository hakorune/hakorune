# Common Loop Physicalizer Design Stop

Status: `closed; worker-reviewed boundary fixed; next row is Prelude argument receipt P0`
Date: 2026-08-07
Parent: `LOOP-PHYSICAL-PREPARE-STATIC-CALL-FIXTURE-D0`
Authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Purpose

The callable static-prefix source cells are now complete through
declaration-derived ABI and one positive Prepared relation. Before opening a
physicalizer implementation, freeze the exact common boundary once:

```text
Prepared profile product
  -> fresh function lowering session
  -> ReadyLoopEntry receipt
  -> common recursive Loop physicalizer
  -> callable Tail / Completion owner
  -> one typed function finish
  -> DraftSeal
```

## Design questions to close

- the common physicalizer consumes only `VerifiedLoopPhysicalDemandV1` and a
  private entry receipt, never callable Tail/ABI/Completion;
- one fresh session owns CFG, Binding SSA, PHI transaction, and Completion
  consumption; no same-session retry or repair is allowed;
- physical failure discards the unpublished function session and restores the
  caller once;
- `finish_for_draft_seal` is the only issuer of `ReadyFunctionDraftSealV1`;
- callable Tail and Loop After remain distinct, and DraftSeal/collector remain
  the sole publication owners;
- the physicalizer has one recursive Loop algebra and no profile-name or
  legacy-route dispatch.

The worker audit fixed two mandatory pre-canary locks:

- `VerifiedLoopPhysicalDemandV1` must cross the boundary through a consuming,
  move-only API. Borrowing, cloning, a second co-seal, or MIR reconstruction
  is rejected.
- Prelude/input materialization must use an existing resolver-issued argument
  product, or a separate typed argument receipt that records each required
  `BindingRef`. The current arity-only `SourceCallBoundaryShapeV1` is not
  enough; AST reread, name lookup, and arity-only reconstruction are forbidden.

The private `ReadyLoopEntryV1` receipt therefore proves both facts: all exact
logical entry keys are installed in the fresh session, and each required
`BindingRef` has its verified entry materialization. It remains session-local,
non-Clone, and single-use.

The concrete Prelude argument product is now fixed as
`VerifiedCallablePreludeArgumentListV1`: ordered, move-only rows containing
`SourceExprSiteV1`, resolver-issued `BindingRefV1`, and exact `i64` ABI. The
first issuer accepts only `ResolvedLexicalRefV1::Local` rows owned by the
caller; Upvar, literal, nested expression, and unknown forms are typed
`NoSafeSlice`. This is a callable-boundary product and is consumed before
`ReadyLoopEntryV1`; the common physicalizer never receives it.

## Explicit non-goals

```text
Builder / ValueId / BasicBlockId implementation -> not opened here
production selector / I0                         -> not opened here
retry / fallback / legacy deletion               -> not opened here
new Recipe kinds or profile-specific physicalizers -> forbidden
```

## Exit condition

Only after the design questions and the exact pre-effect/effect boundary are
recorded may one bounded physicalizer canary task open. If the boundary is
not mechanically enforceable, stop and revise the design; do not probe by
adding another route or fixture.

The next implementation prerequisite after this stop is intentionally small:

```text
resolver-backed static fixture
  -> issue/consume Prelude argument receipt
  -> no AST reread or name lookup
```

This row remains caller-zero and pre-effect. After it closes, the common
recursive physicalizer canary opens. G0, production selection, retry/fallback
retirement, and legacy deletion are later rows.
