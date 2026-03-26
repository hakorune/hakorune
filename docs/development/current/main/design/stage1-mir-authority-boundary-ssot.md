---
Status: SSOT
Decision: accepted
Date: 2026-03-26
Scope: Stage1 canonical MIR の authority/materializer/consumer 境界を固定し、Rust line の meaning/policy residue をどこまで削るかを 1 枚で読む。
Related:
  - docs/development/current/main/design/stage1-mir-dialect-contract-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P16-STAGE1-CANONICAL-MIR-CUTOVER.md
  - lang/src/runner/stage1_cli_env.hako
  - lang/src/mir/builder/MirBuilderBox.hako
  - lang/src/mir/builder/func_lowering/call_methodize_box.hako
  - src/host_providers/mir_builder/handoff.rs
  - src/runner/mir_json_emit/mod.rs
  - src/runner/mir_json_emit/emitters/calls.rs
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
  - tools/hakorune_emit_mir.sh
---

# Stage1 MIR Authority Boundary (SSOT)

## Purpose

- `Stage1 -> Stage2` の clean target を `Rust 0 lines` ではなく `meaning/policy owner cutover` として固定する。
- `.hako` / Rust / native consumer のどこが MIR dialect の意味を持ってよいかを明示する。
- `P16` の exact task を `.hako canonicalize -> Rust materializer demote -> pure-first semantic widening` の順に固定する。

## Boundary Lock

### 1. Canonical authority

- Stage1 canonical MIR dialect owner は `.hako` だよ。
- `.hako` 側が owner するもの:
  - canonical op set
  - method/constructor/global call canonicalization policy
  - route / fallback policy
  - normalization contract
- current owner route:
  - `lang/src/runner/stage1_cli_env.hako`
  - `lang/src/mir/builder/MirBuilderBox.hako`
  - `lang/src/mir/builder/func_lowering/call_methodize_box.hako`

### 2. Rust materializer seam

- Rust line は残ってよいが、long-term で dialect policy owner にはしない。
- Rust 側が当面持ってよいもの:
  - parse / validate
  - metadata / path / env handoff
  - schema-preserving serialization
  - compat export
- current live seam:
  - `src/host_providers/mir_builder/handoff.rs`
  - `src/runner/mir_json_emit/mod.rs`
  - `src/runner/mir_json_emit/emitters/calls.rs`
- current truth:
  - `calls.rs` はまだ `mir_call` と `boxcall` の choice を materialize している
  - したがって today の Rust seam は単なる serializer ではなく `dialect materializer with policy residue` だよ

### 3. Native consumer boundary

- `Stage2` native consumer は canonical MIR consumer に徹する。
- dialect policy をここへ押し戻さない。
- current consumer:
  - `lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc`
- consumer の責務:
  - canonical MIR を受ける
  - semantic coverage を広げる
  - dialect split を抱え込まない

## Current Mixed State

1. `.hako` 側には canonicalization pieces が既にある
2. Rust 側は still-live materializer seam として外向き MIR(JSON) を構築している
3. tools/wrappers には mixed operation が残っている
4. したがって current repo は `.hako authority target / Rust materializer current / native consumer` の混成期だよ

## Cutover Rule

- 成功条件は「Rust を消すこと」ではない
- 成功条件は「schema choice / dialect policy が Rust に残っていないこと」だよ
- `calls.rs` に残してよいのは:
  - schema-preserving emit
  - explicit compat/export-only lane
- `calls.rs` に残してはいけないのは:
  - mainline Stage1 dialect choice
  - `boxcall` vs `mir_call` の policy decision

## Fixed Order

1. authority boundary を docs で lock する
2. `.hako` Stage1 producer を canonical MIR authority に寄せる
3. Rust `mir_json_emit` を `dialect materializer` から `thin materializer/transport seam` へ降格する
4. mixed tool route を authority 読みに同期する
5. その後で pure-first semantic widening / perf optimization を再開する

## Non-Goals

- この wave で Rust source を大量 delete すること
- pure-first consumer に broad `boxcall` support を足して設計差を隠すこと
- wrapper fallback の都合で canonical authority を曖昧にすること

## Exit Condition

- `.hako` が Stage1 canonical MIR authority だと docs/code/probe で一貫して読める
- Rust seam は schema choice を持たず、canonical MIR を materialize/transport するだけになる
- Stage2 native consumer は canonical MIR consumer としてだけ widen される
