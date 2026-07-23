# CUT0-I0 OWNER-RETENTION0-POST Consultation

Status: **Decision locked — Candidate PR-prime selected; OWNER-RETENTION0-POST-P0 next**
Date: 2026-07-23
Scope: decide the postprocess owner boundary before any production outer
executor or atomic CUT0 wiring.

Related:

- `docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-prod-activation-p0-r1-failure-consultation-2026-07-23.md`

## One-paragraph question

`ModulePostprocessOwnerV1` already owns the family-derived stage schedule and
keeps Raw verifier errors reportable, but its fatal branches currently return a
bare `ModulePostprocessErrorV1` and drop the mutated unpublished input. Its
successful `into_external_commit_parts` also reduces the route product to a
token, Builder readiness owner, module, and verification result, dropping the
continuation, physical receipt/inventory evidence, and callable capability.
Before production activation, should postprocess failures return a
route-neutral rejected owner containing the exact current `ModulePostprocessInputV1`
and stage error, and should successful postprocess handoff retain those route
evidence facts through commit or consume them once into an explicit sealed
evidence witness? The answer must preserve no retry, no rollback clone, no
source re-resolution, and no new ambient fault switch.

## Current evidence

The current implementation has this shape:

```text
FinalizedModuleInvocationV1
  -> ModulePostprocessOwnerV1::run / run_raw
  -> process_input(mut input)
  -> rune refresh
  -> optimizer
  -> contract refresh
  -> pre-transform verifier
  -> family RC policy
  -> semantic refresh
  -> callsite canonicalization
  -> canonical final verifier
  -> PostprocessedModuleInvocationV1
```

The real fallible branches are:

```text
optimizer diagnostics (only when the existing diagnostic policy is enabled)
contract refresh / validation
canonical final verification
```

The pre-transform verifier is deliberately not a fatal branch for the existing
Raw reportability contract. RC insertion, semantic refresh, and callsite
canonicalization are currently infallible APIs; this row must not invent RC
failure injection merely to enlarge the failure matrix.

The current success handoff drops route evidence here:

```text
PostprocessedModuleInvocationV1
  -> into_external_commit_parts()
  -> token + PreparedBuilderExternalCommitV1 + MirModule + verification
```

That is weaker than the accepted owner-chain law because the continuation,
physical receipt/inventory witness, and callable capability are no longer
available for a final commit proof.

## Questions for decision

### Q1 — fatal postprocess failure owner

Which product must be returned when a real fatal postprocess stage fails?

```text
1. Full rejected owner (recommended candidate):
   run(self) / run_raw(self) returns
   RejectedModulePostprocessV1 { input, schedule, stage, typed error }.
   The input is the exact current unpublished owner, including any
   in-place mutations made before the failure. Only error inspection and
   discard are exposed; retry, resume, replacement manifest, and fallback
   terminals do not exist.

2. Publication-zero only:
   retain the current bare error because no external commit is issued.
   OWNER-RETENTION0-POST would claim no owner retention.

3. Stage transactions:
   clone or snapshot the module before every stage and return a pristine
   owner on failure. This introduces a second state/history authority and is
   rejected unless rollback is a separately selected requirement.
```

### Q2 — mutation law on failure

If Q1=1, what does the rejected owner promise?

```text
1. In-place unpublished mutation (recommended candidate):
   the module may contain the successful prefix of postprocess stages at the
   failure point. It remains unpublished, is discard-only, and is never
   retried. The live Builder is unchanged because the candidate session is
   still owned by the rejected product.

2. Rollback to the pre-postprocess module:
   requires Clone/snapshot or an inverse log and is outside this row.

3. Stop after a preflight-only stage list:
   cannot cover the existing optimizer/contract/verifier semantics without
   moving postprocess work elsewhere.
```

### Q3 — failure vocabulary and deterministic fixtures

Which failures belong to this row?

```text
1. Existing real fatal stages only (recommended candidate):
   OptimizerDiagnostics, ContractRefresh, and canonical FinalVerification.
   Raw pre-transform verifier Err remains reportable evidence, not rejection.
   No new env toggle, fault field, malformed plan, catch_unwind, or RC fault
   API is introduced.

2. Add a test-only postprocess fault product:
   deterministic but creates a new failure authority before CUT0.

3. Make RC and every refresh operation fallible first:
   larger semantic row; defer to POST-FAILURE0.
```

### Q4 — evidence retention at successful handoff

How must postprocess success preserve route evidence until the paired commit?

```text
1. Consume route evidence once into a compiler-private
   PostprocessEvidenceSealV1 (recommended candidate):
   inventory/receipt/capability/continuation correspondence is checked at
   the one-shot prepare terminal, then the sealed witness travels inside
   PreparedModuleExternalCommitV1. No source or module re-observation is
   permitted.

2. Keep the entire route-specific postprocessed input inside
   PreparedModuleExternalCommitV1 until commit:
   strongest retention, but couples the commit product to every route
   physical type.

3. Drop route evidence after postprocess:
   smaller product, but breaks the source -> receipt -> drain -> commit proof
   and is not acceptable for CUT0.
```

## Candidate PR-prime decision closeout (2026-07-23)

The worker audit and user review select:

```text
Q1 = 1   full rejected owner
Q2 = 1   in-place unpublished mutation, discard-only
Q3 = 1   existing fatal stages only; Raw verifier Err remains reportable
Q4 = 1   one-shot PostprocessEvidenceSealV1 at paired commit preparation
```

This keeps the owner chain linear without adding rollback state:

```text
FinalizedModuleInvocationV1
  -> postprocess
  -> PostprocessedModuleInvocationV1
       or RejectedModulePostprocessV1 { current input, stage, error }
  -> prepare paired commit
  -> PostprocessEvidenceSealV1
  -> PreparedModuleExternalCommitV1
  -> one-shot commit
```

The following are now durable policy:

```text
RejectedModulePostprocessV1 exposes only error inspection and discard.
It has no retry, resume, replacement-manifest, fallback, or recovery terminal.
The rejected owner may retain an in-place mutated unpublished module prefix.
Raw pre-transform verifier Err remains reportable evidence.
RC failure retention is not claimed while RC insertion is infallible.
PostprocessEvidenceSealV1 is required at paired commit preparation; dropping
continuation/physical inventory/receipt/capability as bare fields is forbidden.
```

## Non-claims while implementing the next row

```text
production postprocess consumer = 0
production outer executor = 0
atomic CUT0/G0 = 0
optimizer/contract/RC universal fault matrix = 0
rollback or retry capability = 0
Raw pre-transform verifier Err becomes fatal = 0
source/catalog/current_module re-observation = 0
```

## Smallest next executable row

```text
OWNER-RETENTION0-POST-P0
  -> add route-neutral rejected postprocess owner
  -> retain the exact current input on every fatal branch
  -> keep Raw reportable verifier evidence unchanged
  -> add natural invalid-CFG and contract/readiness fixtures only
  -> preserve production consumers = 0
```

The row must stay below 800 lines per touched source/check file and must not
wire the outer executor, external commit, fallback, retry, or canonical/Raw
public ingress.

The next code-facing row is `OWNER-RETENTION0-POST-P0`. `POST-FAILURE0` owns
any later deterministic optimizer/RC fault matrix, and production consumers
remain zero until the atomic CUT0 patch.

## OWNER-RETENTION0-POST-P0 closeout (2026-07-23)

The selected rejected-owner boundary is implemented. Fatal optimizer,
contract-refresh, and canonical final-verifier branches now return
`RejectedModulePostprocessV1` with the current unpublished input, family-owned
schedule, failure stage, and stage error. The owner exposes only stage/error
inspection and discard. In-place mutation before failure is retained; no
rollback clone, retry, resume, fallback, or recovery terminal was added.

Raw pre-transform verifier errors remain reportable evidence, and RC failure
remains outside this row because the current RC insertion API is infallible.
The focused final-verifier rejection fixture, P0-R1 compatibility fixture,
POST0 guard, cargo check, and diff check are green. `PostprocessEvidenceSealV1`
is still the next implementation boundary, and production consumers remain
zero.

## OWNER-RETENTION0-POST-EVIDENCE0 closeout (2026-07-23)

The selected Q4 handoff is implemented. Postprocess success now yields a
route-specific evidence input, and paired commit preparation consumes it once
into `PostprocessEvidenceSealV1` after checking token brand/family against the
physical receipt, inventory, ledger/root, and callable capability witnesses.
The prepared commit product retains that seal until its one-shot commit.

Canonical continuation is retained with its exact receipt/inventory; callable
routes additionally retain capability evidence; Raw retains sealed ledger/root
evidence. The old bare token/Builder/module/verification tuple is no longer
the complete handoff proof. `POST-FAILURE0` remains the owner for any later
deterministic optimizer/RC fault matrix, and production consumers remain zero.
