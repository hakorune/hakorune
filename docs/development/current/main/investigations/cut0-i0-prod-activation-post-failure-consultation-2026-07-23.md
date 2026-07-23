# CUT0-I0 POST-FAILURE0 Consultation

Status: **Decision locked and natural row closed — Candidate NF-prime-r1 selected; atomic CUT0/G0 next**
Date: 2026-07-23
Scope: decide the smallest typed failure evidence needed after POST-P0 and
POST-EVIDENCE0, before any production outer executor or atomic CUT0 wiring.

Related:

- `docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-prod-activation-owner-retention0-post-consultation-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-prod-activation-p0-r1-failure-consultation-2026-07-23.md`

## One-paragraph question

`ModulePostprocessOwnerV1` now retains a discard-only rejected owner for its
existing fatal optimizer-diagnostic, contract-refresh, and canonical final
verification branches. The real route currently has no sanctioned deterministic
way to force every stage: optimizer diagnostics depend on existing environment
policy, contract refresh needs a naturally invalid module-level carrier, and RC
insertion is currently infallible. Before CUT0, should POST-FAILURE0 add a
test-only sealed failure disposition, rely only on naturally invalid facts and
the existing environment policy, or keep the remaining stage matrix as an
explicit non-claim? The answer must not add an ambient fault switch, malformed
source plan, catch-unwind policy, retry, rollback clone, or production consumer.

## Current evidence

Already closed:

```text
canonical final-verifier failure
  -> invalid MIR edge
  -> RejectedModulePostprocessV1

Raw pre-transform verifier error
  -> reportable ModuleVerificationEvidenceV1::Raw

postprocess owner
  -> exact unpublished input retained
  -> discard-only
```

Still unproven:

```text
optimizer diagnostics -> typed rejected owner on the real route
contract refresh     -> typed rejected owner on the real route
RC failure           -> no failure API exists
```

The existing optimizer policy reads `NYASH_OPT_DIAG_FAIL` and
`NYASH_OPT_DIAG_FORBID_LEGACY`. These are existing diagnostic inputs, not a
new POST-FAILURE0 authority, but using them as the only fixture control would
make the result depend on ambient process state. RC insertion, semantic
refresh, and callsite canonicalization are currently infallible APIs.

## Questions for decision

### Q1 — optimizer failure authority

Which evidence may force an optimizer-stage rejection?

```text
1. Existing policy plus a naturally diagnostic MIR shape:
   use the existing optimizer diagnostic count and explicit test-scoped
   policy setup; do not add a new fault field or route. The fixture must save
   and restore the existing policy input and remain disconnected.

2. Test-only sealed postprocess disposition:
   add one compiler-private, non-ambient test product consumed by the
   postprocess owner. This is deterministic but introduces a second failure
   authority before CUT0.

3. No new optimizer fixture:
   retain the existing stage-order and final-verifier proof; optimizer
   failure remains an explicit non-claim until a real production diagnostic
   route exists.
```

### Q2 — contract-refresh failure

How should contract failure be covered?

```text
1. Natural invalid module fact only:
   construct a real post-lowering module whose existing contract refresh
   rejects, without changing the contract API or adding a fault hook.

2. Reuse the same test-only sealed disposition selected for Q1:
   one failure authority covers optimizer and contract stages.

3. Keep contract failure deferred:
   record the rejected owner implementation as complete, but do not claim a
   contract-failure fixture until a naturally invalid route is identified.
```

### Q3 — RC and refresh operations

What is the correct boundary while RC/refresh APIs are infallible?

```text
1. Keep RC/refresh failure outside POST-FAILURE0:
   preserve the existing infallible calls and explicitly record that RC
   failure retention is not claimed.

2. Make RC and every refresh operation fallible first:
   larger semantic refactor before CUT0; no silent error mapping.

3. Add test-only panic/fault wrappers:
   rejected because they create a duplicate failure authority and do not
   prove typed RC failure semantics.
```

### Q4 — row boundary and production claim

What may POST-FAILURE0 close?

```text
1. Close only the naturally observable matrix:
   final verifier, Raw reportable verifier error, and any selected natural
   optimizer/contract fixture. Keep the rest as non-claims.

2. Require universal optimizer/contract/RC coverage before closing:
   activation remains stopped until every stage is made fallible.

3. Wire the outer executor to observe failures:
   rejected because production consumers remain zero until atomic CUT0.
```

## Non-claims while stopped

```text
real-route optimizer failure matrix = 0 until Q1
real-route contract failure matrix  = 0 until Q2
typed RC failure retention           = 0
panic-to-no-commit on real route     = 0
production postprocess/outer executor = 0
atomic CUT0/G0                       = 0
```

## Required response

Select Q1–Q4, identify which evidence remains disconnected-only, and define
one smallest executable row. The decision must reuse the existing
`RejectedModulePostprocessV1`, preserve `PostprocessEvidenceSealV1`, and keep
all production consumers at zero.

Until this consultation closes, do not add a fault field, environment toggle,
malformed source plan, `catch_unwind`, RC fallibility refactor, retry,
fallback, or public-ingress wiring.

## NF-prime-r1 decision closeout (2026-07-23)

Candidate **NF-prime-r1** is selected:

```text
Q1 optimizer  = 1  existing NYASH_OPT_DIAG_FAIL plus natural diagnostic MIR
Q2 contract   = 1  orphan Static Table plan without a source spec
Q3 RC/refresh = 1  keep existing APIs infallible; typed failure is not claimed
Q4 closeout   = 1  close only the naturally observable failure matrix
```

The optimizer fixture starts from a real canonical trivial route, adds one
existing diagnostic `Call` shape, and scopes the existing process policy with
a test-only save/restore mutex. It removes optimizer-disable and strict
planner-required inputs for the fixture and restores every prior environment
value on drop. No production field, new environment variable, or fault
disposition is introduced.

The contract fixture starts from the same real canonical route and adds only a
post-lowering `StaticDataPlan` without its existing
`StaticTableContractSpec`. The source-bound plan and lowering route remain
valid; the existing Static Table contract refresh rejects the orphan fact with
`[type/static_table_contract_spec_missing]`.

RC insertion, rune refresh, semantic refresh, and callsite canonicalization
remain infallible and are explicit non-claims. Real-route panic evidence is
also not claimed. `RejectedModulePostprocessV1` remains the sole fatal owner;
`PostprocessEvidenceSealV1` remains the sole successful handoff evidence.

The smallest executable row is `POST-FAILURE0-NATURAL-P0`. It adds only the
two test-only natural fixtures, a measured guard, and this closeout. Production
postprocess, external commit, outer executor, public ingress, retry, fallback,
and atomic CUT0 remain zero.

## POST-FAILURE0-NATURAL-P0 closeout (2026-07-23)

The natural failure matrix is closed as a disconnected proof. A real
canonical trivial owner with one existing unlowered type-op `Call` rejects at
the optimizer stage under a serialized, test-scoped `NYASH_OPT_DIAG_FAIL`
policy. A real canonical owner with an orphan `StaticDataPlan` rejects at
contract refresh with the existing Static Table missing-spec tag. Both paths
retain `RejectedModulePostprocessV1` and remain discard-only.

The focused two-fixture test, POST-FAILURE0 guard, existing POST0/P0-R1
fixtures, cargo check, and pointer guard are green. RC/refresh fallibility,
real-route panic, production postprocess, external commit, outer executor,
retry, fallback, and public-ingress wiring remain non-claims. The next row is
the single atomic `CUT0-I0-ATOMIC-CUT0/G0` activation boundary.
