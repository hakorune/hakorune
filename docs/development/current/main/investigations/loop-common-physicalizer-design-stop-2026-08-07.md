# Common Loop Physicalizer Design Stop

Status: `active; design-only stop after callable ABI/Prepared evidence`
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
