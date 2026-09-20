---
Status: selected__fast__2026-09-20__LoopTrueSourceFactsProduct
Task: MIR-CALL-PARSER-LOOPCOND-SOURCE-HANDOFF-I0
Date: 2026-09-19
Parent: mir-call-parser-nested-loop-source-promotion-d0-2026-09-19.md
ProductionCaller: selected normal MIR/static-receiver route only
Implementation permission: the accepted D0 permits the bounded LoopTrue source
Facts product and its reject guards. The Facts/Recipe issuer remains the
existing owner; source-port physical consume, route completion, publication,
and old-edge retirement remain closed
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

The source exit entry is intentionally limited to direct `ExitLeaf` rows. It
does not reinterpret an `ExitIf` or nested recipe as a direct exit, so those
shapes remain explicit next work rather than entering a raw fallback.

### Source-port ExitIf shape preflight receipt

The next preparation slice reuses the existing `parts::exit_branch` owner for
the first `ExitIf { block: None }` source shape. Before any Builder allocation,
the located source port now requires an `If` with no `else`, an exactly one
statement then body, and a value-bearing `return`; the return value is resolved
through the same child expression port. Branch preludes, value-less returns,
break/continue tails, optional else branches, `ExitIfTree`, nested loops, and
all physical lowering remain explicit rejects for later slices. The validator
does not allocate a copied body or issue a Recipe/MIR result.

The focused source-port validator matrix passes 3/3, and the existing LoopCond
utility matrix passes 4/4. This receipt proves only the bounded ExitIf shape
preflight; source caller cutover, physical branch lowering, static publication,
old-edge retirement, and source-to-MIR acceptance remain open.

Warning cleanup remains a separate parked hygiene task,
`MIRBUILDER-WARNING-BASELINE-REFRESH-I0`, in the workstream cleanup map. It
must first classify lib/lib-test lint/file/owner/role before a single unused
import cohort is changed; this LoopCond slice neither suppresses nor widens
that warning baseline.

### Fourth-day audit reconciliation — source bridge and external closeout

The audit's eager-bridge concern is not reproduced as a current implementation
failure at the reviewed boundary. `CallableLoopSourceBridgeV1::from_input`
skips only the typed `UnsupportedAncestor` projection reject; locate, owner,
forest, member, exit, and duplicate errors still fail fast. The focused
`nested_scope_loop_does_not_abort_callable_source_bridge` test is green (1/1),
and the direct `UnsupportedAncestor` path guard is green (1/1). This proves the
existing `if` ancestry regression is closed. The equivalent `try`, task-scope,
fastmem, and catch fixtures are not yet observed, so they remain a verification
item before source caller cutover rather than a new semantic slice.

The borrowed Map residence row is already `Open, not selected` in the hygiene
punchlist; its production caller and direct tests were present before the audit.
The raw-pointer/root-lifetime API remains an independent owner concern and does
not reopen this parser LoopCond lane. The fixed 11-entry real-app EXE suite is
also a separate existing closeout owner; its missing receipt does not authorize
publication, fallback, or old-edge work in this card.

### Worker audit checkpoint — physical consumer decomposition

A read-only worker audit on 2026-09-19 decomposed the remaining physical
consume into the concrete recipe inventory and per-variant owner surface. The
root loop recipe for the acceptance tuple is statically determined: four
`Stmt` items (:82, :99, :100, :122), one `ExitIfTree{mode:ExitIf}` (:83-86),
four `ProgramBlock` items with `stmt_only: None` (:88-97, :101-120, :124-151,
:153-210), and one `GeneralIf` (:123). The accept kind is `MixedIf`, the body
lowering policy is `RecipeOnly`, and `propagate_nested_carriers` is false. One
residual predicate stays a verification item: `loop(true)` at :182 requires
`is_supported_bool_expr_with_canon` to accept the literal for its `LoopV0`
container arm, and the LoopCond-exclusive route selection requires the strict
+ `planner_required` environment, which acceptance evidence must pin.

The nested children :131/:182 are not `NestedLoopDepth1` items. They sit
inside `ProgramBlock` if-bodies and lower through `RecipeItem::LoopV0` ->
`parts::loop_::lower_loop_v0`, whose contract already avoids
`nested_loop_depth1_route` and `LoopRouteContext`. No route re-entry is on
this tuple's path; the nested-loop concern reduces to threading the located
port into the LoopV0 entry, not to bypassing route classification.

The audit also confirms the co-seal gap named by this card: neither
`CallableLoopSourceRouteTokenV1` nor `SourceLoopCondPhysicalInputV1` currently
proves the `ExactI64`/required-ordinal `[1]` tuple, and no production caller
consumes `lower_loop_cond_item_input` yet (its callers are test-only). The
permitted edit surface for the remaining slices is the existing-owner set
already used by the landed seams plus the `plan/parts/associated_source*`
provider/hooks machinery, `plan/parts/loop_/*`, and the ported `parts::stmt`/
if-join equivalents; all are existing-owner extensions, not new authorities.

Accepted decomposition (same-owner, no new authority):

1. `ExactI64`/`[1]` co-seal check: the physical-input boundary consumes the
   existing `project_static_exact_i64_requirement_v1` authority
   (`callable_result_representation/static_exact_i64_requirement.rs`) for the
   co-sealed `source_target`, cross-checks the target key, and rejects
   `callable-loop/loop-cond/result-requirement-mismatch` before Builder
   allocation when the tuple is not `ExactI64` with required ordinal `[1]`.
   The catalog attach point is confirmed during implementation; if the
   catalogs are not reachable through `source_ledger`, the check moves to the
   Facts-issuer stage where they are in scope.
2. One `PartsAssociatedSourceV1` sibling provider over
   `CallableLoopSourceExpressionPortV1` plus one
   `PartsAssociatedLoweringHooksV1` implementation, placed under
   `plan/parts/associated_source/` (the sealed trait and dispatcher are
   `parts`-private). It reuses `lower_verified_parts_associated_block` and
   the existing ported owners: `lower_simple_effect_stmt_input` for opaque
   statements and `lower_loop_cond_exit_source_input` for opaque exits.
   Recipe block contracts (the `try_build_exit_allowed_block_recipe`
   derivation on a `ProgramBlock` statement, the issued ExitOnly branch
   recipes on `ExitIfTree`) remain the structural packaging authority on
   already-co-sealed statements; they classify no new semantics. Located
   statements resolve through `body_stmt`/`child_body_from_stmt`, and a
   `Synthetic` body input must not appear on the located spine because it
   drops located identity for every descendant.
3. `lower_raw_loop_v0` hook arm -> a port-parametric entry on the existing
   `loop_v0` owner: the issued `LoopKindV0`/`body_contract`/`features` stay
   the semantic payload while the located loop statement's condition and
   body resolve through port children; nested body items recurse through the
   same provider. `call_source` already returns `LocatedMethodCall` for any
   sourced method call, so nested calls keep located identity without a
   recipe schema change or a per-loop forest-binding consumer; the
   projection's member coverage stays an input-validation property.
4. Item arms in `lower_loop_cond_item_input`: `ProgramBlock` (ExitAllowed
   driver on the located if statement), `GeneralIf` (NoExit), and
   `ExitIfTree` (ExitOnly); `Stmt`/`ExitLeaf` are already landed. Every other
   variant (`TailBreak`, `ContinueIfWithElse`, `ConditionalUpdateIf`,
   `Else*`, `NestedLoopDepth1`, `ExitIf` with `block: Some`) stays a named
   reject, and `Ok(None)` at the driver is fail-fast; it never falls through
   to `lower_loop_cond_item`.
5. Carrier collection (`collect_outer_from_body`,
   `collect_carrier_vars_from_condition`) stays on the co-sealed
   condition/body: it only collects variable names for carrier discovery on
   shape-verified input and issues no authority.
6. A port-parametric `lower_loop_cond_break_continue` sibling consumes the
   validated `SourceLoopCondPhysicalInputV1` end to end
   (`LoopBlocksStandard5::allocate` -> carriers -> phi materializer ->
   `lower_loop_header_cond_with_port` -> ported item driver -> cleanup ->
   verifier -> `CorePlan::Loop`). The named `source-port-lowering-missing`
   terminal remains until this entry is green.
7. Caller cutover replaces the terminal in `raw_loop_child_entry.rs`; the
   `callable_handoff=None` legacy edge and the raw item branches retire only
   after positive plus negative evidence (tasks 4-6 unchanged).

The pre-cutover verification items are unchanged and now include the
`loop(true)` bool-expr residual above, the `try`/task-scope/fastmem/catch
`UnsupportedAncestor` fixtures, and confirmation that the armed
`RawInvocationChildPortV1` path is the one reached by the `parse/2`
production compile. Non-claims are unchanged: no publication, caller cutover,
old-edge deletion, VM route, or fallback is authorized by this checkpoint.

### ExactI64/[1] co-seal receipt — decomposition item 1

The selected-result requirement is now co-sealed onto the source target
relation. `VerifiedStaticCallResultPublicationOwnerV1` exposes
`selected_handoff_for_source`, a read-only peek that borrows one selected
publication row without consuming it; `take_for_source` stays the sole
consumption boundary for the later physical consumer. The peek is forwarded
through `ModuleDraftCollectorV1` and `ModuleLoweringPortV1`, and
`source_target_for_loop` in `raw_loop_child_port.rs` copies the row's
representation and required-i64 ordinals into the new
`CallableLoopSourceTargetRequirementV1` evidence on
`CallableLoopSourceTargetRelationV1`, rejecting
`SourceTargetRequirementMismatch` when the peeked row's site or target does
not match the relation being minted. `SourceLoopCondPhysicalInputV1::
validate_for_source_port` now rejects
`callable-loop/loop-cond/result-requirement-mismatch` before any port or
Builder effect when the co-sealed requirement is not `ExactI64` with required
ordinal `[1]`. This resolves the audit checkpoint's open attach question: the
requirement is captured where `module_port` is in scope (relation issuance),
so no catalog needs to be reachable through `source_ledger`.

Focused evidence: `cargo check --profile quick --lib` green; new tests
`source_target_requirement_copies_selected_handoff_evidence`,
`source_target_relation_accepts_only_exact_i64_ordinal_one`,
`selected_handoff_peek_reads_requirement_without_consuming`, and
`selected_handoff_peek_stays_empty_for_target_only_and_foreign_sites` pass;
the neighboring `static_call_result_publication_owner` (9/9),
`source_loop_cond` (3/3), `source_loop_item` (1/1), and `armed_scope` (1/1)
filters stay green.

This receipt claims only the co-sealed requirement evidence and its
fail-fast check. It does not claim ProgramBlock/LoopV0/ExitIfTree physical
lowering, publication consumption, caller cutover, or old-edge retirement;
the `source-port-lowering-missing` terminal remains the active boundary.

### Located parts provider and hooks receipt — decomposition item 2

`plan/parts/associated_source/` now hosts the located callable-loop sibling
provider recorded in the audit checkpoint. `callable_loop_source.rs`
implements `PartsAssociatedSourceV1` over `CallableLoopSourceExpressionPortV1`
with two block carriers: `located_body` pairs a recipe block with a
`child_body_from_stmt`-projected body and `singleton` pairs a
`from_ref(stmt)` recipe with the already-located statement. Both reject
`Synthetic` carriers up front, re-prove recipe/body 1:1 alignment
(`RecipeBodyMismatch`), reject foreign arenas (`ForeignRawBlock`), wrap
port projection failures as `SourcePortProjection`, and compare the issued
`CondBlockView` against the projected condition (`ConditionViewMismatch`).
`callable_loop_source_lowering.rs` implements
`PartsAssociatedLoweringHooksV1`: opaque statements reuse
`lower_simple_effect_stmt_input` (widened to `pub(in crate::mir::builder)`),
an opaque `If` re-derives the singleton no-exit/exit-allowed container recipe
exactly like the raw return-prelude arm, opaque exits reuse
`lower_loop_cond_exit_source_input`, `ExitOnly`/`ExitAllowed` ifs reuse
`lower_exit_if_state_core` with `lower_cond_expr_to_if_plans_input`, and
`Join` ifs reuse `lower_if_join_state_core`; branch blocks recurse through
the same neutral driver. `LoopV0` stays a named reject
(`loop-v0-source-lowering-missing`) until item 3 wires the port-parametric
`loop_v0` entry. The source-port input enums gained `Clone` for projection
reuse, and the raw error renderer now covers the located-only variants.

Focused evidence: `cargo check --profile quick --lib --tests` green; the new
`callable_loop_source_tests` module passes 15/15 — projection
(`Body(i)`/`IfCondition`/`LoopBody` site verification), negative matrix
(foreign arena, recipe/body mismatch, synthetic carrier, condition-view
mismatch, unlocated vocabulary), and hook coverage through the neutral
driver (no-exit statement block, join if, exit-if under `ExitAllowed` via a
`LoopBody` child carrier, opaque-if singleton recursion, LoopV0 named
reject). The whole `associated_source` filter stays green at 32/32,
including the raw provider, located provider, parity, and dispatch tests.

This receipt claims only the located provider, hooks, and their focused
evidence. It does not claim `lower_raw_loop_v0` (item 3), the ProgramBlock
production wiring (item 4), publication consumption, caller cutover, or
old-edge retirement; `source-port-lowering-missing` remains the active
terminal.

### Port-parametric `loop_v0` core and source hook receipt — decomposition item 3

`parts/loop_/loop_v0.rs` now splits the existing owner into a thin raw facade
and `lower_loop_v0_core`, a `FnOnce`-generic core that owns carrier
discovery, `CoreLoopFrame` construction, header/body/step/after PHIs, the
fallthrough backedge, and `CorePlan::Loop` emission. The raw facade keeps the
same signature and injects `lower_loop_header_cond` (CondBlockView prelude +
raw port) plus the verified contract lowerers; the source hook injects
`lower_loop_header_cond_with_port` plus recursion through
`lower_callable_loop_source_parts_block`. Nested `LoopV0` therefore consumes
already-issued carriers end to end — the header condition enters as a located
`ExprInput`, the co-sealed body block re-enters the same provider/hooks pair,
and an inner `carrier_updates` map is discarded exactly like the raw path,
which threads no updates through a nested loop body. A `BlockExpr` condition
(the `CondBlockView` prelude shape) is a named reject
(`loop-v0-cond-prelude-unlocated`) before any Builder effect, matching the
raw facade's prelude-only authority. `CallableLoopSourcePartsBlockV1` exposes
`recipe_body()` for carrier collection over the issued recipe AST; no
`RecipeBody` rescan for identity, no AST/name lookup, and no raw retry were
added.

Focused evidence: `cargo check --profile quick --lib --tests` green; the
`callable_loop_source` filter passes 38/38, including the new
`driver_lowers_loop_v0_through_the_shared_core` (nested `LoopV0` lowers to
`CorePlan::Loop` through the source port, carrier `tmp` round-trips header +
body sites) and `driver_rejects_a_loop_v0_block_expr_condition_before_effects`
(the forged `Synthetic` BlockExpr condition stops at the named terminal).
Adjacent filters stay green: `associated_source` 33/33, `loop_v0` 6/6,
`nested_depth1` 1/1, `simple_while` 16/16. The `loop_cond` filter shows one
red, `program_block_with_exit_signals_prefers_recipe_only`, reproduced
identically on parent commit `98b5402c58` — classified as known baseline
debt, not a current-change failure.

This receipt claims only the port-parametric `loop_v0` split and the
source-aware nested `LoopV0` hook. It does not claim ProgramBlock production
wiring (item 4), publication consumption, caller cutover, or old-edge
retirement; `source-port-lowering-missing` remains the active terminal for
the outer production caller.

### Unarmed-site take disposition receipt — review fix

`CallableLoopSourceBridgeV1` now records resolver-cataloged sites that the
forest projection deliberately leaves unarmed (unsupported ancestor) in a
separate `unarmed` inventory, and `take_for` returns a typed
`CallableLoopSourceBridgeTakeV1` (`Armed` / `Unarmed` / `BridgeAbsent`).
`missing-site` remains a contract violation for sites the bridge never
cataloged, and an armed take is still one-shot. The `Ready` consumer in
`raw_loop_child_entry` maps `Unarmed`/`BridgeAbsent` to the ordinary
GenericLoop boundary instead of failing the callable.

This fixes a mixed-cohort regression: a callable with an armed root loop and
an `if`-nested loop previously died at `source-bridge/missing-site` before
the unarmed loop could reach `issue_once`, contradicting the
"unsupported nested loops remain unarmed" contract.

Focused evidence: `source_loop_bridge` filter 5/5, `raw_loop_child_*`
filters green including `unarmed_nested_loop_keeps_generic_loop_boundary`
(end-to-end `lower_loop` on the `if`-nested site lowers through the
GenericLoop boundary). The seven `normal_callable`/`loop_cond` reds
(`runtime-box-fate-retired`, `DynamicCarrierMismatch`, `ProgramBlock` recipe
shape, `BorrowedEntryEscape`) reproduce identically on parent `e6f6456425` —
classified as known baseline debt, not a current-change failure.

### Ambient-env pin receipt — review fix

Lowering paths read `GenericLoopFactsPolicyFrameV1::from_environment()` and
`joinir_dev` flags per call, so a test that asserts a route without pinning
the six mode keys can observe a concurrent strict window. The unarmed-boundary
test now runs both modes under `PROCESS_STATE_LOCK`: default mode asserts the
GenericLoop boundary, strict+planner_required asserts the named
`callable-loop source port requires RecipeOnly body` terminal with no partial
MIR. `crate::test_support::JOINIR_DEFAULT_MODE` owns the cleared key set;
`drive_block` (callable-loop source testkit), the three physical-adapter tests
in `normal_callable_loop_source_facts_tests`, and the loop-header facade/core
parity test pin the same default mode around their lowering calls.

Focused evidence: the `associated_source` + `raw_loop` + `normal_callable` +
`cond_lowering_loop_header` filters pass with the pin applied; the same
filters under `HAKO_JOINIR_STRICT=1 NYASH_JOINIR_STRICT=1
HAKO_JOINIR_PLANNER_REQUIRED=1 NYASH_JOINIR_DEV=1` keep the pinned tests
green. The eight `recursive_child_lowering_rawport_tests` reds
(`missing-expression-source-receipt`) and the
`production_skip_while_prepares_dynamic_ingress_before_loop_effects` unwrap
red reproduce identically on session parent `976b65200a` — classified as
known baseline debt, not a current-change failure.

### Merged-route boundary move receipt — review fix

The merged parser program guard
(`merged_parser_program_source_stops_at_named_loop_boundary_before_static_target`)
was red on review: it still asserted the pre-handoff
`GenericLoopV1NotSelected` terminal while the route token now issues
`LoopCondRouteRejected(SourceTargetMissing)`. This is a designed move, not a
threading defect. `source_target_for_loop` reads the selected publication
owner through `ModuleLoweringPortV1::target_for_source`. This receipt
originally hypothesized that the Compatibility materialization path
installed no `static_result_publication_owner`; the later worker audit
("Source-target terminal diagnosis receipt") corrected this — the armed
route is Installed-only and the owner is already installed. The actual
cause is that the first reached armed loop (`StringHelpers.index_of/3`)
binds only `Qualified(Bound)` `substring` items, which carry no static
publication row, so the issue boundary now raises the typed
`SourceTargetUnselected` reject before any Builder effect. The loop is
still LoopCond-selected and its source items are bound, so the stop is one
step deeper than `GenericLoopV1NotSelected` and still strictly before the
static catalog — matching this card's open task 4 (static tuple handoff).

The guard now pins the default JoinIR mode (`JOINIR_DEFAULT_MODE`) alongside
the existing using/macro keys and asserts the new named terminal. The parent
static card's finite stop condition was updated to the same boundary. No
fallback, second publication owner, AST/name rescan, or target synthesis was
added; task 4 must extend the existing publication owner so the
Compatibility caller path can publish the selected row.

Secondary review fixes in the same slice:

- `crate::test_support::JOINIR_MODE_KEYS` is now the single owner of the six
  JoinIR mode keys; `JOINIR_DEFAULT_MODE` and the new
  `JOINIR_STRICT_PLANNER_MODE` preset are derived from it, and the
  GenericLoop extract test support plus the raw-loop strict test consume the
  shared constants instead of duplicating the list.
- `lower_raw_loop_v0` now checks the `BlockExpr` condition prelude before
  `reseal_branch_bindings`, so `loop-v0-cond-prelude-unlocated` literally
  precedes any Builder effect.
- `drive_block` documents that its default-mode pin holds the non-reentrant
  process state lock and must not be nested inside another env scope;
  mode-scoped callers use `lower_callable_loop_source_parts_block` directly.
- `resolved_candidate_snapshot_is_unpublished_and_fresh_reuse_is_stable`
  asserted a `BuilderContract` marker string that
  `5967a0f367` (09-12) removed when cutover diagnostics became typed. The
  assert now pins the same
  `DirectAccum(ExternalCommit(EvidenceMismatch))` terminal as
  `resolved_direct_accum_hardening_p0`. This red was baseline-listed as
  green and is classified as pre-existing baseline debt reconciled to the
  typed terminal, not a current-change failure.

Focused evidence: `cargo test --profile quick --lib -- --test-threads=1`
over the merged-route, `accum_semantic_parity_tests`, `associated_source`,
`normal_callable_loop_source_facts`, `source_loop_bridge`,
`cond_lowering_loop_header`, `raw_loop_child`, and the extract
`mode_pair`/`unarmed_nested` filters is green (86/86).

### LoopCond item arms on the located spine receipt — decomposition item 4

`callable_loop_source_items.rs` now owns `lower_loop_cond_source_item`, the
per-item dispatcher the item-6 sibling will call once per issued
`LoopCondBreakContinueItem`. `build_loop_cond_break_continue_recipe_inner`
issues exactly one item per recipe body statement, so the item ordinal
locates `GeneralIf` (which carries no `StmtRef`) while payload `StmtRef`s
stay the item authority for `ProgramBlock`/`ExitIfTree`. The arms mirror the
raw owners: `ProgramBlock{stmt_only: None}` re-derives
`try_build_exit_allowed_block_recipe` on the located statement (same
`true`-then-`false` order as the raw arm), seals the `singleton` block, and
drives `PartsAssociatedBlockModeV1::ExitAllowed` with
`apply_nested_loop_preheader_freshness` on the emitted plans; `GeneralIf`
seals its issued no-exit recipe against the statement at the item ordinal
and drives `NoExit`; `ExitIfTree` re-proves the issued `cond_view` against
the projected `IfCondition` child (the `CondBlockView` check is now the
shared `require_condition_view_match` free function), co-seals the issued
ExitOnly branch recipes against `IfThen`/`IfElse` child carriers, and runs
`lower_explicit_if` -> `lower_exit_if_state_core` under `ExitOnly`.
`Stmt`/`ExitLeaf` delegate to the existing `lower_loop_cond_item_input`
(widened to `pub(in crate::mir::builder)`), and its `Ok(None)` return is a
fail-fast `loop-cond-item-unhandled`, never a raw fallthrough. Every other
variant is a named `loop-cond-item-unsupported` reject; a `stmt_only`
payload is `program-block-stmt-only-unlocated` because flattened container
recipes cannot align 1:1 with a located carrier (same policy as
`opaque-stmt-container-unlocated`), and a failed exit-allowed re-derivation
is `program-block-recipe-unlocatable` — the raw join/exit-if fallback
owners stay rejected on the located spine, matching `lower_opaque_if_source`.
No raw `lower_loop_cond_item` fallback, `LoopRouteContext`, AST/name rescan,
or second recipe authority was added.

Focused evidence: `cargo check --profile quick --lib --tests` green; the new
`callable_loop_source_items_tests` module passes 10/10 — `Stmt` and
`ExitLeaf` through the located body, `ProgramBlock` through a `LoopBody`
child carrier (`break` only resolves inside a loop body, so the fixture
hosts the if inside `loop(true)` exactly like
`driver_lowers_exit_if_under_exit_allowed_mode`), `GeneralIf` through the
issued no-exit recipe, `ExitIfTree` in both `ExitAll`+else and the
acceptance tuple's `ExitIf`-no-else mode, and named rejects for else parity
drift, item-index drift, `stmt_only`, and unsupported variants. The
`callable_loop_source` filter stays green at 48/48 and the neighboring
`static_call_result_publication_owner`/`cond_lowering_loop_header`/
`raw_loop_child`/`merged_route`/`unarmed_nested`/`mode_pair`/`armed_scope`
filters pass 26/26.

This receipt claims only the located item arms and their focused evidence.
It does not claim the end-to-end physical consumer (items 5-6), caller
cutover, or old-edge retirement; `source-port-lowering-missing` remains the
active terminal for the outer production caller.

### Located-source physical consumer receipt — decomposition items 5-6

`features/loop_cond_bc_source.rs` now owns
`lower_loop_cond_break_continue_source`, the sole physical consumer of the
co-sealed `SourceLoopCondPhysicalInputV1`. It mirrors
`lower_loop_cond_break_continue` step for step — `LoopBlocksStandard5`
allocation, carrier collection over the co-sealed body plus condition reads
(`collect_outer_from_body` and the widened `collect_carrier_vars_from_condition`,
which observe names only and issue no authority), phi materializer
preparation, `lower_loop_header_cond_with_port` on the port-projected
condition input, cleanup, phi closure, and the existing verifier — but every
body statement runs through `lower_loop_cond_source_item` against the
port-projected `CallableLoopSourceBodyInputV1`, and the
`BodyLoweringPolicy::ExitAllowed` arm drives the issued exit-allowed recipe
through `lower_loop_cond_source_exit_allowed_body` (a `located_body` seal plus
`PartsAssociatedBlockModeV1::ExitAllowed` in the same sibling module) with the
identical `if body must be single-exit` fallback into the per-item loop. The
raw `accept_kind` contract pin is now the shared `pin_accept_kind_contract`,
so both consumers fail on the same unhandled kind. The sibling never touches
`LoopRouteContext`, `CondBlockView`, or the raw item owner.

`raw_loop_child_entry.rs` no longer stops at
`callable-loop/loop-cond/source-port-lowering-missing`: the `LoopCondReady`
arm validates and preflights the physical input, lowers it through the
sibling, runs `PlanVerifier::verify`, and consumes the plan through
`PlanLowerer::lower` under the same `GenericLoopV1SourceLoweringContextV1`
the GenericLoop adapter uses — the sole source-to-MIR terminal for the armed
LoopCond edge. `SourceLoopCondPhysicalInputV1` is re-exported at
`pub(in crate::mir::builder)` from `normal_callable_loop_source_facts` so the
consumer signature can name it.

Focused evidence: `armed_loop_cond_edge_lowers_through_the_source_port` in
`raw_loop_child_entry/tests.rs` drives the real pipeline — resolved
`static function caller(flag, text)` fixture with a cataloged
`text.starts_with` row under the loop site, armed forest bridge, projected
binding disposition, forged `ExactI64`/`[1]` target relation minted via
`from_handoff` — through `issue_once` -> `LoopCondReady` ->
`into_physical_input` -> validate/preflight -> sibling -> `CorePlan::Loop` ->
verify -> `PlanLowerer` -> `Ok(ValueId)`. The sibling test fixture uses a
`Variable` receiver so the call lowers without a registered box.
`armed_loop_cond_edge_rejects_missing_source_target` pins the named
`LoopCondRouteRejected(SourceTargetUnselected)` terminal when no
publication row is installed. The `raw_loop_child`/`callable_loop_source`/
`source_loop_bridge`/`merged_route`/`unarmed_nested`/`physical_adapter`
filter set passes 67/67.

This receipt claims only the physical consumer, its caller wiring, and the
focused armed-edge evidence. It does not claim the caller-side publication
installation (production `source_target_for_loop` still yields no relation
until the selected static publication row is wired),
`callable_handoff=None` legacy-edge retirement, raw item-branch deletion, or
the `parse/2` source-to-exe closeout — those stay bounded follow-ups in the
card order.

### Source-target terminal diagnosis receipt — publication audit

A read-only worker audit corrected this card's earlier hypothesis: the
armed source route is selected only when `callable_loop_root_scope` is
`Some`, which only the Installed `RawInvocationChildPortV1` constructor
passes, and `program_root_lowering.rs` installs the
`VerifiedStaticCallResultPublicationOwnerV1` before constructing that port.
`SourceTargetMissing` in the merged run is therefore not a Compatibility
owner-install gap.

`source_target_for_loop` still returns `Ok(None)` for every non-applicable
exit (non-cataloged lineage, missing site, missing/empty ledger items) —
that `None` is load-bearing: the function runs for every loop reaching the
armed scope, including unarmed, GenericLoop, and nested loops that never
consume a target relation. An earlier variant of this slice made those
exits fatal and immediately regressed
`unarmed_nested_loop_keeps_generic_loop_boundary`; the design was reverted.
The diagnosis instead lives at the LoopCond issue boundary, where
`source_items` is already proven non-empty: a missing relation there now
rejects as `SourceTargetUnselected { call_sites }` carrying every probed
item site, while `SourceTargetMissing` stays only at the
`into_physical_parts` physical-transfer boundary.

`merged_parser_static_inventory_probe` pins the inventory facts behind the
terminal: 65 observed callers, 430 calls, 104 targets, no bounded
observation unavailability; `ParserProgramBox.parse/2` does publish
`starts_with/3` target rows inside its loop; but the first reached armed
LoopCond loop is `StringHelpers.index_of/3`, whose two loop items are
`Qualified(Bound)` `substring/2` calls — bound-receiver calls never carry
a static publication row, so `SourceTargetUnselected` is the correct
route-contract reject, not a missing-install bug. The merged guard test
now asserts that terminal.

Focused evidence: `merged_parser` 2/2, `raw_loop_child` 12/12,
`callable_loop_source` 48/48, `normal_callable_loop_source` 22/22,
`loop_cond_bc` 10/10, `normal_callable_loop_source_facts` 16/16.

This receipt claims only the named-terminal diagnosis and the inventory
probe. It does not claim a route-contract resolution — whether armed
LoopCond selection should skip loops whose items are all bound-receiver
calls, or whether the static-target requirement itself should be relaxed
for them, is an open design question for this card. It also does not claim
`callable_handoff=None` retirement, raw item-branch deletion, or the
`parse/2` source-to-exe closeout.

### Applicability/evidence separation decision — ordered next slices (A′)

Review of `6b28f34e0e` accepted the diagnosis above and fixed the direction.
The root issue is that "this loop can lower" and "this particular static call
can publish" are currently the same required condition; `ExactI64/[1]` is
evidence for the selected call, not a condition for LoopCond in general. The
short-term move is **A′** — the source side determines the applicable scope
and the required evidence *before* arming — and the final shape separates the
LoopCond structural contract from the per-call publication obligation. Plain
`None` acceptance (B) is rejected because it cannot distinguish "no
obligation" from "a required row is missing"; the named
`SourceTargetUnselected` terminal (C) stays correct as the stop for this
unfinished family. Production selection follows the existing owner's typed
identity and declared scope — the acceptance tuple is a validation target,
never a name lookup.

Obligation/evidence contract (replaces row-presence checks):

| source-side obligation | evidence | behavior |
| --- | --- | --- |
| no obligation for this publication family | a separate contract covers each call | lower through that contract |
| obligation | exact site + target + result all match | lower through the selected static path |
| obligation | row missing / mismatch / already consumed | named terminal |
| call itself unsupported | no consumer evidence | unsupported terminal |

`observed_rows.is_empty()` is not proof of "no obligation". Requirements come
from the source-determined contract; publication evidence must cover them
with no gap and no surplus. Arming consumes only immutable requirement
information — never the presence of a consumable handoff row, which could
hide ordering or double-consume as "out of scope". Roles stay split: Facts =
loop structure / exits / call sites / receiver bindings; Recipe/admission =
applicable scope, sole route, required call contract; Verify/co-seal =
publication/result/consumer evidence match; Lower = consume the verified
selection.

Corrected premises for the remaining design:

1. Unarming does not return to GenericLoop. The Facts issuer selects the
   route independently (route selection in
   `normal_callable_loop_source_facts.rs`), so a LoopCond shape proceeds
   toward the LoopCond side even without a projection. The explicit
   applicability/capability disposition must connect to the sole route
   selection — no try-new-route-then-fall-back shape.
2. Bound-receiver calls are not proven lowerable through the port path. The
   current connection still searches the receiver by variable name and hands
   off to the existing raw emitter without consuming source binding evidence
   (`plan/normalizer/helpers_value/lower.rs`,
   `plan/lowerer/effect_emission.rs`). `substring` receiver/arguments/Text
   result/comparison/effect-Fault handling must be confirmed before that
   admission opens.
3. `source_target_for_loop` currently rejects `SourceTargetMultiple` on any
   second exact static target (`raw_loop_child_port.rs`), so one selected
   call plus one unrelated static call can already stop the loop. The check
   must cover the *selected obligation set*; every other call is covered by
   its own contract.
4. `Bound` is the resolver's receiver-binding classification, not a
   dynamic-dispatch verdict. The accurate claim is only that these items are
   not same-module static publication targets under this profile.

Ordered next slices:

1. **Separate applicability from evidence-missing.** Deliberately dropping a
   required row must still stop at a named terminal; it must never reclassify
   as "out of scope".
2. **Wire A′ into the sole route selection.** Confirm whether `index_of/3`
   can lower through the existing path; if its receiver-call family is
   unsupported, stop locally with that reason — do not reorder the merged
   program to dodge it.
3. **Close the receiver-only consumer** for the covered family: a contract
   that consumes the same source binding end to end, not a `None`
   acceptance.
4. **Reach the acceptance tuple in the original merged order.** Verify the
   selected call's evidence end to end and retire only its old edge.

Guard cases that must each stay green-by-contract or red-at-a-named-terminal:
required-row removal, static+bound mixed items, multiple static targets, and
a loop with zero calls.

This decision claims only the direction and slice order. It does not
authorize implementation, a new Verified receipt family, caller reordering,
or relaxation of the publication obligation.

### A′ slice 1 receipt — obligation/evidence separation at the issue boundary

The lossy `Option<CallableLoopSourceTargetRelationV1>` handoff is replaced by
`CallableLoopSourceTargetProbeV1`, an immutable classification product that
`source_target_for_loop` (`raw_loop_child_port.rs`) now always emits:

- `selected`: exact-target sites whose selected publication row is still
  present (`target_for_source` reads the inventory-derived `exact_targets`
  map — the non-consumable obligation fact — while
  `selected_static_result_handoff_for_source` only peeks the consumable row).
- `uncovered`: exact-target sites whose selected row is absent — target-only,
  dropped, or already consumed obligations.
- `requirement_mismatch`: a published handoff disagreed with its exact
  target.

The probe never decides fatality, so unarmed/GenericLoop/nested paths are
untouched (the earlier `unarmed_nested_loop` regression shape cannot recur:
the probe is infallible). Terminal mapping lives at exactly one place —
`CallableLoopSourceTargetProbeV1::into_selected_relation`, called by
`issue_with_source_relations` after the item-structure checks:

1. `requirement_mismatch` → `SourceTargetRequirementMismatch`
2. `uncovered` non-empty → `SourceTargetUnselected { call_sites }` — a
   dropped required row can never reclassify as out of scope
3. `selected` length > 1 → `SourceTargetMultiple` — now scoped to the
   *selected obligation set*; an unrelated static call no longer trips it
   (corrected premise 3)
4. `selected` empty → new `SourceCallOutsideSelectedFamily { call_sites }` —
   "no obligation" is a distinct unsupported-family terminal, not evidence
   missing and not silent acceptance
5. otherwise the single relation must bind an item site or reject as
   `SourceTargetSiteMismatch`

The merged run now stops at `SourceCallOutsideSelectedFamily` for
`StringHelpers.index_of/3`: its `substring` items carry no exact same-module
static target (corrected premise 4 — `Bound` is a receiver-binding class, not
proof of dynamic dispatch), so there is no publication obligation and no
evidence gap. `SourceTargetUnselected` is reserved for the genuine
missing-required-row case. `SourceTargetMissing` stays only at
`into_physical_parts`.

Focused evidence (quick profile, one cargo process at a time):
`normal_callable_loop_source_route` 13/13 — including the new boundary
matrix `issue_with_source_relations_{co_seals_the_single_selected_relation,
keeps_unrelated_items_out_of_the_selected_set (two cataloged items, one
selected obligation), maps_a_missing_required_row_to_unselected,
maps_multiple_selected_relations_to_multiple,
maps_no_obligation_to_outside_selected_family,
maps_a_handoff_disagreement_to_requirement_mismatch,
rejects_a_relation_site_outside_the_items}`;
`raw_loop_child_entry` 11/11 — including the new
`armed_loop_cond_edge_rejects_items_outside_the_selected_family`;
`merged_parser` 2/2 with the guard asserting the renamed terminal;
`normal_callable_loop_source_facts`/`raw_loop_child_port` 20/20. The
`static_call_result_publication`/`normal_default_root_catalog`/
`module_lowering_invocation` sweep shows 6 reds, all already listed in
`cargo_lib_red_baseline.tests.txt` (known baseline debt — failure modes are
pre-existing terminals such as `[static-result-ingress/no-exact-static-target]`,
unrelated to this probe). `normal_callable_loop_source_route.rs` was split
into `normal_callable_loop_source_route_tests.rs` for the 760-line headroom
(482 / 650 lines).

Non-claims: the receiver-only consumer for `index_of/3`'s `substring` items
is still closed — `SourceCallOutsideSelectedFamily` is the designed stop, so
the merged order still does not reach `parse/2`. Slice 2 (sole-route wiring)
and slice 3 (receiver-consumer contract) remain. No caller reordering, no
new Verified receipt family, no `None` acceptance was introduced.

### A′ slice 2 audit — receiver-only owner boundary

The existing CoreMethod source machinery was audited before opening the
receiver-only route. `source_call_target::core_method` can co-seal a
resolver-owned lexical receiver with a `StringBox` `length/0` or
`substring/2` target, but its only composed product,
`VerifiedSourceBoundS6CCallRelationV1`, is the parked fixed `ScanWithInit`
shape: exactly one length call, exactly one substring call, one typed-input
relation, and one S6C physical session. It is not consumed by the current
`LoopCondBreakContinue` source bridge, and no production
`CoreMethodInstanceTargetIssuerV1`/target inventory is installed in the
current `ModuleLoweringPortV1` route.

The first armed merged loop is `StringHelpers.index_of/3`. Its loop body has
two resolver-issued `substring/2` bound-receiver items, while the length calls
that establish `n` and `m` are outside the loop root. Reusing the parked S6C
pair would therefore drop source coverage; adding a second target issuer or
reopening the common-V2 lane would create a competing authority. The current
source port also proves the remaining physical gap: its generic MethodCall
receiver arms still consult the raw name map, while only standalone variable
inputs consume `exact_source_variable_value` at the exact source site.

Decision: keep `SourceCallOutsideSelectedFamily` as the named terminal and do
not accept plain `None`, reorder the merged program, or revive S6C. The next
bounded design slice must name one existing production owner for a per-call
bound-CoreMethod target (including receiver binding, argument sites, Text
result, and effect/fault completion) and show how that product is consumed by
the current LoopCond source physical adapter. Until that owner and target
issuer are identified, no route token or new `Verified*` receipt is issued.

Evidence: source inspection of `source_call_target/core_method.rs`,
`resolved_semantics/resolver_core_method_callable_contract.rs`,
`normal_callable_loop_source_port.rs`,
`control_flow/plan/normalizer/helpers_value/lower.rs`, and the merged
`StringHelpers.index_of/3` inventory; the focused `merged_parser` guard
remains 2/2 at the named terminal. This audit does not claim receiver
physical lowering, parser acceptance, caller switch, or old-edge retirement.

### A′ slice 2 decision — route-neutral CoreMethod owner selected

The existing production owner for the receiver-only family is now fixed as
`source_call_target::VerifiedSourceCallTargetCatalogV1`. It already owns the
complete, catalog-branded source MethodCall rows and the route-neutral
Static/DynamicMember target union. The bound CoreMethod row will be a third
route-disjoint arm in that same catalog, keyed by the exact
`(caller, SourceExprSiteV1)` pair; no second target catalog or package-local
name map is allowed.

The canonical issuer remains the generated CoreMethod manifest plus the
existing `CoreMethodInstanceTargetIssuerV1` and
`ResolverCoreMethodCallableContractIssuerV1`. For each exact resolver method
row whose generated `StringBox` operation/arity is supported, one issuer
session produces one move-only target and the resolver contract co-seals the
lexical receiver binding, ordered argument sites, result site, loop
membership/placement, selector alias, `PureRead` effect, Text/I64 result, and
the manifest/schema/policy brands. A fresh issuer session per source call is
permitted because its relation/target brand scopes one call contract; it does
not create a second semantic manifest or deduplication authority. Unsupported
selector/arity, non-lexical receiver, missing loop placement, or any source
row drift remains a named rejection.

The existing normal callable semantic package is the transport owner for this
catalog. It will retain the co-sealed per-call contracts and lend the selected
caller's rows into `CallableSemanticLoweringState`; the source expression port
will consume an exact contract by source site. Receiver variables therefore
use the resolver-issued binding/value relation before any raw name lookup, and
the physical MethodCall emission must verify the contract's receiver,
arguments, result class, effect, and Fault behavior. This is transport of an
existing source-call authority, not a new semantic receipt family.

The next fast slice is bounded to (1) issue the catalog arm from the existing
semantic package using generated manifest rows, (2) transport the selected
caller's contracts through the existing lowering state/source port, and (3)
add positive/negative exact-site tests for two `substring/2` rows, wrong
receiver/argument/result, foreign site, and duplicate take. Only after this
consumer is green may the LoopCond obligation admit the bound family. S6C,
AST/name lookup, route reordering, `None` acceptance, fallback, VM, and old
edge deletion remain closed.

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

### A′ slice 2 transport implementation checkpoint — 2026-09-20

The selected route-neutral catalog now has a third `CoreMethod` arm carrying
the existing generated-manifest/CoreMethod-instance target and resolver-owned
lexical receiver contract. The semantic package issues these rows before
install, co-seals them through `VerifiedSourceCallTargetCatalogV1`, and lends
the selected caller's exact-site map through the existing lowering state and
source expression port. Main static children, cataloged static methods, and
cataloged instance methods use the same transport; unsupported non-lexical
receivers remain unarmed for their existing owner.

Evidence: `CARGO_BUILD_JOBS=4 cargo check --profile quick --lib` passes;
the focused `merged_parser` guard passes 2/2 after the transport change. The
test binary reports the repository's existing warning baseline (555 warnings
for the filtered lib-test build); warning cleanup remains a separate task.
The exact positive/negative receiver/argument/result matrix and production
LoopCond physical acceptance are still pending, so this receipt claims neither
parser source-to-MIR acceptance nor old-edge deletion.

### A′ slice 2 catalog guards — 2026-09-20

The issuer/catalog boundary now has a focused 3/3 guard: the generated
`length/0` and `substring/2` rows retain exact placement, a foreign caller is
rejected, and a duplicate `(caller, source-site)` is rejected. This closes
only the catalog issuance guard. Wrong receiver/argument/result, duplicate
take from lowering state, LoopCond physical acceptance, and old-edge deletion
remain open; the existing 555-warning baseline is unchanged.

### A′ slice 2 exact take guard — 2026-09-20

`CallableSemanticLoweringState` now records consumed CoreMethod source sites.
An unregistered site still returns `None` for the existing family boundary,
while a second take of an already consumed exact site fails with the named
`duplicate-core-method-call-consumption` freeze instead of reopening raw
name-based lowering. The focused duplicate-take test is green (1/1). This
closes only the one-shot transport guard; receiver/argument/result rejection,
LoopCond physical acceptance, and old-edge deletion remain open.

### A′ slice 2 resolver rejection guards — 2026-09-20

The resolver contract rejection matrix now has 5/5 focused passes: the
non-lexical receiver and argument-arity drift are rejected alongside the
existing outside-loop, target-placement, and foreign-membership guards. The
generated target issuer already rejects an incompatible result relation.

The real source expression port now has a focused physical guard,
`source_item_method_calls_consume_exact_core_method_rows` (1/1): a resolver-
issued `length/0` in the LoopCond condition lowers through the existing compare
consumer with an Integer result, and a resolver-issued `substring/2` in the
LoopCond body lowers through the local initializer consumer with a Text result.
Both effects retain the exact parameter receiver and argument arity. This
closes the source-port Text-result consume evidence only; the production
LoopCond physical adapter, parser source-to-MIR cutover, and old-edge deletion
remain open.

### Production LoopCond physical-edge guard — 2026-09-20

`armed_loop_cond_edge_lowers_through_the_source_port` is green (1/1). Its
existing `RawInvocationChildPortV1` fixture now carries the selected static
`starts_with/3` relation together with resolver-issued CoreMethod rows for
`length/0` in the LoopCond condition and `substring/2` in the loop body. The
production preparation, `LoopCondReady` source input, and named source
physical consumer therefore execute as one mixed edge. This is still a
production-edge guard only: ParserProgramBox package/publication acceptance,
selected old-edge deletion, and the 555-warning cleanup remain open.

### Armed source-item catalog guard — 2026-09-20

`RawLoopChildEntry` no longer converts a missing source-item catalog into an
empty relation. After an `Armed` bridge take, the existing catalog must be
present or the named
`[freeze:contract][callable-loop/source-bridge/armed-items-missing]` error is
returned. `Unarmed` and `BridgeAbsent` still carry an empty relation to their
existing GenericLoop boundary. The production edge guard remains green (1/1),
and the unarmed-loop disposition guard remains green (1/1); the repository's
555 compiler-warning baseline is a separate cleanup inventory.

### A′ slice 2 receiver-family frontier — 2026-09-20

The resolver-issued CoreMethod-only family is now transported through the
existing source route and source-port consumer. The merged parser order moves
past the preceding `StringHelpers.index_of/3` receiver-only loop; its nested
conditional-return `if` is handled by the existing source join/exit cores and
does not re-enter the raw normalizer. The next armed parser loop reaches the
existing callable handoff contract and stops at
`[freeze:contract][callable-loop-handoff/carrier-cardinality-0]`. The
diagnostic count is zero: this loop has `loop(true)` and no carrier. This is a
real bounded frontier, not a publication failure. A temporary removal of the
cardinality guard advances only to `GenericLoopV1NotSelected`, because the
existing `LoopCondBreakContinue` Facts owner rejects a true condition; it does
not prove a safe multi-carrier widening.

The merged guard records this named handoff terminal. Keep the handoff
fail-fast until a source-aware `LoopTrue` owner is selected or a bounded
alternative is accepted. No `None` acceptance, route reorder, VM/fallback
path, parser publication, caller switch, or old-edge deletion is claimed. The
555-warning baseline and the native Windows lifecycle evidence remain separate
closeout items.

### Carrier-zero design stop — 2026-09-20

```text
Decision: Preserve the existing handoff cardinality rejection; the observed
  parser boundary is a zero-carrier `loop(true)`, not a proven multi-carrier
  source LoopCond shape.
Source authority + canonical issuer: existing route registry plus
  `LoopCondBreakContinue` Facts/Recipe issuer; the source-aware LoopCond
  physical adapter remains its sole consumer for this card.
Non-authority: a relaxed handoff count, GenericLoop retry, VM/fallback route,
  AST/name reconstruction, and the existing LoopTrue facts without a source
  physical consumer.
Fail-fast boundary: `callable-loop-handoff/carrier-cardinality-0`; relaxing it
  must not turn `GenericLoopV1NotSelected` into implicit fallback.
Smallest next slice: select and audit one existing source-aware LoopTrue
  physical owner for the finite `loop(true)` parser loop, or record NoSafeSlice
  with its exact missing consumer and reopen trigger.
Non-claims: no parser source-to-MIR acceptance, production switch, old-edge
  deletion, or multi-carrier support claim.
```

The temporary diagnostic run used one quick Cargo process and embedded the
carrier count in the error only for observation; the source was restored
afterward. With the guard removed, the focused merged-parser test reached
`[freeze:contract][callable-loop/route-not-front-selected]
GenericLoopV1NotSelected`, confirming that a count relaxation is not a safe
implementation slice.

### External raw-loop audit reconciliation — 2026-09-20

The audit item describing `raw_loop_child_entry.rs:270`
`unwrap_or_default()` is stale at this head. Commit `54c3176a4d` changed the
Armed source-item catalog lookup to `ok_or_else` with the named
`[freeze:contract][callable-loop/source-bridge/armed-items-missing]` terminal;
only `Unarmed` and `BridgeAbsent` retain the empty relation used by their
existing GenericLoop boundary. This is a closed correction, not an open Mid
blocker.

The remaining audit observations are low-priority BoxShape/knowledge debt:
raw and normal parity still have sibling control branches, the two-stage
parity helper remains duplicated, the source-target requirement retains a
residual `Option`, and `normal_callable_loop_source_facts.rs` is at 759 lines.
They do not change the selected authority, source acceptance, or the current
LoopTrue design stop. A future mechanical cleanup must preserve the named
rejects and split the facts owner before adding semantic behavior. The
repository's 555-warning baseline remains a separate cleanup inventory.

### LoopTrue owner-selection audit — 2026-09-20

The finite parser boundary has a different source shape from every existing
production LoopTrue source product. `parser_program_box.hako:182-189` has a
three-statement body (`skip_ws` assignment, an inner conditional `continue`,
and a tail `break`) with no explicit `else`. The existing source projection in
`src/mir/compiler/loop_true_break_continue_projection.rs` requires one body
statement, an explicit else, a single branch whose then/else arms are exactly
`break`/`continue`, and an equality binding. It therefore correctly declines
this parser shape before policy.

The production `LoopTrueEarlyExitFacts` owner has a separate fixed topology:
the first statement must be an exit-only if, followed by one or two assignment
statements, with no nested loop or continue. The parser body also fails that
contract. The broader `LoopTrueBreakContinueFacts` owner can classify an
`ExitAllowed` body, but its lowerer still consumes AST/`StmtRef` rows and a
`LoopRouteContext`; it has no source port or resolver-exit co-seal consumer.
The existing source projection and the broad facts/physical lowerer therefore
cannot be combined by a caller-side adapter without creating a second
authority.

The next design slice is consequently bounded to one source-aware
`LoopTrueBreakContinue` handoff: co-seal the parser root/body/branch contexts,
all resolver exit records, rebind/iteration bindings, and the existing
LoopTrue Recipe before physical effects; then thread one source port through
the existing LoopTrue skeleton/phi/cleanup owner. Missing child, exit, site,
or lowering evidence must discard the whole session. Until this contract is
accepted, `carrier-cardinality-0` remains the terminal and no route widening,
AST rescan, GenericLoop retry, or fallback is authorized.

### Next design contract — LoopTrue source handoff D0

```text
Decision: keep the GenericLoop carrier contract unchanged and add one
  source-aware LoopTrue handoff beside it; the selected parser child may enter
  only after the new product is co-sealed.
Source authority + canonical issuer: the resolver-issued forest/exit ledger
  plus the existing LoopTrueBreakContinue Facts/Recipe issuer, split at the
  current 759-line facts boundary before adding the new arm.
Non-authority: GenericLoop carrier widening, AST/name reconstruction,
  LoopRouteContext, route retry, VM/fallback, and a second Facts/Recipe issuer.
Fail-fast boundary: source root/body/branch, binding, exit target, frame,
  route, or result relation mismatch rejects before skeleton allocation;
  GenericLoop's `carrier-cardinality` terminal remains untouched.
Smallest next slice: design and test the move-only `LoopTrue` source product,
  then add only its source-port physical adapter; no caller switch yet.
Non-claims: no parser acceptance, publication, old-edge deletion, or broad
  LoopTrue shape expansion beyond the finite `parser_program_box.hako` tuple.
```

The D0 entry condition is a responsibility split, not a semantic workaround:
the existing `normal_callable_loop_source_facts.rs` must first be divided at
the GenericLoop issuer/route view boundary so its 759-line owner does not grow
past the design threshold. The new arm may reuse the existing
`LoopTrueBreakContinueFacts.recipe`, phi materializer, cleanup, and verifier,
but it must receive a source port and an exact resolver exit table as one
move-only input. A missing source row is a named reject, never an empty
catalog or a return to the raw normalizer.

### Facts-owner split closeout — 2026-09-20

```text
Decision: complete the accepted BoxShape-only preparation by separating the
  existing GenericLoop Facts owner from its LoopCond sibling behind the same
  module facade; no source shape or semantic route is added.
Source authority + canonical issuer: unchanged existing GenericLoop and
  LoopCond Facts/Recipe issuers; the facade only re-exports those issuers.
Non-authority: a LoopTrue issuer, parser acceptance, AST/name lookup,
  LoopRouteContext adaptation, fallback, and production switching.
Fail-fast boundary: unchanged existing issuer and route rejects; the split
  must preserve private test access and module paths without changing behavior.
Smallest next slice: design the move-only LoopTrue source product and its
  exact co-seal/reject contract before adding the semantic arm.
Non-claims: no LoopTrue source acceptance, publication, old-edge deletion,
  warning cleanup, or whole-library green.
```

The 759-line owner is now `normal_callable_loop_source_facts/generic.rs` at
758 lines, with a 13-line facade and the existing `loop_cond.rs` sibling. The
focused command
`CARGO_BUILD_JOBS=4 cargo test --profile quick --lib normal_callable_loop_source_facts -- --test-threads=1`
finished **16 passed, 0 failed, 0 ignored** (8199 filtered), including the
structural lease and source-Facts tests. `git diff --check` is clean. The
quick lib-test build reports the existing **555-warning** baseline; this slice
does not claim warning reduction. The split is behavior-preserving and is
ready for commit/push after pointer synchronization.

### LoopTrue source product D1 — accepted design — 2026-09-20

The read-only owner audit confirmed that the existing LoopTrue source
projection is a different one-body/explicit-else shape and has no production
physical consumer. The parser loop therefore gets a separate source product
at the existing Facts-owner boundary; this is a bounded extension of the
selected owner, not a caller-side adapter around the wrong projection.

```text
Decision: issue one move-only source-aware LoopTrue Facts product for the
  finite parser child at parser_program_box.hako:182-189, while retaining the
  existing LoopTrueBreakContinueFacts/Recipe as the sole semantic issuer.
Source authority + canonical issuer: same-invocation resolver loop forest,
  ResolvedExitRecordV1 ledger, located source contexts, and the existing
  LoopTrueBreakContinueFacts/Recipe outcome co-sealed by the source Facts owner.
Non-authority: loop_true_break_continue_projection.rs's narrow shape, AST/name
  lookup, LoopRouteContext, GenericLoop/LoopCond retry, VM/fallback, or a
  second Recipe/JoinSig issuer.
Fail-fast boundary: before skeleton allocation, reject owner/frame/root or
  parentage drift; missing/foreign/duplicate forest member or exit; any exit
  whose transfer target is not the selected root/member; Facts/Recipe body or
  route mismatch; missing source item/target/result relation; and source-port
  coverage failure. Never turn a missing row into an empty catalog.
Smallest next slice: construct and guard the move-only source product only;
  then add a source-port physical adapter to the existing LoopTrue
  skeleton/phi/cleanup/verifier owner. No parser switch yet.
Non-claims: no source-to-MIR acceptance, publication, old-edge deletion,
  broad LoopTrue shape expansion, or VM/AOT parity.
```

The product fields are finite and co-sealed: owner/frame/root source context;
resolver forest binding and all exit rows under the root; the planner's exact
`LoopTrueBreakContinueFacts` outcome (including its existing Recipe); the
located condition/body source contexts; resolver-issued item/target relations;
and one borrowed `CallableLoopSourceExpressionPortV1`. Construction requires
the selected `LoopTrueBreakContinue` route and the parser tuple's `ExactI64`
result requirement `[1]`. The physical adapter will consume this product
before `alloc_loop_true_skeleton`, thread the source port through expression
lowering, and reuse the current phi materializer, cleanup, and verifier. It
must not call the existing AST/`LoopRouteContext` entry for the source route.

The finite implementation order is now: (1) issue the product and positive /
negative co-seal guards in the split source-Facts owner; (2) add the source-port
LoopTrue adapter with preflight before Builder effects; (3) run the parser
source-to-MIR matrix; (4) delete only the selected `parse/2 -> starts_with/3`
compatibility edge after the acceptance terminal is observed.
