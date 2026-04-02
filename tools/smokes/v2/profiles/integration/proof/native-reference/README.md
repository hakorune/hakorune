# Phase 2120 Native Reference Bucket

This bucket is the semantic home for the phase2120 native backend reference canaries.
It is proof-only coverage, not owner code.

## Bucket

- `native_backend_binop_add_canary_vm.sh`
- `native_backend_branch_eq_canary_vm.sh`
- `native_backend_compare_eq_canary_vm.sh`
- `native_backend_compare_lt_canary_vm.sh`
- `native_backend_return42_canary_vm.sh`

## Canonical Manifest

- `tools/smokes/v2/suites/integration/proof/native-reference.txt`

## Runner

Use:

```bash
./tools/smokes/v2/run.sh --profile integration --suite proof/native-reference
```

- `tools/smokes/v2/profiles/integration/proof/native-reference/run_native_reference_bucket.sh`

These are minimal native-reference canaries. They stay proof-only and do not define a daily owner.
