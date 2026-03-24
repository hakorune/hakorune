---
Status: Provisional SSOT
Decision: provisional
Date: 2026-03-24
Scope: Rune v0 の syntax / parser parity / AST/direct MIR carrier / backend scope を current implementation truth に合わせて固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
  - docs/reference/language/EBNF.md
  - docs/reference/ir/ast-json-v0.md
  - docs/development/current/main/phases/phase-29cu/README.md
  - src/parser/statements/helpers.rs
  - lang/src/compiler/parser/stmt/parser_stmt_box.hako
  - src/config/env/parser_flags.rs
  - src/tests/parser_opt_annotations.rs
  - lang/src/compiler/pipeline_v2/flow_entry.hako
  - lang/src/compiler/pipeline_v2/pipeline.hako
---

# Rune V0 Contract Rollout (SSOT)

## Goal

- historical Rune proposal を current repo で実装可能な narrow v0 slice に落とす。
- parser scope, AST/direct MIR shape, backend scope を先に固定し、途中で正本を二重化しない。
- Rune を contract-only first slice として始め、runtime semantics や substrate implementation に広げない。

## 1. Fixed Decisions

| Item | Decision |
| --- | --- |
| syntax | dedicated `@rune` |
| parser scope | Rust parser + `.hako` parser の両方 |
| first slice | contract-only |
| metadata carrier | declaration-local `attrs.runes` on AST, mirrored to direct MIR |
| first backend consumer | `ny-llvmc` selected-entry only |
| `llvmlite` | compat/noop keep; safe ignore only |
| feature gate | `NYASH_FEATURES=rune` |

## 2. Surface

Rune v0 is declaration-attached only.

Examples:

```hako
@rune Public
static box Main {
  main() { return 0 }
}

@rune FfiSafe
@rune Symbol("nyash.array.slot_store_hii")
@rune CallConv("c")
extern "c" fn slot_store_i64(handle: I64, idx: I64, value: I64) -> I64
```

### 2.1 Supported targets

- `box`
- `static box`
- free function
- method
- `extern "c" fn`

### 2.2 Supported rune set (v0)

| Family | Forms | Notes |
| --- | --- | --- |
| visibility | `Public`, `Internal` | declaration-level visibility only |
| ABI/FFI | `FfiSafe`, `Symbol("...")`, `CallConv("c")`, `ReturnsOwned`, `FreeWith("...")` | ABI-facing only |
| ownership | `Ownership(owned|borrowed|shared)` | contract-only; no runtime semantics in Rust |

### 2.3 Validation rules

- unknown rune name: fail-fast
- wrong arity or wrong argument shape: fail-fast
- invalid target placement: fail-fast
- duplicate/conflicting rune set on the same declaration: fail-fast

## 3. Deferred From V0

The following remain explicitly deferred:

- `@rune ValueLike { ... }`
- `implements`
- `@derive(...)`
- field/module runes
- optimization promise runes such as `Inline`, `NoAlias`, `InBounds`, `TlsModel(...)`
- `ThreadLocal` as Rune
- VM/runtime semantic behavior keyed directly off Rune

`thread_local` / `TlsCell<T>` stays the preferred final TLS surface; Rune v0 does not claim that role.

## 4. Feature Gate

- gate: `NYASH_FEATURES=rune`
- default: OFF
- no new `NYASH_RUNE_*` env vars in v0
- do not merge Rune v0 into the existing `opt-annotations` gate

Existing `@hint`, `@contract`, and `@intrinsic_candidate` remain a separate provisional lane.

## 5. Current Implementation Status

| Area | Status | Current truth |
| --- | --- | --- |
| docs/task lock | landed | syntax / carrier / backend scope are docs-locked |
| Rust parser | landed | `@rune` behind `NYASH_FEATURES=rune`; declaration-local attrs kept; unknown/arity/declaration-required fail-fast |
| `.hako` parser | partly landed | same Rune surface + arg-shape contract; declaration attrs preserved on parsed defs |
| Rust AST/direct MIR carrier | landed | declaration-local `attrs.runes` survives parser -> AST JSON -> direct MIR |
| `.hako` source-route keep | partly landed | selected-entry attrs survive via synthetic `Main.main` transport shim; not a claim of broad declaration-local MIR parity |
| Program(JSON v0) | locked | retire target; no Rune widening |
| verifier | partly landed | duplicate/conflict + box-target visibility-only checks are live; function-target ABI/placement verifier remains the next exact leaf |
| `ny-llvmc` consumer | landed narrow | selected-entry `Symbol` / `CallConv` semantics only |
| `llvmlite` | unchanged | safe ignore / noop keep only |

### 5.1 Remaining exact leaf

The next Rune slice is verifier-only:

- tighten the function-target placement contract
- lock ABI-facing verifier rules without widening carrier/backend scope
- keep `.hako` source-route transport as a shim, not a second metadata truth

Forbidden:

- one-parser-only active grammar
- backend workaround that reinterprets missing Rune metadata
- introducing a second metadata truth separate from existing declaration metadata paths

## 6. Carrier Shape

Rune v0 uses a uniform declaration-local shape:

```json
{
  "attrs": {
    "runes": [
      { "name": "Public", "args": [] },
      { "name": "Ownership", "args": ["owned"] },
      { "name": "Symbol", "args": ["nyash.array.slot_store_hii"] }
    ]
  }
}
```

Carrier rules:

- both parsers must produce the same declaration-local `attrs.runes` shape
- AST JSON v0 carries Rune metadata on declaration-bearing nodes
- Program(JSON v0) is a retire target and must not be widened for Rune v0
- Rust direct MIR JSON mirrors declaration-local attrs into functions
- current `.hako` source-route keep may use a synthetic `Main.main` def transport shim for selected-entry attrs, but Program(JSON v0) root/body must stay Rune-free
- existing declaration metadata owners such as `metadata.extern_c` stay the extension point; do not invent a parallel Rune-only channel

## 7. Backend Scope

- only `ny-llvmc` consumes Rune metadata in v0
- scope is ABI-facing declaration metadata only
- codegen semantics apply to the selected entry only
- `llvmlite` does not gain active Rune semantics in v0; it must simply ignore extra metadata safely
- Rust VM/runtime remains metadata-carrier or verifier-adjacent only, not Rune-semantic owner
- `hako_module.toml` remains the module/file boundary SSOT
- `.hako` direct MIR lane carries declaration-local attrs instead of widening Program(JSON v0); current source-route keep may use a synthetic selected-entry def only as a transitional transport shim

## 8. Non-Goals

- full historical Rune system in one wave
- protocol/typeclass semantics in v0
- substrate capability implementation via Rune
- `llvmlite` feature parity for Rune semantics
- new public runtime APIs whose only purpose is Rune
- broadening the current `.hako` source-route transport shim into a second metadata channel
