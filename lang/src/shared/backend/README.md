# shared/backend

目的
- backend-zero の official `.hako` caller boundary をここに置く。
- `.hako` 側は backend orchestration だけを持ち、raw LLVM API は持たない。

Current owner
  - `llvm_backend_box.hako`
  - thin backend boundary の caller facade
  - `backend_recipe_box.hako`
  - caller-side compile recipe and evidence owner; prepare route/policy, but do not own transport calls
  - `backend_daily_owner_policy_box.hako`
  - narrow allowlist helper for `.hako ll emitter` daily-owner selection; no route/profile assembly, no transport
  - compare/debug residue for `.hako ll emitter` bridge now lives in `src/host_providers/llvm_codegen/ll_emit_bridge.rs`; LLVM tool execution is split further into `src/host_providers/llvm_codegen/ll_tool_driver.rs`; compare proof itself is archive-suite only
  - `ll_emit/**`
  - explicit compare/debug bridge plus narrow daily owner for flipped boundary shapes
  - `ll_emit/call_policy_box.hako`
  - narrow method-selection policy for `RecipeFactsV0Box`; keep symbol choice out of the facts walker
  - `ll_emit/call_selector_box.hako`
  - narrow emit-side direct-call fallback selector; keep push/set rescue out of `LlTextEmitBox`
  - shared non-empty validation helper also lives here so `LlvmBackendBox` can stay transport-focused without duplicating input guards
  - `MirRootHydratorBox` and `MirBuilderBox.emit_root_from_{program_json,source}_v0(...)` are now the compat root entry for daily backend callers
  - flipped daily profiles now hydrate MIR(JSON) into a root, run `RecipeFactsV0Box -> LlTextEmitBox`, and cross the Rust tool seam via `env.codegen.compile_ll_text(...)`
  - launcher/mainline compile is now root-first; `compile_json_path(...)` is legacy/compare/archive only and no longer the daily transport
  - explicit compare callers may use `BackendRecipeBox.compile_compare_profile(...)` and `LlvmBackendBox.compile_obj_compare_hako_ll(...)` to exercise the `.hako ll emitter` bridge without changing the default owner; the proof smoke is archived out of the active suite
  - daily `compile_route_profile(...)` now keeps legacy `pure-first + compat_replay=none` for unflipped shapes, while `ret_const_min_v1`, `bool_phi_branch_min_v1`, `hello_simple_llvm_native_probe_v1`, `string_length_ascii_min_v1`, `string_indexof_ascii_min_v1`, and `string_concat3_extern_min_v1` use `hako-ll-min-v0` as narrow daily owner
  - `BackendRecipeBox.compile_route_profile(...)` validates the exact owner names and evidence labels before returning the daily profile, so `LlvmBackendBox` can stay transport-focused when calling `env.codegen.*`
  - `BackendRecipeBox` also names the current acceptance basis (`acceptance_policy`) so pure/compat classification does not drift back into C
  - `BackendRecipeBox` also names the current acceptance case (`acceptance_case`) so shape-specific evidence such as `ret-const-v1`, `hello-simple-llvm-native-probe-v1`, `runtime-data-array-get-missing-v1`, `runtime-data-string-length-ascii-v1`, `runtime-data-array-length-v1`, `runtime-data-array-push-v1`, `runtime-data-map-size-v1`, `runtime-data-array-has-missing-v1`, `runtime-data-map-has-missing-v1`, `runtime-data-map-get-missing-v1`, `array-string-indexof-branch-v1`, `array-string-indexof-cross-block-select-v1`, `array-string-indexof-interleaved-branch-v1`, `array-string-indexof-interleaved-select-v1`, `array-string-indexof-select-v1`, `string-indexof-ascii-v1`, `string-length-ascii-v1`, and `method-call-only-small-compat-v1` stays visible in `.hako`
  - the canonical route profile shape is documented in `docs/development/current/main/design/backend-recipe-route-profile-ssot.md`
  - transport layers may still mirror those names to `HAKO_BACKEND_COMPILE_RECIPE` / `HAKO_BACKEND_COMPAT_REPLAY` when crossing the C boundary; route evidence is now carried by root-first compile profiles instead of the retired route-env helper
  - final target は `LlvmBackendBox -> BackendRecipeBox -> .hako ll emitter -> env.codegen.compile_ll_text(...) -> opt/llc` で、legacy C shim は compare/compat keep へ後退する
  - `.hako ll emitter` compare/debug templating residue is now folded into `ll_emit_bridge.rs`, and `.ll` tool execution is isolated in `ll_tool_driver.rs`; remaining cleanup is compare bridge retirement / archive decisions, not env or template plumbing
  - current daily compile/link owner is now split:
    - flipped `.hako ll emitter` profiles stop at `env.codegen.compile_ll_text(...)` / `env.codegen.link_object(...)`
    - launcher root-first daily path no longer stops at `env.codegen.compile_json_path(...)`
  - shared compile/link helper lowering now reaches canonical `env.codegen.*` externs directly; daily compile/link does not depend on `hostbridge.extern_invoke(...)`
  - caller-side codegen request defaults are centralized in `src/config/env/llvm_provider_flags.rs::backend_codegen_request_defaults(...)`; compat bridges may mirror the same names, but daily owners stay explicit
  - MIR normalization (`schema_version: "1.0"` / `metadata.extern_c`) is owned by Rust backend boundary `src/host_providers/llvm_codegen.rs::normalize_mir_json_for_backend(...)`
  - `emit_object` remains compat keep for legacy/provider probes only
  - public first-cut contract:
    - `compile_obj(json_path)` -> object path or `null` with `[llvmbackend/*]`
    - `compile_obj_root(root, evidence_json_path)` -> object path or `null` with `[llvmbackend/*]`
    - `link_exe(obj_path, out_path, libs)` -> `1` or `null` with `[llvmbackend/*]`
      - non-empty `libs` is currently forwarded as a single extra-ldflags string
      - empty `libs` still falls back to `HAKO_AOT_LDFLAGS` under the C boundary

Non-goals
- legacy `llvm_ir/AotFacade` route をここへ混ぜない。
- libLLVM の広い surface を `.hako` へ見せない。
- `ll_emit/**` compare lane を daily owner に暗黙昇格しない。
- legacy delete/archive 候補を ledger なしで消さない。

Pointers
- C helper:
  - `lang/c-abi/shims/hako_aot.c`
- boundary SSOT:
  - `docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md`
 - legacy ledger:
   - `docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md`
