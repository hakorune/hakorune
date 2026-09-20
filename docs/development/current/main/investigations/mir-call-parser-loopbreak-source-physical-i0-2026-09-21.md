---
Status: design_stop__2026-09-21__ParserLoopBreakSourceShapeFrontier
Task: MIR-CALL-PARSER-LOOPBREAK-SOURCE-PHYSICAL-I0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-SOURCE-PHYSICAL-I0
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-source-transport-i0-2026-09-20.md
Implementation permission: the direct three-statement source Recipe/input/consumer
is implemented and type-checked; no broader LoopBreak shape, fallback, publication,
or compatibility retirement is authorized until the source contract below is accepted
NextCard: none
---

# Parser LoopBreak source physical I0

## Six-line brief

```text
Decision: admit the selected direct parser LoopBreak through one source-bound
  input and the existing loop_v0 physical owner after exact source/Facts/Recipe
  co-seal.
Source authority + canonical issuer: resolver-issued
  `issue_loop_break_source_projection_v1`, the existing
  `issue_callable_loop_break_source_facts_v1` planner/terminal co-seal, and
  the package observer `issue_loop_break_source_package_v1`.
Non-authority: synthetic AST/StmtRef construction, name remapping, a second
  LoopBreak Recipe, LoopRouteContext inference, legacy composer, or fallback.
Fail-fast boundary: named pre-effect rejection for missing, foreign, duplicate,
  out-of-root, order-mismatched, or non-direct source rows.
Smallest next slice: source-bound Recipe constructor, move-only physical input,
  raw caller consume, focused direct/negative matrix, and retirement of the
  selected source `GenericLoopV1NotSelected` terminal.
Non-claims: no compatibility-route deletion, publication, backend parity, VM
  work, or other LoopBreak topology.
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

This inventory is the design-stop evidence for the bounded tuple accepted
below. The compatibility route still has a non-source role, so its global
handler remains retained; the selected source terminal is the only old edge
retired by this row. The implementation must bind one move-only physical input
to the existing source Facts/Recipe outcome and prove the exact
loop/condition/body and exit/forest relation before `lower_loop_v0_core`.

The package transport is therefore evidence of retained ownership, not
source-to-MIR acceptance. No production switch, deletion, or publication claim
is made by this audit.

## Recipe/source alignment audit — 2026-09-21

The existing `build_loop_break_recipe` is the correct Recipe owner, but its
current compatibility constructor emits dummy `Span`s, variable nodes, the
break-if node, and assignment nodes from `LoopBreakFacts`. That representation
cannot be handed directly to `CallableLoopSourcePartsBlockV1::located_body`:
the located source port deliberately compares every recipe-body statement with
the resolver-owned source statement and would return `RecipeBodyMismatch`.

The safe implementation shape is therefore a same-owner source constructor,
not a second Recipe authority:

1. accept the already-issued source loop/condition/body carriers and the
   existing `LoopBreakFacts`/`LoopBreakSourceTopologyV1`;
2. build the same `RecipeBlock`/`RecipeBodies` shape while retaining the
   actual located loop, break-if, break-then, carrier-update, and step nodes;
3. prove the topology indices, break exit/forest, `LoopBreakStepPlacement`,
   carrier deduplication, and condition views before any Builder allocation;
4. pass that one recipe through `CallableLoopSourcePartsBlockV1` and the
   existing `lower_callable_loop_source_parts_block` /
   `lower_loop_v0_core` spine.

The source constructor must reject any source/body mismatch, local-prelude or
specialized topology, missing/foreign/duplicate site, and target relation
failure before `lower_loop_v0_core`. It may reuse the existing recipe builder's
verification helpers, but it must not reconstruct source identity from names,
ordinals, dummy AST, or `LoopRouteContext`.

This was the reason the row stayed at design stop before the owner/input/caller
decision. The accepted slice below supplies the source-bound constructor and
caller while retaining the compatibility route for its separate non-source
role.

## Accepted bounded implementation slice — 2026-09-21

The source authority, physical owner, and selected old edge are now named:

```text
source projection/Facts/package
  -> SourceLoopBreakPhysicalInputV1 (same LoopBreak Facts owner)
  -> source-bound build_loop_break_recipe constructor (same Recipe owner)
  -> CallableLoopSourcePartsBlockV1 + lower_callable_loop_source_parts_block
  -> lower_loop_v0_core / PlanVerifier / PlanLowerer
```

The selected caller is the existing `raw_loop_child_entry.rs` source-backed
callable-loop entry. It consumes the package candidate by exact parent source
site before the generic route issuer. The delete tuple is limited to that
source candidate's current `GenericLoopV1NotSelected` terminal and its direct
source-side handoff; the compatibility `route_loop_break_recipe` remains for
non-source callers and is not deleted in this slice. A candidate never falls
through to the generic route, and an absent/unsupported candidate keeps its
existing typed terminal.

Pre-effect validation must prove owner/function origin/source kind, parent /
condition / body lineage, the three direct statement sites, forest and explicit
break exit, target relation, recipe-body equality, and `LoopBreakStepPlacement`.
The physical adapter must fail before `lower_loop_v0_core` allocation on any
missing, foreign, duplicate, out-of-root, local-prelude, specialized, or
source/recipe mismatch.

Required focused evidence is one direct parser candidate reaching source-to-MIR,
one supported non-candidate, one specialized topology rejection, and
foreign/duplicate/out-of-root relation rejects; the existing compatibility
route tests remain separate evidence.

## Implementation acceptance evidence

The design stop is closed by the accepted owner/input/caller/delete tuple above.
The source-bound adapter and raw-caller consume are now type-checked, and the
existing focused LoopBreak source/package matrix is green (15/15 tests). The
merged parser lifecycle smoke also remains green, but it still terminates at
the named `GenericLoopV1NotSelected` boundary because the parser's
`starts_with/3` call is inside the larger LoopCond body rather than this
direct three-statement LoopBreak shape. This is dependency evidence only;
the direct parser source-to-MIR, negative physical matrix, publication, and
selected old-edge retirement remain open.

## Non-claims

The parser `starts_with/3` tuple remains stopped before physical lowering and
publication. No Cataloged/Selected claim, backend parity claim, source-to-MIR
claim, or legacy retirement claim is made here.

## Design stop — parser composite LoopBreak frontier — 2026-09-21

The direct physical slice is complete as a source-bound owner, but the merged
parser acceptance does not satisfy its direct-shape contract. The parser body
contains assignments, nested conditionals, and multiple exits; the existing
`LoopBreakRecipe` terminal is therefore outside the direct three-statement
candidate and currently stops the full package before catalog installation.
Treating that terminal as an empty/noncandidate row would hide a required
physical route, while widening the direct adapter would mix two topologies.

```text
Decision: keep the direct LoopBreak physical owner closed at its exact
  three-statement contract and design one separate source-aware composite
  LoopBreak boundary for the finite parser route before implementation.
Source authority + canonical issuer: resolver loop forest/exit ledger plus the
  existing LoopBreak Facts/Recipe issuer, co-sealed by the source Facts owner;
  the bridge supplies evidence only and does not issue route or terminality.
Non-authority: `LoopRouteContext`, AST/name rescan, direct-terminality proof,
  generic retry, compatibility fallback, VM route, or a second Recipe/JoinSig.
Fail-fast boundary: exact callable/owner/root/body/exit/target relation and
  complete source item coverage before any physical frame allocation; missing,
  foreign, duplicate, or unsupported composite rows remain named rejects.
Smallest next slice: finite census of the parser composite LoopBreak topology,
  then an owner decision for a source-port input that reuses the existing
  LoopBreak Recipe/physical spine or records `NoSafeSlice`.
Non-claims: no composite source-to-MIR, publication, caller switch, old-edge
  deletion, backend parity, or warning cleanup.
```

The existing direct adapter and its 15/15 source/package matrix remain valid
evidence for the direct shape. The merged parser smoke is dependency evidence
only: it reaches the named `GenericLoopV1NotSelected` terminal because the
composite LoopBreak route has no accepted source physical owner yet.

### Finite parser-shape census

| source owner/site | observed shape | boundary |
| --- | --- | --- |
| `ParserProgramBox.parse/2`, `parser_program_box.hako:81` | `loop(cont_prog == 1)` with the `skip_ws` assignment, multiple conditional exits, progress/guard assignments, and nested `starts_with/3` calls | composite LoopBreak; outside the direct three-statement owner |
| `ParserProgramBox.parse/2`, `parser_program_box.hako:182` | `loop(true)` with a `skip_ws` assignment, conditional `continue`, and tail `break` | LoopTrue composite; owned by the sibling LoopTrue boundary, not this direct row |

This census covers the two loop sites in the selected `ParserProgramBox.parse/2`
body that can precede the `ParserStringUtilsBox.starts_with/3` obligation. It
excludes imported parser callables and unrelated compatibility loops; those are
not acceptance evidence for this tuple. The next design decision must name one
source owner for the first row or explicitly seal it as `NoSafeSlice` before any
caller or terminal change.
