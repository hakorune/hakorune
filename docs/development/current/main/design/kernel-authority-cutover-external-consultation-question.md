---
Status: Draft
Scope: external consultation prompt for remaining kernel family migration to `.hako`
Related:
- CURRENT_TASK.md
- docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
- docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md
- docs/development/current/main/design/string-transient-lifecycle-ssot.md
- docs/development/current/main/design/rep-mir-string-lowering-ssot.md
- lang/src/runtime/kernel/
- lang/src/runtime/numeric/
- lang/src/runtime/collections/
- lang/src/runtime/kernel/string/search.hako
- lang/src/runtime/kernel/numeric/matrix_i64.hako
- lang/src/runtime/collections/array_core_box.hako
---

# External Consultation Question: Kernel Authority Cutover / `.hako` Migration

## Context

I am migrating the remaining runtime kernel surface of a language implementation from Rust/C substrate into `.hako` policy owners.

The current state is already split:

- `lang/src/runtime/kernel/string/search.hako`
  - owns `find_index` / `contains` / `starts_with` / `ends_with` / `split_once_index`
- `lang/src/runtime/kernel/numeric/matrix_i64.hako`
  - owns the `MatI64.mul_naive` loop/body
- `lang/src/runtime/collections/array_core_box.hako`
  - still owns the `ArrayBox.length/len/size` observer path
- `lang/src/runtime/numeric/mat_i64_box.hako`
  - is a thin `new MatI64(rows, cols)` wrapper around the kernel owner

Current design direction:

- surface semantics remain `Everything is Box`
- `.hako` owns method contract / acceptance / control structure
- Rust/C substrate keeps raw leaf implementation, handle registry, allocation, GC/finalization, and boundary transport
- temporary backend-local lowering pilots are allowed only if they do not become new permanent Rust meaning owners

## Current Kernel Boundary

Relevant SSOT / docs:

- `docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md`
- `docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md`
- `docs/development/current/main/design/string-transient-lifecycle-ssot.md`
- `docs/development/current/main/design/rep-mir-string-lowering-ssot.md`

Current observed rule:

- meaning/policy should move to `.hako`
- substrate should stay thin and should not become the long-term owner of the language semantics
- widening should happen one kernel family at a time, with fixture + smoke + SSOT updates

## Current Kernel Families

Already moved or partially moved:

1. `string`
   - search/control surface has landed in `.hako`
2. `numeric`
   - `MatI64.mul_naive` loop/body landed in `.hako`
3. `array`
   - observer path is still in collections ring1
4. `map`
   - currently stays in collections ring1 and is not part of the active kernel lane

## Main Question

What is the cleanest way to continue the kernel authority cutover so that:

1. `.hako` becomes the durable owner of kernel policy and control structure
2. Rust does not become a new permanent meaning owner
3. raw leaf substrate stays thin and replaceable
4. future self-hosting does not get harder because we buried semantics in Rust

## Specific Questions

1. Which kernel family should be moved next?
   - `array`
   - further `numeric`
   - or another string-related control surface

2. What is the cleanest boundary between:
   - `.hako` policy/control structure
   - backend-local lowering substrate
   - raw leaf substrate

3. For kernels that are mostly observers:
   - should they stay in collections ring1 as wrappers for longer?
   - or should they get dedicated `.hako` kernel modules earlier?

4. For kernels that still need allocation or handles:
   - what is the right way to keep allocation / handle birth out of the policy owner?
   - should there be a dedicated `freeze` / `birth` boundary in the lowering contract?

5. If we want a temporary AOT-only lowering pilot:
   - what is the smallest shape that is safe to pilot?
   - how do we keep Rust from accumulating durable semantics while still making the pilot useful?

## What I Want In The Answer

Please classify the recommendation into:

- clearly safe
- safe only with an explicit contract/runtime change
- unsafe / likely wrong

I want a concrete recommendation for the order of the remaining kernel migrations, not a generic “move more code to `.hako`” answer.

Please also tell me:

- which parts should remain in Rust/C substrate for now
- which parts should move to `.hako` next
- where a `freeze` / `birth` boundary should be drawn so the design stays readable and self-host-friendly

