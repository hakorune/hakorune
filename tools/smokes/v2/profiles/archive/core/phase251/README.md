`phase251` legacy extern-lowering canaries are quarantined here.

- archived surfaces:
  - `selfhost_mir_extern_codegen_basic_vm.sh`
  - `selfhost_mir_extern_codegen_basic_provider_vm.sh`
- why archived:
  - neither script is in active suites
  - both are hard-skipped under the current Stage-B / quick-profile setup
  - no root-first selfhost lowering proof replaces this exact legacy `env.codegen.emit_object` lowering surface yet
- keep purpose:
  - preserve the legacy lowering probe as replay evidence
  - keep the blocker visible while `extern_provider.hako` remains compat-only keep
- active direct proof-only keep now lives separately in:
  - `tools/smokes/v2/suites/integration/compat/llvmlite-monitor-keep.txt`
- archive replay bundle:
  - `tools/smokes/v2/suites/archive/phase29x-legacy-emit-object-evidence.txt`
