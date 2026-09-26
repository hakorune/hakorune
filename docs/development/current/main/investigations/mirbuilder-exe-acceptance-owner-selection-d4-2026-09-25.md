# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D4

Status: decision__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-ORDINARY-NEW-BIRTH-SITE-INDEX-S0
  (landed — destination-less birth-recipe site index)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0/D1/D2/D3 selection lineage (D2 was NoSafeSlice).

## Problem

The birth-recipe site index landed: all three
`ordinary-new/birth-global-legacy-stopped` entries cleared and now
stop at `[freeze:contract][callable-loop/facts-absent]`. Select the
next bounded design slice from the remaining 7 failing real-app EXE
entries.

## Fresh class map (receipt after BIRTH-SITE-INDEX-S0)

| class | entries | first named stop |
|---|---|---|
| `callable-loop/facts-absent` | 3 | boxtorrent_mini, binary_trees, mimalloc_lite |
| `callable-loop/parts loop-cond-item-unsupported` | 1 | json_stream_aggregator (`ConditionalUpdateIf` cond in a `JsonLine` static child) |
| `CoreMethodSource/NamedArray(TextSourceMissing)` | 1 | allocator_stress |
| backend `no_lowering_variant` (toolchain) | 1 | typed_object_newbox_min |
| inference panic `return_type_strategy` | 1 | typed_object_untyped_field_min |

`facts-absent` fires from `raw_loop_child_entry.rs:541` — the
callable lane reaches a `loop(...)` statement whose loop Facts were
never issued for that callable. All four non-singleton entries are
loop-family but present different evidence shapes: `facts-absent`
(no facts issued at all for the callable) vs
`loop-cond-item-unsupported` (facts issued, `ConditionalUpdateIf`
item shape rejected).

## Questions

1. Do the three `facts-absent` entries share one issuer gap (loop
   Facts not issued for constructor/child callables), or do they
   fork into distinct loop-source families?
2. Is `loop-cond-item-unsupported` the next terminal past
   `facts-absent` once facts exist, i.e., would fixing the issuer
   merely move all four entries to the same item-shape boundary?
   If so, which class owns the bounded first slice?
3. Which class has a bounded slice: single owner + fail-fast tuple
   + acceptance coverage?

## Boundary

- Includes: owner census for the 3-entry `facts-absent` class;
  divergence check vs `loop-cond-item-unsupported`; one Decision
  with a six-line brief or NoSafeSlice with the highest-information
  class named.
- Excludes: implementation; backend toolchain (`no_lowering_variant`
  stays parked-debt); inference panic family (prior D0 lineage);
  NamedArray source-demand family.

## Exit

- [x] Decision recorded (bounded slice OR sealed NoSafeSlice with
      reopen triggers) with the six-line brief.
- [ ] Next S-card emitted OR the named design card opened;
      pointers synced.

## Census answers (main-thread probes + source read)

### Q1 — do the three `facts-absent` entries share one family?

Yes: one semantic family — **a conditional loop whose body carries
zero exit-signal items**. First-stop loops observed:

| app | first failing loop | shape |
|---|---|---|
| boxtorrent_mini | `HakoAllocPage.seedBlocks/0` (imported `page_heap_box.hako:65`) | `loop(i < capacity) { 4×push; i=i+1 }` |
| binary_trees | `BinaryTreesBench.iterationCheck` (`main.hako:105`) | `loop(i <= iterations) { 2×local; 2×accum; i=i+1 }` |
| mimalloc_lite | `HakoAllocPage.seedBlocks/0` (same import) | same as boxtorrent |

(Lowering order: `using` deps merge before the main file, instance
boxes are immediate work, methods lower alphabetically — so
`seedBlocks` precedes boxtorrent's own loops and `iterationCheck`
precedes `run`. boxtorrent additionally carries a second pure-stmt
no-exit site at `releaseFrom`, `main.hako:200`.)

Debug trace on boxtorrent
(`HAKO_JOINIR_DEBUG=1 NYASH_RING0_LOG_LEVEL=debug`) shows the recipe
accepts every statement (`Stmt` items build fine) and the extractor
rejects only at the post-recipe gate
`reason=no_break_or_continue handoff=out_of_scope` — the family is
deliberately scoped to loops carrying an exit signal
(break/continue/return/exit-if/conditional-update), and `OutOfScope`
names no other owner.

Vocabulary audit (read, no test-probing of mappings):
`try_build_loop_facts_inner` runs 18 extractors;
`loop_simple_while` requires a literal bound and a step-only body;
`loop_cond_break_continue` requires ≥1 exit-signal item
(`NoBreakOrContinue`/`NoExitIf` rejects); all other families are
narrower specializations. The callable lane's sole families are only
`LoopCondBreakContinue` and `LoopTrueBreakContinue`
(`CallableLoopSoleFamilyV1`). The node-route winner spine
(`issue_loop_node_winner_recipe_v1`) offers only DirectAccum /
NestedPredicate / LoopTrue / LoopCond / GenericG0 — `LoopCond`'s
source projection requires `body==[If with else]` and GenericG0 is a
fixed nested-loop benchmark shape. **No owner for no-exit conditional
loops exists anywhere in the pipeline**; routing `FactsAbsent` to the
node route would just re-freeze at `zero selected family candidates`.
Physical capacity already exists: `RecipeContractKind::LoopWithExit`
makes exits optional and `lower_loop_cond_break_continue_source` is a
general standard-5-block `LoopCond` lowerer whose `Stmt` items are
already driven by `lower_loop_cond_source_item`.

Probes (debug binary, MIR-emit path only):
`loop(i<n){i=i+1}` → `facts-absent`; `loop(i<10){i=i+1}` passes facts
(literal bound → `loop_simple_while`) and reaches a later unrelated
terminal — confirming the divider is exit-signal/bound vocabulary, not
lane plumbing.

### Q2 — is `loop-cond-item-unsupported` the same boundary?

No — it is a **distinct owner**, one stage later. json's
`JsonStreamAggregator.ingest` `loop(start < n)` body
`[local end; if end<0{end=n}; ingestLine; start=end+1]`
(`main.hako:145` — inside `ingest`, not the `JsonLine` static child
as the D3 row loosely stated)
produces `LoopCondBreakContinue` facts today (a `ConditionalUpdateIf`
item counts as an exit signal), reaches `LoopCondReady`, then the
located-source parts driver `lower_loop_cond_source_item` hits
`ConditionalUpdateIf` in its `_` arm —
`callable_loop_source_items.rs:233`. The driver covers only
`Stmt|ExitLeaf|ProgramBlock|GeneralIf|ExitIfTree`; the raw path
already lowers this item via
`lower_conditional_update_if_assume_with_break_phi_args_recipe_first`
(`loop_cond_bc_item.rs:286`). Fixing the F1 vocabulary moves
`ConditionalUpdateIf`-bearing loops (json, plus latent
boxtorrent `digest`/`join` and binary-trees `run` ifs that update
carriers) onto exactly this arm — F1 alone cannot finish those apps.
Also noted: ANY `if` (incl. `GeneralIf`) already counts as an exit
signal upstream, so F1's extractor in practice only sees bodies with
zero `if`s — its item allowlist stays `{Stmt, ProgramBlock,
GeneralIf}` defensively.

### Q3 — bounded slice selection

Two inventoried candidates:

- **F1** (×3, dominant): new extractor `loop_cond_no_exit` producing
  `LoopCondBreakContinueFacts` — additive BoxCount on the Facts
  vocabulary; sole family/issue/lower unchanged.
- **F2** (×1 direct + latent behind F1): `ConditionalUpdateIf` arm in
  the located-source parts driver, porting the existing raw-path
  lowerer onto `port.body_stmt` + `lower_explicit_if` machinery.

Promote **F1** as the next slice (dominant class; unblocks mimalloc
fully and advances boxtorrent/binary-trees to named terminals). F2 is
the next inventoried candidate, not blocked by F1 ordering.

## Decision

```text
Decision:
  Loop-facts failures fork into two families. Promote F1:
  add `loop_cond_no_exit` as a new extractor feeding the existing
  `LoopCondBreakContinueFacts` field (new `LoopCondBreakAcceptKind::
  NoExitBody`), so no-exit conditional loops reach the existing sole
  family `LoopCondBreakContinue` -> `loop_cond::issue` ->
  `lower_loop_cond_break_continue_source`. F2 (`ConditionalUpdateIf`
  parts arm) is the next inventoried candidate.

Source authority + canonical issuer:
  CallableGenericLoopSourceFactsIssuerV1::issue_once stays the sole
  issuer; the new extractor lives in plan/facts/ and runs inside
  `try_build_loop_facts_inner` after `loop_cond_break_continue`
  (acceptance sets are disjoint: this extractor requires zero
  exit-signal items). Reuses `build_loop_cond_break_continue_recipe`;
  recipe items restricted to {Stmt, ProgramBlock, GeneralIf} so the
  parts driver covers every emitted item — ConditionalUpdateIf and any
  exit-bearing item stay out, preserving honest terminals.
  Facts keep no Recipe keys/IDs; `LoopCondBreakContinueFacts` is
  constructed directly with `accept_kind = NoExitBody`,
  `continue_branches = []`, `body_exit_allowed = None`.

Non-authority:
  No dispatch of `FactsAbsent` to `lower_loop_or_freeze_v1` (the node
  route has no family for this shape either — it would re-freeze).
  No relaxation of `loop_cond_break_continue`'s exit gates, no change
  to `loop_simple_while`/`VariableAccumRecurrence`, no new Facts type,
  no `Option` fallback, no `.hako` workaround.

Fail-fast boundary:
  Extractor returns Ok(None) on `loop(true)`, any
  break/continue/return/exit-if/ConditionalUpdateIf item, nested
  loops, or an unsupported condition — `FactsAbsent` stays the named
  terminal for uncovered shapes. `pin_accept_kind_contract` gains the
  `NoExitBody` arm; `release_allowed()` keeps it non-release until
  evidence says otherwise. Skeleton/features still required via the
  existing `has_any` -> `try_extract_loop_skeleton_facts` path.

Smallest next slice (MIRBUILDER-EXE-ACCEPTANCE-LOOP-COND-NO-EXIT-S0):
  extractor + wiring + `NoExitBody` accept kind + focused tests
  (positive: `loop(i<n){i=i+1}` and carrier/method-call bodies emit
  loop MIR through LoopCondReady; negative: exit-bearing and
  ConditionalUpdateIf bodies still route to the existing extractor or
  stay `facts-absent`) + guard pin + fresh real-app receipt.

Non-claims:
  Does not claim json_stream_aggregator (needs F2), does not claim
  boxtorrent/binary-trees full green (their `ConditionalUpdateIf`
  items land on `loop-cond-item-unsupported` — F2), does not touch the
  node-route winner spine, does not retire FactsAbsent.
```
