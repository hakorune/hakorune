---
Status: closed__DecisionRecorded__GenericG0PolicyModeAuthority__2026-09-11
Task: LOOP-G0-POLICY-MODE-AUTHORITY-D0
Date: 2026-09-11
Priority: identify the production authority for Generic G0 mode and loop-window coverage before issuer implementation
Parent: mirbuilder-loop-g0-canonical-issuer-i0-2026-09-11
NextCard: LOOP-G0-CANONICAL-ISSUER-I0
---

# Generic G0 policy mode authority D0

## Six-line brief

```text
Decision: stop the Generic G0 issuer until policy mode and loop-window coverage have one explicit production authority.
Source authority + canonical issuer: unresolved; the next design slice must name the resolver/config source and one co-sealing issuer.
Non-authority: cfg(test) observation adapters, GenericG0PolicyContext defaults, fixture mode labels, legacy registry/planner reads, and Release/Complete guesses.
Fail-fast boundary: unresolved or mismatched mode/coverage rejects before G0 candidate, Recipe demand, package bind, Builder/session mutation, or legacy dispatch.
Smallest next slice: census existing mode/coverage providers, select one source-backed authority or record NoSafeSlice, then define the I0 handoff input without duplicating the policy owner.
Non-claims: no GenericG0 plan, production selection, package token, physical lowering, publication, fallback, or old-edge deletion.
```

## Census boundary and evidence

Boundary: `CanonicalLoweringPreflightV1::verify` -> the future G0 source probe
-> policy candidate -> Generic Recipe demand. It includes only the source of
the mode/coverage pair and its fail-fast mapping. It excludes physical
lowering, Completion/DraftSeal, publication, Callable, Dynamic, Raw, and the
legacy Generic registry.

The current code proves the gap:

| evidence | observed boundary | consequence |
| --- | --- | --- |
| `src/mir/compiler/generic_g0_projection/mod.rs` | `handoff` is `cfg(test)` and its source-window helper is named/test-scoped | no production G0 handoff issuer |
| `src/mir/compiler/generic_g0_observation.rs` | the compiler source-attempt adapter is `cfg(test)` | no production translation from source facts to mode/coverage |
| `GenericG0PolicyContextV1` | accepts exact mode and coverage, but its production call path is the test observation adapter | `Release`/`Complete` cannot be assumed |
| `loop_recipe_contract/generic_g0_demand.rs` | demand validates mode/coverage from `CanonicalLoopFamilySelectionV1` evidence | bypassing the evidence would create a second or synthetic authority |
| Loop observation SSOT | explicitly requires an explicit mode/coverage context and records the production caller as zero | I0 entry conditions are not closed |

The current authority inventory has no named production provider. A config
read, environment flag, fixture label, legacy planner witness, or enum default
may not fill this gap without a separate accepted design decision.

## Required design decision

The next consultation must answer all of these with source-backed names:

1. Which resolver/config product issues the mode
   (`Release | Strict | StrictPlannerRequired`) for the selected compile?
2. Which resolver/source product proves `Complete` loop-window coverage, and
   what is the exact finite boundary when coverage is incomplete?
3. Does that provider already own the common window lease, or must the
   existing resolver lease be extended without reissuing source identity?
4. Which one issuer co-seals mode, coverage, owner/origin/source-kind/site/
   frame, G0 policy handoff, and `NumericTarget::host` before Recipe demand?
5. For a known non-G0 shape, what exact `Declined` result is source-backed;
   for missing mode/coverage, what exact `Unresolved`/`NoSafeSlice` terminal is
   used; and where does no-fallback enforcement live?

The answer must preserve the existing policy matrix. It may promote the
current neutral types only if their source authority is made explicit; it may
not make the test adapter production by renaming it or insert a default mode.

## Acceptance for reopening I0

Reopen `LOOP-G0-CANONICAL-ISSUER-I0` only after a tracked Decision records:

- one production mode issuer and one production coverage issuer (or one
  source product owning both), with no second reader;
- the exact G0 candidate/decline/unresolved/reject mapping and pre-bind
  rejection boundary;
- the existing `BindingSsaTrivial`/`Single` package map and no new token or
  continuation variant;
- positive, non-G0 decline, missing-mode/coverage negative, foreign-identity,
  and no-reentry guard requirements; and
- the bounded I0 implementation files plus README/reference and serialized
  quick-profile gate.

Until then this is a design stop. Do not use the goal blocker for this single
newly discovered authority gap; return to it only if the same unresolved
provider premise recurs after the required census.

## Decision recorded

The existing invocation-policy snapshot is the authority; no new environment
reader or semantic mode owner is needed:

```text
BuilderInvocationConfigV1::snapshot_for_canonical
  -> immutable BuilderEmitDebugPolicySnapshotV1
  -> explicit GenericG0PolicyModeV1 projection
  -> canonical G0 preflight issuer
```

The projection is exact and rejects the inconsistent combination
`planner_required=true` with `strict=false`; it maps only
`(!strict,!planner_required) -> Release`, `(strict,!planner_required) -> Strict`,
and `(strict,planner_required) -> StrictPlannerRequired`. G0 policy does not
read the environment directly and does not choose a default mode.

`Complete` coverage is issued by the productionized G0 source projector only
after its existing structural/type/numeric handoff has verified the finite
source coverage. A missing or opaque source fact remains a typed unresolved or
rejected source outcome; it is never converted into `Complete`.

The one issuer therefore co-seals the invocation mode snapshot, complete G0
source handoff, owner/origin/source-kind/site/frame, `NumericTarget::host`,
declaration header, and lifecycle identity before Recipe demand or package
bind. I0 may promote the existing handoff module and add this explicit mode
projection, but must keep the policy consumer environment-free and must not
issue a second window lease/header/continuation.

The design stop is closed. Reopen
`LOOP-G0-CANONICAL-ISSUER-I0` with this authority tuple; if the exact snapshot
cannot be threaded without taking a second snapshot or introducing a new
semantic owner, return to `NoSafeSlice`.
