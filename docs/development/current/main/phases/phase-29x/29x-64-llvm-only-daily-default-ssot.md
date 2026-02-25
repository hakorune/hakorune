---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X38 daily/milestone の既定入口を LLVM-only lane へ切り替える。
Related:
  - docs/development/current/main/phases/phase-29x/29x-63-llvm-cabi-link-gate-ssot.md
  - tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh
  - tools/checks/abi_lane_guard.sh
---

# Phase 29x X38: LLVM-Only Daily Default SSOT

## 0. Goal

Phase 29x の daily/milestone 入口を
「LLVM+C ABI line を先に確認する」運用へ切り替える。

これにより、日常導線で Rust runtime lane の重い gate を必須にしない。

## 1. Contract

`phase29x_llvm_only_daily_gate.sh` を daily default gate とし、次を固定する。

1. `abi_lane_guard.sh` が PASS（非canonical ABI 混入なし）
2. `phase29x_llvm_cabi_link_min.sh` が PASS（X37 契約）
3. 失敗時は step 名を fail-fast で出して停止

補足:
- selfhost runtime route / joinir heavy gate は「節目の互換確認」に残す。
- default daily では Rust runtime build を必須手順にしない。

## 2. Acceptance

1. `phase29x_llvm_only_daily_gate.sh` が PASS
2. `README / 29x-90 / 29x-91 / CURRENT_TASK` が X38 完了状態に同期
3. 次タスク `X39`（Rust lane を tools/compat 専用へ隔離）へ進める

## 3. Evidence (X38)

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh`

## 4. Next Step

X39 で Rust lane を compat 専用コマンドへ隔離し、
通常運用から明示 opt-in へ段階縮退する。
