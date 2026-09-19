---
Status: closed__no_safe_slice__parser_loop_forest_exit_coseal__2026-09-19
Task: MIR-CALL-PARSER-LOOP-FOREST-EXIT-COSEAL-D0
Date: 2026-09-19
Parent: mir-call-parser-loop-source-admission-d0-2026-09-19.md
Implementation permission: false; define the missing source product only
Classification: BoxCount; one finite parser-loop forest and exit contract
---

# Parser loop forest and exit co-seal D0

## Six-line brief

```text
Decision: define whether the existing callable-loop source authority can co-seal the finite ParserProgramBox.parse/2 loop forest, binding schedule, and resolved exits as one product.
Source authority + canonical issuer: resolver-owned CallableSemanticSourceLedgerView (`resolved_loop_source_context`/`resolved_loop_source_forest`/`resolved_exits`) consumed by the existing callable-loop source issuer; no second issuer.
Non-authority: LoopBreak AST facts, parser names/lines, AST/MIR rescans, route-local coordinates, VM/compatibility fallback, and independently paired exit or binding receipts.
Fail-fast boundary: exact outer loop site, ordered forest parent indices, owner/frame identity, every break/return transfer target, and source binding coverage must co-seal before Recipe or physical lowering.
Smallest next slice: map those source products to the existing portable LoopRecipe source binding and JoinSig/physical boundary, then name the first missing consumer field.
Non-claims: no parser implementation, static catalog/publication, resolver If support, legacy route re-entry, production switch, or compatibility retirement.
```

## Finite source contract

```text
owner       = ParserProgramBox.parse/2
root loop   = loop(cont_prog == 1)
children    = direct condition/body plus nested static_semis and loop(true)
exits       = resolved break/return records, each targeting its sealed loop/function region
bindings    = condition reads and body rebinds from the existing callable handoff
target      = ParserStringUtilsBox.starts_with/3 (consumer evidence only)
```

The parent source-admission card established that the loop itself is the
blocker. This card designs only the missing co-seal; it must not infer any
relation from the target call or from Hako source line numbers.

## Existing products and gaps

| Product | Reusable evidence | Missing for this shape |
| --- | --- | --- |
| `CallableLoopReadyBodyOnlyProductV1` | owner, loop site, direct condition/body binding rows, affine pre-effect claim | nested loop forest and resolved break/return transfer set |
| `VerifiedLoopSourceForestBindingV1` | resolver-issued source paths and parent-index validation against a portable Recipe | no parser-loop Recipe/JoinSig consumes this forest today |
| `ResolvedExitRecordV1` | exact origin and control-transfer target for each exit | no callable-loop source product retains exits with the binding schedule |
| `GenericLoopV1SemanticRecipeV1` | existing source Recipe and physical adapter | rejects `NestedLoopOutsideFirstCohort` and cannot carry this exit contract |
| `LoopBreakFacts` | legacy three-statement break facts | parser body is stateful/nested and its AST facts are not source authority |

The product must remain move-only and owner-branded. It cannot expose a
reacquirable `(forest, exits, bindings)` tuple or let a later consumer pair
independently issued rows.

## Read-only product mapping receipt — 2026-09-19

The existing source adapter can consume only part of this contract. `bind_resolved_loop_source_forest_v1`
converts the resolver forest into `VerifiedLoopSourceForestBindingV1`, preserving the owner,
ordered source paths, and parent indices. `into_source_binding` then checks dense Recipe loop
coverage, root-parent shape, and every parent index. This is reusable only after a parser Recipe
with the same forest exists; it does not create that Recipe.

`ResolvedExitRecordV1` already co-seals the exit origin, containing source region, and typed
transfer (`Continue`, `Break`, or `Return`). There is no exit field in
`PreparedCallableGenericLoopSourceFactsPayloadV1`, `CallableLoopReadyBodyOnlyProductV1`, or
the borrowed `CallableGenericLoopSourceRelationViewV1`. The existing callable handoff therefore
cannot prove that every parser break/return belongs to the sealed loop forest.

The portable Recipe schema has `loops`, `blocks`, and `exits`, and
`LoopJoinSigElaboratorV1` can consume those rows after Recipe verification. Its
`VerifiedLoopContinuationContractV1` transports one JoinSig-derived `After` binding; it is not a
source exit inventory. The current `CallableGenericLoopV1PhysicalAdapterV1` receives only the
GenericLoop condition/body view and lowers through `compose_source_generic_loop_v1_recipe_with_port`;
it has no forest or exit consumer.

An adjacent source-aware owner does not close this gap. `issue_nested_predicate_source_projection_v1`
and `produce_nested_predicate_recipe_v1` do consume a resolver forest, Recipe, and JoinSig, and the
canonical nested-predicate lowerer has a physical owner. That owner is deliberately a different
finite shape: it requires exactly two forest members (`[None, Some(0)]`), fixed root/child body
lengths and i64 recurrence roles, and its emitted Recipe has `exits = Vec::new()`. It therefore
cannot consume the parser's `cont_prog` loop with nested scans and break/return transfers without
widening its semantic contract. Reusing it by relaxing a shape predicate would create a new
meaning under the wrong owner.

**Mapping decision:** the first missing consumer field is an atomic, owner-branded
`loop-forest + resolved-exit-set` relation carried with the existing callable source Facts/Recipe
claim. Adding only a forest binding, only an exit receipt, or only a physical adapter argument
would recreate the forbidden post-hoc pairing. Until one existing source-aware issuer and one
Recipe/JoinSig/physical consumer co-seal that relation, this D0 remains a design stop and the
parent static tuple remains at `GenericLoopV1NotSelected`.

## Negative matrix receipt — 2026-09-19

| Counterexample | Existing boundary | Status for the co-seal product |
| --- | --- | --- |
| foreign owner or frame | resolver owner verification, `VerifiedLoopSourceForestBindingV1` owner check, and the physical adapter's ledger-owner check | reusable pre-effect guards; the new product must carry one owner/frame token through all fields |
| forest parent-index drift or root with a parent | `ParentIndexOutOfRange`, `RootParentMismatch`, and `RecipeParentMismatch` in `into_source_binding` | reusable only after a parser Recipe has dense loop keys; no parser Recipe exists yet |
| missing exit or exit for a different loop/function | resolver verifies `source_region` and typed transfer targets, but callable source Facts retain no exit set | **uncovered**; the co-sealed product must reject before Recipe/physical effects |
| duplicate exit/site membership | resolver maps are keyed by sealed source sites, but the callable handoff does not consume a complete exit inventory | **uncovered at the selected owner**; no deduplication or post-hoc merge may be added |
| condition/body binding-site drift | existing `ParentSiteMismatch`, `ConditionSiteMismatch`, `BodySiteMismatch`, and pre-effect schedule checks | reusable for direct rows; nested descendant coverage is still missing |
| nested loop omitted from the forest | `nested-loop-profile-not-admitted` and `NestedLoopOutsideFirstCohort` | current typed terminal; promotion requires the same co-sealed forest to reach Recipe |
| binding row without matching forest member | direct body-only schedule validates its own rows, but has no forest/exit relation | **uncovered**; reject as an unconsumed sibling rather than dropping the row |

This matrix closes the negative-design inventory for the finite parser shape. It does not
authorize a new issuer or a predicate-only route change: the three uncovered cases are all
missing fields in one co-sealed product, not independent fallbacks.

## D0 progress decision — 2026-09-19

Tasks 1 and 2 are closed at the design level: the required atomic product is named, and its
existing forest, exit, Recipe, JoinSig, continuation, and physical consumers are mapped. Task 4
is also closed by the negative matrix above. Task 3 has a typed physical result: the current
callable GenericLoop adapter consumes only the one-condition/one-body view, so it cannot accept
this product without a new source-aware consumer contract. Task 5 is closed by retaining
`NoSafeSlice`: no existing source-aware issuer and Recipe/physical owner can consume this finite
parser shape without a new semantic loop contract. The parent static tuple remains parked at
`GenericLoopV1NotSelected`.

Any future promotion card must name the exact extension owner and its delete-set
before any field is added. This closed D0 creates no implementation permission.

## Ordered design tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Co-seal shape | Define the one product's fields, ownership, frame, forest order, and exit coverage; include direct and nested loop identities. |
| 2 | Recipe mapping | Show which existing `LoopRecipeSourceBindingV1`, `LoopJoinSigElaboratorV1`, and continuation contracts consume each field; missing mapping stays a typed gap. |
| 3 | Physical boundary | Inspect the existing loop physical adapter and name whether it can consume the extended Recipe without a second owner; otherwise retain `NoSafeSlice`. |
| 4 | Negative matrix | Pin foreign owner/frame, forest parent drift, missing exit, wrong transfer target, duplicate site, binding-site drift, and nested coverage rejection. |
| 5 | Decision/next card | Choose one authority extension or retain the typed terminal. If extension is accepted, create a separate implementation I0 with focused guards; no code in this D0. |

## Acceptance and non-claims

Acceptance is a source-to-Recipe authority matrix with one named physical
consumer or a typed `NoSafeSlice`. The static parent remains before
Cataloged/Selected until this matrix and its negative evidence are complete.
No new parser issuer, fallback, AST rewrite, MIR scan, VM route, or production
caller switch is authorized here.

## Closeout receipt — 2026-09-19

The forest/exit mapping and negative matrix are complete. The only missing
field is the atomic owner-branded relation that co-seals the parser loop
forest and all resolved exits with the callable source facts. No current
Recipe/JoinSig/physical consumer carries that relation, so promotion is not a
safe slice. This card is closed as `NoSafeSlice`; it grants no implementation
permission and does not reopen the legacy LoopBreak route. The independent
source-hint red recovery was a separate fixture-only I0 and is now closed;
this parser-loop terminal remains unchanged.
