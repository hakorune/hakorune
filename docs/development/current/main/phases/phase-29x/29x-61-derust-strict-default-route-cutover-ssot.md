---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X35 strict/dev 既定 route を `.hako` route 契約へ切り替える。
Related:
  - docs/development/current/main/phases/phase-29x/29x-58-derust-route-orchestrator-skeleton-ssot.md
  - src/runner/route_orchestrator.rs
  - tools/smokes/v2/profiles/integration/apps/phase29x_derust_strict_default_route_vm.sh
---

# Phase 29x X35: De-Rust Strict Default Route Cutover SSOT

## 0. Goal

strict/dev 既定の `--backend vm` で、
route 選択の主導を `.hako` route 契約（skeleton parity）へ切り替える。

## 1. Contract

`src/runner/route_orchestrator.rs` で、`NYASH_VM_ROUTE_TRACE=1` 時に
`[derust-route/select]` を観測可能にする。

安定タグ:

1. strict/dev default: `[derust-route/select] backend=vm lane=vm-hako source=hako-skeleton reason=strict-dev-prefer`
2. strict/dev explicit rust-thin: `[derust-route/select] backend=vm lane=vm source=rust-thin-explicit reason=default`
3. strict/dev explicit fallback: `[derust-route/select] backend=vm lane=compat-fallback source=hako-skeleton reason=env:NYASH_VM_USE_FALLBACK=1`

## 2. Acceptance

1. strict/dev 既定で `source=hako-skeleton` が観測される
2. Rust thin は `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` 明示時のみ `source=rust-thin-explicit` で観測される
3. `README / 29x-90 / 29x-91 / CURRENT_TASK` が X35 完了状態に同期される
4. 次タスク `X36`（de-rust done 同期）へ進める

## 3. Evidence (X35)

1. `cargo check -q --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_strict_default_route_vm.sh`

## 4. Next Step

X36 で de-rust done 判定・rollback 条件・証跡リンクを docs へ最終同期する。
