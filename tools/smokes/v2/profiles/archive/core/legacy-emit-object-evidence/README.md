# Legacy Emit Object Evidence Bundle

This directory is the archive replay bundle for legacy emit_object evidence.
It is replay-only coverage, not owner code.

## Bundle

- `s3_link_run_llvmcapi_ternary_collect_canary_vm.sh`
- `s3_link_run_llvmcapi_map_set_size_canary_vm.sh`
- `selfhost_mir_extern_codegen_basic_vm.sh`
- `selfhost_mir_extern_codegen_basic_provider_vm.sh`

## Why This Bundle Exists

- `phase2111` carries the archived emit/link canaries.
- `phase251` carries the archived selfhost lowering probes.
- both are replay evidence for the legacy `emit_object` lane.
- `phase29ck` provides the exact root-first replacements for the `phase2111` pair.
- `extern_provider.hako` still lacks an exact root-first lowering proof, so the `phase251` pair stays archived evidence.

## Canonical Manifest

- `tools/smokes/v2/suites/archive/legacy-emit-object-evidence.txt`

## Runner

- `tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/run_all.sh`

Use:

```bash
./tools/smokes/v2/run.sh --profile archive --suite legacy-emit-object-evidence
```
