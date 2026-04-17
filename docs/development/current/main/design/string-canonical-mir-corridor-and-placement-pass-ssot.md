---
Status: Provisional SSOT
Decision: provisional
Date: 2026-04-17
Scope: borrowed-view hot corridor を `.hako policy -> canonical MIR facts -> rewrite target -> Rust thin executor -> LLVM` の順で generic substrate として固定し、delete-oriented に進める設計と実装順を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/optimization-task-card-os-ssot.md
  - docs/development/current/main/design/llvm-line-ownership-and-boundary-ssot.md
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

- string hot lane を「Rust helper を当てる場所」ではなく「Rust に渡る前の corridor truth を固定する場所」として扱う
- `substring_hii` / `len_h` の leaf tuning を、上流の borrowed-view corridor / boundary 決定へ戻す
- `.hako -> canonical MIR -> rewrite target -> Rust thin executor -> LLVM` の owner split を固定する
- public IR dialect や public syntax を増やしすぎずに pure Rust lower bound へ近づける

## Scope Terms

- `.hako scope`
  - lexical scope, value meaning, control-flow, and user-visible contract
  - this remains the only language-meaning scope
- `proof_region`
  - MIR-side region where an already-legal borrowed corridor fact is proven to hold
  - examples:
    - borrowedのまま流してよい
    - escapeしない
    - final consumerまでhandle化しなくてよい
- `publication_boundary`
  - MIR-side non-widening contract that says where a specialized executor may be published
  - this is not lexical scope
  - this is not runtime route re-recognition

Reading lock:

- do not use `scope_lock` as the architecture term in this lane
- use `proof_region` plus `publication_boundary`
- if a cut cannot state both cleanly, it is still research and not ready for code
- generic contract vocabulary such as `same-corridor unpublished outcome` is
  owned by
  [optimization-task-card-os-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-task-card-os-ssot.md)
- this lane-specific SSOT may refine that contract as
  `string-lane unpublished text outcome`

## Current Perf Reading

Current active broader-corridor front is `kilo_micro_substring_concat`.
Current accept gate is `kilo_micro_substring_only`.

- live reread on 2026-04-17:
  - `kilo_micro_substring_only`
    - `C: instr=1,622,875 / cycles=484,287 / ms=3`
    - `Ny AOT: instr=1,668,892 / cycles=1,012,862 / ms=3`
  - `kilo_micro_substring_concat`
    - `C: instr=1,622,874 / cycles=485,494 / ms=3`
    - `Ny AOT: instr=260,619,140 / cycles=70,100,232 / ms=21`
- current top symbols on the same artifact family:
  - `piecewise_subrange_hsiii_fallback closure`
  - `__memmove_avx512_unaligned_erms`
  - `malloc`
  - `_int_malloc`
  - `std::sync::once_lock::OnceLock<T>::initialize`
- current top symbols on the same artifact family:
  - delete-oriented `mir-rewrite` is already landed
  - `publication_boundary` is now landed too: active `insert_hsi -> substring_hii` lowers to runtime-private `piecewise_subrange_hsiii`
  - the new dominant owner is the runtime executor body itself, not route selection or generic fallback re-entry

Reading:

- this is no longer a missing `substring` semantics problem
- this is a borrowed-view lane continuity problem on the `substring -> concat`
  corridor, with the remaining gap now concentrated inside the runtime fallback
  executor
- `BorrowView` already exists as classification, and the delete-oriented
  `mir-rewrite` already removed producer-substring churn from the active front
- the next win should come from thinning the landed single-session piecewise
  executor, not from widening generic helper bodies or reopening MIR routing
- the current end-state already deletes hot `substring_hii` re-entry from the
  active front; the remaining gap is allocator/memmove pressure inside the
  executor-local copy/materialize path
- the current contract already allows delayed publication on the active
  corridor; the problem is not “string must be boxed immediately” as a language
  rule
- the current implementation still lacks a natural mainline unpublished-outcome
  carrier, so the executor tail keeps collapsing the result into
  `StringBox`/`Arc`/handle publication
- repeated local executor-only probes on the current representation are now
  non-wins, so the next live action is a focused consult on result
  representation / result ABI rather than another helper-thin experiment

## Adopted Reading

- keep the generic substrate as `borrowed-view / materialize-on-escape`
- do not add a new string-specific public MIR dialect
- keep MIR as the owner of the corridor contract, Rust as the mechanical
  executor, and LLVM as the consumer of truthful exported facts
- keep the translation single-pass:
  - `.hako` chooses semantics once
  - MIR proves the corridor and selects the rewrite target
  - runtime executes the selected runtime-private executor only
  - LLVM consumes the result
- current naming split:
  - generic contract:
    - `same-corridor unpublished outcome`
  - phase-137x lane realization:
    - `string-lane unpublished text outcome`
  - runtime-private executors such as `piecewise_subrange_hsiii` remain
    string-specific mechanics below that contract
  - this split is intentional:
    - the contract is generic and reusable
    - the current mainline realization is still string-first and should not be
      genericized before another real consumer exists
- shim-local remembered/deferred piecewise state is transport glue only:
  - `remember_deferred_piecewise_subrange(...)`
  - `find_deferred_piecewise_subrange(...)`
  - these may carry MIR-owned publication metadata across the pure-first seam
  - they must not become legality owners or route owners
- internal result manifest / internal direct-kernel ABI split is owned by
  [value-repr-and-abi-manifest-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md)
- this lane only decides when unpublished outcome is legal and when publication
  must happen; it does not redefine the manifest row shape
- treat handle/TLS/cache lookup as the cold adapter path, not as the steady-state
  hot lane
- the landed arm split says `ViewSpan` is the only live slow-plan arm on the
  current front, and the landed delete-oriented rewrite already consumed that
  proof
- executor-local measurement is now closed:
- `piecewise_subrange total=300000`
- `single_session_hit=300000`
- `fallback_insert=0`
- `all_three=300000`
- landed `mir-proof` now keeps the next primary owner on `runtime-executor`
- the current target is a two-card return:
  - landed `mir-proof`:
    - keep generic borrowed-view plan truth in MIR
    - add the publication truth `publish-now not required before first external boundary`
      as plan metadata
    - keep the landed runtime-private `piecewise_subrange_hsiii`
      publication narrow:
      - active corridor only
      - generic helper body unchanged
      - broad callers untouched
  - next `runtime-executor`:
    - split runtime-private freeze vs publish using existing seams such as
      `OwnedBytes` / `TextPlan`
    - delete only the eager publication tail on the active corridor:
      - `StringBox`
      - `Arc`
      - fresh `handle_issue`
    - do not reopen route logic, piece-shape branching, transient box/handle
      carriers, widen `insert_hsi`, or jump to public-ABI edits on this lane
    - a pure `freeze -> publish` helper split is not a keeper by itself; keep it
      only if it also deletes eager publication on the exact front
- current design verdict:
 - the publication-boundary design is not the blocker on this front
  - the remaining exact gap is concentrated in final owned materialize ->
    objectize -> fresh handle issue
- if executor-local measurement plus thin cuts stop producing wins, only then
  open a later representation/ABI card for “beat C” work
 - that threshold is now reached on the active `piecewise` front
 - consult + source review triage:
   - adopt:
     - semantic result birth and public handle publication must be separated
     - `proof_region` and `publication_boundary` remain MIR-owned
     - new MIR dialect is unnecessary
     - runtime-private seams should start from existing `OwnedBytes` /
       `TextPlan`
   - hold:
     - the exact private outcome shape and result ABI details
   - reject:
     - runtime/shim re-recognition of legality
     - generic helper widening
     - public ABI rethink on this lane
 - the consult must explicitly include:
   - handle-based public surface
   - final `owned String -> boxed handle`
   - whether runtime-private result representation can change without widening
     MIR/publication/public ABI
 - current test gate note:
   - use `cargo test -q -p nyash_kernel --lib -- --test-threads=1` as the
     deterministic library gate on this lane
   - treat parallel `cargo test -q -p nyash_kernel --lib` as monitor-only until
     cache/view isolation is fixed
 - next architecture lock needed before this lane can be “10/10”:
   - Birth / Placement vocabulary must connect to an internal result manifest
   - internal direct-kernel ABI must carry unpublished outcome without
     widening the public handle facade

## Publication / Objectization Legality Direction

This SSOT owns the legality rules for publication/objectization on the active
string corridor.

Target verifier reading:

- if a lane promises `same-corridor unpublished outcome`, it is illegal to
  enter `FreshRegistryHandle` before the first external boundary
- if the lane does not demand stable object identity yet, it is illegal to
  insert `StableBoxNow`
- if a direct-kernel lane promises unpublished outcome continuity, it is
  illegal to replay the public ABI facade before the boundary
- if the lane promises unpublished continuity, it is illegal to use the host
  handle registry as the unpublished carrier

Phase-137x reading:

- this remains string-lane-specific legality
- the phase-137x minimal internal class is `OwnedBytes`
- the verifier should reject “early publish” and “registry carrier” mistakes
  before another runtime-local widening is attempted

Do not move these legality checks into runtime route re-recognition.
Runtime remains executor only.
 - latest runtime-private seam reread:
   - `freeze_owned_bytes(...) -> publish_owned_bytes(...)` on the active
     `piecewise_subrange_hsiii` tail compiled and passed targeted tests
   - exact-front reread still moved to `261,219,101 instr / 21 ms`
   - treat that seam as reject on this lane; it validates the split shape but
     does not remove the hot publication tax
 - rejected shared-materialize OwnedBytes seam:
   - making the phase-137x minimal internal-result seam explicit through the
     shared materialize/publication path kept the exact front near the keeper
     band (`261,218,390 instr / 19 ms`) but regressed `kilo_kernel_small_hk`
     to `1965 ms`
   - read this as shared-helper scope widening, not as a rejection of the
     `OwnedBytes` carrier idea itself
   - do not reopen this shape by rewriting shared `string_handle_from_owned`
     style helpers; keep future `OwnedBytes` work corridor-local or
     direct-kernel-local
 - rejected registry-backed deferred publication:
   - storing fresh `piecewise_subrange_hsiii` results as deferred owned text
     behind the public handle surface regressed the exact front to
     `655,162,062 instr / 65 ms`
   - the top symbols moved back to `insert_const_mid_fallback`,
     `substring_hii`, and `LocalKey::with`
   - read this as a loop-carried fast-path breakage, not as a publication-tail
     win
   - the broken property was next-iteration pure-string continuity on the
     landed `piecewise_subrange_hsiii` route; registry-backed deferred text was
     readable through `TextReadSession`, but it was not a transparent carrier
     for deferred-piecewise transport identity
   - do not reopen registry-backed deferred owned-text publication on this lane
     without proof that next-iteration pure-string consumers stay on the landed
     piecewise route
 - latest corridor-local cache-seed reread:
   - skipping `string_len_fast_cache_store(...)` for fresh non-empty
     `piecewise_subrange_hsiii` results stayed flat at
     `261,218,548 instr / 21 ms`
   - treat per-iteration len-cache seeding as secondary; it is not the
     dominant publication-tail cost on this front

## Generic Minimum

The generic substrate must stay narrower than a string-only MIR dialect.

Allowed MIR-side corridor truth:

- root / provenance
- start
- len
- source_kind
- materialize_policy
- consumer_capability

Reading lock:

- the plan is generic
- the first consumer may be string-specific
- future consumers (`len`, `compare`, `store`) should reuse the same corridor
  truth without widening MIR vocabulary
- helper names are not MIR truth
- runtime-private executor names are also not MIR truth

This means:

- `BorrowedViewPlan` is generic substrate
- `piecewise_subrange_exec(...)` is the next runtime-private consumer/executor
- string-specific executor names must stay below the MIR contract seam
- `publication_boundary` is the line that keeps this executor from becoming a
  generic helper replacement
- rejected runtime-private carrier probe:
  - do not model the next cut as a transient piecewise box/handle carrier
  - exact front evidence showed `clone` / `TextPlan::from_pieces` /
    allocation costs dominating that path

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

Current follow-on reading:

- `src/runner/mir_json_emit/mod.rs` already exports `string_corridor_facts`,
  `string_corridor_relations`, and `string_corridor_candidates`
- landed follow-on:
  - MIR JSON now exports `metadata.string_kernel_plans`
  - `hako_llvmc_ffi_string_loop_seed.inc` now consumes that plan first for the
    stable-length `substring_concat` len route
- landed bridge shrink:
  - the old loop matcher no longer accepts the 14-op len-route fallback once the
    plan-first keeper parity was proven
- the remaining missing seam for exact-seed retirement is shrinking the old
  full-loop shape matcher itself when a generic plan-selected full-loop route exists,
  not raw metadata export itself
- active structural follow-on:
  - `phase-180x` is now the current cleanup lane
  - next work is seam cleanup, not another exact proof:
    - extract `StringKernelPlan` owner
    - stop `relation -> candidate` reverse dependency
    - split shim metadata readers away from generic owner files
- landed local observation before the next runtime cut:
  - `borrowed_substring_plan_from_live_object(...)` is now split by
    `ReturnHandle` / `ReturnEmpty` / `FreezeSpan` / `ViewSpan`
  - live evidence on the active front is `ViewSpan=600000`, others `0`
  - keep that split as frozen observe-only evidence; it is not a new public
    carrier and not a second IR family

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

Rust does not own:

- corridor legality
- publication boundary
- route re-recognition for specialized executors

That ownership stays in MIR.

Delete-oriented reading:

- `substring_hii` generic handle corridor stays as legacy/cold adapter
- hot `substring -> concat` should move to a plan-native executor path
- runtime must not own the decision of when that path is legal
- generic helper bodies stay semantically broad; active-corridor specialization
  must arrive through publication boundary, not by silently widening the helper

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
- `StringOutcomeFact` / `StringPlacementFact` stay string-local for now; `phase-166x` explicitly deferred a generic lifecycle/boundary extraction until another real lifecycle consumer exists
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

1. keep the owner split fixed: MIR contract, Rust executor, LLVM consumer
2. preserve `borrowed-view / materialize-on-escape` as the generic substrate
3. keep the landed arm split as frozen evidence and do not reopen it unless new measurements disagree
4. isolate the cold `handle_to_plan` / `plan_to_handle` adapter seam
5. reopen only a narrow `runtime-executor` card on `kilo_micro_substring_concat`
6. keep `publication_sink` / `materialization_sink` / `direct_kernel_entry`
   as canonical MIR consumers, not as new public ops
7. only then resume exact-seed retirement and any further runtime leaf cuts

This replaces the earlier reading where the next move was another
`substring_hii`-local provider/cache split.
