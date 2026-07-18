---
Status: SITEPROJ0-S0 closed; SITEPROJ0-P0 is next
Date: 2026-07-19
Decision baseline: 906f994d12
Parent: callable-result-i64-site0-r0-expression-spine-loop0-p0b-o0-design-stop-2026-07-19.md
Scope: Loop-port-only canonical body-item site projection before O0-P0
Decision: Candidate A
---

# LOOP0 O0-P0 canonical body-item site projection stop

## Finding

The first actual O0-P0 proof reached a pre-existing PATH0 boundary rather than
an O0 representation failure. The sealed cleanup statement is exact, but its
expression carrier and the activation row use two intentionally different path
surfaces:

```text
PATH0 carrier:
  Body(4) / LoopBodyRoot / LoopBody(5) / Value

canonical activation row:
  Body(4) / LoopBody(5) / Value
```

The failing focused proof is retained only in stash:

```text
wip/loop0-o0-p0 actual path0 cleanup-site mismatch
```

It must not be restored wholesale. The actual representation checks preceding
the site comparison were green in both default and strict modes.

This mismatch is already documented by the closed BODYDOMAIN0 task. Semantic
body roots such as `LoopBodyRoot` are body-scope identities, while activation
rows use the compact typed item family. BODYDOMAIN0 deliberately kept generic
`body_stmt` fail-closed and parked canonical nested statement access until the
corresponding If/Loop capability row. Therefore this is not a test expectation
that may be normalized locally.

## Authority split

| Concern | Existing authority |
| --- | --- |
| semantic body-root carrier | `LegacyBodyInputV1.parent` |
| canonical activation parent | `LegacyBodyInputV1.domain_parent` |
| exact item vocabulary | `SourceBodyKindV1::item_segment` |
| canonical activation rows | callable-result activation plan |
| exact Loop-only child demand | `LocatedLoopPlanExpressionPortV1` |

There is currently no canonical projection from a semantic-root body item to
an activation-compatible statement/expression carrier. Generic `body_stmt`
must not become that projection because doing so would reopen the direct
nested If/Loop bypass closed by BODYDOMAIN0.

## Candidates

### Candidate A — Loop-port-only typed projection (recommended)

Add one narrow callable-result source-view operation, reachable only through
`LocatedLoopPlanExpressionPortV1::exact_body_stmt`, that constructs the exact
item statement site from:

```text
located body plan/caller brand
  + exact domain_parent
  + exact SourceBodyKindV1
  + checked direct item ordinal
```

The projection law is:

```text
canonical item site = domain_parent + kind.item_segment(index)
```

It does not strip a root segment after construction, inspect AST equality, or
look up an activation row. Generic `VerifiedCallableResultLegacySourceViewV1::body_stmt`
keeps its semantic-root publication and current fail-closed behavior.

### Candidate B — globally canonicalize `body_stmt` (reject)

This violates the BODYDOMAIN0 closeout and would make every nested body caller
capable of publishing compact child statements before its capability row.

### Candidate C — add body-root segments to activation rows (reject)

This changes the canonical source/activation vocabulary and all existing
catalog, ledger, ordering, and fixture authorities merely to fit one consumer.

## Proposed task order

```text
LOOP0-P0b-O0-SITEPROJ0-D0
  consultation decision
  code delta = 0

LOOP0-P0b-O0-SITEPROJ0-S0
  one disconnected typed compact-item projection
  production located consumers = 0

LOOP0-P0b-O0-SITEPROJ0-P0
  LoopBodyRoot/LoopBody exact parity and negative matrix
  generic body_stmt behavior unchanged

LOOP0-P0b-O0-SITEPROJ0-G0
  one projection owner and one Loop-port consumer guard

resume:
  LOOP0-P0b-O0-P0
  -> LOOP0-P0b-O0-G0
  -> LOOP0-P0b-T0
```

## Required fixtures

Pass:

```text
exact located Loop body ordinal 5
  -> compact Body(4)/LoopBody(5)
  -> AssignmentValue equals the existing selected activation site

condition carrier remains Body(4)/LoopCondition
default/strict actual ParserBox representation parity
foreign plan/caller rejects
out-of-range ordinal rejects
```

Preservation:

```text
generic body_stmt still publishes semantic-root paths
If/Else/Scope/TaskScope/FastMem generic behavior unchanged
activation-row producer delta = 0
caller-ledger body-domain behavior delta = 0
```

## Stop conditions

Stop if implementation requires:

1. changing generic `body_stmt` publication;
2. adding/removing root segments in activation rows;
3. a global root-segment equivalence or normalization pass;
4. stripping path segments by spelling;
5. AST equality, span, name, target, ValueId, or activation-row lookup;
6. a persistent Builder path table;
7. exposing the projection outside the located Loop port;
8. Builder mutation, composer execution, ledger claims, fallback, or retry;
9. restoring the failing WIP wholesale;
10. a source/check file reaching 800 lines.

## Consultation question

May LOOP0 select Candidate A as the first canonical nested body-item
capability: one Loop-port-only typed projection built from the already sealed
`domain_parent + SourceBodyKindV1::item_segment(index)`, while generic
`body_stmt`, activation rows, and BODYDOMAIN0 remain unchanged? If not, which
existing authority should own the compact activation-compatible child site
without introducing a global root-segment equivalence rule?

## Decision closeout

Candidate A is selected after two independent worker reviews and explicit user
authorization to decide locally. The reviews agree that the existing branded
body carrier already owns every required fact and that the projection is a
capability view rather than a second path authority.

The implementation lock is:

```text
exact Located body
  -> same plan/caller preflight
  -> domain_parent must exist
  -> checked direct ordinal
  -> domain_parent + kind.item_segment(index)
  -> compact located statement carrier
```

The sole direct consumer is
`LocatedLoopPlanExpressionPortV1::exact_body_stmt`. Generic `body_stmt`, its
semantic-root publication, activation rows, caller-ledger body-domain logic,
and every source-path vocabulary owner remain unchanged. Nested IfThen/IfElse
bodies reached through the sealed Loop representation reuse the same typed
body-kind projection; no Loop-name or segment-spelling special case is added.

`SITEPROJ0-S0` is now the sole next code-facing row. `O0-P0`, `O0-G0`, and T0
remain forbidden until the SITEPROJ0 S0/P0/G0 evidence is green.

## SITEPROJ0-S0 closeout

One `project_compact_body_stmt` capability now projects an exact located nested
body item from `domain_parent + SourceBodyKindV1::item_segment(index)`. Its sole
direct consumer is `LocatedLoopPlanExpressionPortV1::exact_body_stmt`.
Generic `body_stmt` remains unchanged and its semantic-root regression stays
green. The focused port proof observes both surfaces explicitly:

```text
generic semantic carrier:
  Body(0)/LoopBodyRoot/LoopBody(0)/Initializer(0)

Loop-port compact carrier:
  Body(0)/LoopBody(0)/Initializer(0)
```

No activation-row lookup, segment stripping, new path vocabulary, Builder
state, ledger claim, fallback, or retry was added. Focused projection 1/1,
expression-port 10/10, located-legacy 15/15, all-target check, the existing
public expression-spine guard, pointer guard, and the 800-line cap are green.
`SITEPROJ0-P0` is next and owns only the actual/nested projection matrix; the
full default/strict representation proof remains in later `O0-P0`.
