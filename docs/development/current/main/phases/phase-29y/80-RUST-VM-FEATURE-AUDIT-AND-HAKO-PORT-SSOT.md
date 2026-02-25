---
Status: Active (new phase)
Decision: accepted
Date: 2026-02-17
Scope: app-first 運用を維持しつつ、Rust VM 依存機能を棚卸しして `.hako VM` へ計画移植する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/design/de-rust-runtime-meaning-decision-red-inventory-ssot.md
  - docs/development/current/main/phases/phase-29y/50-LANE-GATE-SSOT.md
  - docs/development/current/main/phases/phase-29y/70-APP-FIRST-OUTSOURCE-INSTRUCTIONS.md
  - src/runner/modes/vm_hako.rs
  - lang/src/vm/boxes/mir_vm_s0.hako
---

# Phase 29y.2 Rust VM Feature Audit and .hako VM Port

Lane boundary:
- この文書は de-rust lane C（runtime port）専用。
- lane A/B の順序は `docs/development/current/main/design/de-rust-lane-map-ssot.md` を参照する。

## 0. Why this phase

- app-first 実装で `vm-hako` 未対応に当たるたびに、個別回避すると場当たりになる。
- 先に「Rust VM で使っている機能」と「.hako VM の未対応」を同じ表で固定し、移植順を決める。

## 1. Fixed policy

1. app 開発は継続する（停止しない）。
2. Rust 側へ意味決定を増やさない。
3. `.hako VM` 移植は 1機能=1コミット=1gate で進める。
4. no-compat mainline gate を毎回維持する。

## 2. Scope and non-goals

In scope:
- Rust VM と `.hako VM` の機能差分棚卸し
- blocker 機能の段階移植（例: `newbox(FileBox)` など）
- 移植ごとの parity/fail-fast 契約固定

Out of scope:
- 一括移植（big-bang）
- 言語仕様拡張
- fallback 常用化

## 3. SSOT evidence (current boundary)

- `.hako VM` は subset-check で未対応を fail-fast する設計。
  - `src/runner/modes/vm_hako.rs:57`
- 現フェーズの subset は `S5l` で固定（未対応 `newbox(*)` は拒否対象）。
  - `src/runner/modes/vm_hako.rs:9`
  - `src/runner/modes/vm_hako.rs:1396`
- `.hako VM` 側の実行責務は `mir_vm_s0.hako` の subset 契約に固定。
  - `lang/src/vm/boxes/mir_vm_s0.hako:1`

## 4. Phase tasks (fixed order)

1. `RVP-1` feature matrix 作成（Rust VM usage vs .hako VM support）
2. `RVP-2` blocker top3 を確定（app実装で実際に詰まる順）
3. `RVP-3` 1機能ずつ移植（1 commit each）
4. `RVP-4` app cutover review（対象appを vm-hako へ切替）

## 5. Deliverables

1. Matrix SSOT:
   - `docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md`
2. Capability fixtures:
   - `apps/tests/vm_hako_caps/*.hako`
3. Capability smokes:
   - `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_*.sh`

## 6. Acceptance per commit

1. 追加した capability smoke が PASS
2. `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh` PASS
3. `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh` PASS

## 7. Done criteria

- matrix 上の blocker top3 が `ported` になっている
- 対象 app で Rust VM 固定フラグなしでも vm-hako route で実行可能
- red inventory に新しい Rust meaning decision が増えていない
