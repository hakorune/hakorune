---
Status: SSOT
Decision: provisional
Date: 2026-03-29
Scope: `stage2` の主体を `.hako` に寄せつつ、`.inc` を thin shim に収束させる owner / substrate boundary を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
  - docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/final-metal-split-ssot.md
  - lang/README.md
  - lang/c-abi/shims/README.md
---

# Stage2 `.hako` Owner vs `.inc` Thin Shim (SSOT)

## Goal

- stage2 の主体を `.hako` に寄せる。
- `.inc` は ABI shaping / marshal / fail-fast の thin shim に薄化する。
- native は metal keep として残し、ABI / alloc / GC / TLS / atomic / backend emission の最終 leaf だけを担う。
- 評価軸は行数ではなく owner 比率で読む。

## Fixed Reading

### `.hako semantic owner`

- meaning
- policy
- route
- acceptance
- control structure
- box orchestration
- string / collection low-level algorithm control

### `.hako capability substrate`

- `hako.value_repr`
- `hako.mem`
- `hako.buf`
- `hako.ptr`
- `RawArray`
- `RawMap`
- `hako_alloc` policy/state rows

### `.inc thin shim`

- ABI I/O shaping
- owned / borrowed boundary preservation
- fail-fast and boundary checks
- symbol stub / marshal
- backend entry glue

### native metal keep

- LLVM/backend emission
- final ABI entry stubs
- actual alloc/free/realloc backend
- OS VM glue
- handle registry / slot table
- object layout
- raw copy / flatten
- GC hooks / root trace / barrier
- platform TLS / atomic fallback

## Current Truth

- Current `lang/c-abi/shims/*.inc` are not yet pure thin shims.
- `hako_llvmc_ffi_common.inc` is closest to a thin shim + native support bundle.
- `hako_llvmc_ffi_string_concat_match.inc` is mostly compiler-state / semantic placement owner.
- `hako_llvmc_ffi_string_concat_lowering.inc` now wraps the concat emit seam and is no longer the main owner surface.
- `hako_llvmc_ffi_string_concat_emit.inc` now holds the string concat emit helpers and route-adjacent trace hooks.
- `hako_llvmc_ffi_string_chain_policy.inc` now mirrors the first `.hako` string-chain policy vocabulary, including concat route names plus retained-form / post-store observer names, so compiler-local string placement traces no longer hardcode those owner terms directly.
- `lang/src/runtime/collections/method_policy_box.hako` is the second `.hako` semantic-owner landing for generic collection/runtime method emit vocabulary.
- `hako_llvmc_ffi_generic_method_policy.inc` now mirrors that generic method vocabulary so emit-kind naming is no longer owned inside `generic_method_match.inc`.
- `hako_llvmc_ffi_generic_method_len_policy.inc` now mirrors the first generic method action route (`len`) so `generic_method_lowering.inc` no longer owns that route ladder directly.
- `hako_llvmc_ffi_generic_method_push_policy.inc` now mirrors the second generic method action route (`push`) so append-route ownership is no longer owned inside `generic_method_lowering.inc`.
- `hako_llvmc_ffi_generic_method_has_policy.inc` now mirrors the third generic method action route (`has`) so contains/probe ownership is no longer owned inside `generic_method_lowering.inc`.
- `hako_llvmc_ffi_generic_method_substring_policy.inc` now mirrors the fourth generic method action route (`substring`) so insert-window vs direct substring ownership is no longer owned inside `generic_method_lowering.inc`.
- next narrow target is `hako_llvmc_ffi_generic_method_get_policy.inc`, but only for the final fallback route; window/RMW/indexOf-defer analysis stays compiler-state-owned for now.
- `hako_llvmc_ffi_generic_method_get_policy.inc` now mirrors that fallback route, so `generic_method_lowering.inc` keeps the producer-window probes but no longer owns the final `MapBox.get` vs `ArrayBox.get` decision directly.
- `hako_llvmc_ffi_generic_method_lowering.inc` is mostly semantic owner plus final call emission.
- `hako_llvmc_ffi_compiler_state.inc` now holds the shared copy/origin/type/const helper tables and is the first compiler-state seam landed.
- `hako_llvmc_ffi_pure_compile.inc` is compiler orchestrator owner and still carries route decisions.
- `lang/src/runtime/kernel/string/chain_policy.hako` is the first `.hako` semantic-owner landing for string-chain route / retained-form vocabulary.
- Therefore the migration problem is not “every `.inc` already fits `.hako` syntax”; the real gap is the missing split between compiler-state capability, lowering builder seam, and thin emit shim.

## Migration Order

1. Fix this SSOT and keep the existing stage2 / ABI SSOTs consistent.
2. Classify `.inc` bodies into semantic owner, compiler owner, thin shim, and native leaf.
3. Introduce a compiler-state capability boundary for origin/type/source-reg/future-use facts.
4. Introduce a lowering builder seam so direct LLVM IR text emission is no longer the owner surface.
5. First code slice: extract emit primitives into `hako_llvmc_ffi_emit_seam.inc` before any semantic owner migration.
6. Second code slice: split generic method classification into `hako_llvmc_ffi_generic_method_match.inc` so method routing is no longer mixed with emit logic.
7. Third code slice: extract compiler-state helpers into `hako_llvmc_ffi_compiler_state.inc` so shared origin/type/const state is no longer in the orchestrator body.
8. Fourth code slice: split string concat emit helpers into `hako_llvmc_ffi_string_concat_emit.inc` so concat lowering no longer owns the emit body.
9. First semantic-owner slice: land string-chain route / retained-form vocabulary under `lang/src/runtime/kernel/string/`.
10. Fifth code slice: mirror that route vocabulary in `hako_llvmc_ffi_string_chain_policy.inc` so orchestrator ladders shrink before full `.hako` cutover.
11. Second semantic-owner slice: land generic collection/runtime method vocabulary under `lang/src/runtime/collections/`.
12. Sixth code slice: mirror that method vocabulary in `hako_llvmc_ffi_generic_method_policy.inc`.
13. Seventh code slice: mirror the generic method `len` action route in `hako_llvmc_ffi_generic_method_len_policy.inc`.
14. Eighth code slice: mirror the generic method `push` action route in `hako_llvmc_ffi_generic_method_push_policy.inc`.
15. Ninth code slice: mirror the generic method `has` action route in `hako_llvmc_ffi_generic_method_has_policy.inc`.
16. Tenth code slice: mirror the generic method `substring` action route in `hako_llvmc_ffi_generic_method_substring_policy.inc`.
17. Eleventh code slice: mirror the generic method `get` fallback route in `hako_llvmc_ffi_generic_method_get_policy.inc`.
18. Twelfth code slice: extract generic method GET window helpers into `hako_llvmc_ffi_generic_method_get_window.inc` so producer-side probe logic is not welded to the orchestrator body.
19. Thirteenth code slice: extract generic method GET lowering orchestration into `hako_llvmc_ffi_generic_method_get_lowering.inc` so the dispatcher stops owning the window/fallback bundle directly.
20. Fourteenth code slice: extract string concat producer-window helpers into `hako_llvmc_ffi_string_concat_window.inc` so producer/use/future-use logic is not welded to `string_concat_match.inc`.
21. Move remaining semantic owner and compiler-owner decisions into `.hako`.
22. Shrink `.inc` to thin shim responsibilities only.

## Landed Slices

- `hako_llvmc_ffi_emit_seam.inc`
- `hako_llvmc_ffi_generic_method_match.inc`
- `hako_llvmc_ffi_compiler_state.inc`
- `hako_llvmc_ffi_string_concat_emit.inc`
- `lang/src/runtime/kernel/string/chain_policy.hako`
- `hako_llvmc_ffi_string_chain_policy.inc`
- `lang/src/runtime/collections/method_policy_box.hako`
- `hako_llvmc_ffi_generic_method_policy.inc`
- `hako_llvmc_ffi_generic_method_len_policy.inc`
- `hako_llvmc_ffi_generic_method_push_policy.inc`
- `hako_llvmc_ffi_generic_method_has_policy.inc`
- `hako_llvmc_ffi_generic_method_substring_policy.inc`
- `hako_llvmc_ffi_generic_method_get_policy.inc`
- `hako_llvmc_ffi_generic_method_get_window.inc`
- `hako_llvmc_ffi_generic_method_get_lowering.inc`
- `hako_llvmc_ffi_string_concat_window.inc`

## Non-Goals

- Do not delete native keep in one wave.
- Do not add a third public ABI.
- Do not force every `.inc` byte into `.hako` before the capability vocabulary is ready.
- Do not mix this owner/shim cut with the perf-kilo hot-path lane.
