---
Status: SSOT
Date: 2026-06-10
Scope: `kilo_micro_substring_concat` stable-length exact route lowering decision.
Related:
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/design/hako-optimization-toolbox-usability-ssot.md
  - docs/development/current/main/design/hako-inspect-scope-dump-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed_emitters.inc
  - lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed_substring_concat.inc
---

# Substring Concat Length Closed-Form Lowering SSOT

## Decision

`kilo_micro_substring_concat` has moved past source/MIR witness work.

The current residual is a lowering/codegen issue:

```text
source:
  OK. The benchmark expresses the stable length witness.

MIRBuilder:
  OK. The witness is not being lost at builder time.

MIR relation / string corridor:
  OK. StableLengthScalar exists and is consumed by the route.

route selection:
  OK. The exact route enters the length-only substring-concat emitter.

remaining owner:
  lowering / emitted IR / final assembly shape.
```

Do not add a new `.hako` annotation, MIRBuilder witness, or runtime string
representation change for this slice.

## Current Problem

The stable-length route currently calls:

```text
hako_llvmc_emit_substring_concat_len_ir(...)
```

That emitter has already avoided substring/concat byte materialization, but it
still emits a counted loop:

```text
for i in 0..loop_bound:
  acc += seed_len + middle_len
return seed_len + acc
```

Only `mem2reg` runs before `llc -O3`. That removes allocas, but it does not act
like the C compiler middle-end that can collapse this loop into a scalar closed
form. The remaining gap is therefore not evidence for source or MIRBuilder
limitations.

## Required Next Slice

Task id:

```text
SUBCONCAT-LEN-CLOSED-FORM-001
```

Implement a closed-form scalar emitter for the stable-length route:

```text
ret = seed_len + loop_bound * (seed_len + middle_len)
```

Preferred implementation:

```text
1. Add hako_llvmc_emit_substring_concat_len_closed_form_ir(...)
   in lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed_emitters.inc.

2. In lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed_substring_concat.inc,
   call the closed-form emitter when StableLengthScalar with
   window_contract=stop_at_merge is consumed.

3. Keep hako_llvmc_emit_substring_concat_len_ir(...) as the old loop-shaped
   fallback until the closed-form route is proven and guarded.
```

Do not widen the global LLVM `opt` pipeline in this first slice. Changing
`mem2reg` to a broader pipeline can affect unrelated exact seed fronts; the
closed-form emitter is local to this route.

## Semantics

Use the same wrapping integer semantics as the existing route. The existing
emitter does not use `nsw` / `nuw`, so the closed-form `mul i64` and `add i64`
must not introduce stronger overflow assumptions.

The route remains length-only:

```text
no string materialization
no runtime StringBox representation change
no substring view ABI change
no source-level workaround
```

## Evidence To Add

If a report/check surface is added, prefer fields that describe how the existing
truth was consumed by lowering:

```text
selected_kernel_route=substring_concat_len_closed_form
stable_length_scalar_consumed=1
substring_concat_len_closed_form_lowered_count=1
substring_concat_len_loop_ir_lowered_count=0
substring_concat_len_byte_shuffle_lowered_count=0
llvm_ir_loop_count=0
llvm_ir_alloca_count=0
```

The evidence should not infer new optimization truth from helper names. It
should render route metadata and lowering decisions.

## Verification

Minimum local checks:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/perf/build_perf_release.sh
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 3
PERF_MICROASM_RUNNER_MODE=direct KEEP_PERF_MICROASM_ARTIFACTS=1 bash tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_concat 'ny_main' 1
git diff --check
```

Expected shape:

```text
ny_main has no counted length-only loop for the stable-length exact route.
The hot body is scalar multiply/add/return or equivalent folded assembly.
```

## Stop Line

If closed-form lowering does not reduce the gap, the next question is the LLVM
compile recipe or generic SSA LLVM producer. Do not go back to `.hako`,
MIRBuilder, or runtime string representation until the emitted IR and assembly
prove that closed-form lowering is insufficient.
