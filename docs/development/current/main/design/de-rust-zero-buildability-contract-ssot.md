---
Status: SSOT
Decision: provisional
Date: 2026-03-31
Scope: `0rust` を「Rust meaning owner zero」と定義し、daily/distribution を原則 Rust/Cargo 非依存にしつつ、Rust ベースの build/bootstrap route を常時保持する契約を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/kernel-replacement-axis-ssot.md
  - docs/development/current/main/phases/phase-29cm/README.md
  - docs/development/current/main/design/de-rust-backend-zero-fixed-order-and-buildability-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/backend-legacy-preservation-and-archive-ssot.md
---

# 0rust Buildability Contract (SSOT)

## Purpose

- `0rust` は Rust を意味の owner から降ろすための定義であり、Rust の buildability を消すための定義ではない。
- `0rust` の default reading は、daily/distribution が通常運用で Rust/Cargo を要求しないこと。
- daily mainline の owner が `.hako` 側へ移っても、repo は Rust ベースの build/bootstrap route を常時保持する。
- この default は `K2-core` と `K2-wide` の両方で維持する。
- operational reading は `stage0 Rust bootstrap keep / stage1 proof line / stage2-mainline zero-rust daily mainline / stage2+ umbrella` で固定する。
- buildability は workaround ではなく contract である。

## Boundary Lock

1. meaning / policy owner は `.hako` 側へ移してよい。
2. Rust は build/bootstrap / substrate / compat keep として残してよい。
3. Rust ベースの build route は、migration slice の前後でいつでも再実行できる状態を保つ。
4. daily owner が Rust から外れても、Rust build route を silent delete しない。
5. `stage0` は Rust bootstrap でよいが、`stage2-mainline` daily mainline と standard distribution は Rust/Cargo dependency ではない状態を default target にする。
6. bootstrap / recovery / reference / archive / canary routes は explicit keep として残してよい。

## What Must Remain Buildable

- stage0 / bootstrap build paths
- stage1 proof artifact build paths
- stage2-mainline daily mainline rebuild path
- stage2-mainline daily mainline の reference rebuild path
- compat / canary build paths
- `.hako` mainline を Rust から再構築するための最小導線
- archive / preservation-first restore path

## What `0rust` Is Not

- `Rust source を全消しする` policy ではない
- `build only from .hako` に固定することではない
- buildability を失う代わりに LOC を減らすことではない
- `native zero` や `metal zero` の意味ではない
- ring0 へ semantics を押し込むことではない
- `stage0 bootstrap まで Rust-free` を immediate acceptance にすることではない

## Relationship to Current Phases

- `phase-29cm`: kernel authority migration の fixed order を進めつつ、Rust ベースの buildability は維持する
- `phase-29ck` / `de-rust-*`: backend/runtime cutover で Rust build route を壊さない
- backend-zero の daily order は `de-rust-backend-zero-fixed-order-and-buildability-ssot.md` を正本にする

## Done Shape

- meaning owner は `.hako` にある
- Rust は build/bootstrap / substrate / compat keep として再実行可能である
- `stage0` first build / recovery lane として Rust を残してよい
- `stage2-mainline` daily mainline build と standard distribution は `.hako` owner で回り、Rust/Cargo は user-facing normal dependency ではない。`stage2+` は umbrella / end-state 読みである
- migration slice を切った後でも、Rust から build できることを確認できる
