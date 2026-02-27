---
Status: Active
Decision: accepted
Date: 2026-02-27
Scope: plugin lane を「Rust plugin 維持 + `.hako` plugin 拡張」の二重運用で進め、最終的に Rust plugin を retire する順序を固定する。
Related:
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/design/de-rust-scope-decision-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - docs/development/current/main/design/code-retirement-history-policy-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-104-plg04-filebox-wave1-min6-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-177-wsm-p4-min7-buffer-file-binary-contract-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - CURRENT_TASK.md
---

# 29cc-178 PLG-07 Plugin De-Rust Cutover Order SSOT

## 0. Purpose

`de-rust done (non-plugin)` を壊さずに、plugin lane だけを再起動して
「Rust plugin と `.hako` plugin の同時保守 -> `.hako` 主経路化 -> Rust plugin retire」
を段階実施する。

## 1. Why dual-run first

1. portability（Windows/将来 macOS）を壊さずに `.hako` 側を拡張するため。
2. plugin ABI の契約破壊を避けるため（先に parity を固定）。
3. Rust plugin を即削除すると rollback 距離が長くなるため。

## 2. Fixed Order (implementation)

1. `PLG-07-min1` docs lock
   - 追加メソッド契約を docs-first で固定（1 plugin family ずつ）。
   - 先行対象: `FileBox.readBytes/writeBytes`。
2. `PLG-07-min2` Rust plugin parity
   - Rust plugin 実装に同契約を追加して gate を固定。
   - 既存利用者を壊さない非破壊追加のみ。
3. `PLG-07-min3` `.hako` plugin parity
   - `.hako` plugin 実装に同契約を追加。
   - Rust plugin と同一 fixture を使って parity 比較。
4. `PLG-07-min4` dual-run gate lock
   - Rust plugin route / `.hako` plugin route を同じ入力で比較し、出力同値を lock。
5. `PLG-07-min5` default switch
   - 既定 route を `.hako` plugin に切替。
   - Rust plugin は compat route（既定OFF）へ縮退。
6. `PLG-07-min6` Rust plugin retire decision
   - retire 条件を満たした場合のみ Rust plugin route を撤去。
7. `PLG-07-min7` Rust compat route retire execution
   - min6 readiness 条件を満たした後、compat/dual-run 導線を retire して `.hako` 単一路線へ固定。

## 3. Rust plugin retire/archive criteria (strict)

`PLG-07-min6` は次を全て満たした時のみ許可する。

1. dual-run parity gate が連続 2 milestone 緑。
2. `tools/checks/dev_gate.sh portability` が連続 2 milestone 緑。
3. `.hako` plugin route で FileBox binary API を使う実ケース（fixture+smoke）が緑。
4. rollback 手順（compat route 再有効化）が docs に固定済み。

注記:
- 「archive」はコードコピー退避ではなく route retirement を意味する。
- 旧 Rust 実装の別名保存はしない（`code-retirement-history-policy-ssot.md` 準拠、履歴は git を正本）。

## 4. Immediate next

1. `PLG-07-min1/min2` は `29cc-179` で lock 済み。
2. `PLG-07-min3` は `29cc-180` で lock 済み。
3. `PLG-07-min4` は `29cc-181` で lock 済み。
4. `PLG-07-min5` は `29cc-182` で lock 済み。
5. `PLG-07-min6` は `29cc-183` で readiness lock 済み。
6. `PLG-07-min7` は `29cc-204` で retire execution lock 済み。
7. 次は `none`（PLG-07 closeout complete; monitor-only）。

## 5. Non-goals

1. BufferBox を即 plugin 化すること（現時点は core route を維持）。
2. plugin lane と wasm lane の同時大規模切替。
3. fallback/silent recovery の導入。
