---
Status: Active design boundary
Date: 2026-08-03
Decision: accepted — broad partial cutover rejected; scoped bridge permitted
Scope: Generic-isolated Loop migration before the final atomic cutover
Related:
  - docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - docs/development/current/main/design/joinir-generic-post-effect-debt-classification-ssot.md
  - docs/development/current/main/design/joinir-loop-pre-effect-product-ssot.md
---

# Scoped non-Generic Loop cutover

## Decision

The proposal “switch every non-Generic route first and leave Generic on the
legacy scheduler” is **not safe in the current architecture**. It is rejected
as a broad production cutover.

A narrower bridge is permitted only for a source row whose production-derived
raw schedule is a singleton, whose route is non-Generic, and whose verified
portable recipe/JoinSig/PHI/physicalizer path is already complete. Generic and
all overlapping rows remain on the existing witness scheduler. This bridge is
optional migration work, not the final execution SSOT.

The final cutover remains one atomic operation:

```text
M10a: optional disjoint singleton bridge (Generic legacy retained)
M10b: all-route verified Recipe consumer + old scheduler/PHI deletion
```

`M10b` still requires the Generic D2 winner/disjointness proof. M10a may make
the Generic proof non-critical for the selected singleton pilot, but it cannot
close M4 or authorize global Retry/fallback deletion.

## Why broad partial cutover is rejected

The current `route_loop` issues one production frame from one raw schedule and
then executes the legacy witness. The schedule is not partitioned by semantic
family. For example, a simple-while source can expose:

```text
[LoopSimpleWhile, GenericLoopV0]
```

`diagnostic_effective` hides some Generic rows but is not execution authority.
If a new non-Generic physicalizer mutates the candidate and then falls through
to Generic, the fallback is a dirty-candidate retry. If it freezes instead,
legacy programs that previously reached Generic may change behavior. The
whole compile candidate is an abort boundary, not a route rollback boundary.

The physical side is not ready for a broad split either. Route-specific PHI
materializers, `LoopHeaderPhiBuilder`, and legacy SSA repair are still live;
M6 has not established the single verified-JoinSig PHI owner. Wiring a new
non-Generic physicalizer now would create two physical authorities even if
Generic itself stayed legacy. Reusing `PlanLowerer` would only wrap legacy
authority, not consume the portable Recipe contract.

## Allowed bridge contract

The bridge may be opened only when all conditions hold:

1. The shared production preflight frame proves a singleton raw schedule.
2. The selected route is non-Generic and has no raw suffix or overlap.
3. A pure policy disposition is issued before any Builder effect; it does not
   derive authority from `diagnostic_effective` or a route name alone.
4. The complete verified Recipe -> JoinSig -> shared PHI -> physicalizer path
   exists for that row. The physicalizer runs inside the existing unpublished
   compile candidate and returns terminal success or `Freeze`.
5. A bridge failure drops the whole candidate. It never dispatches the old
   scheduler and never retries through Generic.
6. Generic rows remain byte-for-byte on the old witness path; no portable
   output is passed to the legacy handlers.
7. Candidate reuse, MIR/PHI/type/result parity, late-failure discard, fresh
   reuse, and production caller census are green.

The bridge must have static guards for:

```text
portable singleton branch -> old handlers = 0
Generic legacy branch -> portable outputs = 0
portable physicalizer production callers = exactly 1
partial bridge fallback/retry = 0
```

Before M6 closes, this is test-only. Production M10a is allowed only after
the shared CFG/JoinSig/PHI owner exists; an explicit temporary-owner ledger is
not a substitute for that gate.

## Ordered tasks

### N0 — singleton/disjoint census

Use full `try_build_outcome` -> shared frame -> raw selector. Record every
candidate row that is genuinely singleton and non-Generic. A direct extractor
pair is not enough; the production selector must show no Generic suffix.

### N1 — shared-owner prerequisite

Complete the M5 caller-zero pilot and M6 shared CFG/JoinSig/PHI owners. Keep
all portable production callers at zero until this gate is green.

### N2 — `JOINIR-LOOP-NONGENERIC-DISJOINT-SUBSET-CUTOVER0`

Optionally wire one or more N0 rows through the shared portable physicalizer.
This is M10a only. It does not remove the ordered scheduler, Generic receipts,
or any old PHI writer globally.

### N3 — final M10b

After M4/D2 and M7-M9 close, switch the whole `route_loop` authority once and
delete the old scheduler, selected JoinIR edges, Retry/fallback, and legacy PHI
writers in the same atomic commit.

## Stop conditions

Stop the bridge and keep the old path when a singleton proof is missing, a
Generic suffix exists, a physicalizer returns `Option`, a failure would need
legacy retry, PHI ownership is duplicated, or parity/candidate isolation is
unmeasured. Do not rename an overlapping row as “non-Generic” to make it fit.

