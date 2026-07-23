# RAW-SOURCE0 LOWER ROOT0 — CALLMAIN0 design question

Status: **Design stop — callable-Main compatibility child is not yet wired**  
Date: 2026-07-24  
Scope: the sealed Raw `Selected`/`NotSelected` callable-Main boundary only.

CHILDREN0-S0 is closed as a pre-root static-helper owner. It deliberately
does not reserve, lower, or collect the optional callable `Main.main/N`
compatibility child. BODY0 must not begin until this selection and failure
boundary is fixed.

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/cut0-i0-raw-source0-lower-root-consultation-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-raw-source0-lower-root-children0-s0-execution-task-2026-07-24.md`
- `src/mir/compiler/raw_source_binding.rs`
- `src/mir/compiler/raw_root_children.rs`
- `src/mir/builder/raw_root_physical.rs`

## Current boundary

`RawSourceContinuationV1` already seals the compatibility disposition:

```text
NotSelected
Selected
```

The source projection may retain a callable-Main locator even when the
disposition is `NotSelected`. Locator presence is not selection authority.
CHILDREN0 retains the disposition and leaves the callable child untouched.

The next owner must consume the CHILDREN0 completion product without opening a
second shell, collector, ledger, or Builder session:

```text
RawChildrenCompleteInvocationV1
  -> CALLMAIN0 selection terminal
  -> RawCallableMainReadyInvocationV1
  -> BODY0 inline root-body owner
```

No public ingress, production consumer, root-body lowering, Main/condition
batch, drain, finalizer, postprocess, external commit, retry, or fallback is
allowed by this consultation.

## Questions to decision-lock

### Q1 — selection authority

Which fact may select the compatibility child?

```text
1. Read only RawSourceContinuationV1::callable_main().
   NotSelected performs no reservation, descent, or receipt production.
   Selected retrieves the exact sealed locator from the source plan.

2. Treat a present callable-Main locator as Selected.

3. Try the child opportunistically and continue with the inline root body if
   it fails.
```

Recommendation: **1**. A locator is source identity; the continuation
disposition is the route policy. They must not be conflated.

### Q2 — physical owner

Where does the selected child borrow shell/collector/ledger state?

```text
1. Add one consuming terminal to RawRootPhysicalStateV1. It opens a short
   child loan internally and returns a named success/rejection product.

2. Export `(shell, collector, ledger)` to compiler code and rebuild a new
   child owner there.

3. Reuse RawDraftInvocationV1 or MainPending/MainCaptured as an adapter.
```

Recommendation: **1**. The physical owner remains the sole loan authority;
Main-only state and the disconnected child owner remain unused.

### Q3 — sequencing

When may inline root-body lowering start?

```text
1. Selected callable Main must validate, reserve, lower, admit, restore, and
   ledger-complete first. Inline root-body lowering starts only after success.

2. Lower inline root body first, then attempt callable Main.

3. Run both paths and keep whichever succeeds.
```

Recommendation: **1**. Selected failure aborts the whole unpublished root
owner and prevents body descent; there is no silent omission or fallback.

### Q4 — role and evidence

How is the compatibility child distinguished from static helpers?

```text
1. Derive one `CallableMainCompatibility` work request from the exact sealed
   locator and continuation disposition. Retain its branded receipt beside
   the CHILDREN0 successful prefix; do not add it to the BODY0 tracker.

2. Reuse `StaticMethod` and infer the role from the symbol spelling.

3. Re-read the source catalog after lowering and repair the role there.
```

Recommendation: **1**. Role is semantic source authority, not a symbol
blacklist or post-hoc catalog reconstruction.

### Q5 — failure product

What is returned when selected callable Main fails?

```text
1. A discard-only rejected owner retains the exact CHILDREN0 completion,
   physical state, ledger state, failed locator, and typed Primary/Cleanup/
   Admission cause. It exposes inspection and discard only.

2. Drop the successful helper prefix and return a bare error.

3. Mark callable Main omitted and continue to BODY0.
```

Recommendation: **1**. The selected child is part of the same Raw root
authority and cannot be downgraded after failure.

## Required decision output

```text
Decision: CALLMAIN-prime-r1 (or another candidate)
Q1 = ...
Q2 = ...
Q3 = ...
Q4 = ...
Q5 = ...
first executable row = ...
non-claims = ...
```

The smallest executable row after the decision should be a disconnected
CALLMAIN0 selection/physical terminal. It must retain production consumers at
zero and leave BODY0, root batching, drain, finalization, postprocess,
external commit, public ingress, JSON behavior, retry, and CUT0 activation
untouched.
