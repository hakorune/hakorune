# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D3

Status: accepted__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-MAIN-WRAPPER-PARAM-ENTRY-S0
  (landed — injected-local entry adoption)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0/D1/D2 selection lineage (D2 was NoSafeSlice).

## Problem

Wrapper entry adoption landed (`main(args)` installs declared
`Parameter` bindings from the injector's published locals; the lane
now reaches body lowering and publication for arity-bearing mains).
Select the next bounded design slice from the remaining 7 failing
real-app EXE entries.

## Fresh class map (receipt after PARAM-ENTRY-S0)

| class | entries | first named stop |
|---|---|---|
| `ordinary-new/birth-global-legacy-stopped` | 3 | boxtorrent_mini, binary_trees, mimalloc_lite |
| `callable-loop/parts loop-cond-item-unsupported` | 1 | json_stream_aggregator (`ConditionalUpdateIf` cond in a `JsonLine` static child) |
| `CoreMethodSource/NamedArray(TextSourceMissing)` | 1 | allocator_stress |
| backend `no_lowering_variant` (toolchain) | 1 | typed_object_newbox_min |
| inference panic `return_type_strategy` | 1 | typed_object_untyped_field_min |

All three `birth-global-legacy-stopped` entries stop in a static
child's `birth` on the ordinary-new lane — lowered before the root
body, so none exercises the just-landed wrapper-param adoption yet.

## Questions

1. Do the three `birth-global-legacy-stopped` entries share one
   owner edge (ordinary-new claim authority for `birth` bodies), or
   do they fork (return-position `new` vs local-initializer `new`
   vs other)?
2. Is `loop-cond-item-unsupported` (ConditionalUpdateIf in cond
   position) the same family as the tracked loop-facts chain, or a
   new class?
3. Which class has a bounded slice: single owner + fail-fast tuple
   + acceptance coverage?

## Boundary

- Includes: owner census for the 3-entry birth class; divergence
  check vs the other three singleton terminals; one Decision with a
  six-line brief or NoSafeSlice with the highest-information class
  named.
- Excludes: implementation; backend toolchain (`no_lowering_variant`
  stays parked-debt); inference panic family (prior D0 lineage).

## Exit

- [x] Decision recorded (bounded slice OR sealed NoSafeSlice with
      reopen triggers) with the six-line brief.
- [x] Next S-card emitted OR the named design card opened;
      pointers synced.

## Census result (worker + principal verification)

Issuer: `ordinary_new_admission.rs:94-103` — an intentional named
terminal left by R6-S1 when the arity-malformed
`LegacyCallV0{Global(StaticBoxMethod(<Class>.birth/N))}` carrier was
retired. The missing authority is the ordinary-new claim, which is
**local-commit-shaped**: `OrdinaryNewAdmissionClaimV1` demands
`destination: BindingRefV1` + `declaration: SourceBindingSiteV1`,
and both the issuer walk (`issue_ordinary_source_cohort_v1`,
`is_direct_local_initializer` = `[Body, Initializer]`) and the
consumer gate (`try_take_ordinary_new_claim`,
`complete_ordinary_new_expression`) hard-filter to that position.

Probes (read-only, /tmp fixtures): `return new Foo(7)` and
`me.f = new Inner(3)` both reproduce `birth-global-legacy-stopped`
on `target/quick/hakorune --emit-mir-json`.

Per-app first stops (worker census + probes):

| app | site | position | lowered birth |
|---|---|---|---|
| boxtorrent-mini | `me.allocator = new HakoAllocHeap()` :60 in `BoxTorrentStore.birth` | field-assign `[Body, Rhs]` | `HakoAllocHeap.birth/0` |
| binary-trees | `return new TreeNode(null, null, value)` :28/:36 in `BinaryTreeBuilder.make` | return `[Body, Value]` | `TreeNode.birth/3` |
| mimalloc-lite | desugared `me.small_page = new HakoAllocPage(0, LayoutBox.class_size(0), ...)` (`page_heap_box.hako:185-194`) in `HakoAllocHeap.birth` | field-assign `[Body, Rhs]` in an external-file constructor | `HakoAllocPage.birth/3` |

Worker verdict was "fork" premised on extending the claim schema
(each family needs a different `destination` shape). The reframed
reading: a **destination-less birth-recipe site index** sidesteps
destination authority entirely — the claim type keeps its
local-commit contract untouched, and the index only answers "does
this exact `new` site have a verified `Birth` recipe?".

Why this is one bounded edge:

- `ResolvedExpressionSourceInventoryV1.constructions` already
  records **every** `new` site with class + argument sites
  (`expression_source.rs:356-387`) — position-agnostic inventory.
- `instance_constructors.rows()` exposes `lowering_input(program)`
  (`instance_constructor_semantic.rs:268`), and the loan's program
  is the merged module AST — external-file constructor bodies
  (`page_heap_box.hako`) are enumerable from the same issuer, which
  already takes `instance_constructors` as a parameter.
- `birth_for(box_source, arity)` already verifies box name, arity,
  `published_birth_key` namespace `BirthConstructor`, unit
  completion, and `OpaqueObservable` effect — identical checks to
  `OrdinaryNewCandidate::resolve`; nothing new is minted.
- Physical consumer is the existing sole owner
  `lower_ordinary_raw_new_with_port_v1`, whose `Birth` branch only
  reads `claim.constructor()` (`recipe.target()`, `recipe.abi()`,
  `recipe.physical_effect_mask()`); a recipe-only take path feeds
  the same `Callee::BirthConstructor` emit.
- `current_source_site_v1` is updated for nested positions
  (`with_prepared_child_source_v1` + `Exact` on
  target/value/index/receiver descents), so `[Body, Rhs]` /
  `[Body, Value]` sites can be matched.
- `null` literal args are already handled by the raw expression
  lane (`LiteralValue::Null` arm), so binary-trees needs no
  argument-kind authority.

## Decision

```text
Decision: bounded slice accepted — unhomed birth-recipe site index.
Source authority + canonical issuer:
  ResolvedExpressionSourceInventoryV1.constructions (every `new`
  site) + VerifiedInstanceConstructorSemanticBatchV1::birth_for;
  issued by issue_ordinary_source_cohort_v1 into a
  site -> VerifiedOrdinaryNewBirthRecipeV1 index on the ledger.
Non-authority:
  destination/home/lifecycle. The index mints no admission claim;
  the enclosing statement/expression owns the produced dst flow
  (unchanged legacy shape: bare NewBox + typed BirthConstructor
  call, no fault frames / reclaim). Admission-claim sites
  ([Body,Initializer]) keep precedence; builtin/uncovered classes
  and classes without a lowered birth keep current behavior.
Fail-fast boundary:
  duplicate site in index; reuse the candidate birth checks —
  box_name/arity mismatch, non-BirthConstructor key, non-unit
  completion, non-OpaqueObservable effect all stay hard errors.
Smallest next slice:
  MIRBUILDER-EXE-ACCEPTANCE-ORDINARY-NEW-BIRTH-SITE-INDEX-S0 —
  issuer index walk over (a) walked callable declarations'
  non-admission constructions and (b) instance-constructor rows via
  lowering_input; one new port take method (default Ok(None)); one
  prepared-field + consumer branch in the raw owner.
Non-claims:
  no lifecycle/fault/reclaim tracking for unhomed objects (legacy
  parity); downstream terminals unchanged (`FieldContractUnsupported`
  for init{} boxes, non-trivial/qualified args driven raw may stop
  later, selected-path argument_rows untouched); no EXE-green claim.
```

## Exit record

- [x] Decision recorded — bounded slice, six-line brief above.
- [x] Next S-card emitted:
      `mirbuilder-exe-acceptance-ordinary-new-birth-site-index-s0-2026-09-25.md`;
      pointers synced in `CURRENT_STATE.toml` + workstream row H.
