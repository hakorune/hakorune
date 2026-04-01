`phase2111` explicit emit/link canaries are archived here.

- superseded by:
  - `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_ternary_collect_runtime_proof.sh`
  - `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_map_set_size_runtime_proof.sh`
- keep purpose:
  - replay the legacy `hostbridge.extern_invoke("env.codegen", "emit_object"/"link_object", ...)` lane on demand
  - preserve proof evidence while `emit_object_from_mir_json(...)` remains archive-later
- active direct proof-only keep now lives separately in:
  - `tools/smokes/v2/suites/integration/phase2044-llvmlite-monitor-keep.txt`
