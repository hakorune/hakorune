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
  - args-only helpers are `emit_object_args` and `link_object_args`
  - shared payload normalization helper is `array_arg_or_null`
  - 1-arg convenience wrappers were removed so caller normalization stays explicit
  - do not treat as final backend-zero daily caller stop-point
  - caller-side backend recipe defaults are centralized in `src/config/env/llvm_provider_flags.rs::backend_codegen_request_defaults(...)`; this bridge may mirror compat names, but it does not own a hidden daily route and daily callers should stay explicit at `LlvmBackendBox`
  - this bridge now owns the legacy optional-arg `env.codegen.*` normalization used by `HostFacadeBox` / `MirVmS0BoxcallExecBox`, so the caller shape lives in one place instead of being duplicated
  - shared host/vm compile-link helpers now lower directly to canonical `env.codegen.*` extern calls; do not reintroduce `hostbridge.extern_invoke(...)` for daily backend compile/link routes
  - `HostFacadeBox` / `MirVmS0BoxcallExecBox` remain legacy keep callers, but they delegate their optional-arg normalization directly to `CodegenBridgeBox.*_args`; new daily callers should stay explicit at `LlvmBackendBox`, and omitted recipe/compat fields should be read as legacy compat entry only

Rules:
- Do not call `hostbridge.*` directly from new shared/vm/runtime code.
- If new host operation is needed, add selector in `lang/src/runtime/host/host_facade_box.hako` first.
- New backend-zero daily callers should stop at `lang/src/shared/backend/llvm_backend_box.hako`, not at `CodegenBridgeBox`.
