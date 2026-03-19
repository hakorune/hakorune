# Shared Host Bridge

Responsibility:
- Keep a thin compatibility layer for host box operations used by shared modules.
- Delegate all actual host calls through `HostFacadeBox` (`loader` category).

Scope:
- `HostBridgeBox.box_new`
- `HostBridgeBox.box_call`
- convenience wrappers (`box_new0`, `box_call0`, `box_call1`) for existing call sites.
- `CodegenBridgeBox`
  - temporary bridge for `env.codegen.*`
  - path-based daily compile helper is `compile_json_path[_args]`
  - raw `emit_object[_args]` stays compat keep only
  - do not treat as final backend-zero daily caller stop-point
  - caller-side backend recipe defaults are centralized in `src/config/env/llvm_provider_flags.rs::backend_codegen_request_defaults(...)`; this bridge may mirror compat names, but daily callers should stay explicit at `LlvmBackendBox`
  - shared host/vm compile-link helpers now lower directly to canonical `env.codegen.*` extern calls; do not reintroduce `hostbridge.extern_invoke(...)` for daily backend compile/link routes
  - `HostFacadeBox` / `MirVmS0BoxcallExecBox` still keep the optional-arg `env.codegen.*` caller shape as legacy keep; new daily callers should stay explicit at `LlvmBackendBox`

Rules:
- Do not call `hostbridge.*` directly from new shared/vm/runtime code.
- If new host operation is needed, add selector in `lang/src/runtime/host/host_facade_box.hako` first.
- New backend-zero daily callers should stop at `lang/src/shared/backend/llvm_backend_box.hako`, not at `CodegenBridgeBox`.
