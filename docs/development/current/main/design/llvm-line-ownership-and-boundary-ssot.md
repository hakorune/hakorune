---
Status: SSOT
Decision: current
Date: 2026-04-16
Scope: Hakorune の LLVM daily lane / keep lane / bootstrap lane と ABI/boundary ownership を固定する正本
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/investigations/llvm-native-library-llvmlite-graduation-task-2026-07-22.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
  - docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/phases/phase-137x/README.md
  - crates/nyash-llvm-compiler/README.md
  - src/host_providers/llvm_codegen/README.md
  - src/llvm_py/README.md
---

# LLVM Line Ownership And Boundary SSOT

## Goal

- LLVM line の daily owner と keep lanes を固定して、perf/optimization work が route owner を見失わないようにする
- `VMValue is not i64-only` と `current LLVM line is still i64/handle-heavy` を矛盾なく整理する
- public ABI を増やさず、internal fast path だけを manifest-driven value classes へ寄せる

## Fixed Lane Reading

### Current daily owner

- `ny-llvmc(boundary pure-first)` が daily/mainline LLVM owner
- current mainline route is:
  - `.hako -> MIR -> thin backend boundary -> ny-llvmc(boundary pure-first) -> object/exe`

This describes the current production route, not the terminal ownership split.

### Selected terminal ownership

After the parked native-library and Hako-emitter cutovers are proven:

```text
sealed MIR/backend plan
  -> .hako LLVM-text emitter
  -> versioned libhako_llvmc_ffi LLVM-text/object API
  -> object/exe
```

`ny-llvmc` becomes a CLI/file adapter to the same library operation. The
existing C pure-first MIR(JSON)-to-LLVM lowerer remains a compatibility owner
until family-by-family Hako emitter coverage permits its separate retirement.
New FastMem V1 semantic lowering must not be duplicated into that legacy C
lowerer.

### Keep lanes

- `src/llvm_py/**`
  - compat / keep lane
  - shared contract verification
  - local typed lowering may exist, but it is not the mainline owner
- `--driver harness`
  - explicit compat replay lane

The default ny-llvmc route is non-replay, but other generic/compat ingress
families still reach harness automatically. llvmlite graduation therefore
requires an ingress census and individual retirement; it is not implied by
the default-route setting.

### W6-E handoff and llvmlite retirement

The selected Dynamic W6 lane hands off to the graduation board only after the
Boundary artifact receipt and caller census are complete:

```text
new selected caller = 1
selected old edge = 0
Boundary artifact receipt = 1
Python/harness/VM production consumers = 0
fallback = 0, retry = 0
```

After that handoff, `LLVMLITE-PROD0-G0` (G1) removes automatic production
reachability and makes unsupported native capability a typed fail-fast.
`LLVMLITE-AUTO0`/G2 then removes Python and llvmlite from default build, CI,
and perf gates while keeping explicitly named oracle jobs. `LLVMLITE-KEEP0-RET0`
(G3) is a later, separately approved source/archive decision requiring an
independent oracle and a zero-or-archived consumer census. No native or Hako
failure may retry through llvmlite at any stage.

### Bootstrap / canary lane

- `crates/nyash-llvm-compiler/src/native_driver.rs`
  - bootstrap seam
  - canary/replay lane
  - not the final owner

## Ownership Split

1. `.hako`
   - route policy
   - recipe/profile choice
   - acceptance naming
2. `MIR`
   - canonical contract
   - optimization proof
   - rewrite target
3. `thin backend boundary`
   - transport-only handoff
   - recipe replay
   - object/exe request shaping
4. current `ny-llvmc(boundary pure-first)` / C lowerer
   - transition daily compile/lower/link owner
5. selected `.hako LLVM-text emitter`
   - terminal LLVM lowering owner after cutover
6. selected `libhako_llvmc_ffi`
   - versioned target/session/error boundary
   - LLVM verification, object emission, and link transaction
7. selected `ny-llvmc`
   - CLI adapter to the same native-library operation
8. `LLVM`
   - IR-level consumer
   - profitability / codegen

`src/host_providers/llvm_codegen/**` is transport glue, not a semantic owner.

## Current Reality

### VM side

- `VMValue` is not i64-only
- it already carries:
  - `Integer(i64)`
  - `Float(f64)`
  - `Bool(bool)`
  - `String(String)`
  - `BoxRef(...)`
  - `WeakBox(...)`

### `llvm_py` side

- current `llvm_py` lowering is not fully i64-only
- local IR already uses:
  - `f64`
  - `i1`
  - `i8*`
- but function/call ABI is still mostly `i64 / handle` centered
- this is acceptable only because `llvm_py` is keep lane, not daily owner

### `native_driver` side

- current native subset is effectively i64-only bootstrap:
  - `const(i64)`
  - `ret`
  - `mir_call(print/1)`
- this must stay bootstrap/canary only

## ABI Rule

### Public ABI

Do not add a third canonical public ABI.

Current public/canonical surfaces stay:

1. `Core C ABI`
2. `TypeBox ABI v2`

### Internal backend vocabulary

Internal fast paths may use manifest-driven value classes.

Current allowed vocabulary:

- `imm_i64`
- `imm_bool`
- `handle_owned`
- `handle_borrowed_string`
- `boxed_local`

These classes are backend-private selection vocabulary, not a new public ABI.

## Design Rule For Fast Paths

Allowed:

- hidden fast leaf
- monomorphic direct entry
- manifest-selected helper
- boundary-private lowered executor

Forbidden:

- expanding `llvm_py` into the mainline typed ABI owner
- expanding `native_driver` into the final owner
- exposing backend-private value classes as a third public ABI
- moving route policy back out of `.hako` / MIR into transport glue
- growing `--driver native` into a second LLVM lowering owner
- adding one new semantic lowering rule to both Hako LLVM-text and legacy C
  MIR JSON lowering
- retrying native-library or Hako-emitter failure through llvmlite

## Optimization Rule

For current optimization work, route ownership is fixed like this:

- measure on `.hako -> ny-llvmc(boundary pure-first) -> C ABI`
- treat `llvm_py` as keep lane only
- treat `native_driver` as bootstrap/canary only
- only pivot into a keep lane if the route contract itself is broken

This means current substring/string corridor work should not reopen `llvm_py` or `native_driver` just because they contain similar helper names.

## Immediate Implication For Phase 137x

current front:

- `kilo_micro_substring_concat`

must be read as:

- `MIR` owns borrowed-view continuity proof
- `runtime/kernel` owns the narrow fused executor
- `ny-llvmc(boundary pure-first)` remains the daily lane
- `LLVM` consumes the resulting truthful facts

Do not reinterpret the current gap as a need to grow the native i64 emitter.

## Non-goals

- `native_driver` を final native emitter owner にすること
- `llvm_py` を mainline optimization owner に戻すこと
- public ABI を 3 本目に増やすこと
- `VMValue` の多相性を、そのまま public LLVM ABI の多相性に昇格させること
- transport glue に route policy や optimization semantics を戻すこと
