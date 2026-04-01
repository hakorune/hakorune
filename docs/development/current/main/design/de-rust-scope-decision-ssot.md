---
Status: SSOT
Decision: accepted
Date: 2026-02-25
Scope: de-rust done 宣言の対象範囲（non-plugin / plugin）を固定する。
Related:
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md
  - docs/development/current/main/phases/archive/phase-29cc/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-92-non-plugin-rust-residue-task-set.md
  - docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md
  - CURRENT_TASK.md
---

# De-Rust Scope Decision (SSOT)

## 0. Purpose

- de-rust done 判定に「どこまで含めるか」を 1 箇所で固定する。
- non-plugin lane の closeout を、plugin 移植の有無と分離して運用する。
- L5 scope decision を明示契約にして、done 宣言時の解釈差分をなくす。

## 1. Decision (accepted)

1. de-rust done 宣言の対象は `non-plugin` の runtime/compiler 導線に限定する。
2. plugin 実装の全面 `.hako` 置換は de-rust done 宣言の必須条件に含めない。
3. plugin 置換は separate lane（future decision）として扱い、必要時のみ blocker を起動する。

## 2. Rationale

1. 既存 lane A/B/C + 29cc の完了判定を、plugin 由来の長期課題で遅延させないため。
2. selfhost mainline の受理条件（planner-required / lane gate）を短い手順で維持するため。
3. cross-platform 維持（Linux/WSL Windows smoke、将来 macOS）を、Rust plugin baseline で安定化するため。

## 3. Done Declaration Binding

de-rust done 宣言は次を全て満たした時のみ許可する。

1. `de-rust-master-task-map-ssot.md` の L1-L4 が充足済み。
2. `phase-29x/29x-62-derust-done-sync-ssot.md` の X32-X35 replay が PASS。
3. 本文書の decision が accepted のまま維持されている。

## 4. Reopen Rule (L5 only)

次のいずれかが発生した場合、L5 scope decision を failure-driven で reopen する。

1. de-rust done 判定に plugin 置換を必須化する提案が出た場合。
2. non-plugin lane の gate が plugin 実装に依存して green 維持できなくなった場合。
3. release/policy 上の理由で done 対象範囲の変更が必要になった場合。

reopen 時の固定手順:
1. docs-first で decision を provisional に戻す。
2. `CURRENT_TASK.md` に blocker 2行を追記する。
3. 1 blocker = 1 commit で scope 変更を反映する。

## 5. Operational Note

- plugin lane を起動しない期間は、`non-plugin done` 判定を維持する。
- plugin 側の保守は「互換維持・ビルド維持」を優先し、仕様拡張とは分離する。
- plugin 移植準備は `phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md` を正本とする。
- plugin lane `PLG-01` acceptance は `phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md` を正本とする。
- plugin lane `PLG-02` gate pack lock は `phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md` を正本とする。
