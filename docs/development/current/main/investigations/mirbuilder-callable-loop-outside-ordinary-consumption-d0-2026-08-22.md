---
Status: Design stop; successor frontier after Ready R0
Task: MIR-CALLABLE-LOOP-OUTSIDE-ORDINARY-CONSUMPTION-D0
Current execution row: MIR-CALLABLE-LOOP-OUTSIDE-ORDINARY-CONSUMPTION-D0
Date: 2026-08-22
Priority: decide whether grouped Outside source evidence has one safe named consumer
Parent: MIR-CALLABLE-LOOP-READY-GENERIC-LOOP-V1-RECIPE-AUTHORITY-D0
PreviousCard: mirbuilder-callable-loop-ready-generic-loop-v1-recipe-authority-d0-2026-08-22.md
NextCard: none until Decision
---

# Callable Loop Outside ordinary consumption D0

## Six-line brief

Decision: not accepted yet. The current safe behavior is a typed terminal for
`Outside(BodyOnlyRebind)`. First census whether a real source-backed ordinary
consumer exists; if it does not, keep Outside terminal and close this row
without inventing a second lowering authority.

Source authority + canonical issuer: `normal_callable_loop_handoff` owns the
source-only projection and `CallableLoopOutsideReasonV1` owns the grouped,
move-only rows. A future ordinary consumer must be one named issuer/consumer
that consumes those rows exactly once; it must not re-observe AST or Builder
state.

Non-authority: separate binding/site arrays, `String`-flattened reasons,
`lower_loop_or_freeze_v1`, `LoopRouteContext`, registry selection, AST/name/
ordinal/pointer pairing, Builder maps, `ValueId`, empty/default facts, and any
fallback from Outside to the legacy route.

Fail-fast boundary: immediately after source classification and before the
first Builder effect. Missing relation, foreign owner/site, unsupported role,
or unavailable named consumer is a typed terminal with zero effect.

Smallest next slice: read-only worker/census of the exact source-backed
consumer boundary and its required owner relation. If no safe relation exists,
formalize `Outside -> terminal` as the bounded completion. If one exists,
write the accepted type/state/guard design before implementation.

Non-claims: ordinary Loop lowering, route selection, new Recipe/Join meaning,
physical MIR, `ValueId`/PHI, publication, nested or multi-carrier expansion,
fallback retirement outside the Ready edge, performance, and backend work.

## Current boundary

Ready R0 is now connected:

```text
Ready
  -> Facts issuer once
  -> claim_all once
  -> semantic GenericLoopV1 Recipe
  -> named physical adapter
  -> unpublished lower
```

Outside is intentionally different:

```text
Outside(BodyOnlyRebind)
  -> exact grouped source rows
  -> typed terminal
  -> Builder effect = 0
```

The old ordinary route must not be treated as an implicit consumer. The
current `CallableLoopOutsideReasonV1` is diagnostic/terminal evidence, not a
permission to reconstruct a plan from names or counts.

## Required census before implementation

Record exact counts and owners for:

```text
Outside production callers                         = current typed terminal
Outside -> lower_loop_or_freeze_v1                 = 0
Outside -> LoopRouteContext/route_loop              = 0
Outside -> registry/PlanLowerer/Builder             = 0
Outside reason structured rows preserved to boundary = yes/no
body-only rebind source row relation                = exact grouped rows
ordinary source-backed consumer owner               = named or absent
second AST/source walk                               = 0
```

If the only available consumer requires a new semantic `Verified*` or
`Prepared*` product, stop and name its source authority and sole issuer first.
Do not fill the gap with `Option`, empty maps, a route kind, or a MIR lookup.

## Candidate decision

| Candidate | Decision | Reason |
| --- | --- | --- |
| Terminal Outside | preferred safe default | complete source evidence is retained and no physical effect is guessed |
| Existing ordinary JoinIR | reject until a source-aware consumer exists | it re-observes route/Builder meaning and can hide a fallback |
| New source-backed ordinary consumer | conditional | only after owner, exact row relation, fail-fast boundary, and effect evidence are accepted |
| Default/empty Facts or synthetic carrier | reject | absence is not a source authority |

## Finite state

| State | Owner | Effect | Allowed next step |
| --- | --- | ---: | --- |
| `Located` | raw source handoff | 0 | classify rows |
| `OutsideObserved` | source projection | 0 | `TerminalOutside` or accepted consumer preparation |
| `TerminalOutside` | typed Outside owner | 0 | terminal discard |
| `ConsumerPrepared` | future named consumer | 0 | one consume/commit path |
| `RejectedBeforeEffect` | source validator | 0 | terminal discard |
| `Lowered` | future sole physical owner | unpublished only | terminal success |

There is no `Outside -> old route` transition in this state table.

## Acceptance / guards

```text
Outside grouped row issuer = 1
Outside production consumer = 0 or 1 named path
Outside -> lower_loop_or_freeze_v1 = 0
Outside -> route/registry re-selection = 0
Outside -> Builder effect before acceptance = 0
separate bindings[] + sites[] pairing = 0
String conversion before the terminal boundary = 0
AST/source second walk = 0
default/empty state merge = 0
all new Rust files < 760 lines
```

Positive evidence must show exact owner/site/role grouping. Negative evidence
must show foreign, missing, duplicate, and unsupported Outside rows stop with
no Builder effect. A caller-zero result is valid only for the terminal
decision; it is not evidence of ordinary consumption.

## NoSafeSlice conditions

Keep this design stop if:

```text
ordinary consumption needs a second AST/source observation
Outside rows can only be paired by name/ordinal/pointer/digest
the old route is the only available consumer
body-only rebind must be promoted to a Ready carrier without source proof
the consumer needs Builder state before source validation
structured rows are flattened before the terminal/consumer boundary
the new consumer would create a second Recipe/Join authority
```

The preferred bounded outcome is a documented terminal Outside lane. Open an
implementation slice only after a worker-reviewed Decision identifies one
named source-backed consumer and proves that the old route remains unreachable.
