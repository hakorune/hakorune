---
Status: fast__source_route_evidence
Task: MIR-CALL-PARSER-ARRAY-PUSH-ROUTE-OVERLAP-D0
Date: 2026-09-22
Parent: mir-call-parser-recursive-string-result-authority-d0-2026-09-22.md
NextCard: MIR-CALL-PARSER-ARRAY-PUSH-B2-I0 (after overlap decision)
Implementation permission: split pre-route structural source evidence and source-only route admission in the existing GenericLoop owner; preserve raw overlap, runtime ABI, production cutover and unrelated D WIP
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

The existing route census was executed on the changed tree:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib \
  'mir::builder::control_flow::joinir::route_entry::registry::generic_selection_matrix_tests' \
  -- --nocapture
result: 9 passed, 0 failed, 542 warnings
```

The `generic_both_fixture_records_overlap_without_deciding_precedence` row
still records `[GenericLoopV0, GenericLoopV1]`. The V0 lowerer and V1 direct
body lowerer both accept ordinary `MethodCall` statements, so this is a real
raw semantic overlap, not an accidental source-only extraction discrepancy.
The source two-push acceptance red is therefore a current design blocker, not
known baseline debt; the first literal and substring rows passed before the
third row stopped at route selection.

## Accepted decision — source admission over a real raw overlap

Keep the raw V0/V1 overlap unchanged. Do not change global V0/V1 precedence,
because raw compatibility fixtures intentionally observe both products and
V0 remains a valid physical lowerer for ordinary method effects. Instead,
extend the existing source GenericLoop Facts/Recipe owner with an explicit
source route evidence check. The evidence is co-sealed from the same source
BindingRef/continuation mapping and the already-issued V1 carrier relation;
it is not inferred from method name, call count, MIR, or `ArrayBox` spelling.

The source issuer may consume the existing V1 route token when:

```text
raw route set contains V1 (including the exact [V1] case or the admitted
source-overlap case [V0,V1])
and the source continuation/BindingRef/carrier relation is complete
and every admitted statement has a V1 source port mapping
```

It must reject before Builder effects when V1 is absent, when V0/V1 overlap is
present without the source evidence, or when any source item is missing,
foreign, duplicated, value-demanded, or reassigned. The raw schedule remains
observable; the source evidence is an admission contract, not a filter or a
retry of V0. Existing route selection tests must continue to prove that raw
overlap has no global winner.

The counterexample that remains rejected is the same two-route shape without a
complete source carrier/continuation mapping. A `routes.contains(V1)` shortcut,
method-name preference, unconditional `Ok(None)`, or issuer-side V0 drop is
not this decision and remains forbidden.

## Authority-cycle audit — implementation remains stopped

The read-only owner audit on 2026-09-23 found this current dependency order:

```text
issue_once
  -> verify_located_generic_loop_v1 (currently exact [V1] only)
  -> Ready
  -> claim_all / consume_pre_effect
  -> CallableGenericLoopV1SemanticRecipeIssuerV1
  -> carrier_relation::issue
```

`carrier_relation::issue` currently receives the selected source Facts receipt,
so it cannot be used as evidence to create the route token that is required to
create that receipt. Adding an overlap exception with a bool or an empty seal
would allow evidence from a different source session to be paired. This is a
design blocker, not a reason to relax the exact-selection check.

The next design must therefore define one move-only pre-route product from the
same source context, planner `GenericLoopV1Facts`, and pre-effect binding rows.
It must include exact source-item/continuation coverage and the existing carrier
relation inputs without requiring the selected route token. A later source
admission step co-seals that product with the raw `RecipeFirstRouteSelectionV1`
and the same planner outcome. Only the resulting source admission may accept
raw `[V1]` or `[V0,V1]`; other routes, V1 absence, and incomplete evidence stay
pre-effect rejects. The raw registry schedule and its neutral overlap tests do
not change.

### Pre-route product contract to settle

The candidate product is structural transport, not a new semantic receipt:

```text
PreparedCallableGenericLoopSourceEvidenceV1
  owner + exact parent/condition/body source contexts
  same planner outcome's GenericLoopV1Facts
  consumed CallableSemanticLoopHandoffPreEffectReceiptV1
  CallableLoopCarrierRelationV1 issued from that pre-effect + V1 facts
  ordered source items and source-target dispositions from the same probe
  continuation/source-port coverage and session identity
```

Its issuer runs before route admission and has no Builder, physical `ValueId`,
or selected route token. `carrier_relation::issue` must be split to consume
the existing pre-effect/source context/V1 facts directly; it must not receive
the later selected receipt. The source-target probe is consumed into ordered
dispositions at the same boundary, so an empty or foreign probe cannot be
reused by another route.

The subsequent source admission consumes this move-only product together with
the exact `PlanBuildOutcome` and raw `RecipeFirstRouteSelectionV1`. It may issue
the existing V1 route token for `[V1]` or the source-authorized `[V0,V1]` case
only when the product's owner, source sites, continuation rows and facts
identity match. This co-seal is the constructor boundary that prevents a token
from being paired with evidence borrowed from another source session. The
product is discarded before effects on any mismatch.

### Route admission constructor decision

Keep `RecipeFirstRouteSelectionV1` and its raw
`verify_located_generic_loop_v1` unchanged. The source owner receives a
private structural aggregate named `CallableGenericLoopSourceRouteAdmissionV1`;
it is not a `Verified*` or `Prepared*` language-semantic receipt and is not
visible to raw route execution. Its sealed route kind is:

```text
Exact(VerifiedLocatedGenericLoopV1SelectionV1)
SourceOverlap(opaque source-overlap seal)
```

The sole constructor consumes the same `PlanBuildOutcome`, raw selection,
`PreparedCallableGenericLoopSourceEvidenceV1`, and a source-session key made
from the owner plus the exact parent/condition/body source sites. It delegates
the `[V1]` case to the existing exact verifier. It creates `SourceOverlap` only
for raw `[V0,V1]` after the move-only evidence, continuation rows, carrier
relation, and source-item dispositions all match that session. Every other
route set, absent V1, or mismatched key is rejected before `claim_all` or
Builder effects. The aggregate retains raw selection and evidence together;
there is no later pairing by name, method count, MIR value, or bool flag.

## Ordered task queue

1. **Census** — add or reuse one structural test helper that records the finite
   table above and the raw overlap-family rows. No new source receipt is issued.
2. **MIR-CALL-PARSER-ARRAY-PUSH-SOURCE-EVIDENCE-D0** — accept the
   `PreparedCallableGenericLoopSourceEvidenceV1` contract above: source-session
   identity, exact continuation rows, carrier inputs, and source-item
   dispositions issued before route selection. Split the current 690-line
   GenericLoop source owner before it approaches the 760-line design threshold.
3. **Route admission design** — accepted above: the source-only structural
   admission aggregate binds raw selection, planner outcome, evidence, and
   session key in one constructor; raw registry precedence stays neutral.
4. **MIR-CALL-PARSER-ARRAY-PUSH-ROUTE-EVIDENCE-I0** — implement only after
   tasks 2–3 are accepted. Preserve raw route observability, keep all failures
   pre-effect, and split before 760 lines; 800 is a hard stop.
5. **Focused evidence** — prove literal, substring, and two-push source rows
   for optimize off/on; prove value-demand rejection, missing carrier,
   duplicate/foreign route, and the existing raw overlap corpus remain rejects.
6. **Return to B2** — only after one exact source route is selected, resume
   BindingRef carrier projection and conditional NamedArray construction/
   receiver/NoValue co-seal. Then consume the existing ArrayElementWrite row.

## I0 implementation checkpoint — 2026-09-23

Current execution row: `MIR-CALL-PARSER-ARRAY-PUSH-ROUTE-EVIDENCE-I0-FOCUSED-NEGATIVE`.

The first implementation part of `MIR-CALL-PARSER-ARRAY-PUSH-ROUTE-EVIDENCE-I0`
is now present in the existing GenericLoop owner. The source owner issues a
move-only `PreparedCallableGenericLoopSourceEvidenceV1` before source route
admission. It co-seals the consumed pre-effect, the carrier relation issued
from pre-effect plus GenericLoopV1 facts, the source item batch, ordered item
dispositions, and the owner/parent/condition/body session key. The route
admission retains that evidence beside the raw registry selection and admits
only `[GenericLoopV1]` or the source-authorized `[GenericLoopV0,
GenericLoopV1]` schedule. The raw registry verifier and its neutral overlap
tests are unchanged.

Carrier validation is still re-run against the planner outcome at Recipe
issuance so the existing mutation/negative tests remain fail-closed; it does
not use a selected route token as evidence. The physical adapter's additional
session/site recheck remains in the prior uncommitted carrier-projection WIP
and is not counted as this route-admission receipt.

Focused evidence collected on the working tree:

```text
generic carrier/source-facts tests: 6 + 13 passed
GenericLoop raw selection matrix: 9 passed
NamedArray source -> retained typed write -> C frame: 1 passed
quick profile warnings: 542 (same as the pre-slice baseline)
```

This is a source-route admission result, not an ArrayPush publication or
production cutover. The remaining I0 work is the named negative corpus for
missing/foreign/duplicate/value-demanded source evidence and a reusable guard
for the two admitted raw schedule shapes. Only after that corpus is green may
the B2 BindingRef/NamedArray physical co-seal resume.

## Stop conditions and acceptance

The raw-overlap and authority-cycle decisions above are accepted for the next
bounded implementation slice. The structural evidence aggregate is not a new
language-semantic receipt. Do not add a different `Verified*`/`Prepared*`
semantic product, source fallback, route retry, or backend-specific exception.
Do not claim the ArrayPush row or delete its old edge from a local green test.

Acceptance for this D0 is the named decision, finite table, preserved negative
route evidence, and a pointer/card update. It is not production cutover. The
existing uncommitted carrier-projection and NamedArray test WIP remains a
handoff artifact until this decision is accepted; it is not a receipt or a
caller switch.
