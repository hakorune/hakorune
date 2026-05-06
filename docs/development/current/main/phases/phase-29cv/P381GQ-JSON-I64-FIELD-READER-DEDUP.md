# P381GQ JSON i64 Field Reader Dedup

Date: 2026-05-06
Scope: deduplicate C shim required `yyjson` i64 field reads.

## Context

Several `lang/c-abi/shims/*.inc` files carried local helpers with the same
shape:

```text
obj + required field key -> numeric yyjson value -> signed i64
```

The call sites still need policy-specific validation around the field read, but
the required numeric read itself was repeated in each shim.

## Change

- Added `hako_llvmc_json_read_i64_field(...)` to
  `hako_llvmc_ffi_common.inc`.
- Replaced local duplicate required-i64 readers in generic method, MIR call,
  map lookup fusion, and module leaf function emit shims.
- Kept `lowering_plan_read_i64_field(...)` as the lowering-plan local seam, now
  delegating to the common helper.

## Result

The shared C ABI shim seam now owns the pure required-i64 JSON field read. Route
semantics, optional-reader behavior, and policy-specific validation remain local
to their existing shims.

This is source-owner cleanup only. It does not add a compiler acceptance shape
and does not change emitted Program(JSON), lowering-plan JSON, or route metadata.

Diff size:

```text
9 files changed, 49 insertions(+), 111 deletions(-)
```

## Validation

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_llvm.sh
bash tools/checks/stage1_emit_program_json_runtime_helper_guard.sh
cargo test --release global_call_route_plan -- --nocapture
cargo test --release generic_method_route_plan -- --nocapture
```
