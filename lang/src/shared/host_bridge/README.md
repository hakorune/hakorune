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
  - do not treat as final backend-zero daily caller stop-point

Rules:
- Do not call `hostbridge.*` directly from new shared/vm/runtime code.
- If new host operation is needed, add selector in `lang/src/runtime/host/host_facade_box.hako` first.
- New backend-zero daily callers should stop at `lang/src/shared/backend/llvm_backend_box.hako`, not at `CodegenBridgeBox`.
