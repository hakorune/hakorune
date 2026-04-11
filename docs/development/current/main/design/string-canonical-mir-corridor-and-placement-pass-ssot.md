---
Status: Provisional SSOT
Decision: provisional
Date: 2026-04-09
Scope: string hot lane を `.hako policy -> canonical MIR facts -> placement/effect pass -> Rust microkernel -> LLVM` の順で薄くする設計と実装順を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/design/birth-placement-ssot.md
  - docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md
  - docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md
  - docs/development/current/main/design/rune-v1-metadata-unification-ssot.md
  - docs/development/current/main/design/rune-v0-contract-rollout-ssot.md
  - docs/development/current/main/design/rep-mir-string-lowering-ssot.md
  - crates/hakorune_mir_core/src/effect.rs
  - crates/hakorune_mir_defs/src/call_unified.rs
  - crates/nyash_kernel/src/exports/string_helpers.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - crates/nyash_kernel/src/hako_forward_bridge.rs
---

# String Canonical MIR Corridor And Placement/Effect Pass SSOT

## Goal

- string hot lane を「Rust helper を当てる場所」ではなく「Rust に渡る前の意味の曖昧さを減らす場所」として扱う
- `substring_hii` / `len_h` の leaf tuning を、上流の corridor / boundary 決定へ戻す
- `.hako -> canonical MIR -> placement/effect pass -> Rust microkernel -> LLVM` の owner split を固定する
- public IR dialect や public syntax を増やしすぎずに pure Rust lower bound へ近づける

## Current Perf Reading

Current active local front is `kilo_micro_substring_views_only`.

- exact reread on 2026-04-09:
  - `instr=34,372,749`
  - `cycles=6,415,829`
  - `cache-miss=8,601`
- top:
  - `nyash.string.substring_hii 85.99%`
  - `ny_main 7.30%`
- annotate reading:
  1. `SUBSTRING_ROUTE_POLICY_CACHE` decode
  2. provider state read + `SUBSTRING_VIEW_ARC_CACHE` TLS entry/state check
  3. steady-state compare path
  4. slow plan / materialize is not the dominant block on this front

Current reading says the next large win is not another local helper rewrite.
It is reducing per-call route/provider/cache-entry tax by deciding more of the
borrowed corridor before Rust runtime mechanics run.
The first sink candidate on that path is now `nyash.string.substring_len_hii`:

- mixed accept gate reread on 2026-04-09:
  - `instr=47,270,021`
  - `cycles=28,264,307`
  - `cache-miss=9,191`
  - `ny_aot_ms=8`
- split exact reread on the same date:
  - `kilo_micro_substring_views_only: instr=34,372,839 / cycles=6,483,811 / cache-miss=8,932 / AOT 5 ms`
  - `kilo_micro_len_substring_views: instr=16,072,530 / cycles=4,296,034 / cache-miss=8,783 / AOT 4 ms`

## Fixed Decisions

### 1. Canonical MIR stays single-source

- do not add a permanent second public MIR dialect such as `OptimizedKernelMIR`
- keep one canonical MIR surface
- add string-lane `outcome/effect facts` on top of canonical MIR
- let a placement/effect pass consume those facts and rewrite/sink boundaries

This keeps:

- one public MIR truth
- one naming surface for docs / dumps / compiler reasoning
- one owner line for semantic corridor decisions

### 2. `.hako` stays policy-only

`.hako` owns:

- route vocabulary
- retained-form choice
- boundary choice
- visible semantic outcome choice

`.hako` does not own:

- runtime cache layout
- epoch token shape
- handle compare order
- provider/TLS state machine
- publication mechanics

### 2.5. Plan metadata, not a new IR

The next genericization step should extend existing string-corridor metadata.
Do not introduce a new public IR family or a second canonical string dialect.

Reading lock:

- canonical MIR remains the only IR truth
- `string_corridor_facts` remain the observation layer
- `string_corridor_candidates` should grow into proof-bearing plan metadata
- backend/lowering should consume that plan metadata instead of re-deriving
  benchmark-shaped decisions from helper names or exact block shapes

Plan fields that are allowed to grow here:

- borrowed root/source identity
- `start` / `end`
- known-length proof
- publication demand
- materialization demand
- direct-kernel-entry legality

Do not encode:

- runtime cache layout
- ABI/private token details
- a benchmark-specific special form such as a permanent `InsertMid` IR op

### 3. `@rune` is not the next tool

Do not widen `@rune` for this wave.

Reasons:

- current Rune v0/v1 SSOT fixes `@rune` as declaration-local metadata only
- statement-position canonical runes are fail-fast today
- boundary/control/runtime state hints would pollute the current surface

If a future boundary hint is still needed after MIR inference hardens, treat it
as a later language-design question, not as the first move of this lane.

### 4. Rust stays the microkernel

Rust keeps only stateful mechanics:

- borrowed view/span lifetime
- `TextReadSession`
- `drop_epoch` invalidation
- handle table / cache
- handle reissue
- objectization / publication
- observer backend

Rust should not keep semantic ambiguity that the compiler can decide earlier.

### 5. AOT internal path must not replay ABI facade

- AOT-internal string corridor should select direct kernel entry where possible
- ABI / FFI entry keeps the facade
- internal borrowed corridor should not repeatedly pay the same dispatch/publish boundary if the boundary is not externally visible

## Fact Vocabulary

Use Birth / Placement outcome names from the existing SSOT as the MIR-facing
corridor vocabulary:

1. `ReturnHandle`
2. `BorrowView`
3. `FreezeOwned`
4. `FreshHandle`
5. `MaterializeOwned`
6. `StoreFromSource`

Reading lock:

- `ReturnHandle` is an outcome, not a standalone executor op
- `BorrowView` is a non-owning corridor result
- `FreezeOwned` is a sink outcome
- `FreshHandle` / `MaterializeOwned` remain backend events below the semantic corridor

Do not add `box_id` to this vocabulary.

## Canonical MIR Rule

Canonical MIR should carry the string corridor through canonical ops such as:

- `str.slice`
- `str.len`
- `freeze.str`

The lane should not model helper names as semantics.
It should model:

- which semantic outcome the op is allowed to produce
- whether objectization/publication is demanded now or can sink later
- whether the result stays inside a borrowed corridor
- whether direct kernel entry is legal for the current consumer path

Current migration note:

- canonical fact inventory lives in `src/mir/string_corridor.rs`
- legacy/helper/runtime-name semantic recovery is quarantined in `src/mir/string_corridor_compat.rs`
- later string-domain passes may still recognize helper shapes as compat consumers, but fact ownership should stay canonical-first

## Placement/Effect Pass Rule

The new pass is an optimizer pass over canonical MIR facts, not a new public IR.

Its first responsibilities are:

1. publication sinking
2. materialization sinking
3. borrowed corridor fusion
4. direct kernel entry selection

Its first non-goals are:

- runtime cache mechanics
- epoch/provider/TLS lowering details
- VM/plugin/FFI widening
- new public token types

## Rust Microkernel Rule

The pass may decide that a corridor stays borrowed longer.
Rust still executes the stateful mechanics below that choice.

Target reading:

- MIR decides whether the path is still `BorrowView`
- Rust decides how that borrowed path is guarded, cached, reissued, or published

This preserves the current stop-line:

- semantic corridor above
- mechanics below

## Cross-Lane Scope Control Table

This table exists to keep the pilot from warping the whole design around
`string` alone.

Rule:

- `string` is the active proving ground
- other lanes are listed only to keep owner split and genericization honest
- this table is not permission to widen the current implementation slice

| Lane family | Current role | Canonical MIR corridor candidate | Placement/effect applicability now | Rust microkernel keep | Syntax status |
| --- | --- | --- | --- | --- | --- |
| `string` borrowed corridor | active pilot | `str.slice`, `str.len`, `freeze.str` | yes; this is the current proving ground | `TextReadSession`, `drop_epoch`, handle table/cache, reissue, objectization/publication | no new syntax |
| `string` scalar consumers | follow-on reuse target | `str.len`, future `str.eq*` / search leaves | later; only after the first corridor win lands | read-only runtime guards and cache mechanics | no new syntax |
| `array/map` visible owner lanes | comparison row only | existing canonical collection ops, not this borrowed-string corridor | not in this wave; only revisit if a repeated internal borrowed corridor appears | raw substrate, handle/cache, runtime state | no new syntax |
| plugin / FFI / ABI boundary | fixed public boundary | no internal borrowed corridor across public ABI | no; facade must remain | host boundary, ownership, publication, handle world | existing declaration-local Rune only |
| generic hot-lane framework | deferred | none yet | blocked until two lanes show the same keeper invariant | n/a | no new syntax |

Interpretation:

- only the first row is an active implementation target
- the other rows are structural guardrails
- if a proposal starts forcing `array/map` or ABI paths to look like `string`,
  it is probably overfitting the pilot

## Implementation Order

### Step 1. Docs-first lock

- lock this corridor design in docs
- make `CURRENT_TASK.md` and `phase-137x/README.md` point to this design
- freeze the rule that substring leaf tuning is no longer the first move

### Step 2. MIR inventory

- inventory where current string canonical ops or their current surrogates are created
- inventory where current lowering still bakes helper/route identity into the compiler path
- identify the narrowest carrier for string outcome/effect facts with no runtime behavior change
- landed:
  - `src/mir/string_corridor.rs` now refreshes per-function string corridor inventory from current MIR instructions
  - current carrier reading stays on existing MIR shapes: `MethodCall`, `GlobalLoweredFunction`, `RuntimeExport`, `CanonicalIntrinsic`

Acceptance:

- docs + code map show where `str.slice`, `str.len`, and `freeze.str` facts attach

### Step 3. Fact carrier with no behavior change

- add a canonical MIR-side fact carrier for string outcome/effect reading
- keep current runtime behavior unchanged
- dumps/inspection must show the facts
- landed:
  - `FunctionMetadata.string_corridor_facts` is the no-op carrier
  - `MirCompiler` refreshes the facts after the current pipeline finishes
  - `MirPrinter::verbose()` shows the facts without adding a second MIR dialect

Acceptance:

- compiler emits the same runtime behavior
- debug/dump path can show string outcome/effect facts

### Step 4. Placement/effect pass scaffold

- add a no-op or trace-only placement/effect pass
- it must read the new facts and report candidate decisions without changing runtime behavior yet
- landed:
  - `src/mir/string_corridor_placement.rs` now refreshes per-function candidate decisions from `FunctionMetadata.string_corridor_facts`
  - candidate surface is inspection-only and currently covers:
    - borrowed corridor fusion
    - publication sinking
    - materialization sinking
    - direct kernel entry
  - `MirCompiler` refreshes the candidates after fact refresh and before returning the compiled module
  - `MirPrinter::verbose()` shows `FunctionMetadata.string_corridor_candidates`

Acceptance:

- pass runs in the pipeline without changing output
- dumps/traces show candidate sinking/fusion decisions

### Step 5. First real transform: borrowed corridor sinking

- pilot on the narrowest useful string corridor first
- prefer `str.slice -> str.len` or equally narrow borrowed consumer chains
- sink publication/materialization when the path remains internal and borrowed
- landed structural pilot:
  - `src/mir/passes/string_corridor_sink.rs` rewrites single-use `substring(...).length()` chains to `nyash.string.substring_len_hii`
  - `nyash.string.substring_len_hii` is now available in both the kernel export layer and the MIR interpreter extern fallback
  - current status is structural plus perf-positive candidate: compile/test are green, and the mixed accept gate now rereads at `instr=47,270,021 / cycles=28,264,307 / cache-miss=9,191 / AOT 8 ms`

Acceptance:

- exact/micro proof moves
- dumps show fewer forced boundaries before Rust microkernel

### Step 6. Next genericization slice: plan metadata widening

- keep using the existing `string_corridor_candidates` carrier
- enrich it from inspection-only candidate rows toward proof-bearing plan rows
- do not add a separate `StringRecipe` / `OptimizedStringMIR` family
- the first widening should support the current broader-corridor reopen front
  `kilo_micro_substring_concat`

Acceptance:

- dumps show enough plan evidence to explain the chosen broader-corridor transform
- lowering no longer needs to infer the same shape from exact helper names alone

### Step 7. First generic transform family: `publication_sink`

- after the landed narrow borrowed-corridor sink, the next real generic transform
  is `publication_sink`
- use `kilo_micro_substring_concat` as the exact reopen front
- keep the semantic reading as:
  - canonical meaning = concat/slice corridor
  - specialization = a plan-selected internal route, not a new canonical op

Acceptance:

- broader concat/slice corridors stay borrowed longer without forcing early
  publication on the internal path
- exact/front evidence moves on `kilo_micro_substring_concat`

### Step 8. Follow-on generic transform: `materialization_sink`

- once publication sinking is explicit in plan metadata, sink materialization to
  the last semantically required boundary
- keep `freeze.str` / externally visible sinks as the forcing boundaries

Acceptance:

- no new public syntax or IR
- fewer early owned-materialization points appear in the broader corridor path

### Step 9. Direct kernel entry as plan consumer

- `direct_kernel_entry` should be selected from plan metadata close to lowering
- builder should not decide it
- this is the intended replacement direction for the remaining exact seed logic

Acceptance:

- AOT/internal path uses plan-selected direct kernel entry
- public ABI path still goes through the facade

### Step 10. Exact-seed retirement rule

- exact seed logic in backend shims is a temporary bridge, not the target design
- when a generic plan-selected route wins on the same exact front, shrink the
  corresponding exact seed instead of growing another permanent benchmark path

Current bridge target:

- `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc`

Acceptance:

- generic corridor plan owns more of the route choice
- exact seed surface shrinks only after the generic path proves the same keeper

### Step 11. Only then revisit syntax

- only if MIR inference still cannot express a needed boundary
- only after at least one corridor win is proven
- only with a new syntax proposal that does not violate current Rune stop-lines

## Do Not Do Yet

- no new public MIR dialect
- no `@rune borrow/publish/materialize` expansion
- no `.hako` runtime cache/epoch/provider mechanics
- no new public runtime token types
- no new substring-local Rust cache shape just to chase this micro

## Active Reading For Phase 137x

For the current lane, read the next work as:

1. upstream corridor/fact design
2. compiler-side fact carrier
3. placement/effect pass
4. proof-bearing plan metadata on `string_corridor_candidates`
5. `publication_sink` as the first broader generic transform
6. `materialization_sink`
7. plan-selected `direct_kernel_entry`
8. only then more exact-seed retirement and any new runtime leaf cuts

This replaces the earlier reading where the next move was another
`substring_hii`-local provider/cache split.
