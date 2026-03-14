---
Status: Accepted
Decision: accepted
Date: 2026-03-13
Scope: `phase-29ci` で `phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako` の direct CLI route と Rust / language-level mirbuilder source surface を exact call-chain で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - src/runner/modes/mir.rs
  - src/host_providers/mir_builder.rs
  - lang/src/mir/builder/MirBuilderBox.hako
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako
---

# P4 MirBuilder Route Split

## Goal

`phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako` について、

- direct CLI `--backend mir --emit-mir-json`
- Rust host-provider `source_to_mir_json(...)`
- language-level source surface `lang.mir.builder.MirBuilderBox.emit_from_source_v0`

が同じ route ではないことを exact call-chain で固定する。

これにより、

- helper cleanup
- route/boundary cleanup
- JoinIR BoxCount repair

を混ぜない delete-order を維持する。

## Exact Route Matrix

### 1. Direct CLI MIR route

- entry:
  - [src/runner/modes/mir.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/modes/mir.rs)
- chain:
  1. `NyashParser::parse_from_string(...)`
  2. `compile_with_source_hint(...)`
  3. `MirCompiler::compile_with_source(...)`
- current observed result on `phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako`:
  - default release route now lowers successfully
  - strict/dev shadow route now lowers successfully
- interpretation:
  - it does not pass through Program(JSON v0)
  - route split remains important even after this direct CLI repair, because the other source surfaces still have different ownership and delete-order

### 2. Rust host-provider source route

- entry:
  - [src/host_providers/mir_builder.rs](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder.rs)
- chain:
  1. `source_to_mir_json(...)`
  2. `emit_program_json_v0_for_strict_authority_source(...)`
  3. `program_json_to_mir_json_with_user_box_decls(...)`
  4. `runner::json_v0_bridge::parse_json_v0_to_module(...)`
  5. `module_to_mir_json(...)`
- current observed result on the same fixture:
  - lowers successfully
  - returns MIR(JSON) only on the cross-crate source surface
- interpretation:
  - this is the bootstrap-only authority helper route
  - it exercises the Program(JSON v0) bridge, not the direct JoinIR CLI route
  - `mir_builder.rs` is now the live owner for source-route handoff, explicit Program(JSON) route, shared `user_box_decls` shaping, and shared MIR(JSON) emission seam
  - [src/host_providers/mir_builder/lowering.rs](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder/lowering.rs) remains only as test-only Program(JSON)/AST(JSON) evidence, not the live blocker owner

### 3. Language-level source surface

- user-facing name:
  - `lang.mir.builder.MirBuilderBox.emit_from_source_v0`
- current runtime owner:
  - [crates/nyash_kernel/src/plugin/module_string_dispatch.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch.rs)
- chain:
  1. `handle_mir_builder_emit_from_source_v0(...)`
  2. `source_to_mir_json(...)`
- current observed result on the same fixture:
  - lowers successfully
- interpretation:
  - current success here is still a kernel-dispatch-owned source surface success
  - the transient Program(JSON) tuple no longer leaks out of the live host provider route; `source_to_program_and_mir_json(...)` is now test-only evidence
  - source-route `user_box_decls` injection is now owned inside `mir_builder.rs`
  - this must not be misread as proof that the pure `.hako` internal body in [lang/src/mir/builder/MirBuilderBox.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/mir/builder/MirBuilderBox.hako) handled the fixture on its own
  - current `.hako` inventory says the source-entry shim there is already thin (`_emit_program_json_from_source_checked(...)` -> `emit_from_program_json_v0(...)`) and the Program(JSON)->MIR route sequencing now sits behind owner-local helpers
  - the thicker `.hako` policy surface is now the Program(JSON)->MIR body plus raw/env runner lanes, not the source-entry shim itself

## Guardrails

- do not convert the previous direct CLI failure into a generic “all mirbuilder routes fail” claim
- do not convert this fixture-level direct CLI repair into a generic “all JoinIR route debt is fixed” claim
- treat this as route/boundary debt first
- keep call-chain ownership exact even when the same fixture now lowers across multiple routes

## Current Decision

- continue helper-local / boundary-local cleanup in `phase-29ci`
- keep this fixture pinned as “direct CLI release + shadow + Rust host-provider + kernel-dispatch source surface all lower”, while still treating the routes as distinct owners
- treat the direct CLI repair as a narrow route-entry BoxCount slice, not as a reason to merge shell/helper retirement with route work
- keep `P4-MIRBUILDER-ROUTE-SPLIT.md` as the exact route-evidence SSOT so later `.hako` / shell retirement work does not blur pure `.hako` proof with kernel-dispatch-owned success
