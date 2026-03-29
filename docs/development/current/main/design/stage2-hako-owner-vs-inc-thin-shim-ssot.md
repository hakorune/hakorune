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
- `hako_llvmc_ffi_generic_method_lowering.inc` is mostly semantic owner plus final call emission.
- `hako_llvmc_ffi_compiler_state.inc` now holds the shared copy/origin/type/const helper tables and is the first compiler-state seam landed.
- `hako_llvmc_ffi_pure_compile.inc` is compiler orchestrator owner and still carries route decisions.
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
9. Move semantic owner and compiler-owner decisions into `.hako`.
10. Shrink `.inc` to thin shim responsibilities only.

## Landed Slices

- `hako_llvmc_ffi_emit_seam.inc`
- `hako_llvmc_ffi_generic_method_match.inc`
- `hako_llvmc_ffi_compiler_state.inc`
- `hako_llvmc_ffi_string_concat_emit.inc`

## Non-Goals

- Do not delete native keep in one wave.
- Do not add a third public ABI.
- Do not force every `.inc` byte into `.hako` before the capability vocabulary is ready.
- Do not mix this owner/shim cut with the perf-kilo hot-path lane.
