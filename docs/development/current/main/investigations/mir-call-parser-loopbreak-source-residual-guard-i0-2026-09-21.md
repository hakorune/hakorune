---
Status: closed__2026-09-21__ParserLoopBreakResidualGuard
Task: MIR-CALL-PARSER-LOOPBREAK-SOURCE-RESIDUAL-GUARD-I0
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-source-package-i0-2026-09-20.md
Implementation permission: true for the existing LoopBreak package-loan residual check and its focused tests only
NextCard: MIRBUILDER-QUALIFIED-METHOD-RECIPE-ADAPTER-DELETE-D0
---

# Parser LoopBreak source residual guard I0

## Six-line brief

```text
Decision: make the existing LoopBreak package loan fail closed when a
  candidate remains after the lowering state finishes; a dropped candidate is
  a named contract error, not a successful finish.
Source authority + canonical issuer: the existing
  LoopBreakSourcePackageLoanV1 and its sealed
  VerifiedCallableLoopBreakSourceFactsV1 candidate inventory.
Non-authority: candidate count as a success proof, Drop, route reselection,
  GenericLoop retry, AST/name inference, a new receipt, or a parser fallback.
Fail-fast boundary: CallableSemanticLoweringState::finish must reject a
  nonempty candidate loan before the callable session can publish or commit.
Smallest next slice: add the loan's residual check, call it from the existing
  state finish boundary, and cover unconsumed, consumed, and typed-absence rows.
Non-claims: no parser composite-loop promotion, production caller switch,
  old-edge deletion, VM/AOT parity, or publication completion.
```

## Verified gap and finite boundary

The selected owner is finite and already in the production lowering path:

| owner | current evidence | required change |
| --- | --- | --- |
| `LoopBreakSourcePackageLoanV1` | `Candidate` retains a `VerifiedCallableLoopBreakSourceFactsV1`; `take_candidate_for_site` removes only the exact site and leaves other candidates in the facts slice | expose one named `finish_empty`/residual check without changing candidate meaning |
| `CallableSemanticLoweringState::finish` | records only `is_candidate()` in `loop_break_transport_kind`; it never checks the remaining candidate slice | invoke the existing loan check before successful finish |
| `raw_loop_child_entry.rs` | takes one candidate when the LoopBreak physical route is entered; route mismatch or an unarmed path can leave the candidate untouched | preserve the existing route and surface the residual at finish |

Census boundary: package-owned LoopBreak loan issuance -> lowering-state
finish -> callable session close; includes Candidate, SupportedNonCandidate,
exact-site take, and the existing finish call; excludes the parser composite
LoopBreak physicalizer, compatibility routes, VM/AOT, and all other semantic
loans.

The static proof of the gap is the pair of facts above: the candidate slice is
mutable and can remain nonempty, while the only state-level observation is a
boolean transport-kind diagnostic. A candidate loan with an unconsumed row
therefore has no named terminal today. The correction must preserve the
existing `SupportedNonCandidate` typed absence as a successful empty loan.

## Implementation contract

1. Add a private-to-`mir` residual check on
   `LoopBreakSourcePackageLoanV1`. `Candidate` succeeds only when its facts
   candidate slice is empty; otherwise it returns a stable
   `[freeze:contract][callable-loop-break/source-package/residual-candidates]`
   error with owner and residual count. `SupportedNonCandidate` succeeds.
2. Call that check from `CallableSemanticLoweringState::finish` before the
   existing incomplete-consumption success path. Do not consume or replace the
   loan merely to inspect it, and do not turn a residual into `None`.
3. Add focused tests at the existing LoopBreak package owner for an
   unconsumed candidate rejection, a candidate consumed at its exact site, and
   a typed absence that finishes successfully. Reuse the current fixtures;
   do not add a parser-specific fixture.
4. Keep `loop_break_transport_kind` as diagnostics only. It must not become an
   acceptance condition and no new `Verified*`/`Prepared*` product is issued.

## Acceptance

Run the focused package/state tests and then the normal quick library check
sequentially. Record the exact test filters and the check log in this card.
The expected behavior is:

- unconsumed candidate: named residual error;
- exact candidate take followed by finish: success;
- `SupportedNonCandidate`: success;
- all existing LoopBreak source/package tests remain green.

A local green result closes only this residual-consumption guard. It does not
advance the parser package past `GenericLoopV1NotSelected` and does not delete
an old production edge.

## Closeout evidence

The existing loan now has a named residual check and the callable state invokes
it before successful finish. The focused existing package fixture suite was run
with the non-empty filter `loop_break_source_tests::`:

| check | result | evidence |
| --- | ---: | --- |
| LoopBreak package/state focused tests | **7 passed, 0 failed** | `/tmp/hakorune-loopbreak-residual-guard-tests-20260921.log` |
| `cargo check --profile quick --lib -j4` | **exit 0; 1,756 warnings** | `/tmp/hakorune-loopbreak-residual-guard-check-20260921.log` |
| `cargo fmt --all -- --check` | **exit 0** | local closeout run |

The first exact module filter matched zero tests and was discarded; it is not
counted as evidence. The accepted filter exercised unconsumed-candidate
rejection, exact-site consumption, typed absence, and the existing parser
package rows. No production route, fallback, or publication behavior changed.

## Separate next candidate

The audit's second P1, the caller-zero `QualifiedMethodRecipePortV1`
implementation on `NormalCallableSemanticPackagePortAdapterV1`, is a separate
BoxShape/Delete candidate. Static census currently finds two implementations:
`MainQualifiedMethodRecipePort` is called by the selected Main route, while the
normal-package adapter implementation has no caller of the qualified-port
entrypoint and discards `site`/publication. It must not be removed in this I0:
its caller-zero proof and deletion guard belong to the next D0 card.
