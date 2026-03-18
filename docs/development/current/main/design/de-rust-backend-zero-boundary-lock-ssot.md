---
Status: SSOT
Decision: accepted
Date: 2026-03-14
Scope: backend-zero の最終アーキテクチャ境界を固定し、`native_driver.rs` が temporary seam から mainline owner へ誤昇格しないようにする。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-provisional-inventory-ssot.md
  - docs/development/current/main/design/backend-legacy-preservation-and-archive-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P0-BACKEND-ZERO-OWNER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ck/P1-NY-LLVMC-NATIVE-EMITTER-TASK-PACK.md
  - docs/development/current/main/phases/phase-29ck/P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md
  - docs/development/current/main/design/backend-recipe-route-profile-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - lang/README.md
  - lang/c-abi/README.md
  - lang/c-abi/shims/hako_aot.c
  - lang/src/shared/backend/llvm_backend_box.hako
  - crates/nyash-llvm-compiler/README.md
---

# De-Rust Backend-Zero Boundary Lock (SSOT)

## Purpose

- backend-zero の final shape を「`native_driver.rs` を育て切ること」ではなく、`.hako` から呼べる thin backend boundary に固定する。
- `native_driver.rs` は bootstrap / canary 用 temporary seam に限定し、長期 owner にしない。
- canonical ABI surface を増やさず、Core C ABI / TypeBox ABI v2 の 2 面固定を守る。

## 1. Boundary Lock

1. canonical ABI surface は引き続き 2 面だけである。
   - Core C ABI
   - TypeBox ABI v2
2. backend-zero で必要になる backend helper contract は、migration/tooling 用の thin boundary として扱う。
   - 第 3 の canonical semantic ABI にはしない。
3. `.hako` 側は libLLVM の広い API に直接結合しない。
   - `.hako` が持つのは backend orchestration と高レベル最適化である。
   - LLVM object/exe emission の詳細は thin boundary の内側へ閉じる。
4. `crates/nyash-llvm-compiler/src/native_driver.rs` は temporary seam であり、backend-zero の final owner ではない。
5. legacy `llvm_ir` script-builder / AotFacade route は archive keep であり、daily owner へ戻さない。

## 2. Final Shape

backend-zero の final target は次の形に固定する。

1. `.hako` compiler / optimizer
   - MIR generation
   - 高レベル最適化
   - backend call orchestration
2. thin backend boundary
   - first target paths:
     - `lang/src/shared/backend/llvm_backend_box.hako`
     - `lang/c-abi/shims/hako_aot.c`
   - expected responsibilities:
     - `MIR(JSON) -> object`
     - `object -> executable`
     - 必要なら diagnostics / option plumbing
3. backend implementation
   - `ny-llvmc`
   - optional FFI helper behind `hako_aot`
   - future non-Rust backend plugin / C-family implementation
4. runtime / plugin ABI
   - Core C ABI / TypeBox ABI v2 の 2 面固定を維持する

補足:
- `.hako` から raw `extern "C"` 相当の広い LLVM surface を直接叩くのが目的ではない。
- `.hako` から thin Box/C-ABI facade を呼び、その内側で object/exe emission を完結させるのが目的である。

## 3. Current Temporary Shape

2026-03-14 時点の current line はこうである。

1. mainline:
   - caller-facing route is `hakorune -> llvm_codegen boundary-first -> C ABI boundary -> backend helper/native boundary -> object/exe`
   - `crates/nyash-llvm-compiler/src/main.rs` now delegates input shaping into `src/compile_input.rs` plus emit/link driver dispatch into `src/driver_dispatch.rs`, and `src/driver_dispatch.rs` now further splits harness duties into `src/harness_driver.rs` plus link/finalize duties into `src/link_driver.rs`, so Rust thin-up is now at its stop line and the next move is exe optimization
   - `ny-llvmc` internal default driver now enters the boundary-owned lane first, not `Harness`
   - `src/host_providers/llvm_codegen.rs` default object path now also tries the direct C ABI boundary before any wrapper keep lane, and the parent file now delegates MIR normalization / transport helpers into `src/host_providers/llvm_codegen/normalize.rs` plus `src/host_providers/llvm_codegen/transport.rs`
   - `src/host_providers/llvm_codegen.rs::link_object_capi(...)` now forwards linker keeps directly into `hako_aot_link_obj(...)` instead of re-owning runtime archive / env ldflags synthesis on the Rust side
   - `lang/c-abi/shims/hako_llvmc_ffi.c` default compile/link exports now read as `hako_aot` forwarders, and compile fallback/link fallback disable recursive FFI inside the C owner
   - supported boundary compile seeds now try the pure C subset first; `apps/tests/mir_shape_guard/ret_const_min_v1.mir.json`, `apps/tests/hello_simple_llvm_native_probe_v1.mir.json`, `apps/tests/mir_shape_guard/string_length_ascii_min_v1.mir.json`, `apps/tests/mir_shape_guard/runtime_data_string_length_ascii_min_v1.mir.json`, `apps/tests/mir_shape_guard/string_indexof_ascii_min_v1.mir.json`, `apps/tests/mir_shape_guard/runtime_data_array_length_min_v1.mir.json`, `apps/tests/mir_shape_guard/runtime_data_array_push_min_v1.mir.json`, `apps/tests/mir_shape_guard/runtime_data_array_has_missing_min_v1.mir.json`, `apps/tests/mir_shape_guard/runtime_data_array_get_missing_min_v1.mir.json`, `apps/tests/mir_shape_guard/runtime_data_map_size_min_v1.mir.json`, `apps/tests/mir_shape_guard/runtime_data_map_has_missing_min_v1.mir.json`, and `apps/tests/mir_shape_guard/runtime_data_map_get_missing_min_v1.mir.json` are accepted without relying on `NYASH_NY_LLVM_COMPILER`
   - caller-side recipe seam now lives in `lang/src/shared/backend/backend_recipe_box.hako`; `.hako` daily compile now carries explicit `compile_json_path(..., "", "pure-first", "harness")` payload at that boundary via `compile_route_profile(...)`, and Rust transport mirrors those names to env only at the C handoff, so pure-first policy is owned by caller-side recipe payload rather than by ambient env mutation inside the C shim
   - `compile_route_profile(...)` also owns the visible `acceptance_policy` label (`boundary-pure-seed-matrix-v1`), so the current pure/compat acceptance basis is named in `.hako`, not inferred from transport glue
   - `compile_route_profile(...)` also owns the visible `acceptance_case` label for the current narrow evidence rows (`ret-const-v1`, `hello-simple-llvm-native-probe-v1`, `runtime-data-array-get-missing-v1`, `runtime-data-string-length-ascii-v1`, `runtime-data-array-length-v1`, `runtime-data-array-push-v1`, `runtime-data-map-size-v1`, `runtime-data-array-has-missing-v1`, `runtime-data-map-has-missing-v1`, `runtime-data-map-get-missing-v1`, `string-indexof-ascii-v1`, `string-length-ascii-v1`), so the shape-specific reason stays in `.hako` instead of drifting into the C shim
   - recipe-aware daily callers now prefer `hako_llvmc_compile_json_pure_first`; the generic `hako_llvmc_compile_json` export remains the default forwarder / historical compat surface
   - `HAKO_CAPI_PURE=1` remains only as a historical compat alias for phase2120-style pure packs
   - `lang/c-abi/shims/hako_aot_shared_impl.inc` compile command now also uses explicit `--driver boundary`, so the default `hako_aot` command route no longer advertises harness as the daily owner
   - explicit `HAKO_LLVM_EMIT_PROVIDER={llvmlite|ny-llvmc}` keeps remain replayable, but the wrapper path is no longer part of the default route
   - unsupported shapes now replay directly from `lang/c-abi/shims/hako_llvmc_ffi.c -> ny-llvmc --driver harness`, so `llvmlite` remains an indirect compat keep inside the boundary fallback lane
2. bootstrap seam:
   - `crates/nyash-llvm-compiler/src/native_driver.rs`
   - role:
     - Python 非依存 object/exe canary
     - direct runner parity の early evidence
3. future caller boundary seeds:
   - `lang/src/shared/backend/llvm_backend_box.hako`
   - `lang/c-abi/shims/hako_aot.c`
4. parked legacy route:
   - `lang/src/llvm_ir/archive/legacy_script_builder/**`
   - reason:
     - historical script-builder / AotFacade route
     - no longer the preferred caller target for backend-zero

ここで重要なのは、
`native_driver.rs` が green でも backend-zero の final architecture が確定したことにはならないし、
`ny-llvmc` の current internal default と `llvm_codegen.rs` default object path が boundary-first になっても unsupported shapes を compat fallback へ流し続ける限り llvmlite は indirect in-path に残るし、
`native_driver.rs` をその代替 default に上げても final shape から外れる、
という点だよ。

## 4. Temporary Seam Rule

`native_driver.rs` に許されるのは次だけだよ。

1. `BE0-min1..5` の bootstrap evidence
2. old native canary / app-seed parity の限定 replay
3. fail-fast で route/seam を観測すること

`native_driver.rs` に許されないのは次だよ。

1. `.hako` caller の final owner になること
2. backend-zero done の acceptance owner になること
3. Core C ABI / TypeBox ABI v2 と並ぶ恒久 boundary として扱われること

## 5. Retirement Trigger

`native_driver.rs` を retire あるいは canary-only へ降格してよいのは、次を満たした時だけだよ。

1. `.hako` caller が thin backend boundary へ到達している
2. thin backend boundary が `MIR(JSON) -> object/exe` を daily route で replay できる
3. daily caller が `native_driver.rs` を mainline owner として必要としない
4. llvmlite demotion plan と矛盾しない

## 6. Fixed Order

1. boundary lock をこの文書で固定する
2. bootstrap seam evidence（`BE0-min1..5`）を積む
3. thin backend boundary cutover pack を作る
   - `.hako` call site
   - `lang/c-abi` helper
   - runner/build caller wiring
   - legacy `llvm_ir` route archive / compat keep rule
4. `native_driver.rs` を canary-only へ降格する
5. llvmlite demotion / optimization handoff を mainline 側で固定する
6. both `Harness` and `Native` stay explicit keep lanes only

## 7. Non-goals

1. `native_driver.rs` を final native emitter owner にすること
2. backend-zero のために第 3 canonical ABI を増やすこと
3. `.hako` 側へ libLLVM API の広い surface を露出すること
4. `Cranelift keep` を reopen すること
5. archived `llvm_ir` script-builder route を silently daily route へ戻すこと
6. backend-zero 完了を理由に Rust / llvmlite source を即 delete すること

## 8. Preservation Before Retirement

Rust backend lane と Python/llvmlite lane は、backend-zero 後も preservation-first で扱う。

1. current repo から retire してよいのは external archive repo が ready の時だけ
2. archive repo には source snapshot / platform artifacts / checksums / restore docs を置く
3. Windows / Ubuntu/Linux / macOS artifact は少なくとも preservation target に含める
4. iOS lane を実運用する場合は iOS deliverable も preservation target に含める
5. archive repo と release bundle が揃うまで、この repo 側では demote はしても delete はしない
