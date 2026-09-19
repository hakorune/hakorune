---
Status: selected__design_stop__2026-09-19
Task: MIR-CALL-PARSER-LOOPCOND-SOURCE-HANDOFF-I0
Date: 2026-09-19
Parent: mir-call-parser-nested-loop-source-promotion-d0-2026-09-19.md
ProductionCaller: selected normal MIR/static-receiver route only
Implementation permission: false while the same-owner source bridge remains `NoSafeSlice`
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

The read-only bridge audit closed the next boundary as `NoSafeSlice`; no code
or fixture change is authorized until this contract is designed in the
existing owners. `CallableGenericLoopSourceFactsIssuerV1` is the only current
source Facts issuer, but it accepts only the GenericLoop payload and emits
`GenericLoopV1` selection. The local `LoopCondBreakContinueFacts`/Recipe can
express nested AST shapes, yet its `StmtRef` exit items do not retain the
resolver `ResolvedExitRecordV1` rows or the forest parent binding. The existing
LoopCond composer/physicalizer also requires `LoopRouteContext` and re-enters
AST/legacy lowering; the source GenericLoop expression port cannot safely
consume it.

This is an internal authority gap, not an external wait. The reopen contract
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
This recheck does not authorize a new issuer, route token, catalog row,
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

The resolver-input attach point is the existing package scope
`with_callable_source_scope(input, ...)`. `raw_loop_child_entry` currently has
only the projected callable ledger and cannot mint the forest projection by
itself. The scope must therefore lend one source-bridge capability to
`RawInvocationChildPortV1`, and the child entry must consume that capability for
the active loop. The capability may borrow the already-issued
`ResolvedFunctionLoweringInputV1` or carry a one-shot projection made at that
scope; it must not copy the resolver forest into the lowering state or create a
second ledger. If the lifetime cannot be represented at this existing scope
without a second owner, this remains `NoSafeSlice` and the compatibility edge
stays selected.

## Ordered implementation tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Source co-seal | One move-only product binds the exact three-member forest, ordered paths/frame keys, all resolver exits, parser brand/owner, and target/source site. Foreign, missing, duplicate, or orphan rows reject before effects. |
| 2 | LoopCond route token | **Blocked by NoSafeSlice.** First design the same-owner source Facts/Recipe/JoinSig product; then the route registry must yield exactly `[LoopCondBreakContinue]` for this product, with GenericLoop, LoopBreak, overlap, and route re-entry as typed rejects. |
| 3 | Source physical consume | **Blocked by NoSafeSlice.** `loop_cond_bc` needs a source-aware adapter that lowers nested recipe items and exit transfers without constructing `LoopRouteContext`, and discards the whole session on error. |
| 4 | Static tuple handoff | The selected static result reaches the existing statement-If/Equal consumer with ordered arguments and ExactI64 result; duplicate consume and wrong ordinal reject before argument effects. |
| 5 | Negative matrix | Wrong owner/brand, forest parent drift, omitted child, wrong path, missing/duplicate/foreign exit, wrong target/header/result, legacy route re-entry, and extra nested loop all fail closed. |
| 6 | Retirement and acceptance | After positive plus negative evidence, remove only the selected tuple's retained compatibility/static-child disposition and record source-to-MIR acceptance. |

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
