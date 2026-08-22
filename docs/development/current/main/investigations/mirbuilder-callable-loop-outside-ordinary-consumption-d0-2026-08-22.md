---
Status: Decision accepted; terminal Outside closeout
Task: MIR-CALLABLE-LOOP-OUTSIDE-TERMINAL-CLOSEOUT
Current execution row: MIR-CALLABLE-LOOP-OUTSIDE-TERMINAL-CLOSEOUT
Date: 2026-08-22
Priority: close the bounded terminal Outside lane without inventing a consumer
Parent: MIR-CALLABLE-LOOP-READY-GENERIC-LOOP-V1-RECIPE-AUTHORITY-D0
PreviousCard: mirbuilder-callable-loop-ready-generic-loop-v1-recipe-authority-d0-2026-08-22.md
NextCard: none until the next bounded Decision
---

# Callable Loop Outside ordinary consumption D0

## Six-line brief

Decision: accepted. `Outside(BodyOnlyRebind)` remains a typed terminal. The
read-only census found no source-backed ordinary consumer, so no new semantic
product or ordinary route is opened for this bounded cohort.

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

Smallest next slice: closeout evidence only: record the exact caller census,
prove the terminal has zero Builder effect, and add a reusable guard for the
forbidden Outside-to-old-route edges. The next independent lane may be chosen
after this closeout.

Non-claims: ordinary Loop lowering, route selection, new Recipe/Join meaning,
physical MIR, `ValueId`/PHI, publication, nested or multi-carrier expansion,
fallback retirement outside the Ready edge, performance, and backend work.

## Accepted terminal decision

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

The worker audit confirmed:

```text
Outside ordinary consumer = 0
Outside terminal consumer = 1
Outside -> lower_loop_or_freeze_v1 = 0
Outside -> route_loop / LoopRouteContext = 0
Outside -> PlanLowerer = 0
Outside -> Builder mutation = 0
```

The `Carrier` class wording inside an Outside row is retained as a parked
vocabulary cleanup because the row cannot reach the Ready or physical lane.
It is not promoted to an accepted carrier by this closeout.

## Closeout census

Record exact counts and owners for:

```text
Outside production callers                         = current typed terminal
Outside -> lower_loop_or_freeze_v1                 = 0
Outside -> LoopRouteContext/route_loop              = 0
Outside -> registry/PlanLowerer/Builder             = 0
Outside reason structured rows preserved to boundary = yes/no
body-only rebind source row relation                = exact grouped rows
ordinary source-backed consumer owner               = absent
second AST/source walk                               = 0
```

No ordinary consumer is opened. A future consumer would require a new
Decision naming its source authority and sole issuer; this card does not fill
the gap with `Option`, empty maps, a route kind, or a MIR lookup.

## Candidate decision

| Candidate | Decision | Reason |
| --- | --- | --- |
| Terminal Outside | **accepted** | complete source evidence is retained and no physical effect is guessed |
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
Outside terminal helper has no Builder/route/PlanLowerer input
Outside terminal caller count = 1
```

Positive evidence must show exact owner/site/role grouping. Negative evidence
must show foreign, missing, duplicate, and unsupported Outside rows stop with
no Builder effect. The accepted terminal has one production caller and is not
evidence of ordinary consumption.

## NoSafeSlice conditions

These conditions reopen a new design stop rather than changing this terminal
lane:

```text
ordinary consumption needs a second AST/source observation
Outside rows can only be paired by name/ordinal/pointer/digest
the old route is the only available consumer
body-only rebind must be promoted to a Ready carrier without source proof
the consumer needs Builder state before source validation
structured rows are flattened before the terminal/consumer boundary
the new consumer would create a second Recipe/Join authority
```

The bounded outcome of this card is the documented terminal Outside lane.
Ordinary consumption remains closed until a separate worker-reviewed Decision
identifies one named source-backed consumer and proves that the old route
remains unreachable.

## Closeout evidence

```text
normal_callable_loop_handoff focused suite = 6 passed
raw_loop_child_entry focused suite          = 8 passed
  includes outside_terminal_rejects_before_builder_effect
issuer/consumer + Outside forbidden-edge guard = passed
current-state pointer guard                    = passed
git diff --check                               = passed
raw_loop_child_entry.rs                        = 686 lines
normal_callable_loop_source_facts.rs           = 590 lines
normal_callable_loop_physical_adapter.rs       = 45 lines
```

The raw-entry negative test consumes the exact grouped Outside disposition
through `PreparedLocatedRawLoopChildEntryV1::lower_v1`; it observes the typed
terminal and verifies that both `current_function` and `current_block` remain
absent. The worker audit was read-only and made no file changes.
