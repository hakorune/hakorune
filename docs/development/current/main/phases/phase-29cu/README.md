---
Status: Accepted (docs/task lock, active)
Decision: provisional
Date: 2026-03-23
Scope: Rune v0 を current docs/task pack として固定し、parser parity / AST-direct-MIR carrier / `ny-llvmc` selected-entry consumer の実装順を切る。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md
  - docs/development/current/main/design/rune-v0-contract-rollout-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
  - docs/reference/language/EBNF.md
  - docs/reference/ir/ast-json-v0.md
  - src/parser/statements/helpers.rs
  - src/config/env/parser_flags.rs
  - src/tests/parser_opt_annotations.rs
  - lang/src/compiler/parser/stmt/parser_stmt_box.hako
  - lang/src/compiler/pipeline_v2/flow_entry.hako
  - lang/src/compiler/pipeline_v2/pipeline.hako
---

# Phase 29cu: Rune V0 Contract Lane

## Goal

- historical Rune ideas を current repo で進められる narrow v0 lane に縮約する。
- parser parity / AST/direct MIR shape / backend scope を実装前に固定する。
- Rune lane を `phase-29ct` substrate work や `phase-29y` runtime migration と混ぜない。
- `Program(JSON v0)` は Rune で widen しない。

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

This phase is docs/task locked and is the active current implementation lane.

## Fixed Order

### P0. Docs/task lock

Done in this slice.

- lock the current truth / clean end-state reading
- lock Rune v0 syntax, parser scope, metadata carrier, backend scope
- cross-link current SSOTs and historical docs

### P1. Rust parser gate

First code slice:

- touch:
  - [`src/config/env/parser_flags.rs`](/home/tomoaki/git/hakorune-selfhost/src/config/env/parser_flags.rs)
  - [`src/parser/statements/helpers.rs`](/home/tomoaki/git/hakorune-selfhost/src/parser/statements/helpers.rs)
  - [`src/tests/parser_opt_annotations.rs`](/home/tomoaki/git/hakorune-selfhost/src/tests/parser_opt_annotations.rs) or a dedicated Rune parser test file
- deliverables:
  - `NYASH_FEATURES=rune` gate
  - parse `@rune` forms
  - fail-fast on unknown name / arity / placement
  - preserve declaration-local metadata instead of noop dropping

### P2. `.hako` parser parity

Second code slice:

- touch:
  - [`lang/src/compiler/parser/stmt/parser_stmt_box.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/compiler/parser/stmt/parser_stmt_box.hako)
  - parser parity fixtures / smoke under `tools/smokes/v2/profiles/integration/parser/`
- deliverables:
  - accept the same Rune forms as Rust parser
  - same fail-fast contract
  - same declaration metadata shape
  - declaration-local Rune attrs are preserved into direct MIR on the `.hako` route
  - if the `.hako` source-route still passes through Program(JSON v0), it may use a synthetic `Main.main` def transport shim, but it must not widen root/body attrs

### P3. AST / direct MIR carrier

Third code slice:

- extend the existing declaration metadata path rather than inventing a new carrier
- first touch owners:
  - AST declaration owners on the Rust route
  - direct MIR emit owners on the Rust route and `.hako` route
- deliverables:
  - `attrs.runes` survives parser -> AST JSON -> direct MIR
  - no separate Rune-only metadata channel
  - Program(JSON v0) root/body stay no-widen even when the `.hako` source-route uses a transitional selected-entry def shim

### P4. Verifier / consumer activation

Fourth code slice:

- activate compiler-owned checks only:
  - duplicate/conflicting rune rejection
  - declaration visibility checks
  - ABI contract shaping for `extern "c" fn` and ABI-facing functions

### P5. `ny-llvmc` ABI consumer

Fifth code slice:

- `ny-llvmc` becomes the first active Rune consumer
- scope is ABI-facing declaration metadata only
- semantics apply to the selected entry only
- `llvmlite` remains ignore/noop keep

## Planned Gates

1. Rust parser unit tests
   - planned: `cargo test parser_runes -- --nocapture`
2. dual-route parser parity smoke
   - planned: `bash tools/smokes/v2/profiles/integration/parser/parser_runes_dual_route_noop.sh`
3. AST / direct MIR metadata snapshot
   - planned: Rune metadata survives both parser routes with identical declaration-local `attrs.runes`
4. backend proof
   - planned: minimal `ny-llvmc` selected-entry fixture proving Rune metadata use without changing `llvmlite`

## Reopen Rule

`CURRENT_TASK.md` now carries the active Rune lane entry for this phase.
