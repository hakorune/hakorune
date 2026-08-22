Status: cfg(test) retirement I0 complete; F2 design stop next
Task: MIR-CALLABLE-LOOP-STRUCTURAL-LEASE-RETIRE-D0
Date: 2026-08-23
Priority: classify the caller-zero structural lease before any retirement or namespace change
Parent: MIR-CALLABLE-LOOP-OUTSIDE-ORDINARY-CONSUMPTION-D0
Current execution row: MIR-CALLABLE-LOOP-OUTSIDE-OBSERVED-CLASS-D0
CurrentCard: docs/development/current/main/investigations/mirbuilder-callable-loop-structural-lease-retire-d0-2026-08-23.md
NextCard: MIR-CALLABLE-LOOP-OUTSIDE-OBSERVED-CLASS-D0
---

# Callable Loop structural lease retirement D0

## Six-line brief

Decision: the source-Facts -> semantic Recipe -> named physical adapter path is the only current production Ready authority. Dirac's read-only audit confirms the route-neutral structural lease is caller-zero outside tests; the next bounded implementation is `cfg(test)` retirement, preserving its four focused tests as test-only evidence. Complete deletion remains a later cleanup only if no owner needs the experiment.
Source authority + canonical issuer: `CallableGenericLoopSourceFactsIssuerV1` issues the source-located Facts/Recipe disposition; `CallableGenericLoopV1SemanticRecipeIssuerV1` and `CallableGenericLoopV1PhysicalAdapterV1` consume that production path. The structural lease issuer is not a production source authority.
Non-authority: `CallableLoopStructuralLeaseIssuerV1`, `PreparedCallableLoopStructuralHandoffV1`, `CallableLoopReadyStructuralViewV1`, `CallableLoopRouteNeutralStructuralSeedV1`, `CallableLoopSourceBoundStructuralPortV1`, `with_existing_structural_port`, `LoopRouteContext` as a reconstructed source authority, and test-only structural observations.
Fail-fast boundary: the caller-zero census is complete. The I0 change is limited to compile-time registration/import boundaries; any non-test caller, re-export, or production field discovered during the edit is a typed stop, not a compatibility fallback.
Smallest next slice: I0 is closed with test-only structural registration and preserved evidence. The next design stop is `MIR-CALLABLE-LOOP-OUTSIDE-OBSERVED-CLASS-D0`; no Outside row/class refactor starts until its observed-vs-admitted authority is accepted.
Non-claims: no Ready production switch, no Outside consumer, no pure-plan split, no unpublished-session capability, no `GenericLoopV1LoweringContext` redesign, no Builder barrel reorganization, no publication, fallback, parser, or performance work.

## Why this is the next design stop

The current main path is already explicit:

```text
PreparedLocatedRawLoopChildEntryV1
  -> CallableGenericLoopSourceFactsIssuerV1::issue_once
  -> Ready::claim_all
  -> CallableGenericLoopV1SemanticRecipeIssuerV1::issue
  -> CallableGenericLoopV1PhysicalAdapterV1::lower
```

`raw_loop_child_entry.rs` is the named production edge. The adapter consumes
the semantic recipe and currently composes/verifies/lowers through the existing
physical path. It does not call the structural lease.

The structural family is different:

```text
source Facts receipt
  -> CallableLoopStructuralLeaseIssuerV1::prepare
  -> route-neutral seed
  -> HRTB structural view
```

It was a useful design exploration and has focused tests, but the census below
shows no non-test caller for its issuer or callback. Keeping it registered as a
normal production module makes the authority graph look wider than the live
graph. This card therefore decides the retirement shape before touching code.

## Read-only caller census (2026-08-23)

| Surface | Location | Non-test callers | Test callers / evidence | Current meaning |
| --- | --- | ---: | --- | --- |
| `CallableLoopStructuralLeaseIssuerV1::prepare` | `normal_callable_loop_source_facts.rs` | 0 | 3 in `normal_callable_loop_structural_lease_tests.rs` | caller-zero lease issuer |
| `with_existing_structural_port` | `control_flow/joinir/structural_port.rs` | 0 | 1 in `structural_port_tests.rs` | legacy structural diagnostic callback only |
| `issue_route_neutral_structural_seed` | `control_flow/joinir/structural_port.rs` | 0 direct | reached only from the caller-zero lease issuer | seed implementation detail |
| `CallableLoopSourceBoundStructuralPortV1` and `CallableLoopReadyStructuralViewV1` | source-facts module + structural port | 0 | structural lease tests | HRTB transport experiment |
| `CallableGenericLoopSourceFactsIssuerV1::issue_once` | `raw_loop_child_entry.rs` | 1 named production edge | source-facts tests | live source/Facts authority |
| `CallableGenericLoopV1PhysicalAdapterV1::lower` | `raw_loop_child_entry.rs` | 1 named production edge | adapter tests | live semantic Recipe consumer |

The structural module is still production-registered:

```text
src/mir/builder/control_flow/joinir/mod.rs
  pub(in crate::mir::builder) mod structural_port;

src/mir/builder.rs
  mod normal_callable_loop_source_facts;
```

That registration is not itself a production caller. It is the reason the
retirement shape must be explicit rather than inferred from `#[allow(dead_code)]`.

## Worker Decision — Dirac (read-only)

The worker inspected `main` at `2dd627d3b7` without editing or running
mutating commands. The result is:

```text
non-test lease issuer callers       = 0
non-test structural-port callers   = 0
re-exports                         = 0
live Ready path                    = issue_once -> claim_all -> Recipe -> adapter
```

The worker recommends `cfg(test)` as the smallest safe slice because it removes
the family from the production graph while retaining the three lease tests and
one structural-port callback test. Complete deletion is a later option; an
experimental namespace is rejected now because it has no named owner or close
condition.

## Authority boundary

```text
source admission / resolver
  -> CallableGenericLoopSourceFactsIssuerV1
  -> existing PlanBuildOutcome + exact route selection
  -> CallableGenericLoopV1SemanticRecipeIssuerV1
  -> existing physical adapter
```

The structural lease may not become a second Ready issuer, a route selector,
or a physical consumer. Its seed contains no Recipe key, selector, `ValueId`,
block, or publication fact. The current test lease must not be promoted merely
because its HRTB shape is attractive.

## Retirement candidates

| Candidate | Benefit | Cost / risk | Status |
| --- | --- | --- | --- |
| Delete the whole structural lease family and its tests | smallest live graph; removes dead production registration and stale authority vocabulary | loses exploratory HRTB tests unless their contract is intentionally archived or replaced | recommended if no owner needs the experiment |
| Make the family `cfg(test)` | preserves the test-only HRTB contract without compiling it into production | requires splitting source-facts production imports/types from test-only lease glue | **accepted I0** |
| Move it under an explicit experimental namespace | preserves code and test evidence while labeling it non-production | keeps migration weight and another visible authority-shaped family | only if a named experiment owner exists |

No option may leave the family as an unlabeled production module with zero
callers. The final choice must name the owner, exact module boundary, test
location, and the guard proving production caller count remains zero.

## Accepted implementation — MIR-CALLABLE-LOOP-STRUCTURAL-LEASE-RETIRE-I0

Change only the compile-time boundary:

```text
joinir::structural_port module registration       -> #[cfg(test)]
source-facts structural imports and lease types   -> #[cfg(test)]
structural lease tests                            -> retained under cfg(test)
source Facts -> Recipe -> physical adapter        -> unchanged
```

The implementation must not delete the test evidence yet, introduce an
experimental module, alter `CallableGenericLoopSourceFactsIssuerV1`, change
`LoopRouteContext`, or touch `raw_loop_child_entry.rs` / the physical adapter.

Acceptance:

```text
production build does not compile structural_port or lease-only types
test build still runs three structural lease tests and one port test
source-facts production tests and Ready adapter tests remain unchanged
non-test structural symbol/caller/re-export census is zero
live Ready production edge count remains one
source Facts -> Recipe -> adapter has no new route or fallback
touched Rust files remain below 760 lines
```

The reusable guard must fail if a structural lease symbol is restored to a
non-test registration or a non-test caller appears. It must not ban unrelated
legacy `LoopRouteContext` uses in ordinary tests or compatibility routes.

## I0 closeout evidence

The structural port module, source-facts structural imports, and lease-only
handoff types are now `cfg(test)`; the live path remains:

```text
issue_once -> claim_all -> semantic Recipe -> physical adapter
```

Evidence:

```text
non-test cargo check: passed
structural lease tests: 3 passed
structural port callback test: 1 passed
source-facts tests: 7 passed
selected Dynamic suite: 10 passed
source-Facts guard: green
current-state pointer guard: green
git diff --check: green
normal_callable_loop_source_facts.rs: 597 lines
raw_loop_child_entry.rs: 686 lines
```

No production Ready, Outside, Recipe, physical, Builder, or fallback edge was
changed. Complete deletion remains parked until an owner confirms that the
test-only HRTB evidence is no longer useful.

## Finite design states

| State | Authority | Effect | Allowed next | Fallback |
| --- | --- | ---: | --- | --- |
| `CensusConfirmedCallerZero` | read-only `rg` census plus current pointer | none | select one retirement shape | none |
| `ProductionCallerFound` | source owner of the discovered edge | none | reopen source/consumer Decision | no deletion or fallback |
| `RetireDeleteSelected` | accepted D0 Decision | none | bounded delete series | no production lease |
| `RetainCfgTestSelected` | accepted D0 Decision | none | test-only BoxShape series | no production import |
| `RetainExperimentalSelected` | accepted D0 Decision and named experiment owner | none | namespace/registration series | no production caller |
| `NoSafeSlice` | current design authority | none | stop and request owner clarification | never guess |

`CensusConfirmedCallerZero` is not a runtime disposition and must not be
converted into `Outside`, `Absent`, or a compatibility route. The selected
retirement shape is a repository-structure decision only.

## Acceptance for the design stop

The D0 is complete only when all of the following are recorded in this card
and the current pointer:

```text
non-test caller count for every lease/port issuer = 0, or the discovered edge is named
one retirement candidate is selected
owner and exact files are named
test evidence is either retained, migrated, or intentionally removed
production module registration after the next slice is explicit
caller-zero guard shape is specified
source Facts -> Recipe production edge remains unchanged
```

The later implementation slice must add positive/negative evidence for the
chosen shape and a reusable guard. It must not add a new semantic receipt or
change accepted Loop behavior.

## NoSafeSlice conditions

Remain at this design stop if:

```text
any non-test caller constructs or consumes the lease
the structural family is required by a hidden re-export or production type
deleting/cfg(test)-ing it changes Ready source admission or Recipe selection
the tests require production-only lifetime/visibility that has no named owner
the proposed experimental namespace would be another unowned authority
retirement needs the pure-plan, unpublished-session, or Outside-consumer design
```

Do not solve these conditions with an empty seed, default view, `Option`,
`#[allow(dead_code)]`, or a fallback to `LoopRouteContext`.

## Explicitly parked follow-ups

After this D0, separate cards remain:

1. `MIR-CALLABLE-LOOP-OUTSIDE-OBSERVED-CLASS-D0` — observed rows versus Ready
   admitted classes and the private Outside remainder validator.
2. `MIR-CALLABLE-LOOP-UNPUBLISHED-SESSION-CAPABILITY-D0` — physical adapter
   capability boundary; do not assume pure plans.
3. `MIR-CALLABLE-LOOP-PURE-PLAN-BEFORE-EFFECT-D0` — only if the capability
   audit requires a larger symbolic-plan BoxCount.
4. `MIRBUILDER-BARREL-BOXSHAPE-D0` — behavior-neutral barrel census after
   caller-zero classification.
5. `MIRBUILDER-CI-REQUIRED-CHECKS-D0` — repository settings, not a local Rust
   semantic change.

None of these is opened by the structural lease census.
