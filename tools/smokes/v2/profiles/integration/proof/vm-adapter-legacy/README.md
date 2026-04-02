# Phase 2120 VM Adapter Legacy Bucket

This bucket is the semantic home for the phase2120 VM adapter canaries.
It is proof-only coverage, not owner code.

## Bucket

- `s3_vm_adapter_array_len_canary_vm.sh`
- `s3_vm_adapter_array_length_alias_canary_vm.sh`
- `s3_vm_adapter_array_size_alias_canary_vm.sh`
- `s3_vm_adapter_array_len_per_recv_canary_vm.sh`
- `s3_vm_adapter_map_size_struct_canary_vm.sh`
- `s3_vm_adapter_register_userbox_length_canary_vm.sh`
- `s3_vm_adapter_map_len_alias_state_canary_vm.sh`
- `s3_vm_adapter_map_length_alias_state_canary_vm.sh`

## Canonical Manifest

- `tools/smokes/v2/suites/integration/proof/vm-adapter-legacy.txt`

## Runner

Use:

```bash
./tools/smokes/v2/run.sh --profile integration --suite proof/vm-adapter-legacy
```

- `tools/smokes/v2/profiles/integration/proof/vm-adapter-legacy/run_vm_adapter_legacy_cluster.sh`

This bucket stays proof-only. It is not a daily owner lane.
