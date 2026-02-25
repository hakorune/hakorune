# Phase 25.2 — LoopSnapshotMergeBox / Snapshot Merge Unification

Status: completed（LoopSnapshotMergeBox 実装・テスト・代表ケース確認まで完了）

## ゴール

- LoopForm v2 / LoopBuilder 周辺に散在していた「continue / break / exit スナップショットのマージ処理」を
  **LoopSnapshotMergeBox** という小さな箱に集約し、PHI 入力の構成ロジックを一元化する。
- FuncScannerBox.scan_all_boxes/1 の `ValueId(1283) undefined` など、
  複雑な continue/break を含むループでの SSA バグを構造的に解消する。

## 実装内容

### 1. LoopSnapshotMergeBox の導入

- 新規ファイル: `src/mir/phi_core/loop_snapshot_merge.rs`
- 役割:
  - continue_merge 経路用ヘッダ PHI 入力の統合
  - exit ブロック用 PHI 入力の統合（header fallthrough + break snapshots + body-local 対応）
  - 「全て同じ値なら PHI 不要」といった簡易最適化と、重複 predecessor の正規化
- 主なメソッド:
  - `merge_continue_for_header(preheader_id, preheader_vals, latch_id, latch_vals, continue_snapshots)`
    - preheader / latch / 各 continue スナップショットから、変数ごとのヘッダ PHI 入力 `Vec<(bb, val)>` を構成。
  - `merge_exit(header_id, header_vals, exit_snapshots, body_local_vars)`
    - header fallthrough の値と、各 break スナップショットを統合して exit PHI 入力を構成。
    - header に存在しない body-local 変数については break 経路のみから PHI 入力を作る。
  - `optimize_same_value(inputs)`
    - 全て同じ ValueId なら PHI 不要と判断し、その値を返す（単一入力も同様）。
  - `sanitize_inputs(inputs)`
    - 重複する predecessor を最後の値で 1 つに畳み、BasicBlockId 順にソートして安定化。

### 2. LoopBuilder / LoopFormBuilder からの利用

- `src/mir/loop_builder.rs`
  - canonical `continue_merge_id` を使った後段で、
    - 以前は LoopBuilder 内で continue スナップショットを手作業でマージしていたが、
    - Phase 25.2 では `LoopSnapshotMergeBox::optimize_same_value` / `sanitize_inputs` を利用して
      continue_merge ブロック上の PHI を構成し、その結果を 1 つの `merged_snapshot` として `seal_phis` に渡すように整理。
- `src/mir/phi_core/loopform_builder.rs`
  - exit PHI 構築 (`build_exit_phis`) の中で、
    - header での値（pinned/carriers + body-local で header に存在するもの）と、
    - CFG 的に有効な break スナップショットだけをフィルタリングしたリストを用意し、
    - `LoopSnapshotMergeBox::merge_exit` で変数ごとの `Vec<(bb, val)>` を構成。
    - その上で `optimize_same_value` / `sanitize_inputs` を経由して PHI を emit し、必要な場合のみ新しい ValueId を割り当てる。

この結果、continue/exit まわりの「Vec<(bb, val)> 組み立てロジック」は LoopSnapshotMergeBox に集約され、
LoopBuilder / LoopFormBuilder 側は「いつ snapshot を撮るか」「どのブロックが canonical か」に集中できるようになった。

## 動作確認とバグ修正

### 1. 代表テストケース

- ループまわりの既存テスト:
  - `mir_stageb_loop_break_continue::*` ✅
  - `mir_loopform_exit_phi::*` ✅
  - `mir_stageb_like_args_length::*` ✅
- 手書きループの確認:
  - 基本ループ: `sum=10`（0+1+2+3+4） ✅
  - break/continue を含む複雑ループ: `sum=19, i=7` ✅
  - body-local 変数を含むループ: `result=6, i=3`（exit PHI で body-local を正しく統合） ✅

### 2. FuncScannerBox.scan_all_boxes/1 の SSA バグ根治

- 以前の状態:
  - `FuncScannerBox.scan_all_boxes/1` 内の大きなループで、continue 経路やネストした if/merge を通ったときに
    `ValueId(1280)` → `1318` → `1299` → `1283` … のように未定義 ValueId が変遷しつつ発生。
  - これは loop header / exit に向かう PHI 入力が、continue/break スナップショットと header fallthrough の両方を
    部分的にしか見ていなかったことに起因していた。
- Phase 25.2 の結果:
  - LoopSnapshotMergeBox に continue / exit 経路のスナップショットマージを一元化したことで、
    - 13 個の continue を含む複雑なループでも header/exit の PHI 入力が矛盾なく生成されるようになり、
    - `ValueId(1283) undefined` を含む Undefined Value 系のエラーは再現しなくなった。

## 規模と効果

- 変更ファイル:
  - `src/mir/phi_core/loop_snapshot_merge.rs`（新規）
  - `src/mir/loop_builder.rs`（continue_merge まわりのスナップショットマージを整理）
  - `src/mir/phi_core/loopform_builder.rs`（exit PHI 構築を LoopSnapshotMergeBox 経由に変更）
- 行数ベース:
  - 追加: 約 500 行（LoopSnapshotMergeBox 本体＋11 個のテスト）
  - 削除: 約 90 行（LoopBuilder / LoopFormBuilder に散在していた ad‑hoc マージロジック）
- 効果:
  - PHI/スナップショットまわりの複雑度が大幅に低下し、今後 Stage‑B / FuncScanner / BreakFinder のループを触る際に、
    「どこを見れば continue/break のスナップショット統合ルールが分かるか」が明確になった。
  - LoopForm v2 / canonical continue_merge の設計（Phase 25.1e / 25.1q）を、実装レベルで支える小さな箱としての役割を果たしている。

## 今後への接続

- Phase 25.1e で設計した「LoopScope / IfScope の Env_in/out モデル」と、
  Phase 25.1q で導入した canonical `continue_merge` の実装を前提に、
  continue/break/exit スナップショットの統合は LoopSnapshotMergeBox に寄せる方針で定着させる。
- これにより、今後 FuncScanner や Stage‑B 側でループ構造を見直す際も、
  LoopForm/Region/スナップショット統合の責務を分離したまま小さな差分で進められるようになる。 

