# C‑ABI Bridge v0 (Phase 20.38)

Purpose
- Provide a minimal, guarded bridge from Hakorune VM to Rust extern providers without changing behavior.
- Keep default OFF; use tags for observability, return empty string to keep rc=0.

Scope (v0)
- Supported names (Extern):
  - `env.mirbuilder.emit` — program_json → mir_json
  - `env.codegen.emit_object` — mir_json → object path
- Call shapes:
  - Hako provider: `HakoruneExternProviderBox.get(name, arg)`
  - Legacy global: `hostbridge.extern_invoke(name, method, [arg])`

Behavior
- When `HAKO_V1_EXTERN_PROVIDER=1` (provider ON):
  - Hako provider returns empty string (`""`), rc remains 0.
- When `HAKO_V1_EXTERN_PROVIDER_C_ABI=1` (C‑ABI tag ON):
  - Provider prints tags to stderr: `[extern/c-abi:mirbuilder.emit]`, `[extern/c-abi:codegen.emit_object]`.
  - Return remains empty string (rc=0)。

Toggles
- `HAKO_V1_EXTERN_PROVIDER=1` — enable provider path (default OFF).
- `HAKO_V1_EXTERN_PROVIDER_C_ABI=1` — emit C‑ABI tags (default OFF).

Verify
- Use Hakorune primary path (`HAKO_VERIFY_PRIMARY=hakovm`)
- Pass JSON via env: `NYASH_VERIFY_JSON`
- rc extraction: last numeric line

Rollback/Disable
- Unset `HAKO_V1_EXTERN_PROVIDER` (and `HAKO_V1_EXTERN_PROVIDER_C_ABI`) to restore pure stub behavior.

Notes
- v0 is intentionally minimal and behavior‑preserving. v1 may return real values and propagate errors under flags.

