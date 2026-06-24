---
Status: Landed
Date: 2026-06-15
Task: MIR-CALL-COMPARE-OPERAND-FORWARDING-MEASUREMENT-001
Scope: Remeasure object-lifecycle body timing after the CompareOperand
  forwarding implementation from 296x-746.
Related:
  - docs/development/current/main/phases/phase-296x/296x-746-MIR-CALL-COMPARE-OPERAND-FORWARDING-IMPLEMENTATION-001.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
  - tools/allocator/hako_exe_memory_runner.sh
  - tools/allocator/c_mimalloc_explicit_runner.sh
  - tools/allocator/hako_mimalloc_object_lifecycle_body_timing_pair_adapter.py
---

# MIR-CALL-COMPARE-OPERAND-FORWARDING-MEASUREMENT-001

## Result

```text
output_contract=hako-mimalloc-mir-call-compare-operand-forwarding-measurement-v0
source_evidence=296x-746
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
measurement_tmpdir=/tmp/hakorune_row747_measure.QHGPbl
llvm_string_const_declare_deduped=1
llvm_duplicate_declare_error_fixed=1
hako_runner_summary=ok
c_runner_summary=ok
pair_adapter_summary=ok
hako_body_timing_available=1
c_body_timing_available=1
hako_body_timing_repeat_kind=workload-body-env-now-ms-v0
c_body_timing_repeat_kind=workload-body-monotonic-v0
in_process_operation_repeat=8192
allocation_count=524288
free_count=524288
requested_bytes=272416768
hako_body_elapsed_ns=365000000
c_body_elapsed_ns=3727908
body_elapsed_ratio=97.910
hako_external_elapsed_ms=370
c_external_elapsed_ms=1
winner_claim=0
next_optimization_allowed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Measurement Unblocker

The first remeasurement attempt failed before producing Hako evidence because
the generated LLVM contained two declarations for
`nyash.box.from_i8_string_const`.

The duplicate came from two independent declaration emit conditions in the
generic lowering prescan path:

```text
object-storage alignment-result readiness
string-const / same-module-function-definition readiness
```

The fix keeps one declaration site and folds all readiness conditions into it.
This is a declaration-surface cleanup only:

```text
product_default_changed=0
runtime_behavior_changed=0
mirbuilder_behavior_changed=0
allocator_provider_changed=0
```

After rebuilding `target/release/libhako_llvmc_ffi.so`, the same Hako/C body
timing pair completed successfully.

## Decision

The MIR shape target from 296x-746 is still satisfied, but the body timing
measurement does not support a winner claim.

```text
MIR target:
  post_compare_operand_forwarding_candidate_count=0

body timing:
  body_elapsed_ratio=97.910
  winner_claim=0
```

This row therefore closes the CompareOperand forwarding implementation as a MIR
shape cleanup, not as a measured body-time win.

## Stop Line

```text
do not broaden LocalSSA forwarding from this measurement
do not claim a winner from MIR shape cleanup
do not change product NyRT defaults
do not change allocator provider / hook / global allocator state
do not treat Type ABI / hako_check as execution truth
```

## Next

```text
POST-MIR-CALL-COMPARE-OPERAND-FORWARDING-OWNER-REFRESH-001:
  refresh the current post-measurement owner
  classify whether the remaining body gap is runtime/object boundary,
  generated runtime helper boundary, measurement boundary, or another
  compiler-lowering family
  keep implementation closed until a new owner has medium/high confidence
```
