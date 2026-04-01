---
Status: Provisional SSOT
Decision: provisional
Date: 2026-03-30
Scope: Rune v0 の syntax / parser parity / AST/direct MIR carrier / backend scope を current implementation truth に合わせて固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/rune-v1-metadata-unification-ssot.md
  - docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
  - docs/reference/language/EBNF.md
  - docs/reference/ir/ast-json-v0.md
  - docs/development/current/main/phases/archive/phase-29cu/README.md
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
- this is the first contract slice of the existing-language primitive control plane; it does not add a second runtime semantics carrier.

Note:
- this document remains the base Rune v0 SSOT for visibility / ownership / ABI-facing families and carrier/backend scope
- optimization metadata unification (`Hint` / `Contract` / `IntrinsicCandidate` + legacy alias normalization) is now owned by `rune-v1-metadata-unification-ssot.md`

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

- canonical gate/docs surface: `NYASH_FEATURES=rune`
- default: OFF
- no new `NYASH_RUNE_*` env vars in v0
- current parser-front-door truth after Rune v1 unification:
  - `NYASH_FEATURES=opt-annotations` remains a compat alias gate for the unified metadata parser path during the compat window
  - canonical optimization metadata syntax is `@rune Hint(...)`, `@rune Contract(...)`, `@rune IntrinsicCandidate("...")`
  - legacy `@hint` / `@contract` / `@intrinsic_candidate` normalize to declaration-local `attrs.runes`
  - statement-position legacy aliases stay compat/noop
- this document's v0 family set and backend scope remain unchanged

## 5. Current Implementation Status

| Area | Status | Current truth |
| --- | --- | --- |
| docs/task lock | landed | syntax / carrier / backend scope are docs-locked |
| Rust parser | landed | `@rune` behind `NYASH_FEATURES=rune`; declaration-local attrs kept; unknown/arity/declaration-required fail-fast |
| `.hako` parser | partly landed | same Rune surface + arg-shape contract; statement/program routes fail fast on invalid placement; root-entry carrier path fails fast on invalid `CallConv("c")` / `Ownership(owned|borrowed|shared)` values |
| Rust AST/direct MIR carrier | landed | declaration-local `attrs.runes` survives parser -> AST JSON -> direct MIR |
| `.hako` source-route keep | partly landed | root-entry attrs now survive via a real `Main.main` declaration def in `defs[]`; compiler/mirbuilder carries a generic function-rune map from `defs[].attrs.runes`; still not a claim of broad declaration-local MIR parity |
| Program(JSON v0) | locked | retire target; no Rune widening |
| verifier | landed for narrow v0 | duplicate/conflict + box-target visibility-only checks are live; Rust function-target ABI/placement verifier is live; `.hako` statement/program invalid-placement fail-fast and root-entry carrier value-contract fail-fast are live |
| `ny-llvmc` consumer | landed narrow | selected-entry `Symbol` / `CallConv` semantics only |
| `llvmlite` | unchanged | safe ignore / noop keep only |

### 5.1 Current narrow-scope status

The narrow Rune v0 scope is now formally close-synced.

- carrier/backend scope stayed unchanged
- `.hako` source-route transport is now a real root-entry declaration def, not a synthetic shim
- the generic function-rune map remains the only `.hako` MIR-builder carrier truth

### 5.2 Planned follow-up after close sync

The following is intentionally deferred, but it is still planned work:

- `.hako` declaration-local full carrier parity
  - move beyond the current selected-entry transport shim
  - carry declaration-local `attrs.runes` through the `.hako` route to direct MIR with the same truth shape as Rust

This follow-up must keep the existing v0 guard rails:

- `Program(JSON v0)` remains no-widen
- no second metadata truth
- backend semantics stay narrow unless a separate accepted consumer slice widens them

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
- current `.hako` source-route keep uses a real `Main.main` declaration def in `defs[]` for root-entry attrs, while Program(JSON v0) root/body stay Rune-free
- existing declaration metadata owners such as `metadata.extern_c` stay the extension point; do not invent a parallel Rune-only channel

## 7. Backend Scope

- only `ny-llvmc` consumes Rune metadata in v0
- scope is ABI-facing declaration metadata only
- codegen semantics apply to the selected entry only
- `llvmlite` does not gain active Rune semantics in v0; it must simply ignore extra metadata safely
- Rust VM/runtime remains metadata-carrier or verifier-adjacent only, not Rune-semantic owner
- `hako_module.toml` remains the module/file boundary SSOT
- `.hako` direct MIR lane carries declaration-local attrs instead of widening Program(JSON v0); current source-route keep carries root-entry attrs through a real `Main.main` declaration def in `defs[]`

## 8. Non-Goals

- full historical Rune system in one wave
- protocol/typeclass semantics in v0
- substrate capability implementation via Rune
- `llvmlite` feature parity for Rune semantics
- new public runtime APIs whose only purpose is Rune
- inventing a second metadata channel beyond the current declaration-def carrier truth
