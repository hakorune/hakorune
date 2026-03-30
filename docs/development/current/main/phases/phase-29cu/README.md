---
Status: Accepted (formal-close-synced)
Decision: accepted
Date: 2026-03-25
Scope: Rune v0 lane の current truth を landed implementation に合わせて同期し、remaining verifier/consumer leaf を narrow に固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md
  - docs/development/current/main/design/rune-v0-contract-rollout-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
  - docs/reference/language/EBNF.md
  - docs/reference/ir/ast-json-v0.md
  - src/parser/runes.rs
  - src/parser/statements/helpers.rs
  - src/config/env/parser_flags.rs
  - src/tests/parser_opt_annotations.rs
  - src/runtime/mirbuilder_emit.rs
  - src/stage1/program_json_v0.rs
  - tests/json_program_env.rs
  - lang/src/compiler/parser/stmt/parser_stmt_box.hako
  - src/runner/json_v0_bridge/lowering.rs
  - src/runner/json_v0_bridge/lowering/program.rs
  - lang/c-abi/shims/hako_llvmc_ffi.c
---

# Phase 29cu: Rune V0 Contract Lane

## Goal

- historical Rune ideas を current repo で進められる narrow v0 lane に縮約する。
- parser parity / AST/direct MIR shape / backend scope を実装前に固定する。
- Rune lane を `phase-29ct` substrate work や `phase-29y` runtime migration と混ぜない。
- `Program(JSON v0)` は Rune で widen しない。
- stage2-mainline / stage2+ / distribution policy の共有語彙は parent SSOT `execution-lanes-and-axis-separation-ssot.md` を正本にし、この phase は Rune lane のみを owner とする。ここでの `stage2-mainline` は daily lane、`stage2+` は umbrella / end-state reading だよ。

## Current Read

- lane status: `formal-close-synced`
- current narrow v0 scope is complete for the accepted current truth
- current truth is already narrower than the original rollout wording:
  - declaration-local `attrs.runes`
  - Rust direct MIR carrier
  - `.hako` source-route real `Main.main` root-entry carrier
  - `.hako` compiler/mirbuilder generic function-rune carrier from `defs[].attrs.runes`
  - selected-entry-only `ny-llvmc` `Symbol` / `CallConv` semantics
  - `Program(JSON v0)` no-widen
- reopen only if:
  - `.hako` declaration-local full Rune carrier parity resumes
  - or a new exact verifier/consumer gap appears

## Fixed Decisions

1. syntax は dedicated `@rune`
2. parser は Rust と `.hako` の両方が必要
3. first slice は contract-only
4. first active consumer は `ny-llvmc` の selected-entry only
5. `llvmlite` は compat/noop keep
6. `hako_module.toml` は module/file export boundary のまま維持する
7. `Program(JSON v0)` は Rune v0 の carrier にしない

## Non-Goals

- historical full Rune system (`ValueLike`, `implements`, `@derive`)
- protocol/typeclass expansion
- runtime/VM semantics keyed directly off Rune
- substrate capability implementation
- `llvmlite` Rune parity work

## Landed Docs Lock

The docs/task lock for this lane now lives in:

- [`rune-and-stage2plus-final-shape-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md)
- [`rune-v0-contract-rollout-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/rune-v0-contract-rollout-ssot.md)
- [`docs/reference/language/EBNF.md`](/home/tomoaki/git/hakorune-selfhost/docs/reference/language/EBNF.md)
- [`docs/reference/ir/ast-json-v0.md`](/home/tomoaki/git/hakorune-selfhost/docs/reference/ir/ast-json-v0.md)

This phase is docs/task locked and the narrow Rune v0 lane is formally close-synced.

## Current Implementation Status

### P0. Docs/task lock

Landed.

- current truth / clean end-state reading is fixed
- Rune v0 syntax, parser scope, carrier, and backend scope are docs-locked
- current SSOT / historical forward links are already in place

### P1. Rust parser gate

Landed.

- `NYASH_FEATURES=rune` gate is active
- Rust parser accepts the fixed Rune v0 surface
- unknown name / wrong arity / declaration-required placement fail fast
- duplicate rune / conflicting visibility / box-target visibility-only checks are already active

### P2. `.hako` parser parity

Partially landed.

- `.hako` parser accepts the same Rune surface and arg-shape contract
- `.hako` statement/program routes fail fast on Rune invalid placement instead of attaching generic statement attrs
- `.hako` root-entry carrier path now fails fast on invalid `CallConv("c")` / `Ownership(owned|borrowed|shared)` values instead of silently carrying them
- current `.hako` source-route keep does not claim full declaration-local MIR parity yet
- current `.hako` end-to-end keep uses a real root-entry `Main.main` def carrier instead of widening `Program(JSON v0)`

### P3. AST / direct MIR carrier

Partially landed, with route-specific reading.

- Rust route:
  - declaration-local `attrs.runes` survives parser -> AST JSON -> direct MIR
  - direct MIR JSON mirrors declaration-local attrs on functions
- `.hako` source-route keep:
  - root-entry attrs now survive via a real `Main.main` declaration def in `defs[]`
  - `.hako` compiler/mirbuilder state now carries a generic function-rune map derived from `defs[].attrs.runes`
  - `.hako` MIR attrs injection is now function-name driven instead of `main` hardcode
  - `Program(JSON v0)` root/body stay Rune-free

### P4. Verifier / consumer activation

Landed for the current narrow v0 scope.

- landed today:
  - duplicate/conflicting rune rejection
  - box-target visibility-only checks
  - parser-level unknown/arity fail-fast
  - `.hako` parser invalid-placement fail-fast on statement/program routes
  - Rust function-target placement / ABI-facing verifier contract
  - `.hako` root-entry carrier value-contract parity for `CallConv("c")` / `Ownership(owned|borrowed|shared)`
- no new exact implementation leaf remains under the current narrow v0 scope

### P5. `ny-llvmc` ABI consumer

Landed narrow slice.

- `ny-llvmc` reads selected-entry attrs
- active semantics are `Symbol("...")` and `CallConv("c")` only
- `ny_main` stays compat alias when `Symbol(...)` renames the primary entry
- `llvmlite` remains ignore/noop keep

## Current Narrow-Scope Status

Selected-entry shim retirement is landed for the current narrow v0 scope.

- carrier/backend scope stayed unchanged
- `Program(JSON v0)` stayed no-widen
- `.hako` route now uses a canonical real `Main.main` declaration def as the root-entry carrier
- the current generic function-rune map remains the only `.hako` MIR-builder carrier truth

## Planned Future Reopen

The current narrow v0 scope is intentionally smaller than full `.hako` route parity.
That future work stays planned after this formal close sync.

- `.hako` declaration-local full Rune carrier parity
  - carry declaration-local `attrs.runes` on the `.hako` route beyond the current root-entry-only carrier
- guard rails for that reopen:
  - keep `Program(JSON v0)` no-widen
  - do not invent a second metadata truth
  - keep backend semantics narrow unless a separate consumer slice is accepted

## Current Proof Line

1. Rust parser/unit coverage
   - `cargo test parser_opt_annotations -- --nocapture`
2. Rust direct MIR carrier proof
   - `cargo test env_mirbuilder_emit_keeps_rune_attrs_on_selected_entry -- --nocapture`
3. Program(JSON v0) no-widen guard
   - `cargo test source_to_program_json_v0_does_not_widen_with_rune_attrs -- --nocapture`
4. downstream no-duplicate proof
   - `cargo test json_stageb_entry_def_runes_attach_to_main_without_duplicate_main_def -- --nocapture`
5. `.hako` / Stage-B root-entry carrier proof
   - `bash tools/smokes/v2/profiles/integration/parser/parser_rune_decl_local_attrs_selected_entry_trace.sh`
6. backend proof
   - selected-entry `ny-llvmc` `Symbol` / `CallConv` path is already live
   - `llvmlite` remains out of scope except safe-ignore compatibility

## Reopen Rule

`CURRENT_TASK.md` now carries the close-synced Rune lane entry for this phase.
