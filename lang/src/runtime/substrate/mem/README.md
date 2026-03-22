# lang/src/runtime/substrate/mem — `hako.mem` Staging

Responsibilities:
- Lowest staged capability facade under `runtime/substrate/`.
- Future home for:
  - alloc / realloc / free
  - memcpy / memmove / memset / memcmp
  - alignment request vocabulary
- Native intrinsic lowering is allowed when implementation begins.

Current live surface:
- `alloc_i64`
- `realloc_i64`
- `free_i64`

Rules:
- Do not add len/cap policy here.
- Do not add typed pointer/span facade here.
- Do not add allocator state machine here.
- Keep this directory focused on the native memory keep bridge.

Non-goals:
- No allocator policy here yet.
- No `RawArray`/`RawMap` logic.
- No OS VM policy.
