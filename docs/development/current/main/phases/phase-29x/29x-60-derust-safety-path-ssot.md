---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X34 `.hako` safety 経路の最小導入と lifecycle fail-fast 契約固定。
Related:
  - docs/development/current/main/phases/phase-29x/29x-59-derust-verifier-path-ssot.md
  - src/runner/modes/common_util/safety_gate.rs
  - lang/src/vm/safety_gate_skeleton.hako
  - tools/smokes/v2/profiles/integration/apps/phase29x_derust_safety_vm.sh
---

# Phase 29x X34: De-Rust Safety Path SSOT

## 0. Goal

Rust safety gate（source/lifecycle 境界）のうち、
X34 では lifecycle fail-fast 契約を `.hako` 側へ最小移管する。

## 1. Skeleton Contract

`lang/src/vm/safety_gate_skeleton.hako` は次の責務を持つ:

1. source safety（hako-like source は `route=vm-hako` 必須）
2. lifecycle safety（`reason != ok` は fail-fast）
3. clean case は `rc=0` で pass

安定タグ:

1. source freeze: `[freeze:contract][derust-safety/hako-source] route=<...> require=backend:vm-hako`
2. lifecycle freeze: `[freeze:contract][derust-safety/lifecycle] route=<...> fn=<...> bb=<...> inst_idx=<...> reason=<...>`
3. clean check: `[derust-safety/check] route=<...> status=ok source=hako-skeleton`

## 2. Smoke Contract

Canonical command:

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_safety_vm.sh`

この smoke は 2 ケースで lifecycle 契約を固定する:

1. lifecycle violation (`reason=release_strong-empty-values`) は `rc=1` + lifecycle freeze tag
2. clean case (`reason=ok`) は `rc=0` + check tag

## 3. Acceptance

1. lifecycle 契約違反が fail-fast（`rc=1`）で停止する
2. lifecycle freeze 語彙が smoke で固定される
3. `README / 29x-90 / 29x-91 / CURRENT_TASK` が X34 完了状態に同期される
4. 次タスク `X35`（strict/dev 既定を `.hako` route へ切替）へ進める

## 4. Evidence (X34)

1. `cargo check -q --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_safety_vm.sh`

## 5. Next Step

X35 で strict/dev 既定の route 主経路を `.hako` 側へ切り替える。
