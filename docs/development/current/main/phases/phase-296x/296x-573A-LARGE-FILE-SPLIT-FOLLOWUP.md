---
Status: Done
Date: 2026-06-07
Scope: large-file split follow-up.
Related:
  - src/cli/provider_package_hako_derived_build.rs
  - src/cli/provider_package_hako_derived_build/contract.rs
  - src/mir/fastmem_access_plan.rs
  - src/runner/mir_json_emit/metadata.rs
---

# 296x-573A Large-File Split Follow-up

## Purpose

Record the current state of the large-file split feedback and land one
behavior-preserving split without mixing it into MIM-PORT-FMEM-075 body
migration selection.

## Current Inventory

The reported line counts were stale for the current tree. The high-risk files
are already split into module/test seams:

```text
src/mir/builder/fastmem.rs                    236 lines, tests in fastmem/tests/*
src/mir/direct_array_access_plan.rs           460 lines, tests in direct_array_access_plan/tests/*
src/stage1/program_json_v0/lowering.rs        510 lines, tests in lowering/tests.rs
src/mir/function/types.rs                     233 lines
src/runner/mir_json_emit/metadata.rs          423 lines, metadata helpers already split
src/cli/provider_package_hako_derived_build.rs 394 lines after this split
src/mir/fastmem_access_plan.rs                215 lines, planner slices in fastmem_access_plan/*
```

## Landed Slice

`provider_package_hako_derived_build` no longer owns the function-table contract
hash JSON construction inline. The hash input and builder now live in:

```text
src/cli/provider_package_hako_derived_build/contract.rs
```

This keeps the CLI command as the orchestration owner and moves contract-hash
schema construction behind a package-build contract seam.

## Boundaries Kept

```text
behavior-preserving split only
no provider activation change
no generated C template change
no FastMemory MemOp or report/check behavior change
MIM-PORT-FMEM-075 remains the next body migration selection row
```

## Verification

```text
cargo check -q --bin hakorune
cargo test -q cli::provider_package_hako_derived_build
```
