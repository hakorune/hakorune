---
Status: accepted Decision; superseded by the two-stage lifecycle-authority row
Date: 2026-08-16
Work mode: design_stop
Parent: TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-BRIDGE-D0 / I0
Successor: TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-AUTHORITY-R0 / I0
---

# TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-MATERIALIZER-D0

This card closes the design stop opened by the proof-only exit-obligation
prototype. The accepted correction is A-prime: materialization consumes a
source-side admission, while a final backend closure is issued only after the
actual C/LLVM realization is complete. A final backend proof is never required
before the calls and control shape it verifies exist.

## Six-line brief

```text
Decision: accept A-prime: issue AdmittedPinnedTextLifecycleMaterializationV1 before materialization, then issue VerifiedPinnedTextBackendNoUnwindClosureV1 only after final C/LLVM lowering and transforms and before object emission.
Source authority + canonical issuer: PreparedFunctionExitSetV1 remains the sole normal-exit ledger; PinnedTextLifecycleAuthorityIssuerV1 issues the opaque exit stamp, stamp-only finish capability, expected lifecycle obligation, and pre-materialization admission in one session-close scope; the final contract-bound backend verifier only compares that obligation with the realized module.
Non-authority: the prototype's copied rows, an independent exit count, runtime-declaration facts alone, CheckedCallOut alone, C emitter output alone, LLVM attribute strings, JSON/MIR/name/Recipe inference, raw pointers/tokens, fallback, and retry.
Fail-fast boundary: missing or foreign stamps and declarations reject before lifecycle MIR mutation; expected/observed call, fault, finish, EH, no-unwind, no-return, or target drift discards the unpublished module before object emission; absence of the final closure forbids contract-bound publication.
Smallest next slice: resume LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0; the installed child, one Completion owner, physical signature mapping, residence substrate, and backend-frame transport are closed, but the common canonical session/DraftSeal ingress is still missing.
Non-claims: no lifecycle CFG, runtime enter/finish emission, Canonical residence adoption, GEP/load, PinnedTextOp lowering, DraftSeal finish placement, route/perf admission, production caller, literal/StringBox origin, fallback/retry, or main integration.
```

## Accepted correction

The rejected one-stage graph required a final no-unwind proof before the
lifecycle calls and terminal Fault shape existed:

```text
final backend proof -> materialize calls -> inspect calls
```

The accepted graph is strictly forward:

```text
canonical exit ledger + frame/plan/residence stamps + source control policy
  -> expected lifecycle obligation
  -> pre-materialization admission
  -> unpublished lifecycle realization
  -> final backend observation
  -> no-unwind closure
  -> contract-bound object emission
```

The first product says what must be realized and verified. It is not a claim
that the future module is already no-unwind. The final closure says only that
the realized module satisfies the admitted obligation; it does not reissue
source, Text, Completion, target, or lifecycle meaning.

## Exit authority correction

`PinnedTextResidenceExitObligationV1` remains historical prototype evidence.
Its copied source sites, blocks, values, and count are not inputs to the
successor.

`PreparedFunctionExitSetV1` remains the only exit ledger and privately issues
an opaque per-preparation stamp. The corrected
`PreparedTextFormalExitFinishSetV1` carries only that stamp together with the
same function/plan/frame/residence provenance. It carries no exit rows,
cardinality, order, source sites, blocks, values, or return operands.

DraftSeal later borrows the canonical exit set and uses its existing single
iteration. The stamp proves that the finish obligation was issued for that
exact preparation; it does not provide a second way to enumerate exits.

## Current-HEAD chronology correction

The A-prime owner graph is accepted, but current selected-normal production
does not use the canonical exit-set path. It calls the legacy finalizer, which
may write `Return` directly and yields a `MirFunction`; only afterward does
`complete_resolved_child_with_physical_loan` issue the backend-frame contract.
`PreparedFunctionExitSetV1` exists on separate canonical DraftSeal paths.

Moving frame issue earlier on the legacy route would still not create the
canonical exit ledger. A stamp transported through the already-sealed
`MirFunction` cannot return to DraftSeal to place a finish before `Return`, and
scanning those Returns in MIR/JSON would be a second exit authority. Therefore
the lifecycle row is parked until the planned common V2 pre-session/session
fan-in reaches `CanonicalSsaFunctionSessionV2`, Completion, and DraftSeal. The
legacy finalizer is not extended with another lifecycle or finish authority.

## Two-stage no-unwind authority

The source-side admission owns obligations only:

```text
entry acquire role                       exactly one
entry success edge                       exactly one
terminal Fault role                      exactly one
normal finish policy                     every canonical normal exit
finish on Fault path                     zero
source catch edge                        zero
calls within the live residence extent   no-unwind returnable or
                                         no-unwind no-return terminal
```

`normal finish policy` is deliberately not a copied integer count. The
canonical exit set supplies cardinality and iteration when materialization is
opened.

After all call/EH-changing transforms, the contract-bound backend verifier
observes the final candidate and checks the admitted roles, effective call and
declaration attributes, terminal `unreachable`, absence of EH structures,
target realization, and exact plan/function provenance. Only then may it
issue `VerifiedPinnedTextBackendNoUnwindClosureV1`. The object emitter is a
consumer of the closed candidate and cannot issue or repair this proof.

## Successor

The exact owner/timing contract and bounded precommit handoff I0 are owned by:

`text-formal-pinned-residence-lifecycle-authority-r0-2026-08-16.md`.

No materializer, DraftSeal, runtime, MIR, JSON, C, or LLVM execution change is
authorized by this accepted BoxShape alone.
