---
Status: Draft
Scope: external consultation prompt for the phase-137x delete-oriented borrowed-view corridor wave after the rejected generic `insert_hsi` widening probe
Related:
- CURRENT_TASK.md
- docs/development/current/main/phases/phase-137x/README.md
- docs/development/current/main/design/perf-optimization-method-ssot.md
- docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md
- docs/development/current/main/design/optimization-task-card-os-ssot.md
- crates/nyash_kernel/src/exports/string_helpers/concat/const_adapter.rs
- crates/nyash_kernel/src/exports/string_view.rs
- crates/nyash_kernel/src/exports/string_helpers.rs
- src/mir/passes/string_corridor_sink/mod.rs
- benchmarks/bench_kilo_micro_substring_concat.hako
---

# External Consultation Question: Phase-137x Scope-Widening / Borrowed Corridor Design

## What I Need

I am not asking for another local micro-optimization guess.

I need a design review for how to keep optimization scope clean across:

- `.hako` semantics / policy
- MIR proof / rewrite
- Rust runtime executor
- LLVM consumer

The current failure mode is not "we do not know what is hot".
The current failure mode is:

- a narrow exact-front win is possible
- but the implementation widens into a generic runtime helper and regresses whole-program performance

So the real question is:

**How should I structure the `.hako -> MIR -> Rust runtime` flow so a runtime-executor cut stays corridor-local instead of widening into a generic helper rewrite?**

## Fixed Authority Order

This authority order is already fixed and I want to preserve it:

1. `.hako`
   - semantics
   - policy
   - user-visible contract
2. `MIR`
   - proof
   - rewrite eligibility
   - rewrite target
3. `Rust runtime/kernel`
   - thin executor
   - cold adapter
4. `LLVM`
   - truthful-fact consumer
   - inlining / simplification / codegen

Constraints:

- do not add a string-only MIR dialect
- do not widen public ABI
- do not change VMValue/public calling surface
- fail-fast only; no silent fallback
- runtime should not re-recognize route eligibility

## Current Front

Current exact front:

- `kilo_micro_substring_concat`

Accept gate:

- `kilo_micro_substring_only`

Whole guard:

- `kilo_kernel_small_hk`

Current rewrite already landed:

- ordered `substring + const + substring` is rewritten to
  - `nyash.string.insert_hsi`
  - one final `nyash.string.substring_hii`

So the producer-side substring corridor was already deleted from the hot path.

## Current Perf Facts

Current keeper baseline before the latest rejected probe:

- `kilo_micro_substring_concat`
  - `C: instr=1,622,875 / cycles=483,822 / ms=3`
  - `Ny AOT: instr=629,360,804 / cycles=253,790,310 / ms=60`
- `kilo_micro_substring_only`
  - `C: instr=1,622,877 / cycles=484,658 / ms=2`
  - `Ny AOT: instr=1,669,729 / cycles=1,000,442 / ms=2`
- `kilo_kernel_small_hk`
  - `708 ms`

Current top symbols on the keeper artifact:

1. `insert_const_mid_fallback`
2. `nyash.string.substring_hii`
3. `string_span_cache_put`
4. `LocalKey::with`
5. `borrowed_substring_plan_from_handle`
6. `resolve_string_span_from_view`

Reading:

- `substring_only` is already close to C
- the dominant remaining gap is concentrated in the runtime fallback corridor

## What Already Exists

These are already true:

- MIR sink rewrite exists
- `ViewSpan`/borrowed-view classification exists
- cache store exists
- delete-oriented producer rewrite is already landed
- BoxShape cleanup around `concat`, `string_view`, and `host_handles` is already landed

So this is not a missing-recognizer problem.
It is a scope / owner / runtime-corridor problem.

## Rejected Probes

### 1. Transient runtime-private piecewise carrier

Tried:

- issue a transient piecewise box/handle from `insert_const_mid_fallback`
- then short-circuit `substring_hii` through that carrier

Result:

- `kilo_micro_substring_concat`
  - `Ny AOT: instr=1,027,840,243 / cycles=316,717,873 / ms=78`

Rejected reading:

- transient piecewise object birth / clone / allocation dominated the hot lane

### 2. Raw handle-keyed sticky memo shortcut

Tried:

- remember `source_handle/split/middle_ptr` behind the produced handle
- short-circuit the next `substring_hii`

Result:

- `kilo_micro_substring_concat`
  - `Ny AOT: instr=1,027,840,321 / cycles=315,379,190 / ms=80`

Rejected reading:

- it did not delete the hot body
- it only added another shortcut in front of the same corridor

### 3. Generic direct-build widening in `insert_const_mid_fallback`

Tried:

- on the generic non-empty `insert_const_mid_fallback` path
- read the source string in-session
- directly build the inserted string with one allocation
- skip the old `TextPlan`-based path

Result:

- exact front improved strongly:
  - `kilo_micro_substring_concat`
    - `Ny AOT: instr=474,559,696 / cycles=165,012,319 / ms=45`
- accept gate stayed healthy:
  - `kilo_micro_substring_only`
    - `Ny AOT: instr=1,669,350 / cycles=1,050,465 / ms=3`
- but whole guard regressed:
  - `kilo_kernel_small_hk: 789 ms`

Asm top on the rejected widening:

1. `nyash.string.substring_hii`
2. `insert_const_mid_fallback closure`
3. `borrowed_substring_plan_from_handle`
4. `LocalKey::with`
5. `__memmove_avx512_unaligned_erms`

Rejected reading:

- the direction helps the exact front
- but replacing the generic `insert_hsi` fallback body widens too far
- it is not corridor-local enough

## Current Working Reading

The next cut should not be:

- another recognizer
- another cache layer
- another sticky shortcut
- a wider generic `insert_const_mid_fallback` replacement

The next cut should be:

- corridor-local
- runtime-private
- single-session
- executor-local
- delete-oriented

Meaning:

- keep generic helper semantics unchanged for broad callers
- add a narrower runtime-private executor only for the already-proven active corridor
- keep generic borrowed truth in MIR
- keep materialization at the final consumer boundary only

## My Current Hypothesis

The clean design probably looks like this:

```text
.hako semantics/policy
  -> MIR proof: this corridor may stay borrowed until final consumer
  -> MIR rewrite target: route active front to a narrower executor lane
  -> Rust runtime: one single-session corridor-local executor
  -> final materialize once
  -> LLVM: inline / simplify / codegen
```

The key open question is scope:

**How do I make the runtime executor narrow enough that it serves the active front without accidentally turning into a generic `insert_hsi` optimization that hurts whole-kilo?**

## Questions

1. Is the current failure best described as **scope widening**, not a wrong optimization direction?

2. What is the cleanest way to represent the proof at MIR level so the runtime cut stays corridor-local?
   - consumer capability only?
   - corridor-local publication/materialization proof?
   - a narrower "single-session borrowed corridor" contract?

3. Where should the scope boundary live?
   - in MIR rewrite target selection?
   - in a runtime-private executor family selected by existing MIR metadata?
   - somewhere else?

4. How do I avoid this bad pattern:
   - exact front wins
   - helper body becomes generically faster
   - whole-kilo regresses because the helper widened too far

5. Should the next design be framed as:
   - a generic `piecewise_subrange_exec(...)` executor below the existing public ABI
   - but only reachable from a corridor-local MIR rewrite
   - while the old generic helper stays as cold adapter?

6. What is the cleanest owner split for that design?
   - what exactly belongs in `.hako`
   - what exactly belongs in MIR
   - what exactly belongs in Rust runtime
   - what exactly must stay out of LLVM metadata

7. What should the next task-card schema add to prevent this class of mistake?
   - I already have `front`, `accept gate`, `whole guard`, `proof delta`, `delete target`, `reject condition`
   - do I also need an explicit `scope lock` / `non-widening contract` field?

8. If you were rewriting this wave cleanly, how would you define:
   - the corridor-local executor
   - the generic cold adapter
   - the exact boundary where direct materialization is allowed

## What I Want In The Answer

Please structure the answer as:

- A. diagnosis of the latest failure
- B. clean scope model across `.hako -> MIR -> Rust runtime -> LLVM`
- C. recommended end-state architecture
- D. next 1 card in this exact repo
- E. explicit non-goals / reject triggers

Please be blunt about:

- whether this is fundamentally a scope problem
- whether `piecewise_subrange_exec(...)` is the right shape
- whether the current task-card OS is still missing a field that would have prevented this widening

Do not answer with generic "use cache / inline / attrs" advice.
I am explicitly trying to prevent a locally winning exact-front optimization from becoming a globally bad generic helper rewrite.
