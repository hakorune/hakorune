# shared/backend

目的
- backend-zero の official `.hako` caller boundary をここに置く。
- `.hako` 側は backend orchestration だけを持ち、raw LLVM API は持たない。

Current owner
  - `llvm_backend_box.hako`
  - thin backend boundary の caller facade
  - `backend_recipe_box.hako`
  - caller-side compile/link recipe owner; prepare route/policy, but do not own transport calls
  - current `.hako` daily caller passes recipe payload explicitly via `BackendRecipeBox.compile_route_profile(...)` and then into `env.codegen.compile_json_path(...)`
  - `BackendRecipeBox.compile_route_profile(...)` validates the exact owner names and evidence labels before returning the profile, so `LlvmBackendBox` can stay transport-focused when calling `env.codegen.*`
  - `BackendRecipeBox` also names the current acceptance basis (`acceptance_policy`) so pure/compat classification does not drift back into C
  - `BackendRecipeBox` also names the current acceptance case (`acceptance_case`) so shape-specific evidence such as `ret-const-v1`, `hello-simple-llvm-native-probe-v1`, `runtime-data-array-get-missing-v1`, `runtime-data-string-length-ascii-v1`, `runtime-data-array-length-v1`, `runtime-data-array-push-v1`, `runtime-data-map-size-v1`, `runtime-data-array-has-missing-v1`, `runtime-data-map-has-missing-v1`, `runtime-data-map-get-missing-v1`, and `string-indexof-ascii-v1` stays visible in `.hako`
  - the canonical route profile shape is documented in `docs/development/current/main/design/backend-recipe-route-profile-ssot.md`
  - transport layers may still mirror those names to `HAKO_BACKEND_COMPILE_RECIPE` / `HAKO_BACKEND_COMPAT_REPLAY` when crossing the C boundary
  - final target は `LlvmBackendBox -> BackendRecipeBox -> hako_aot -> backend helper`
  - daily compile/link owner now stops directly at `env.codegen.compile_json_path(...)` / `env.codegen.link_object(...)`
  - shared compile/link helper lowering now reaches canonical `env.codegen.*` externs directly; daily compile/link does not depend on `hostbridge.extern_invoke(...)`
  - caller-side codegen request defaults are centralized in `src/config/env/llvm_provider_flags.rs::backend_codegen_request_defaults(...)`; compat bridges may mirror the same names, but daily owners stay explicit
  - MIR normalization (`schema_version: "1.0"` / `metadata.extern_c`) is owned by Rust backend boundary `src/host_providers/llvm_codegen.rs::normalize_mir_json_for_backend(...)`
  - `emit_object` remains compat keep for legacy/provider probes only
  - public first-cut contract:
    - `compile_obj(json_path)` -> object path or `null` with `[llvmbackend/*]`
    - `link_exe(obj_path, out_path, libs)` -> `1` or `null` with `[llvmbackend/*]`
      - non-empty `libs` is currently forwarded as a single extra-ldflags string
      - empty `libs` still falls back to `HAKO_AOT_LDFLAGS` under the C boundary

Non-goals
- legacy `llvm_ir/AotFacade` route をここへ混ぜない。
- libLLVM の広い surface を `.hako` へ見せない。

Pointers
- C helper:
  - `lang/c-abi/shims/hako_aot.c`
- boundary SSOT:
  - `docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md`
