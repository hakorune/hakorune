---
Status: Active Packet
Date: 2026-04-17
Scope: phase-137x exact-front residual gap after publication-boundary + piecewise corridor keeper
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md
  - docs/development/current/main/design/optimization-task-card-os-ssot.md
---

# Phase137x Result Representation External Consult Packet

## Ask

Need a design review for the next step after the landed `piecewise` publication-boundary keeper.

The question is no longer `which route/helper should be used`.
The question is whether the remaining exact gap now requires a change in:

- runtime-private result representation
- runtime-private result ABI

while still keeping:

- `.hako -> MIR proof/publication -> runtime -> LLVM` authority order
- public handle-based surface stable
- string-only MIR dialect forbidden
- generic helper widening forbidden

## Current Facts

### Current keeper

- active front: `kilo_micro_substring_concat`
- accept gate: `kilo_micro_substring_only`
- whole guard: `kilo_kernel_small_hk`

Current keeper reread:

- `kilo_micro_substring_only`
  - `C: instr=1,622,874 / cycles=496,361 / ms=3`
  - `Ny AOT: instr=1,669,422 / cycles=1,066,057 / ms=3`
- `kilo_micro_substring_concat`
  - `C: instr=1,622,876 / cycles=477,384 / ms=3`
  - `Ny AOT: instr=260,618,242 / cycles=65,483,876 / ms=22`
- `kilo_kernel_small_hk`
  - `Ny AOT: 704 ms`

### Landed design

- MIR delete-oriented rewrite is landed:
  - `substring + const + substring -> insert_hsi + final substring_hii`
- publication boundary is landed:
  - active corridor publishes to runtime-private `nyash.string.piecewise_subrange_hsiii`
- active fast path is single-session and all-three-piece:
  - `piecewise_subrange total=300000`
  - `piecewise_subrange single_session_hit=300000`
  - `piecewise_subrange fallback_insert=0`
  - `piecewise_subrange all_three=300000`
- old substring route is gone:
  - `str.substring.route total=0`
  - `slow_plan=0`
  - `slow_plan_view_span=0`

### Current residual counters

- `birth.backend materialize_owned_total=300000`
- `birth.backend string_box_new_total=300000`
- `birth.backend arc_wrap_total=300000`
- `birth.backend handle_issue_total=300000`
- `stable_box_demand text_read_handle_latest_fresh=299999`

### Current top symbols

- `piecewise_subrange_hsiii_fallback closure`
- `__memmove_avx512_unaligned_erms`
- allocator samples are secondary

Reading:

- route/proof/publication are no longer the active blocker
- the remaining gap sits in final result birth:
  - final owned materialize
  - `StringBox` / `Arc` objectize
  - fresh handle issue

## Failed Local Probes

### Rejected transient piecewise carrier

- attempted a transient runtime-private piecewise box/handle carrier
- result:
  - `kilo_micro_substring_concat = 1,027,840,243 instr / 78 ms`
- reading:
  - transient carrier object birth/clone/allocation dominated

### Rejected sticky memo shortcut

- attempted a raw handle-keyed memo shortcut in front of the same hot helper
- result:
  - `kilo_micro_substring_concat = 1,027,840,321 instr / 80 ms`
- reading:
  - shortcut did not delete the helper body

### Rejected generic direct-build widening

- attempted generic non-empty `insert_hsi` direct-build widening
- result:
  - exact front improved:
    - `474,559,696 instr / 45 ms`
  - whole regressed:
    - `kilo_kernel_small_hk = 789 ms`
- reading:
  - the idea can help locally, but generic helper widening is too wide

### Rejected piecewise direct string birth

- attempted a runtime-private direct owned-string birth to bypass part of the generic materialize/objectize path
- result:
  - `kilo_micro_substring_concat = 261,219,009 instr / 23 ms`
- reading:
  - still a non-win on current handle/box representation

### Rejected `with_text_read_session_ready(...)`

- attempted a ready-only source read seam on the active `piecewise` fast path
- result:
  - `kilo_micro_substring_concat = 261,219,612 instr / 22 ms`
- reading:
  - source-read session entry is not the current bottleneck

## Current Design Reading

The active question is now:

- the public surface is handle-based
- the final result still becomes `owned String -> StringBox -> Arc -> handle`

We need to know whether:

1. this representation is still good enough and we should keep iterating locally
2. or the next step should be a dedicated runtime-private result representation / result ABI card

## Constraints

- keep `.hako -> MIR proof/publication -> runtime executor -> LLVM consumer`
- keep public ABI stable unless a later explicit card says otherwise
- no string-only MIR dialect
- no generic helper widening
- no runtime route re-recognition
- fail-fast

## Questions

1. Is the remaining exact gap now primarily a result-representation problem rather than a route/executor-shape problem?
2. If yes, what is the cleanest next design boundary:
   - runtime-private result representation only
   - runtime-private result ABI
   - or a broader public handle-surface rethink
3. Can the public handle-based surface stay fixed while the runtime-private result path stops forcing `owned String -> boxed handle` on the hot corridor?
4. What is the minimum generic design that keeps MIR generic and only changes runtime-private result birth?
5. If a result-ABI card is needed, where should the boundary live:
   - MIR proof/publication
   - runtime executor
   - host handle registry
6. Which part should remain cold adapter:
   - generic `materialize_owned_string`
   - generic `StringBox` objectize
   - generic handle issue
7. What should the next explicit card be:
   - owner
   - proof delta
   - publication boundary
   - delete target
   - preserves
   - reject conditions

## Desired Output

- A. diagnosis: is this now a representation/ABI problem or not
- B. recommended next architecture boundary
- C. what must stay fixed
- D. next explicit card
- E. non-goals

## Short Prompt

```text
Hakorune phase-137x perf design review, strict please.

Current exact front is no longer blocked by route selection or publication boundary.
Those are already landed.

Facts:
- `kilo_micro_substring_only` is near C
- `kilo_micro_substring_concat` is still far:
  - C: `1,622,876 instr / 477,384 cycles / 3 ms`
  - Ny AOT: `260,618,242 instr / 65,483,876 cycles / 22 ms`
- active path is already 100% on one runtime-private fast path:
  - `piecewise_subrange total=300000`
  - `single_session_hit=300000`
  - `fallback_insert=0`
  - `all_three=300000`
- old substring route is gone:
  - `str.substring.route total=0`
  - `slow_plan=0`
- remaining counters are:
  - `materialize_owned_total=300000`
  - `string_box_new_total=300000`
  - `arc_wrap_total=300000`
  - `handle_issue_total=300000`

Repeated local runtime-executor thin cuts are now non-wins.
Generic widening also regressed whole-kilo.

The design question is now:
- public surface is handle-based
- final result still becomes `owned String -> StringBox -> Arc -> handle`

Please answer:
1. Is the next blocker now result representation / result ABI rather than route design?
2. Can the public handle surface stay fixed while runtime-private result birth changes?
3. What is the minimum clean design for that?
4. What should the next explicit card be?

Constraints:
- keep `.hako -> MIR proof/publication -> runtime -> LLVM`
- no string-only MIR dialect
- no generic helper widening
- no runtime route re-recognition
- fail-fast

I want:
- diagnosis
- boundary recommendation
- next card
- non-goals
```
