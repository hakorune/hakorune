---
Status: design_stop__generic_route_overlap
Task: MIR-CALL-PARSER-ARRAY-PUSH-ROUTE-OVERLAP-D0
Date: 2026-09-22
Parent: mir-call-parser-recursive-string-result-authority-d0-2026-09-22.md
NextCard: MIR-CALL-PARSER-ARRAY-PUSH-B2-I0 (after overlap decision)
Implementation permission: design and census only; do not relax source route exclusivity, weaken the natural ArrayPush fixture, add a fallback, or switch a production caller
Classification: BoxShape route-authority decision; no new semantic receipt
---

# ArrayPush source route overlap D0

## Six-line brief

```text
Decision: resolve the existing GenericLoopV0/GenericLoopV1 overlap before the
source-backed ArrayPush carrier projection can be accepted.
Source authority + canonical issuer: the same source loop handoff and existing
GenericLoop Facts/Recipe issuer; the existing route registry remains the sole
route selector.
Non-authority: ArrayBox names, method counts, MIR dumps, source-line lookup,
optimization mode, or an issuer-side filter that hides V0.
Fail-fast boundary: a source body with both route products is rejected or
resolved by the existing overlap owner before Builder effects and before the
source GenericLoop route token is issued.
Smallest next slice: census the finite ArrayPush shapes, settle V0/V1 policy in
the existing Facts/overlap owner, then prove one exact source route.
Non-claims: no ArrayPush publication, runtime Text/Fault or ownership proof,
production caller switch, VM repair, old-edge deletion, or whole parser green.
```

## Observed boundary

The source carrier projection WIP fixed the first natural fixture's dominance
shape: a body-defined value no longer feeds the loop header/exit, and the final
induction value is published back through the same source BindingRef. The exact
focused test then stopped on the two-push fixture with:

```text
[freeze:contract][callable-loop/route-not-front-selected]
NonGenericOrOverlapping {
  routes: [GenericLoopV0, GenericLoopV1]
} site=Body(2)
```

The fixture is the existing natural source shape:

```text
arr.push("one")
arr.push("two")
i = i + 1
```

The source issuer currently consumes `verify_located_generic_loop_v1`, which
requires the shared registry's raw execution schedule to be exactly
`[GenericLoopV1]`. It must not silently discard `GenericLoopV0`. The existing
route census already records the same raw family as an unresolved overlap; this
card turns that observation into the next bounded design decision.

The run also established that the carrier projection is not the current error
for the first literal and substring fixtures: no dominance error appeared before
the two-push route rejection. The run stopped at the third body, so optimized
variants after that point and full production acceptance remain unproven.

## Existing owners

```text
source loop context / BindingRef
  -> normal_callable_loop_source_facts::generic::issuer
  -> control_flow::joinir::route_entry::registry::selection
  -> existing GenericLoopV0/V1 Facts extraction and overlap policy
  -> source GenericLoopV1 Recipe/physical adapter
  -> retained NamedArray -> ArrayElementWrite handoff
```

The route registry and generic Facts extraction are the only candidate owners
for the overlap. The source issuer is a consumer of the route token; changing it
to ignore a second route would make the authority split worse. The physical
adapter cannot decide route meaning from a MIR plan.

## Finite census

The census boundary is:

```text
start: source GenericLoop planner input for Scan.run/1
end: raw route schedule and source issuer disposition
includes: literal push, substring push, two pushes; optimize off/on;
          GenericLoopV0/V1 Facts presence and existing overlap census
excludes: nested loops, If/try/fastmem/task-scope/catch, VM/compatibility,
          runtime allocation/text lifetime, unrelated LoopCond/LoopTrue routes
```

Record for each row:

| row | body | policy/mode | V0 | V1 | raw schedule | disposition |
| --- | --- | --- | --- | --- | --- | --- |
| A | `arr.push("text")` | release, optimize off/on | pending census | pending census | pending census | source route boundary |
| B | `arr.push(s.substring(0, 1))` | release, optimize off/on | pending census | pending census | pending census | source route boundary |
| C | two literal pushes | release, optimize off/on | observed present | observed present | `[V0,V1]` | named overlap stop |

The census must also run the existing raw overlap fixtures so a source fix does
not change their expected V0-only, V1-only, or genuine-overlap classifications.
No route may be selected from the number of calls or the ArrayBox spelling.

## Decision to make

Choose exactly one of these existing-owner outcomes after the census:

1. **Overlap is accidental extraction overlap.** Extend the existing
   `GenericLoopV0`/`GenericLoopV1` shape policy so a V1-owned body cannot emit a
   V0 product. The shared registry then produces `[V1]` for the admitted source
   shape while preserving raw V0 fixtures and genuine ambiguity rejection.
2. **Overlap is semantically real.** Keep the raw overlap and add an explicit
   source-aware route decision to the existing Facts/Recipe owner, carrying a
   co-sealed source continuation requirement. The source issuer may consume the
   resulting token only when the V1 source contract is complete; it may not
   filter the raw schedule or retry V0.

The decision must name the counterexample that remains rejected. A one-line
`routes.contains(V1)` preference, a method-name check, or an unconditional
`Ok(None)` is not a decision and is forbidden.

## Ordered task queue

1. **Census** — add or reuse one structural test helper that records the finite
   table above and the raw overlap-family rows. No new source receipt is issued.
2. **Authority decision** — inspect V0/V1 extraction, `v1_shape_blocks_v0`,
   `pred_generic_loop_v0`, `pred_generic_loop_v1`, and route selection together;
   write the selected outcome and negative counterexample in this card.
3. **Small implementation** — edit only the selected existing overlap/Facts
   owner. Preserve the source issuer's exact-selection check and keep all route
   failures pre-effect. Split a file before 760 lines; 800 is a hard stop.
4. **Focused evidence** — prove literal, substring, and two-push source rows
   for optimize off/on; prove value-demand rejection, missing carrier,
   duplicate/foreign route, and the existing raw overlap corpus remain rejects.
5. **Return to B2** — only after one exact source route is selected, resume
   BindingRef carrier projection and conditional NamedArray construction/
   receiver/NoValue co-seal. Then consume the existing ArrayElementWrite row.

## Stop conditions and acceptance

Stop before code if the census cannot distinguish accidental extraction overlap
from a real semantic overlap. Do not add a new `Verified*`/`Prepared*` product,
source fallback, route retry, or backend-specific exception. Do not claim the
ArrayPush row or delete its old edge from a local green test.

Acceptance for this D0 is the named decision, finite table, preserved negative
route evidence, and a pointer/card update. It is not production cutover. The
existing uncommitted carrier-projection and NamedArray test WIP remains a
handoff artifact until this decision is accepted; it is not a receipt or a
caller switch.

