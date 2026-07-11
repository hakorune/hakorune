# Phase 179x SSOT: StringKernelPlan export and exact-seed retirement

## Goal

Replace the remaining benchmark-shaped string exact-seed bridge with a metadata-first backend consumer path.

The immediate target is not full retirement.
The immediate target is to make the missing contract explicit:

- current string metadata already exists in MIR JSON
- backend still lacks a dedicated exported `StringKernelPlan`
- exact seed still reconstructs semantics from raw block shape

## Fixed Reading

What is already true:

- canonical MIR is the only public MIR truth
- `string_corridor_facts`
- `string_corridor_relations`
- `string_corridor_candidates`
  are already exported in MIR JSON

What is still missing:

- one backend-consumable plan schema for the hot string loop family
- one metadata-first consumer path in `hako_llvmc_ffi_string_loop_seed.inc`

## Minimal Plan Shape

The first `StringKernelPlan` must stay narrow and backend-consumable.

Allowed fields:

- `version`
- `family`
- `corridor_root`
- `source_root`
- `parts`
  - `slice`
  - `const`
  - `slice`
- `known_length`
- `retained_form`
- `barriers`
- `consumer`
- `direct_kernel_entry`
- `legality`
  - `byte_exact`
  - `no_publish_inside`
  - `materialize_at_exit`

Not allowed in the plan:

- raw block counts
- instruction indices
- `interesting[n]`
- `divisor`
- `split`
- helper names as semantic truth

Those remain either:

- historical bridge logic
- or emit-time derived details

## Ownership

Semantic ownership stays:

1. `.hako` / canonical MIR vocabulary
2. MIR facts / relations / candidate formation
3. MIR JSON export of backend-consumable plan
4. C shim as consumer-only emitter

The C shim may still reject a plan on low-level legality grounds.
It may not rebuild the plan semantics from raw MIR shape once the plan exists.

## Sequence

1. lock the schema in docs
2. export the minimal plan in MIR JSON
3. make `string_loop_seed` metadata-first with shape-fallback
4. prove exact keeper parity
5. only then shrink old shape matchers

## Do Not Do

- do not push route policy into LLVM/runtime helpers
- do not widen helper-name semantic recovery
- do not add another benchmark-only exact matcher
- do not mix this corridor with DCE
