---
Status: selected__fast__2026-09-19
Task: MIR-CALL-PARSER-LOOPCOND-SOURCE-HANDOFF-I0
Date: 2026-09-19
Parent: mir-call-parser-nested-loop-source-promotion-d0-2026-09-19.md
ProductionCaller: selected normal MIR/static-receiver route only
Implementation permission: the same-owner input/port contract below is accepted
for one bounded implementation slice; route completion, publication, and old-edge
retirement remain closed
Classification: BoxCount; one source-backed nested-loop handoff and one compatibility-edge retirement
---

# Parser LoopCond source handoff I0

## Six-line brief

```text
Decision: admit one parser-branded nested/exit-driven LoopCond source product
through the existing Facts/Recipe/JoinSig/physical pipeline and switch the
selected ParserProgramBox.parse/2 -> starts_with/3 row through that product.
Source authority + canonical issuer: same-invocation merged source plus the
resolver-issued loop forest and ResolvedExitRecordV1 set; the existing source
Facts issuer is extended as the sole co-seal owner.
Non-authority: AST/name/arity matching, parser line lookup, MIR scans,
LoopBreak legacy route, GenericLoop fallback, VM compatibility, and retries.
Fail-fast boundary: any owner/forest/path/exit/target/result drift rejects before
catalog or argument effects; no partial LoopCond physical session is retained.
Smallest next slice: source co-seal, LoopCond route token, source-port physical
consume, focused matrix, then delete only this tuple's retained old edge.
Non-claims: other parser loops/calls, expression-If PHI, VM/AOT parity, whole
R7 retirement, Windows evidence, or whole-library green.
```

## Exact acceptance tuple

```text
caller       = ParserProgramBox.parse/2
target       = ParserStringUtilsBox.starts_with/3
target site  = resolver-issued source site for parser_program_box.hako:102
route        = LoopCondBreakContinue (exactly one raw execution route)
forest       = root :81 plus children :131 and :182, parent-linked in order
exits        = all resolver rows for the three members, including :85/:95,
               :104/:109/:127/:142/:161/:165/:194 returns, :186 continue,
               and :188 break; :218 is outside the root loop
result       = ExactI64, required ordinal `[1]`
```

The line numbers are the reviewed finite source shape, not lookup keys. The
co-seal must carry `SourceNodeSiteV1`/frame/path identity from the resolver and
the target relation from the same invocation. No later owner may recreate a
site from a Hako line, method name, arity, or MIR.

## Existing owners and permitted edits

```text
resolved source forest/exits
  -> existing normal callable source Facts issuer (split before 760 lines if needed)
  -> LoopCondBreakContinue Facts/Recipe + one-shot route token
  -> existing loop_cond_bc physical pipeline through CallableLoopSourceExpressionPortV1
  -> existing completion/DraftSeal
  -> StaticResultPublicationIngressPortV1 and statement-If/Equal consumer
```

Permitted edits are limited to the existing owners:

- `src/mir/builder/control_flow/facts/loop_cond_break_continue.rs`
- `src/mir/builder/control_flow/recipes/loop_cond_break_continue.rs`
- `src/mir/builder/control_flow/plan/features/loop_cond_bc.rs` and its existing
  split helpers
- `src/mir/builder/control_flow/joinir/route_entry/registry/selection.rs`
- `src/mir/builder/normal_callable_loop_source_route.rs`
- the existing callable source Facts/port modules and their focused tests

The source physical path may not construct a `LoopRouteContext`, invoke the
legacy normalizer, or re-enter route classification. If a touched file would
cross 760 lines, split at the responsibility boundary before adding behavior;
800 lines is a hard stop.

## Progress checkpoint — source forest co-seal

The first implementation step is landed in the existing compiler projection and
neutral structural-facts owner. `issue_loop_cond_break_continue_source_forest_projection_v1`
consumes the resolver's complete nested forest, binds its ordered parent
relations through `VerifiedLoopSourceForestBindingV1`, and retains every
resolver exit under the selected root as a paired source-site/transfer row.
The focused fixture proves a three-member forest with parent indices
`[None, Some(0), Some(0)]` and four nested exits. No route, Recipe, Builder,
MIR, or physical identity is issued by this step; the product remains
caller-zero until the source Facts issuer consumes it.

Evidence: `forest_projection_seals_nested_members_and_all_root_exits` plus the
five existing single-loop projection tests pass under the focused quick
library filter. The remaining I0 work is the source Facts/Recipe handoff and
physical/static consumer path below; this checkpoint does not retire the old
edge or claim source-to-MIR acceptance.

## Design audit checkpoint — typed NoSafeSlice

The read-only bridge audit initially closed the next boundary as `NoSafeSlice`;
at that point no code or fixture change was authorized until this contract was
designed in the existing owners. `CallableGenericLoopSourceFactsIssuerV1` is the only current
source Facts issuer, but it accepts only the GenericLoop payload and emits
`GenericLoopV1` selection. The local `LoopCondBreakContinueFacts`/Recipe can
express nested AST shapes, yet its `StmtRef` exit items do not retain the
resolver `ResolvedExitRecordV1` rows or the forest parent binding. The existing
LoopCond composer/physicalizer also requires `LoopRouteContext` and re-enters
AST/legacy lowering; the source GenericLoop expression port cannot safely
consume it.

This was an internal authority gap, not an external wait. The reopen contract
is one same-owner co-seal that carries the forest binding, all paired exit
records, the selected source target relation, and a source-aware LoopCond
Recipe/JoinSig handoff into a physical adapter that does not construct
`LoopRouteContext`. Until that product and consumer are specified, retain the
selected compatibility edge and do not issue a route token, catalog row,
fallback, or production switch. Evidence: read-only audit of
`normal_callable_loop_source_facts.rs`,
`control_flow/facts/loop_cond_break_continue.rs`,
`control_flow/recipes/loop_cond_break_continue.rs`,
`plan/recipe_tree/loop_cond_composer.rs`, and
`plan/features/loop_cond_bc.rs` on 2026-09-19.

### Bridge audit recheck — same result, narrower reopen slice

An independent read-only recheck confirmed that `LoopPlanExpressionPortV1`
and `CallableLoopSourceExpressionPortV1` are available, but their only
production physical consumer is the GenericLoop adapter. The LoopCond chain
still enters `loop_cond_bc::lower_loop_cond_break_continue`, reads raw
`CondBlockView`/`StmtRef` items, and its nested path constructs a fresh
`LoopRouteContext` through `nested_loop_depth1_route`. The existing
`VerifiedLoopSourceForestBindingV1::into_source_binding` also accepts only a
portable `VerifiedLoopRecipeV1`, so it cannot bind this local LoopCond Recipe.

The next design slice is therefore one same-owner contract, before any code:
`SourceLoopCondPhysicalInputV1` must carry the already-issued forest binding,
paired exit records, exact source body/condition ports, and the selected
LoopCond Recipe/JoinSig into `loop_cond_bc` and its nested-item helpers. The
consumer must thread that port through header, item, nested-exit, and cleanup
lowering without constructing `LoopRouteContext` or reclassifying a route;
missing child/exit/site or any lowering error discards the whole session.
At the recheck boundary this did not authorize a new issuer, route token, catalog row,
fallback, production switch, or compatibility retirement.

## Same-owner bridge decision — accepted design boundary

The bridge reuses the existing local `LoopCondBreakContinueFacts.recipe` and
the existing `LoopCondBreakContinuePhiMaterializer`/cleanup/verifier owner. It
does **not** create a second portable `VerifiedLoopRecipeV1`, a new semantic
JoinSig authority, or a parallel LoopCond issuer. In this card, “JoinSig” means
the existing route-local phi closure produced by that physical owner; the
portable loop-recipe JoinSig family remains outside this row.

The one move-only source input is named
`SourceLoopCondPhysicalInputV1`. The existing source Facts issuer constructs it
after planner Facts/Recipe extraction and before any Builder allocation. Its
contract is:

```text
SourceLoopCondPhysicalInputV1
  owner/frame/source-kind/origin
  forest: VerifiedLoopCondBreakContinueSourceForestProjectionV1
  target: same-invocation source target relation and exact call site
  root/condition/body: located RawInvocationSourceContextV1 values
  item_bindings: every local StmtRef and nested recipe item paired with its
                 resolver-issued SourceNodeSiteV1/context
  exits: the forest's paired (SourceStmtSiteV1, ResolvedExitRecordV1) rows
  facts: exactly one existing LoopCondBreakContinueFacts + its Recipe
  source_port: borrowed CallableLoopSourceExpressionPortV1
```

Construction is a co-seal, not a later join. It rejects before Builder effects
when owner, frame, origin, root path, forest parent, source context, target,
recipe item, or exit evidence differs; when a `StmtRef` is missing, duplicated,
foreign, or recovered by name/line/ordinal; when the planner emits no exact
LoopCond Facts or an overlapping `GenericLoopV1`/`LoopBreak` route; or when the
selected result/argument ordinal is not `ExactI64/[1]`. The item bindings are
issued from the already located body contexts and resolver child roles; the
physical consumer never scans AST, searches Hako lines, or recreates a site.

The source physical adapter consumes this input exactly once and calls the
existing LoopCond lowering helpers through a `LoopPlanExpressionPortV1`. The
raw facade may keep its raw port, but the source path must thread the located
port through header condition, every recipe item, nested-loop entry, exit
transfer, and cleanup. Nested items use the co-sealed child context and forest
parent binding; they must not call `nested_loop_depth1_route`, construct a
`LoopRouteContext`, reselect a route, or enter the legacy normalizer. All
preflight checks happen before `LoopBlocksStandard5::allocate`; any lowerer
error drops the whole input and leaves no source physical session or catalog
effect.

The implementation slice is therefore four existing-owner changes: (1) extend
the current source issuer with the co-sealed item/exit input, (2) parameterize
the LoopCond helper chain over the existing expression port while retaining the
raw facade, (3) issue one exact LoopCond route token and hand the source result
to the existing static-result consumer, and (4) add the finite positive/
negative matrix before deleting only the selected parser tuple's old edge.
No new source authority, portable JoinSig, fallback, VM route, catalog row, or
compatibility retirement is implied until those four steps pass.

### Source bridge attach receipt — bounded task 1

Commit `688902ee67` adds the attach-only portion of this boundary. The
existing `CallableSemanticLoweringState` now constructs one owned projection
inventory from the package-scope `ResolvedFunctionLoweringInputV1`; the
inventory has an exact-site, one-shot `take_for` operation and no resolver
input lifetime is stored. The focused test
`owned_inventory_takes_exact_loop_projection_once` is green, and quick library
`cargo check` is green. This receipt does not claim that `RawLoopChildEntry` or
the LoopCond physical consumer has consumed the capability yet; task 2 remains
the production co-seal/route handoff.

The resolver-input attach point is the existing package scope
`with_callable_source_scope(input, ...)`. `raw_loop_child_entry` currently has
only the projected callable ledger and cannot mint the forest projection by
itself. The accepted attach is a move-only, owned projection inventory created
once from `input` at that scope and stored as one optional field in the existing
`CallableSemanticLoweringState`. The existing callable ledger is the sole
carrier: a child Loop takes the exact projection for its resolver-issued site,
and the scope restores the parent state after the callback. No third lifetime
parameter is added to `RawInvocationChildPortV1`, the resolver forest is not
copied into a second ledger, and no projection is re-created from AST, name, or
line. A missing or duplicate projection is a typed source-bridge rejection;
ordinary GenericLoop callers remain unarmed. This closes the previous lifetime
`NoSafeSlice` at the attach boundary, while route selection and physical
consumption remain pending implementation evidence.

### Source route-token receipt — bounded task 2 preparation

The bridge inventory now admits only forest roots. Nested loop sites are
subsumed by their nearest resolver-issued root, so one callable scope cannot
issue overlapping projections for the same forest. The root-site filter has a
focused positive test (`root_inventory_drops_nested_loop_roots`) in addition to
the existing exact take-once test.

`normal_callable_loop_source_route.rs` adds the route-neutral, move-only
`CallableLoopSourceRouteTokenV1` constructor. It co-seals the existing
`PlanBuildOutcome`/`RecipeFirstRouteSelectionV1` with the exact owned forest
projection and rejects missing Facts, missing LoopCond Facts, non-exclusive or
overlapping routes, foreign owner, and source identity drift. The same owner
now also exposes a strict co-seal constructor for resolver-issued method-call
item bindings and an exact target relation; it rejects an empty item set,
foreign/out-of-root items, a missing target, or a target site absent from the
item inventory. The focused route filter remains green (3/3), and the explicit
no-method-row inventory reject is green (1/1).

This is still a preparation product, not a production switch: no production
caller has supplied the target relation or item inventory, and no physical
consumer has been attached. The next slice must thread these fields from the
same-invocation source-target owner through the existing source Facts owner
before any caller can consume the token, lower `loop_cond_bc`, publish a
result, or delete the compatibility edge.

### Eager bridge regression audit — corrected bounded behavior

The fourth-day audit reproduced a real availability regression in the attach
step: a declared callable containing a loop under an `if` reached the source
bridge, whose eager forest projection returned
`ForestBinding(Source::UnsupportedAncestor { segment: IfThen(0) })`. Because
`CallableSemanticLoweringState::from_exact_source` propagated that error, the
whole callable failed before route selection even though the loop was outside
the selected LoopCond source route. This violated the GenericLoop-unarmed
boundary.

The bridge now skips only the typed `UnsupportedAncestor` projection reject and
leaves that loop unarmed; all locate, owner, forest, member, exit, and duplicate
errors remain fail-fast. The focused parser fixture
`nested_scope_loop_does_not_abort_callable_source_bridge` proves the nested
`if` case, while the existing exact-take and root-inventory tests remain green.
This changes no route, Recipe, physical consumer, publication, fallback, or
compatibility-retirement behavior. The remaining physical lowerer endpoint is
still the selected next task.

### Same-owner production threading receipt — bounded task 2 endpoint

The next bounded step now threads the resolver method-call rows and the
canonical target lookup through the existing source bridge. The callable
semantic state retains the function origin/source kind and an exact-root item
inventory; `RawLoopChildEntry` moves those fields, the forest projection, and
the target relation into the source Facts payload before Builder allocation.
The Facts issuer accepts the existing exclusive `LoopCondBreakContinue`
selection and issues one `CallableLoopSourceRouteTokenV1` carrying the
co-sealed projection, items, and target. Missing identity, parent, items,
target, target-site mismatch, and multiple target candidates remain typed
fail-fast boundaries. The route selection has focused positive, missing, and
overlap tests, and the existing source-route matrix remains green.

The source physical consumer is intentionally still absent. When the selected
LoopCond disposition reaches the raw entry, it consumes the route token and
stops at the named `callable-loop/loop-cond/source-physical-consumer-missing`
terminal before any Builder/legacy route effect. This receipt therefore claims
source production threading only; it does not claim source-to-MIR acceptance,
static-result publication, caller cutover, or old-edge deletion. The raw entry
test module was split at the existing responsibility boundary to keep the
implementation file below the 760-line design threshold.

### Physical consumer audit — typed NoSafeSlice

The read-only physical audit confirms that calling the existing LoopCond
adapter is unsafe. `LoopCondBreakContinueFacts.recipe` and its phi/cleanup/
verifier owner are reusable, but the current physical entry requires a
`LoopRouteContext`, reads raw `CondBlockView`/`RecipeBody` through `StmtRef`,
and nested lowering re-enters `nested_loop_depth1_route`. The current route
token would also discard the source contexts when consumed, so it cannot be
passed directly to that owner without losing the resolver relation.

The accepted design boundary is one same-owner `SourceLoopCondPhysicalInputV1`
transport aggregate issued by the existing source Facts owner. It must carry
the existing LoopCond Facts/Recipe, located root/condition/body contexts,
forest and exit rows, exact item/nested bindings, and the same-invocation target
relation. A source-port LoopCond entry and the existing split helpers then
consume it once, reject missing/foreign/duplicate item or exit bindings and
nested route re-entry before `LoopBlocksStandard5::allocate`, and propagate
failure to the existing rejected-session disposal. No `LoopRouteContext`, AST
or name remap, GenericLoop adapter, second Recipe/JoinSig, or physical layout
authority may enter this path.

This is an internal design gap, not an external wait. The current selected
route remains at the named physical-consumer terminal until the input contract
and port-parametric `loop_cond_bc`/item/nested/exit lowering are implemented.
The static publication tuple, old-edge deletion, VM route, and fallback remain
closed. Worker audit evidence: existing source adapter and route-token seams
reviewed on 2026-09-19; no code or Cargo changes were made for this audit.

The same-owner input/port contract is now accepted for a bounded fast slice.
The first implementation step may materialize `SourceLoopCondPhysicalInputV1`,
validate its co-sealed source relations before Builder allocation, and thread
the existing callable source port into the LoopCond entry. It must leave the
physical route terminal named until the port-parametric item/nested/exit
lowering is complete; no publication or compatibility retirement is implied.

## Ordered implementation tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Source bridge attach | Build the one-shot owned projection inventory in the existing callable semantic state and consume it through the existing callable ledger. Scope restoration, exact-site take, duplicate/missing rejection, and GenericLoop-unarmed behavior are focused and green; unsupported source ancestry is explicitly unarmed rather than a callable-wide error. |
| 2 | Source co-seal and LoopCond route token | One move-only product binds the exact three-member forest, ordered paths/frame keys, all resolver exits, parser brand/owner, and target/source site; the route registry yields exactly `[LoopCondBreakContinue]`, with GenericLoop, LoopBreak, overlap, and route re-entry as typed rejects. **Source-side production threading is now landed; the physical consumer remains the open endpoint.** |
| 3 | Source physical consume | Materialize and validate `SourceLoopCondPhysicalInputV1`, thread the existing source port, then parameterize `loop_cond_bc`/item/nested/exit lowering. Keep the named terminal until the full physical consumer is green. |
| 4 | Static tuple handoff | The selected static result reaches the existing statement-If/Equal consumer with ordered arguments and ExactI64 result; duplicate consume and wrong ordinal reject before argument effects. |
| 5 | Negative matrix | Wrong owner/brand, forest parent drift, omitted child, wrong path, missing/duplicate/foreign exit, wrong target/header/result, legacy route re-entry, and extra nested loop all fail closed. |
| 6 | Retirement and acceptance | After positive plus negative evidence, remove only the selected tuple's retained compatibility/static-child disposition and record source-to-MIR acceptance. |

## Progress checkpoint — physical input preflight

The first fast implementation slice now consumes the move-only
`CallableLoopSourceRouteTokenV1` into `SourceLoopCondPhysicalInputV1`. The
existing planner Facts/Recipe, source forest projection, source item inventory,
target relation, source contexts, binding pre-effect, and a borrowed
`CallableLoopSourceExpressionPortV1` move together; the shape and parent-site
checks reject before any Builder allocation. The raw entry invokes the input
validator and then stops at the named
`callable-loop/loop-cond/source-port-lowering-missing` terminal. No AST/name
remap, second Recipe/JoinSig, compatibility fallback, or physical layout is
introduced.

Evidence at `2026-09-19`: `CARGO_BUILD_JOBS=4 cargo check --profile quick
--lib -j4` passed; the focused route filter
`cargo test --profile quick --lib source_loop_cond_route_token
-- --test-threads=1` passed 3/3, including the missing-target physical transfer
reject. This receipt covers input materialization, source-port attachment, and
preflight only. The production-threaded preflight now consumes the located port
through the loop condition, every body statement, nested `if`/`loop` bodies,
and method-call receiver/argument carriers before the named terminal. It still
does not allocate MIR or claim physical lowering. The source-port
parameterization of header/item/nested/exit lowering, static tuple publication,
old-edge retirement, and source-to-MIR acceptance remain open.

### Source-port header extraction receipt

The first physical-lowering preparation slice now exposes
`lower_loop_header_cond_with_port` from the existing loop-header owner. The
raw `CondBlockView` facade keeps its prelude validation and delegates its tail
through the same `LoopPlanExpressionPortV1` core that a located source
consumer will use. The helper performs no route selection, AST lookup, or
fallback; it only lowers an already-issued expression input. The focused raw
facade parity test passes with one exact test selected. This is a
behavior-preserving preparation step: the source LoopCond entry still stops
before Builder allocation, and item/nested/exit lowering, publication,
retirement, and source-to-MIR acceptance remain open.

### Source-port simple-statement preparation receipt

The next physical-lowering preparation slice now routes the existing raw
LoopCond simple-statement facade through a port-parametric owner. Assignment,
local initialization, method/function calls, generic calls, and print effects
use the existing `LoopPlanExpressionPortV1` child roles and associated-input
helpers; the raw facade supplies `RawLoopPlanExpressionPortV1`, preserving its
pre-port behavior. BlockExpr loop preludes remain in their existing specialized
owner because their nested source contexts are a separate co-sealed boundary.

The quick library check passed, the focused direct-exit module passed 2/2, and
the associated-input parity module passed 3/3. This receipt only proves the
shared statement owner and raw parity preparation. The source LoopCond caller
has not yet been switched to the located port, and nested/item/exit physical
consumption, publication, old-edge retirement, and source-to-MIR acceptance
remain open.

### Source-port recipe item lookup receipt

The next preparation slice adds a shared body-input accessor for LoopCond
`StmtRef` items. It resolves the reference through the already-issued
`LoopPlanExpressionPortV1::body_stmt` input and then enters the existing simple
statement owner, so a located source caller can carry its statement context
without a `RecipeBody` rescan, line lookup, or name reconstruction. The raw
facade remains available through `RawLoopPlanExpressionPortV1`.

The focused body-input test and the direct-exit tests pass 3/3. This is still a
preparation seam: no source LoopCond production caller consumes it yet, and
complex item variants, nested/exit transfer, physical allocation, publication,
old-edge retirement, and source-to-MIR acceptance remain open.

### Source-port direct-item dispatch receipt

The LoopCond item owner now exposes a source-port preparation entry for the
`Stmt` recipe variant. It resolves the `StmtRef` through the supplied body
input and delegates to the shared simple-statement owner. Complex variants
return no source lowering result deliberately; they cannot fall through to the
raw item dispatcher until their nested and exit contexts are parameterized.

The raw body-input item test remains green at 3/3. This is a preparation seam,
not a production switch: no complex item lowering, nested/exit transfer,
physical allocation, publication, or compatibility retirement is claimed.

### Source-port direct-exit transfer receipt

The existing `parts::exit` owner now has a port-parametric direct exit entry.
`ExitLeaf::Return` lowers its value through the exact source input, while
depth-one break/continue reuse the existing PHI closure. The LoopCond source
item seam accepts this `ExitLeaf` shape and resolves its statement through the
same body input; unsupported depths remain named rejects.

The focused LoopCond utility matrix passes 4/4, including a return value and
the direct item path. This does not cover exit-if preludes, nested loops,
complex recipe branches, physical allocation, publication, or old-edge
retirement.

## Focused validation

Use one `cargo test --profile quick --lib` process with at most four build jobs
and exact filters for the new source handoff, route selection, physical
consume, and static tuple lifecycle. Add structural guards for no
`LoopRouteContext`/legacy fallback on the source path, no AST/name remap, and
the selected old edge reaching zero. Existing baseline reds remain separately
classified; they do not widen this row.

No Windows or whole-library run is required for this I0. The parent static
card owns the later phase14/16/17 source-to-exe closeout; this card must first
prove the selected source-to-MIR handoff and its old-edge deletion.
