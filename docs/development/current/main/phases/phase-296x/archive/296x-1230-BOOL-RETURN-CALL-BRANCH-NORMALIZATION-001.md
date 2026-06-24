---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Normalize bool-returning user/global call results before branch/not use on EXE/AOT.
Related:
  - apps/rust-subset-to-hako/probes/regression/bool_return_call_branch_probe.hako
  - apps/rust-subset-to-hako/probes/regression/schema_bool_shape_probe.hako
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
  - apps/rust-subset-to-hako/STATUS.md
---

# BOOL-RETURN-CALL-BRANCH-NORMALIZATION-001

## Problem

User/global function calls return through the current i64 ABI. When the
lowering plan marks the target return type as bool/i1, later branch and `not`
shapes expect an i1 register.

Before this row, a bool-returning call could produce IR shaped like:

```text
%rN = call i64 @"..."
br i1 %rN, ...
```

That is invalid LLVM IR because `%rN` is defined as i64 but consumed as i1.

## Decision

Normalize bool-returning call results at the C ABI global-call lowering seam.

```text
truth owner=lowering_plan target_return_type
normalization=i64 ABI result -> icmp ne i64 raw, 0 -> i1 register
backend_consumer=branch/not may read i1
```

This keeps source syntax and converter code out of the fix. The decision is
not based on function names.

## Result

```text
bool_return_call_branch_normalization_enabled=1
source_function_name_special_case_count=0
schema_helper_special_case_count=0
abi_return_shape=i64
normalized_register_shape=i1
```

Regression probes:

```text
apps/rust-subset-to-hako/probes/regression/bool_return_call_branch_probe.hako
apps/rust-subset-to-hako/probes/regression/schema_bool_shape_probe.hako
```

## Reproduction

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe /tmp/bool_return_call_branch_probe apps/rust-subset-to-hako/probes/regression/bool_return_call_branch_probe.hako
/tmp/bool_return_call_branch_probe
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe /tmp/schema_bool_shape_probe apps/rust-subset-to-hako/probes/regression/schema_bool_shape_probe.hako
/tmp/schema_bool_shape_probe
```

Expected:

```text
bool.return=ok
schema.bool=ok
Result: 0
```

## Stop Lines

```text
do not special-case schema helper names
do not make bool-return calls emit i1 ABI directly in this row
do not change .hako converter logic for this backend shape
do not re-enable VM product route
```

## Contract

```text
output_contract=bool-return-call-branch-normalization-v0

return_false_call_branch_probe=ok
return_true_call_not_probe=ok
schema_bool_shape_probe=ok
target_return_type_owner=lowering_plan
abi_i64_to_i1_normalization=1
source_name_branch_count=0

summary=ok
```
