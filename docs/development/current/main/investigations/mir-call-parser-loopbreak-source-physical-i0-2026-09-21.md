---
Status: design_stop__2026-09-21__ParserLoopBreakSourcePhysical
Task: MIR-CALL-PARSER-LOOPBREAK-SOURCE-PHYSICAL-I0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-SOURCE-PHYSICAL-I0
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-source-transport-i0-2026-09-20.md
Implementation permission: false until the source-bound physical owner and delete set are accepted
NextCard: none
---

# Parser LoopBreak source physical I0

## Six-line brief

```text
Decision: design one source-bound LoopBreak adapter that reuses the existing
  loop_v0 physical owner after exact source/Facts/Recipe co-seal.
Source authority + canonical issuer: resolver-issued
  `issue_loop_break_source_projection_v1`, the existing
  `issue_callable_loop_break_source_facts_v1` planner/terminal co-seal, and
  the package observer `issue_loop_break_source_package_v1`.
Non-authority: synthetic AST/StmtRef construction, name remapping, a second
  LoopBreak Recipe, LoopRouteContext inference, legacy composer, or fallback.
Fail-fast boundary: named pre-effect rejection for missing, foreign, duplicate,
  out-of-root, order-mismatched, or non-direct source rows.
Smallest next slice: inventory the existing source-parts/loop_v0 consumer,
  exact LoopBreak source site and exit/forest relation, and finite old-edge set.
Non-claims: no route execution, publication, backend parity, VM work, or delete.
```

## Design tasks

| order | task | acceptance evidence |
| --- | --- | --- |
| 1 | Identify the existing physical owner | one method receives the transported candidate and reuses `CallableLoopSourceParts`/`lower_raw_loop_v0`; no new physical owner |
| 2 | Close source relation | candidate site, break/update/step items, exit/forest relation, package brand, and selected callable owner are co-sealed before effects |
| 3 | Define pre-effect rejects | missing, foreign, duplicate, out-of-root, order mismatch, and specialized/non-direct topology each have named rejection rows |
| 4 | Enumerate production callers and old edges | finite caller/terminal inventory and exclusive delete tuple; no caller-zero shortcut |
| 5 | Decide implementation boundary | accepted bounded slice or `NoSafeSlice` with missing issuer/owner named; no synthetic receipt or fallback |

## Existing transport handoff

`MIR-CALL-PARSER-LOOPBREAK-SOURCE-TRANSPORT-I0` already retains the package
product through install and moves it once into the selected lowering state.
That product is a transport input only. This card must consume the exact
source candidate rather than re-scan the AST or reconstruct a route from a
selected key.

The existing physical core is reusable only after source alignment is proven.
The legacy `loop_break_composer` and `route_loop_break_recipe` are not source
consumers. `LoopRouteContext`, synthetic `StmtRef`, and unconditional fallback
remain outside the authority chain.

## Static owner and edge audit — 2026-09-21

The issuer is present; the earlier package-level `NoSafeSlice` premise was
superseded by the transport row. The current source authority chain is finite:

```text
issue_loop_break_source_projection_v1
  -> issue_callable_loop_break_source_facts_v1
  -> issue_loop_break_source_package_v1
  -> LoopBreakSourcePackageLoanV1
```

The reusable physical owner is the existing associated-source lowering spine:
`lower_callable_loop_source_parts_block` together with
`CallableLoopSourcePartsLoweringHooksV1::lower_raw_loop_v0` and the shared
`lower_loop_v0_core` frame/edge owner. A future source adapter may call this
spine once it owns a source-aligned LoopBreak recipe; it must not add a second
Recipe or re-enter `LoopRouteContext`.

The production inventory is currently:

| caller/edge | observed role | fate in this row |
| --- | --- | --- |
| `raw_loop_child_entry.rs` → `CallableGenericLoopSourceFactsIssuerV1` | source-backed callable loop entry; LoopBreak currently terminates as `GenericLoopV1NotSelected` before physical lowering | candidate source caller to be wired after the physical input contract is accepted |
| `route_entry/registry/handlers/routes.rs::route_loop_break_recipe` | compatibility route using `LoopRouteContext` and `RecipeComposer::compose_loop_break_recipe` | retained until a named source caller switches; it is not a source consumer |
| `loop_break_composer.rs::compose_loop_break_recipe` | exclusive composer called by the compatibility route | delete only with the route-handler switch; do not remove the shared `build_loop_break_recipe` used by matcher/tests |
| `callable_loop_source_lowering.rs` / `loop_v0.rs` | neutral physical frame and located source-part owners | retained and reused |

This inventory does not yet form an exclusive delete tuple: the source caller
currently stops before the compatibility route, while the compatibility route
still has a non-source role. Therefore the implementation permission remains
false. The next design decision must bind one move-only physical input to the
existing source Facts/Recipe outcome, prove the exact loop/condition/body and
exit/forest relation before `lower_loop_v0_core`, and then name the first
source caller plus the exact compatibility edge that it exclusively replaces.

The package transport is therefore evidence of retained ownership, not
source-to-MIR acceptance. No production switch, deletion, or publication claim
is made by this audit.

## Required design-stop evidence

The card cannot enter `fast` until the same owner names the source issuer,
physical consumer, pre-effect terminal, and deletion set. Focused evidence must
cover one direct candidate, one typed absence, one specialized rejection, and
foreign/duplicate/out-of-root relations. A local transport green is not
source-to-MIR acceptance.

## Non-claims

The parser `starts_with/3` tuple remains stopped before physical lowering and
publication. No Cataloged/Selected claim, backend parity claim, source-to-MIR
claim, or legacy retirement claim is made here.
