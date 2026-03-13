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
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - lang/src/mir/builder/MirBuilderBox.hako
  - apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako
---

# P4 MirBuilder Route Split

## Goal

`phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako` について、

- direct CLI `--backend mir --emit-mir-json`
- Rust host-provider `source_to_program_and_mir_json(...)`
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
  - still rejects with `[joinir/freeze]`
  - reject reason includes `nested_loop_not_allowed`
- interpretation:
  - this is the current direct JoinIR route blocker
  - it does not pass through Program(JSON v0)

### 2. Rust host-provider source route

- entry:
  - [src/host_providers/mir_builder.rs](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder.rs)
- chain:
  1. `emit_program_json_v0_for_strict_authority_source(...)`
  2. `program_json_to_mir_json(...)`
  3. `runner::json_v0_bridge::parse_json_v0_to_module_with_imports(...)`
- current observed result on the same fixture:
  - lowers successfully
  - returns Program(JSON v0) plus MIR(JSON)
- interpretation:
  - this is a bootstrap-only authority helper route
  - it exercises the JSON bridge, not the direct JoinIR CLI route

### 3. Language-level source surface

- user-facing name:
  - `lang.mir.builder.MirBuilderBox.emit_from_source_v0`
- current runtime owner:
  - [crates/nyash_kernel/src/plugin/module_string_dispatch.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch.rs)
- chain:
  1. `handle_mir_builder_emit_from_source_v0(...)`
  2. `source_to_program_and_mir_json(...)`
  3. `inject_stage1_user_box_decls_from_program_json(...)`
- current observed result on the same fixture:
  - lowers successfully
- interpretation:
  - current success here is a kernel-dispatch-owned source surface success
  - this must not be misread as proof that the pure `.hako` internal body in [lang/src/mir/builder/MirBuilderBox.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/mir/builder/MirBuilderBox.hako) handled the fixture on its own

## Guardrails

- do not convert the direct CLI failure into a generic “all mirbuilder routes fail” claim
- do not convert the Rust / kernel-dispatch success into a generic “JoinIR route is fixed” claim
- treat this as route/boundary debt first
- only promote a BoxCount repair slice after the same reject point is pinned for the exact direct route that still fails

## Current Decision

- continue helper-local / boundary-local cleanup in `phase-29ci`
- keep the direct CLI reject pinned as its own route
- when direct CLI repair starts, scope it as a separate BoxCount slice instead of mixing it with shell/helper retirement
