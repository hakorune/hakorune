---
Status: Draft
Scope: external consultation prompt for the `substring -> concat3 -> length` transient/span-first wave
Related:
- CURRENT_TASK.md
- docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md
- docs/development/current/main/design/perf-optimization-method-ssot.md
- docs/development/current/main/design/box-identity-view-allocation-design-note.md
- docs/development/current/main/investigations/substring-concat-observer-fast-path-and-upstream-cut-2026-03-18.md
- crates/nyash_kernel/src/exports/string.rs
- crates/nyash_kernel/src/exports/string_view.rs
- src/box_trait.rs
- src/runtime/host_handles.rs
- benchmarks/bench_kilo_micro_substring_concat.hako
---

# External Consultation Question: Transient String Chain / Birth Boundary Design

## Context

I am optimizing a Rust runtime for a language implementation.

The current hot microbenchmark is a string loop shaped like this:

1. `left = text.substring(0, split)`
2. `right = text.substring(split, len)`
3. `out = left + "xx" + right`
4. `acc = acc + out.length()`
5. `text = out.substring(1, len + 1)`  // loop-carried state

Current benchmark source:

- `benchmarks/bench_kilo_micro_substring_concat.hako`

The design goal is to make the inner `substring -> concat3 -> length` chain more transient/span-first,
while keeping `text = out.substring(1, len + 1)` as the first escape boundary.

## Current Runtime Model

Relevant code:

- `crates/nyash_kernel/src/exports/string_view.rs`
  - `borrowed_substring_plan_from_handle(...)`
  - `StringViewBox`
  - `StringSpan`
- `crates/nyash_kernel/src/exports/string.rs`
  - `substring_hii`
  - `concat3_hhh`
  - `string_handle_from_owned(...)`
- `src/box_trait.rs`
  - `next_box_id()`
  - `BoxBase::new()`
- `src/runtime/host_handles.rs`
  - handle registry / allocation path

Current behavior:

- short substring results may materialize to `StringBox`
- mid substring results may become `StringViewBox`
- escaped values become handles and enter the registry
- object identity is correctness-bearing:
  - `BoxBase::new()` assigns a fresh `box_id`
  - live boxes must not share ids
  - GC / finalization / identity-sensitive paths depend on that

## Current Perf Reading

Whole-program baseline is still far from C:

- `kilo_kernel_small_hk`
  - `c_ms=79`
  - `py_ms=111`
  - `ny_aot_ms=804`
  - `ratio_c_aot=0.10`

Current `substring_concat` asm-top is dominated by:

1. `Registry::alloc`
2. `BoxBase::new`
3. `substring_hii`
4. `concat3_hhh`

Recent rejected experiment:

- explicit observer fast path in `string_len_from_handle(...)`
- isolated micro improved to `265893951 cycles / 68 ms`
- but stable whole-program median regressed to `1066 ms`

So I do not want generic “make length faster” advice.
The real problem seems to be object/handle birth density in the transient string chain.

## Design Constraint

I want to preserve these invariants:

1. object identity remains correctness-bearing
2. live boxes must not share `box_id`
3. GC/finalization correctness must not change
4. `StringViewBox` should not silently become a mere alias object
5. current flat short-slice policy (`<= 8 bytes`) should stay fixed in this wave

## Working Hypothesis

The clean direction seems to be:

- non-escaping transient string values should not birth box/handle objects
- only escaped / retained / observable values should birth boxes

Concretely:

- the inner `left`, `right`, `out`, and immediate `out.length()` chain should stay in a transient/span layer as long as possible
- the loop-carried `text = out.substring(...)` step is the first escape boundary and may still materialize / birth

## Main Question

What is the cleanest design for a runtime like this?

I want guidance on how to split the string pipeline into layers such as:

1. authority / contract layer
2. transient span/token layer
3. birth boundary layer
4. substrate layer (`StringBox`, `StringViewBox`, handle registry, GC)

Specifically:

1. Is “non-escaping primitive/transient chain does not birth; escaped value births” the right rule?
2. What is the cleanest internal representation for the transient layer?
   - borrowed span
   - internal token
   - deferred concat recipe
   - something else
3. Where should the escape boundary be drawn so the design stays understandable and debuggable?
4. How would you structure the code so these layers remain visible instead of mixing planning, escape classification, and birth execution?

## What I Want In The Answer

Please distinguish clearly between:

- clearly safe
- safe only with an explicit contract/runtime change
- unsafe / likely wrong

Please do not answer with generic allocation tips alone.
I want a design recommendation for a runtime where:

- object identity matters
- transient string chains are hot
- the goal is to reduce birth density without breaking the box/handle model

