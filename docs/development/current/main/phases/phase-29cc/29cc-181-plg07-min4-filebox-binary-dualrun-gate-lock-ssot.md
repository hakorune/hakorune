---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: PLG-07-min4 として FileBox binary API の dual-run parity gate（Rust route / `.hako` route）を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-178-plg07-plugin-derust-cutover-order-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-180-plg07-min3-filebox-binary-hako-parity-lock-ssot.md
  - tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_dualrun_vm.sh
  - tools/checks/phase29cc_plg07_filebox_binary_dualrun_guard.sh
---

# 29cc-181 PLG-07-min4 FileBox Binary Dual-Run Gate Lock

## Purpose
Rust route と `.hako` route を同じ payload 契約で比較し、min5 default switch 判定の前提を固定する。

## Decision
1. dual-run smoke を追加し、Rust fixture と `.hako` fixture を同一条件で実行する。
2. 2ルートの marker 値を比較し、同値でない場合は fail-fast する。
3. payload file の値も両ルートで検証し、非破壊同値を lock する。
4. guard script を追加し、milestone 判定コマンドを 1 入口にする。

## Contract
1. dual-run 実行時の provider policy は strict-plugin-first で固定する。
2. parity 判定は marker 値（`file_bytes` / `file_bytes_hako`）の同値で行う。
3. payload 値は `PLG07_BINARY_OK` 固定。
4. いずれか 1 ケース失敗で gate 全体を失敗させる（silent fallback 禁止）。

## Acceptance
1. `cargo check --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_dualrun_vm.sh`
3. `bash tools/checks/phase29cc_plg07_filebox_binary_dualrun_guard.sh`

## Next
1. `PLG-07-min5` は `29cc-182` で lock 済み。
2. `PLG-07-min6` は `29cc-183` で readiness lock 済み。
3. 次は `PLG-07-min7` retire execution lock（accepted-but-blocked）へ進む。
