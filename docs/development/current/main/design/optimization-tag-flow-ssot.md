---
Status: SSOT
Scope: exe optimization wave で使う tag / knob / selector の到達範囲
Related:
- docs/development/current/main/design/perf-optimization-method-ssot.md
- docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
- docs/development/current/main/design/optimization-ssot-string-helper-density.md
- tools/perf/lib/aot_helpers.sh
- tools/ny_mir_builder.sh
- CURRENT_TASK.md
---

# Optimization Tag Flow SSOT

## Goal

最適化フェーズで使う tag / knob が、どこまで届くかを 1 枚で固定する。

この文書で区別するのは 4 つだけ。

1. `.hako` / MIR 生成前後でだけ効くもの
2. `ny-llvmc(boundary)` を越えて object / exe 生成まで届くもの
3. keep-lane selector で、perf AOT では無効なもの
4. perf runner だけで効くもの

## Route Contract

perf AOT lane の正本ルートは次だけ。

- `.hako -> ny-llvmc(boundary) -> C ABI`

この lane では、`llvmlite` / `native` / harness は比較対象にしない。
正本は [tools/perf/lib/aot_helpers.sh](/home/tomoaki/git/hakorune-selfhost/tools/perf/lib/aot_helpers.sh)。

## Coverage Matrix

| Tag / Knob family | Primary examples | Effective zone | Crosses `ny-llvmc` boundary | Current reading |
|---|---|---|---|---|
| Language optimization annotations | `@hint(inline)`, `@contract(pure)`, `@intrinsic_candidate(...)` | parser / Program(JSON) metadata | No | parse/noop only; backend use is not active yet |
| AotPrep / MIR shaping | `NYASH_AOT_COLLECTIONS_HOT`, `NYASH_MIR_LOOP_HOIST`, `NYASH_AOT_MAP_KEY_MODE`, `NYASH_AOT_NUMERIC_CORE`, `HAKO_APPLY_AOT_PREP` | `.hako` / MIR before `ny-llvmc` | No | valid pre-boundary shaping knobs |
| Boundary compile request | `HAKO_BACKEND_COMPILE_RECIPE`, `HAKO_BACKEND_COMPAT_REPLAY` | Rust/C boundary transport | Yes | reaches compile/link boundary as explicit route/profile request |
| LLVM opt / link contract | `HAKO_LLVM_OPT_LEVEL`, `NYASH_LLVM_OPT_LEVEL`, `NYASH_EMIT_EXE_NYRT`, `HAKO_AOT_LDFLAGS` | object / exe generation | Yes | valid post-boundary knobs |
| FAST runtime/link assist | `NYASH_LLVM_FAST` | runtime hot path + link mode | Partially | not a pure LLVM tag; runtime/link helpers may read it after boundary |
| Keep-lane selectors | `NYASH_LLVM_BACKEND=llvmlite|native`, `NYASH_LLVM_USE_HARNESS=1`, `HAKO_LLVM_EMIT_PROVIDER=llvmlite` | route selection only | No for perf AOT | invalid in perf AOT; fail-fast |
| Perf orchestration | `PERF_AOT_SKIP_BUILD`, `PERF_VM_FORCE_NO_FALLBACK`, `PERF_AOT_DIRECT_ONLY`, `PERF_AOT_PREFER_HELPER` | benchmark runner | No | measurement-only; do not treat as backend optimization tags |

## Exact Reading Per Family

### 1. Language annotations

- `@hint` / `@contract` / `@intrinsic_candidate` are not backend-active in the current wave.
- They parse and survive as noop metadata only.
- Do not use them as proof that `ny-llvmc` or LLVM is honoring a new optimization.

正本:
- [optimization-hints-contracts-intrinsic-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md)

### 2. Pre-boundary shaping knobs

- `NYASH_AOT_COLLECTIONS_HOT`
- `NYASH_MIR_LOOP_HOIST`
- `NYASH_AOT_MAP_KEY_MODE`
- `NYASH_AOT_NUMERIC_CORE`
- `HAKO_APPLY_AOT_PREP`

これらは `.hako` / MIR shaping に効くが、`ny-llvmc` の後段へ tag としては残らない。

正本:
- [lang/src/llvm_ir/boxes/aot_prep.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/llvm_ir/boxes/aot_prep.hako)
- [lang/src/llvm_ir/boxes/aot_prep/README.md](/home/tomoaki/git/hakorune-selfhost/lang/src/llvm_ir/boxes/aot_prep/README.md)

### 3. Boundary-passing knobs

- `HAKO_BACKEND_COMPILE_RECIPE`
- `HAKO_BACKEND_COMPAT_REPLAY`
- `HAKO_LLVM_OPT_LEVEL`
- `NYASH_LLVM_OPT_LEVEL`
- `NYASH_EMIT_EXE_NYRT`
- `HAKO_AOT_LDFLAGS`

これは Rust transport / C ABI / `ny-llvmc` / link に届く。

正本:
- [src/config/env/llvm_provider_flags.rs](/home/tomoaki/git/hakorune-selfhost/src/config/env/llvm_provider_flags.rs)
- [src/host_providers/llvm_codegen/transport.rs](/home/tomoaki/git/hakorune-selfhost/src/host_providers/llvm_codegen/transport.rs)
- [src/host_providers/llvm_codegen/route.rs](/home/tomoaki/git/hakorune-selfhost/src/host_providers/llvm_codegen/route.rs)
- [crates/nyash-llvm-compiler/src/link_driver.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash-llvm-compiler/src/link_driver.rs)

### 4. Keep-lane selectors

- `NYASH_LLVM_BACKEND=llvmlite|native`
- `NYASH_LLVM_USE_HARNESS=1`
- `HAKO_LLVM_EMIT_PROVIDER=llvmlite`

これは boundary 前の route selector で、perf AOT lane の比較では無効。
perf AOT は fail-fast にする。

正本:
- [tools/perf/lib/aot_helpers.sh](/home/tomoaki/git/hakorune-selfhost/tools/perf/lib/aot_helpers.sh)
- [tools/ny_mir_builder.sh](/home/tomoaki/git/hakorune-selfhost/tools/ny_mir_builder.sh)

### 5. Perf-only controls

- `PERF_AOT_SKIP_BUILD`
- `PERF_VM_FORCE_NO_FALLBACK`
- `PERF_REQUIRE_AOT_RESULT_PARITY`
- `PERF_AOT_DIRECT_ONLY`
- `PERF_AOT_PREFER_HELPER`
- `PERF_AOT_HELPER_ONLY`

これは benchmark orchestration だけで使う。
backend optimization coverage の議論に混ぜない。

## Current Wave Rule

今の exe optimization wave では、次の読みを固定する。

1. route contract が壊れていない限り `llvm_py` keep lane を reopen しない
2. `@hint` 系は backend-active ではないので、perf 差分の説明に使わない
3. `array_getset` の next exact cut は Rust substrate leaf に置く
4. tag を疑う前に、ASM top symbol owner を先に見る

## Current Exact Leaf

`kilo_micro_array_getset` の current exact leaf は次。

- cache seam: [handle_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/handle_helpers.rs)
- landed read slice:
  - [array_slot_load.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/array_slot_load.rs)
- current probe target:
  - [array_slot_store.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/array_slot_store.rs)
  - [handle_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/handle_helpers.rs)
  - [mod.rs](/home/tomoaki/git/hakorune-selfhost/src/boxes/array/mod.rs)
- wrapper status:
  - [array_index_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/array_index_helpers.rs)
  - [array_route_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/array_route_helpers.rs)
  - these are now thin wrappers and are no longer the primary edit target

`map` stays parked for this wave because its raw aliases still sit too close to `MapBox.get/set/has` semantics.

したがって、この lane で `route not supported after ny-llvmc` を再調査する必要はない。
