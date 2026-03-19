---
Status: SSOT
Decision: provisional
Date: 2026-03-19
Scope: `0rust` を「Rust meaning owner zero」と定義し、Rust ベースの build/bootstrap route を常時保持する契約を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cm/README.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/backend-legacy-preservation-and-archive-ssot.md
---

# 0rust Buildability Contract (SSOT)

## Purpose

- `0rust` は Rust を意味の owner から降ろすための定義であり、Rust の buildability を消すための定義ではない。
- daily mainline の owner が `.hako` 側へ移っても、repo は Rust ベースの build/bootstrap route を常時保持する。
- buildability は workaround ではなく contract である。

## Boundary Lock

1. meaning / policy owner は `.hako` 側へ移してよい。
2. Rust は build/bootstrap / substrate / compat keep として残してよい。
3. Rust ベースの build route は、migration slice の前後でいつでも再実行できる状態を保つ。
4. daily owner が Rust から外れても、Rust build route を silent delete しない。

## What Must Remain Buildable

- stage1 / bootstrap build paths
- compat / canary build paths
- `.hako` mainline を Rust から再構築するための最小導線
- archive / preservation-first restore path

## What `0rust` Is Not

- `Rust source を全消しする` policy ではない
- `build only from .hako` に固定することではない
- buildability を失う代わりに LOC を減らすことではない
- ring0 へ semantics を押し込むことではない

## Relationship to Current Phases

- `phase-29cm`: kernel authority migration の fixed order を進めつつ、Rust ベースの buildability は維持する
- `phase-29ck` / `de-rust-*`: backend/runtime cutover で Rust build route を壊さない

## Done Shape

- meaning owner は `.hako` にある
- Rust は build/bootstrap / substrate / compat keep として再実行可能である
- migration slice を切った後でも、Rust から build できることを確認できる

