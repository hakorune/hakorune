# Phase 256: StringUtils.split/2 route support（historical labels `6/7`）

Status: Completed
Scope: Loop route recognition for split/tokenization operations
Related:
- Phase 255 完了（loop_invariants 導入、scan_with_init route 完成）
- Phase 254 完了（scan_with_init route / historical label `6` index_of 実装）
- North star（設計目標）: `docs/development/current/main/design/join-explicit-cfg-construction.md`

## Current Status (SSOT)

- Current first FAIL: `json_lint_vm / StringUtils.last_index_of/2`（loop with early return shape - unsupported）
- `StringUtils.split/2` は VM `--verify` / smoke まで PASS
- scan_with_init route（historical label `6` index_of lane）は PASS 維持
- 次フェーズ: Phase 257（`last_index_of/2` の reverse scan + early return loop）
- 直近の完了:
  - P1.13: LoopBreak（historical label: `2`）boundary entry_param_mismatch 根治（`join_module.entry.params` SSOT 化）
  - P1.13.5（= Phase 256.8.5）: LoopContinueOnly / ScanWithInit / SplitScan（historical labels: `4/6/7`）でも `boundary.join_inputs` を `join_module.entry.params` SSOT に統一（hardcoded ValueId/PARAM_MIN を撤去）
  - P1.10: DCE が `jump_args` 参照を保持し、`instruction_spans` と同期するよう修正（回帰テスト追加）
  - P1.7: SSA undef（`%49/%67`）根治（continuation 関数名の SSOT 不一致）
  - P1.6: pipeline contract checks を `run_all_pipeline_checks()` に集約
- 次の作業: Phase 257（last_index_of pattern - loop with return support）
- 設計メモ（ChatGPT Pro 相談まとめ）: `docs/development/current/main/investigations/phase-256-joinir-contract-questions.md`
- Known issue（非ブロッカー）:
  - SplitScan（historical label: `7`）integration smoke で `phi predecessor mismatch` が残っている（今回の boundary SSOT 統一とは独立）

---

## Background (P0 Archive)

このセクションは初期の失敗詳細とP0設計の記録。現状の作業は上記の Current Status を参照。

### 失敗詳細

**テスト**: json_lint_vm (quick profile)
**エラー**: `[joinir/freeze] Loop lowering failed: JoinIR does not support this pattern`
**関数**: `StringUtils.split/2`

#### エラーメッセージ全体

```
[trace:dev] loop_canonicalizer:   Decision: FAIL_FAST
[trace:dev] loop_canonicalizer:   Missing caps: [ConstStep]
[trace:dev] loop_canonicalizer:   Reason: Phase 143-P2: Loop does not match read_digits(loop(true)),
    skip_whitespace, parse_number, continue, parse_string, or parse_array pattern
[ERROR] ❌ MIR compilation error: [joinir/freeze] Loop lowering failed: JoinIR does not support this pattern,
    and LoopBuilder has been removed.
Function: StringUtils.split/2
Hint: This loop pattern is not supported. All loops must use JoinIR lowering.
```

### 期待される動作

`StringUtils.split(s, separator)` が正常にコンパイルされ、文字列分割が動作すること。

### 実際の動作

Loop canonicalizer が `ConstStep` を要求しているが、このループはステップが複雑で定数ではない。

### 最小再現コード

```nyash
split(s, separator) {
    local result = new ArrayBox()

    // Early return for empty separator
    if separator.length() == 0 {
        result.push(s)
        return result
    }

    local start = 0
    local i = 0

    // Main scan loop
    loop(i <= s.length() - separator.length()) {
        if s.substring(i, i + separator.length()) == separator {
            result.push(s.substring(start, i))
            start = i + separator.length()
            i = start  // Variable step - moves by separator.length()
        } else {
            i = i + 1  // Constant step - moves by 1
        }
    }

    // Push remaining segment
    if start <= s.length() {
        result.push(s.substring(start, s.length()))
    }

    return result
}
```

### 分析

#### ループ構造

1. **条件**: `i <= s.length() - separator.length()`
2. **ボディ**:
   - If branch: マッチング検出時
     - `result.push()` でセグメント追加
     - `start = i + separator.length()` で次の開始位置更新
     - `i = start` で大きくジャンプ（可変ステップ）
   - Else branch: マッチなし
     - `i = i + 1` で 1 進む（定数ステップ）
3. **特徴**:
   - **可変ステップ**: マッチング時は `separator.length()` 分ジャンプ
   - **複数キャリア**: `i`, `start`, `result` を更新
   - **MethodCall**: `substring()`, `push()`, `length()` を使用

#### Canonicalizer の問題

```
Missing caps: [ConstStep]
```

- 既存の early route families（historical labels `1-6`）は定数ステップを想定
- このループは条件分岐で異なるステップ幅を使う
- loop_break family（historical label `2`; 당시 balanced_depth_scan 系）に近いが、可変ステップがネック

### 実装計画

#### Option A: Split/Tokenization route family（historical label `7`）

**新しい route family 追加**:
- 可変ステップサポート
- 複数キャリア（i, start, accumulator）
- If-else での異なるステップ幅処理

**検出条件**:
1. Loop condition: `i <= expr - len`
2. Body has if statement:
   - Then: `i = something_big` (可変ジャンプ)
   - Else: `i = i + 1` (定数ステップ)
3. Accumulator への追加操作 (`push` など)

#### Option B: loop_break family（historical label `2`）拡張

**既存 loop_break family を拡張**:
- ConstStep 要件を緩和
- If-else で異なるステップ幅を許可
- balanced_depth_scan_policy を拡張

#### Option C: Normalization 経路

**ループ正規化で対応**:
- 可変ステップを定数ステップに変換
- Carrier 追加で状態管理

### 次のステップ（P0時点の初期計画）

1. **StepTree 詳細解析**: split ループの完全な AST 構造確認
2. **類似パターン調査**: 他の可変ステップループ（indexOf, contains など）
3. **Option 選択**: historical label `7` 新設 vs label `2` 拡張 vs Normalization
4. **実装戦略策定**: 選択した Option の詳細設計

---

## Phase 256 指示書（P0 / 完了済み）

### 目標

- `StringUtils.split/2` の loop を JoinIR で受理し、`json_lint_vm` を PASS に戻す。
- by-name 分岐禁止（`StringUtils.split/2` だけを特別扱いしない）。
- workaround 禁止（fallback は作らない）。

### 推奨方針（P0）

Option A（SplitScan route family / historical label `7`）を推奨。

理由:
- 可変 step（then: `i = start` / else: `i = i + 1`）は既存の ConstStep 前提と相性が悪い。
- loop_break family（historical label `2`）を膨らませず、tokenization 系の専用 route family として箱化した方が責務が綺麗。

### P0 タスク

1) 最小 fixture + v2 smoke（integration）
- `apps/tests/phase256_p0_split_min.hako`
- `tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_llvm_exe.sh`

2) DetectorBox（構造のみ）
- ループ条件が `i <= s.length() - sep.length()` 形
- body に `if substring(i, i + sep.length()) == sep { ... i = start } else { i = i + 1 }`
- `result.push(...)` を含む（ArrayBox accumulator）
- ループ後に “残り push” がある（任意だがあると精度が上がる）

3) 抽出箱（Parts）
- `i` / `start` / `result` / `s` / `separator` を抽出
- then/else の更新式（可変 step と const step）を抽出

4) JoinIR lowerer（専用）
- loop_state: `i`, `start`
- invariants: `s`, `separator`, `result`（result は更新されるので carrier 扱いが必要なら role を明確に）
- then/else で異なる `i_next` を `Select` もしくは branch で表現（設計 SSOT は JoinIR 側で決める）

5) 検証
- integration smokes 2本が PASS
- `./tools/smokes/v2/run.sh --profile quick` の最初の FAIL が次へ進む

### 注意（P0ではやらない）
- 既存 Pattern の大改造（Pattern 2 の全面拡張）は避ける
- 正規化（Normalization 経路）は P1 以降の検討に回す

## 備考

- Phase 255 で loop_invariants が導入されたが、このケースは invariants 以前の問題（可変ステップ）
- Phase 254-256 の流れで Pattern 6 → Pattern 7 の自然な進化が期待される
- split/tokenization は一般的なパターンなので、汎用的な解決策が望ましい

---

## 進捗（P0/P1）

### P0: SplitScan 基本実装（historical label `7`, 完了）

- Fixture & smokes（integration）:
  - `apps/tests/phase256_p0_split_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_vm.sh`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_llvm_exe.sh`
- SplitScan route family:
  - Detector / Extractor / JoinIR lowerer / MirBuilder 統合まで実装
  - JoinIR lowerer は可変 step を `JoinInst::Select` で表現

### P1: Carrier/param 配線の整流（完了）

- “Carriers-first” の順序を SSOT として固定し、latch incoming 不足を解消
- exit bindings を明示的に構築（`i`, `start`）

### P1.5: JoinIR→MIR 変換の根治（進行中）

背景:
- Pattern 7 は `JoinInst::Select` を使用するが、JoinIR→MIR 変換経路で Select が未対応だったため、
  “dst が定義されない ValueId” が発生し得る。

対応（完了）:
- boundary の伝播を bridge 経路全体へ追加
- `MirInstruction::Select` の追加、および JoinIR→MIR 変換 / remap / value collection を実装

現状（ブロッカー）:
- integration VM が still FAIL:
  - `use of undefined value ValueId(57)`（`StringUtils.split/2`）

#### 追加の事実（2025-12-20）

- Pattern 6（index_of）回帰: PASS（Select 導入後もインフラは健全）
- `ValueId(57)` の正体:
  - JoinIR `ValueId(1004)` の remap 先（JoinIR→Host の remap は成功している）
  - `sep_len = sep.length()` のローカル値（loop_step 内で `BoxCall("length")` で定義される想定）
- つまり「remap の欠落」ではなく、「定義命令（dst を持つ BoxCall）が MIR に落ちていない/順序が壊れている」問題が濃厚

#### 次（P1.5 Task 3）: sep_len 定義トレース（SSOT）

目的: `sep_len` の「定義（dst）」が MIR に存在し、かつ use より前に配置されることを確認し、欠落しているなら根治する。

1) MIR ダンプで「%57 の定義があるか」を確認
   - `./target/release/hakorune --backend vm --dump-mir --mir-verbose apps/tests/phase256_p0_split_min.hako > /tmp/split.mir`
   - `rg -n \"\\bValueId\\(57\\)\\b|%57\" /tmp/split.mir`
   - 期待: `BoxCall` かそれに相当する命令で `dst=%57` が use より前に出る

2) 定義が無い場合: JoinIR→MIR converter を疑う
   - `JoinInst::Compute(MirLikeInst::BoxCall { dst: Some(sep_len), method: \"length\", .. })`
     が `MirInstruction::BoxCall { dst: Some(remapped), .. }` として出力されているかを点検する
   - もし `dst: None` になっている／命令自体が落ちているなら、converter か value-collector の取りこぼしを修正する

3) 定義があるが use より後の場合: ブロック内の命令順の生成/マージ規約を疑う
   - joinir→mir の「生成順」か merge の「挿入位置」がおかしい
   - 期待: “def-before-use” が各 BasicBlock 内で成立する

4) 受け入れ基準（P1.5）
   - `tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_vm.sh` が PASS
   - `tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_llvm_exe.sh` が PASS
- 既存: `tools/smokes/v2/profiles/integration/apps/archive/phase254_p0_index_of_vm.sh` が PASS 維持

#### 診断アップデート（2025-12-20）

`ValueId(57)`（= `sep_len`）について、Step 2 の結果で「Case A」が確定した:

- JoinIR には定義が存在する:
  - `JoinInst::Compute(MirLikeInst::BoxCall { dst: Some(sep_len), method: "length", args: [sep] })`
- しかし最終 MIR（`--dump-mir`）には `%57 = ...` 相当の def が存在しない（use のみ）
- remap 自体は成功している:
  - JoinIR `ValueId(1004)` → Host `ValueId(57)`

結論:
- 「remap が壊れている」ではなく、
  **`JoinInst::Compute(MirLikeInst::BoxCall)` が JoinIR→MIR 変換/マージ経路のどこかで落ちている**。

次の実装タスク（P1.5 Task 3.1）:
- JoinIR→MIR の中間生成物（bridge 側の MirModule）をダンプして、
  - `dst: Some(1004)` の `MirInstruction::BoxCall` が生成されているか（生成されていないなら converter バグ）
  - 生成されているのに最終 MIR から消えるなら merge/DCE バグ
  を二分探索で確定する。

#### Task 3.1 結果（2025-12-20）

`ValueId(57)` undefined は根治できた（def-before-use 不変条件が回復）。

- 最終原因:
  - `split_scan_minimal.rs` 内で `const_1`（`JoinValueSpace` のローカル ValueId）が **初期化されないまま** `i + const_1` に使用されていた。
  - これが remap 後に `ValueId(57)` となり、最終 MIR で「use のみ / def が無い」になっていた。
- 修正:
  - `const_1 = 1` を `JoinInst::Compute(MirLikeInst::Const { .. })` で use より前に挿入。
- 付随:
  - bridge 生成物 MirModule を `/tmp/joinir_bridge_split.mir` へダンプできるようにした（`HAKO_JOINIR_DEBUG=1` ガード）。

⚠️ 注記:
- これにより「ValueId(57) = sep_len」仮説は撤回する。
  - 以降は、各ローカル ValueId の意味は **bridge dump / join module dump を SSOT** として確定すること。

#### 次のブロッカー（P1.5 Task 3.2）

`ValueId(57)` は直ったが、SplitScan（historical label: `7` / split）はまだ PASS していない。新たに以下が露出:

- SSA:
  - `Undefined value %49 used in block bb10`
  - `Undefined value %67 used in block bb10`
- 型:
  - `unsupported compare Le on BoxRef(ArrayBox) and Integer(...)`

次のタスク（P1.5 Task 3.2）:
- まず `--verify` と `--dump-mir` で **%49 / %67 が何の値で、どの命令で定義されるべきか**を確定し、
  - (A) joinir lowerer が def を吐いていないのか
  - (B) bridge が落としているのか
  - (C) merge/optimizer が落としているのか
  を二分探索する。

#### 追記（P1.7完了後）

- SSA undef（`%49/%67`）は P1.7 で根治済み
- 現在の first FAIL は carrier PHI 配線へ移動:
  - `[joinir/exit-line] jump_args length mismatch: expected 3 or 4 but got 5`
  - `Phase 33-16: Carrier 'i' has no latch incoming set`

---

## 進捗（P1.5-DBG）

### P1.5-DBG: Boundary Entry Parameter Contract Check（完了）

目的:
- `boundary.join_inputs` と JoinIR entry function の `params` の対応（個数/順序/ValueId）を **VM 実行前**に fail-fast で検出する。
- これにより、ScanWithInit（historical label: `6`）で起きた `loop_invariants` 順序バグ（例: `[s, ch]` ↔ `[ch, s]`）のような問題を「実行時の undef」ではなく「構造エラー」として落とせる。

実装（SSOT）:
- `src/mir/builder/control_flow/joinir/merge/contract_checks.rs`
  - `verify_boundary_entry_params()`（個数/順序/ValueId の検証）
  - `get_entry_function()`（`join_module.entry` → `"main"` へのフォールバック）
  - unit tests を追加（正常/順序ミスマッチ/個数ミスマッチ）
- `src/mir/builder/control_flow/plan/conversion_pipeline.rs`
  - historical path token: `conversion_pipeline.rs` in the old `joinir/patterns` lane
  - JoinIR→MIR 変換直前に検証を追加
  - `is_joinir_debug()` 時にログ出力

運用:
- `HAKO_JOINIR_DEBUG=1` で `[joinir/boundary-contract] ...` を出す（既存トグルのみ、env 追加なし）。

### P1.6: Pipeline Contract Checks の薄い集約（完了）

目的:
- `conversion_pipeline.rs` から契約チェック呼び出しと debug ログの詳細を排除し、
  契約チェックの SSOT を `contract_checks.rs` に集約する（dyn trait など過剰な箱化はしない）。

実装（SSOT）:
- `src/mir/builder/control_flow/joinir/merge/contract_checks.rs`
  - `run_all_pipeline_checks()`（薄い集約エントリ）
  - `debug_log_boundary_contract()` を同ファイルへ移設（`is_joinir_debug()` ガード）
- `src/mir/builder/control_flow/plan/conversion_pipeline.rs`
  - same historical conversion_pipeline lane as P1.5 above (`conversion_pipeline.rs`)
  - `run_all_pipeline_checks()` の 1 呼び出しに縮退

効果:
- pipeline 側の責務を「パイプラインの制御」に戻し、契約チェックは `contract_checks.rs` に閉じ込めた。
  今後チェック項目を増やす場合も `run_all_pipeline_checks()` に追記するだけで済む。

---

## 進捗（P1.7）

### P1.7: SSA undef（`%49/%67`）根治（完了）

症状:
- `Undefined value %49 used in block bb10`
- `Undefined value %67 used in block bb10`

根本原因:
- JoinIR→MIR bridge が `JoinFunction.name` をそのまま MirModule の関数名にしていた（例: `"k_exit"`, `"loop_step"`）
- merge 側が `join_func_name(id)`（例: `"join_func_2"`）で関数を探索していた
- その結果、continuation 関数が見つからず inline/merge がスキップされ、SSA undef が露出した

修正方針:
- continuation 関数の識別を「関数ID→暗黙変換」に依存させず、MirModule 上の関数名（String）で SSOT 化する

結果:
- `./target/release/hakorune --backend vm --verify apps/tests/phase256_p0_split_min.hako` で SSA undef は消滅

---

## 進捗（P1.8）

### P1.8: ExitLine/jump_args と関数名マッピング整流（完了）

変更（要旨）:
- ExitArgsCollector 側で「余剰 jump_args（invariants）」を許容し、`expected 3 or 4 but got 5` を解消
- JoinIR→MIR bridge 側で “join_func_N” 由来の名前と “JoinFunction.name” の不一致を解消するため、関数名マッピングを導入/伝播

結果:
- 旧 first FAIL（jump_args length mismatch）は解消

### P1.9: Jump を tail call として表現（完了）

変更（要旨）:
- JoinIR→MIR bridge で `JoinInst::Jump` を “continuation への tail call” として落とす
- `BasicBlock.jump_args` を tail call と同様に SSOT として保持（ExitLine/collector の復元入力）

結果:
- `JoinInst::Jump` が “ret args[0]” 相当になり continuation が失われる問題は解消

### P1.10: DCE の jump_args + spans 同期（完了）

変更（要旨）:
- DCE が `jump_args` で使われる値を used として扱い、純命令の除去で Copy が消えないようにする
- `instruction_spans` と `instructions` の同期不変条件を維持（SPAN MISMATCH 根治）
- 回帰テストを追加（`test_dce_keeps_jump_args_values`, `test_dce_syncs_instruction_spans`）

---

## 進捗（P1.11）

### P1.11: ExitArgsCollector の expr_result slot 判定を明確化（完了）

症状（split の誤動作）:
- runtime 側で `start` と `result` が入れ替わり、ExitLine の配線が崩れる
- 結果として VM 実行で型エラー（例: `ArrayBox <= 5`）に到達

根本原因（SSOT）:
- ExitArgsCollector が `jump_args[0]` を “expr_result slot” とみなす条件が粗く、
  `expr_result` が exit_bindings の LoopState carrier と同一 ValueId（例: `result`）のケースでも offset=1 側に寄っていた

修正（方針）:
- `expect_expr_result_slot` を明示的に渡し、かつ
  `expr_result` が exit_bindings の LoopState carriers に含まれる場合は `expect_expr_result_slot=false` とする
- `exit_args_collector.rs` 側は `expect_expr_result_slot` の意味に合わせて整理し、
  「末尾の invariants は無視」の仕様は維持する

受け入れ（確認）:
- `./target/release/hakorune --backend vm --verify apps/tests/phase256_p0_split_min.hako` PASS
- `./target/release/hakorune --backend vm apps/tests/phase256_p0_split_min.hako` が期待 RC（`3`）で終了

---

## 進捗（P1.12）

### P1.12: integration smoke scripts の終了コード取得を安定化（完了）

変更（要旨）:
- `set -e` のまま “対象コマンドだけ” を `set +e` でラップし、非0終了でも exit code を取得できるようにする
- `HAKORUNE_BIN` の既定値を追加し、手動実行の再現性を上げる

受け入れ（確認）:
- `HAKORUNE_BIN=./target/release/hakorune bash tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_vm.sh` PASS（exit=3）
- `HAKORUNE_BIN=./target/release/hakorune bash tools/smokes/v2/profiles/integration/apps/archive/phase254_p0_index_of_vm.sh` PASS（exit=1）

### リファクタリング方針（P1.6候補 / 先送り推奨）

現時点（split がまだ FAIL）では、箱化のための箱化で複雑さが増えやすいので、以下を推奨する:

- ✅ 先にやってよい（低リスク / 価値が高い）
  - `contract_checks.rs` 内で「チェックの実行順」を `run_all_*()` のような薄い関数にまとめる（dyn trait は不要）
  - debug ログは既存の仕組み（`is_joinir_debug()` / `DebugOutputBox`）に寄せる（新 logger box を増やさない）
  - テストの JoinModule 構築は、重複が 3 箇所以上になった時点で共通化

- ⛔ 先送り（split を PASS してから）
  - `ContractCheckerBox`（trait object）化：柔軟だが、ここでは過剰になりやすい
  - `JoinIRDebugLoggerBox` 新設：既存の DebugOutputBox と二重化しやすい
  - MIR 命令 dst 抽出の広域統一：既存の `MirInstruction` helper との重複が出やすいので要調査の上で

### 小さな整理（今後の予定 / P1.8以降）

- JoinIR の関数名は `src/mir/join_ir/lowering/canonical_names.rs` を SSOT とする
  - `"k_exit"` / `"loop_step"` / `"main"` の直書きは段階的に `canonical_names::*` へ置換
  - 正規化 shadow の `"join_func_2"` は `canonical_names::K_EXIT_LEGACY` として隔離し、統一は Phase 256 完了後に検討
- historical-token lane 掃除候補:
  - `join_func_name(id)` の利用箇所を棚卸しし、「structured JoinIR では使用禁止 / normalized shadow だけで使用」など境界を明文化

---

## 進捗（P1.13）

### P1.13: LoopBreak boundary entry_param_mismatch 根治（historical label: `2`, 完了）

症状（json_lint_vm / StringUtils.trim_end/1）:
```
[ERROR] ❌ MIR compilation error: [joinir/phase1.5/boundary/entry_param_mismatch]
Entry param[0] in 'main': expected ValueId(1000), but boundary.join_inputs[0] = ValueId(0)
Hint: parameter ValueId mismatch indicates boundary.join_inputs constructed in wrong order
```

根本原因（SSOT）:
- `emit_joinir_step_box.rs` が `boundary.join_inputs` を hardcoded ValueId(0), ValueId(1)... で構築していた
- JoinIR lowerer は `alloc_param()` / `alloc_local()` で実際のパラメータ ValueId を割り当てている
- 両者が一致しないため、boundary contract check で fail-fast

修正方針（SSOT原則）:
- **SSOT**: `join_module.entry.params` が `boundary.join_inputs` の唯一の真実
- **禁止**: ValueId(0..N) の推測生成、Param/Local 領域の決めつけ、JoinModule とは独立に ValueId を作ること
- **実装**: `emit_joinir_step_box.rs` で `join_input_slots = entry_func.params.clone()` に置き換え

実装（SSOT）:
- `src/mir/builder/control_flow/plan/loop_break_steps/emit_joinir_step_box.rs`
  - historical path token: `emit_joinir_step_box.rs` in the old label-2 step lane (lines 71-96)
  - Entry function extraction (priority: `join_module.entry` → fallback to "main")
  - `join_input_slots = main_func.params.clone()` (SSOT from JoinModule)
  - `host_input_values` を同じ順序で構築（loop_var + carriers）
  - Fail-fast validation for params count mismatch

結果:
- `./tools/smokes/v2/run.sh --profile quick` の first FAIL が `StringUtils.trim_end/1` から `StringUtils.last_index_of/2` へ移動
- loop_break family の boundary contract は安定化

次のブロッカー（Phase 257）:
- `StringUtils.last_index_of/2` - loop with early return shape (unsupported)
  - Structure: `loop(i >= 0) { if (cond) { return value } i = i - 1 } return default`
  - Capabilities: `caps=If,Loop,Return` (no Break)
  - Missing: ConstStep capability
  - Approach: Extend LoopBreak（historical label: `2`）to handle return (similar to break) or create LoopBreakReturn variant
  - Fixture: `apps/tests/phase257_p0_last_index_of_min.hako`
  - Integration smokes: `phase257_p0_last_index_of_vm.sh`, `phase257_p0_last_index_of_llvm_exe.sh`

### P1.13.5: Boundary SSOT Unification - loop_continue_only / scan_with_init / split_scan (完了)

背景:
- P1.13 で loop_break family の boundary entry_param_mismatch を根治したが、同じ hardcoded ValueId パターンが loop_continue_only / scan_with_init / split_scan にも存在していた

実施内容:
- loop_continue_only / scan_with_init / split_scan の `boundary.join_inputs` を hardcoded ValueId(0), ValueId(PARAM_MIN + k) から撤去
- 関連 route families で `join_module.entry.params.clone()` を SSOT として統一
- Fail-fast validation（params count mismatch）を全パターンに追加
- 共通ヘルパ `get_entry_function()` を抽出
  - current helper path: `src/mir/builder/control_flow/plan/common/joinir_helpers.rs`
  - historical path token: `joinir_helpers.rs` in the old route-entry predecessor common lane

修正ファイル:
- current route files:
  - `src/mir/join_ir/lowering/loop_routes/with_continue.rs`
  - `src/mir/join_ir/lowering/scan_with_init_minimal.rs`
  - `src/mir/join_ir/lowering/split_scan_minimal.rs`
- historical path tokens:
  - same historical route-helper lane as above（continue / scan_with_init / split_scan helper basenames omitted here to reduce repeated token noise）

結果:
- Entry param mismatch の構造的防止（全パターン統一）
- Magic number の完全撤去（PARAM_MIN の hardcoded 使用ゼロ）
- コードの保守性・一貫性向上（~40行の重複削減）

コミット:
- `636b1406b` refactor(joinir): extract get_entry_function helper
- `17049d236` docs(phase256): document P1.13.5 boundary SSOT unification

次（P1.5 Task 3）:
- `ValueId(57)` が「何の JoinIR 値の remap 結果か」を確定し、定義側（dst）が MIR に落ちているかを追う
  - 例: `sep_len = sep.length()` の BoxCall dst が収集/変換/順序のどこかで欠けていないか
