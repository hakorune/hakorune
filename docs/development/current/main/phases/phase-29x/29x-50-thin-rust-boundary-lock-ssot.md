---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X24 thin-rust boundary lock（route orchestration / verifier / safety の責務境界固定）。
Related:
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md
  - docs/development/current/main/phases/phase-29x/29x-91-task-board.md
  - src/runner/dispatch.rs
  - src/runner/selfhost.rs
  - src/runner/modes/common_util/selfhost/runtime_route_contract.rs
---

# Phase 29x X24: Thin-Rust Boundary Lock SSOT

## 0. Goal

X24 の目的は、de-rust 移行中に Rust 側へ責務が再拡散しないように、
`route orchestration / verifier / safety` の境界を 1 枚で固定すること。

この文書は「実装の詳細」ではなく「責務の境界」を定義する。
実装変更は X25-X31 でこの境界に沿って行う。

## 1. Thin-Rust Responsibility Split

1. Route orchestration（Rust thin で保持）
   - backend/lane の選択と route 理由タグの出力
   - compat lane の明示 opt-in 判定
   - 子プロセス/entrypoint の起動制御
2. Verifier gate（Rust thin で保持）
   - route 決定後、backend 実行前の契約検査
   - 契約不一致は fail-fast（救済 fallback なし）
3. Safety gate（Rust thin で保持）
   - lifecycle/unsafe 境界の runtime 契約チェック
   - ABI 境界の不整合検知（args borrowed / return owned 契約）

## 2. Boundary Contract (must)

1. `Program(JSON) -> MIR(JSON)` 変換は route/safety 層へ再導入しない。
2. route 判定は 1 箇所（orchestrator）で行い、mode 側で再判定しない。
3. verifier/safety は route の下流で 1 箇所ずつ呼び、複数入口を作らない。
4. strict/dev で暗黙 fallback は禁止。必要時はタグ付き fail-fast。

## 3. Anti-Patterns (forbidden)

- route 判定の重複実装（dispatch と mode の二重判断）
- verifier の救済実行（warn のみで継続）
- safety 失敗時の silent continue
- compat lane を既定挙動へ混入させる変更

## 4. Sequence Lock (X25-X31)

1. X25: route orchestration 入口一本化
2. X26: route observability 契約固定
3. X27: compat bypass fail-fast 化
4. X28: verifier gate 一本化
5. X29: safety gate 一本化
6. X30: thin-rust Core C ABI 最小面固定
7. X31: thin-rust gate pack 固定

順序を入れ替えて verifier/safety の一本化を先に行うことを禁止する。

## 5. Evidence for X24

Docs:
- `docs/development/current/main/phases/phase-29x/29x-50-thin-rust-boundary-lock-ssot.md`
- `docs/development/current/main/phases/phase-29x/README.md`
- `docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md`
- `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`

Code anchors:
- `src/runner/dispatch.rs`
- `src/runner/selfhost.rs`
- `src/runner/modes/common_util/selfhost/runtime_route_contract.rs`
