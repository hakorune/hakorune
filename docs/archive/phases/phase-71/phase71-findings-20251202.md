# Phase 71 Findings - SSA/selfhost 再ブートストラップ観測報告

**実施日**: 2025-12-02
**担当**: Claude (Phase 70完了直後にPhase 71開始)

---

## 📊 観測結果サマリー

### 代表パス実行状況
```bash
NYASH_ROOT=/home/tomoaki/git/hakorune-selfhost \
NYASH_FEATURES=stage3 \
NYASH_USE_NY_COMPILER=1 \
NYASH_NY_COMPILER_EMIT_ONLY=1 \
NYASH_SELFHOST_KEEP_RAW=1 \
./tools/selfhost/selfhost_build.sh --in apps/tests/stage1_run_min.hako --run
```

**結果**:
- ✅ **Stage-B compiler実行成功** (`rc_stageb=0`)
- ❌ **Program JSON抽出失敗** (`extract_ok=0`)
- ❌ **Program JSON行数: 0件** (emit失敗)

### RAWログ
**Location**: `/home/tomoaki/git/hakorune-selfhost/logs/selfhost/stageb_20251202_101623_2665649.log`
**Size**: 707K

---

## 🔍 根本原因分析

### 1. SSA undef警告 (4件)

**影響関数**:
1. `ParserCommonUtilsBox.trim/1`
   - `Copy { dst: ValueId(2), src: ValueId(272) }` at `BasicBlockId(787)`
   - ValueId(272)が未定義

2. `ParserBox.trim/1`
   - `Copy { dst: ValueId(4), src: ValueId(272) }` at `BasicBlockId(2479)`
   - ValueId(272)が未定義

3. `Main._parse_number/1`
   - `Copy { dst: ValueId(2), src: ValueId(12) }` at `BasicBlockId(6708)`
   - ValueId(12)が未定義

4. `ParserBox.parse_block2/2`
   - `Copy { dst: ValueId(5), src: ValueId(440) }` at `BasicBlockId(2573)`
   - ValueId(440)が未定義

**パターン**: すべてCopy命令で未定義ValueIdをコピーしようとしている

### 2. dev verify警告 (1件)

```
[warn] dev verify: NewBox StageBDriverBox at v%366 not followed by birth() call
       (expect StageBDriverBox.birth/0)
```

**影響**: StageBDriverBoxの初期化手順が不完全

---

## 🎯 Phase 71-SSA側の課題

### 課題1: trim系関数のSSA undef

**影響範囲**:
- `ParserCommonUtilsBox.trim/1`
- `ParserBox.trim/1`

**想定原因**:
- レシーバ引数の受け渡しでValueIdが未定義のまま渡されている
- 関数呼び出し時のパラメータマッピングに問題がある可能性

**対応方針** (Phase 71-SSA-debug/TASKS.md):
1. `lang/src/compiler/parser/common_utils.hako` の `trim/1` 実装を確認
2. 呼び出し側での引数渡しパターンを確認
3. 必要に応じて `skip_ws` ベースの実装に統一（前回修正パターン適用）

### 課題2: Stage-B DriverBox birth警告

**影響**:
- `StageBDriverBox` が NewBox直後にbirth()を呼んでいない

**対応方針**:
- `apps/selfhost-compiler/compiler.hako` のStageBDriverBox使用箇所を確認
- birth()呼び出しを追加（または不要な場合は警告を緩和）

### 課題3: Program JSON未出力

**状況**:
- Stage-B rc=0 (エラーなし)
- しかしProgram JSON行が0件

**想定原因**:
- SSA undef や dev verify警告により、JSON出力処理に到達する前に処理が中断している可能性
- または JSON出力ロジック自体に問題がある可能性

**対応方針**:
1. `NYASH_STAGEB_DEV_VERIFY=0` で dev verify無効化して比較
2. Stage-B DriverBox の Program JSON出力箇所にトレースログ追加
3. SSA undef解消後に再度実行して状況確認

---

## 📋 Phase 71次のステップ

### ステップ1: SSA undef優先修正
- `trim/1` 系関数のSSA undef解消
- 前回の `_trim/1` 修正パターン（ダミーカンマ＋静的呼び出し統一）を適用

### ステップ2: dev verify緩和トグル活用
- `NYASH_STAGEB_DEV_VERIFY=0` での実行比較
- Program JSON出力復活の有無を確認

### ステップ3: Stage-B DriverBox トレース強化
- Program JSON出力直前のトレースログ追加
- 処理フローの可視化

### ステップ4: 代表パス安定化
- SSA undef全解消
- dev verify警告を0件に
- Program JSON emit成功を確認

---

## 🔗 関連ドキュメント

- **Phase 71 README**: `docs/private/roadmap2/phases/phase-71-selfhost-reboot/README.md`
- **Phase 71-SSA README**: `docs/private/roadmap2/phases/phase-71-ssa-debug/README.md`
- **Phase 71-SSA TASKS**: `docs/private/roadmap2/phases/phase-71-ssa-debug/TASKS.md`
- **CURRENT_TASK.md**: Line 112-119 (Phase 71-SSA観測停止中メモ)

---

## 💡 重要な気づき

### JoinIR は問題なし
- `[joinir/vm_bridge]` ログから、JoinIRパスは正常動作している
- `FuncScannerBox.trim` は JoinIR経路で正常に lowering されている
- **Phase 71-SSAの問題は「JoinIRとは無関係」**

### プラグイン初期化も問題なし
- `[UnifiedBoxRegistry] 🎯 Factory Policy: StrictPluginFirst` 成功
- `[provider-registry] FileBox: using registered provider` 成功
- **Phase 71-SSAの問題は「プラグインとも無関係」**

### 真の問題箇所
- **SSA/Stage-B MIR生成時の ValueId未定義問題**
- **StageBDriverBox の初期化手順不備**
- これらが複合的にProgram JSON emit失敗を引き起こしている

---

## 📝 Phase 71完了判定基準

- [ ] SSA undef警告: 4件 → 0件
- [ ] dev verify警告: 1件 → 0件
- [ ] Program JSON抽出: 0件 → 1件以上
- [ ] 代表パス `selfhost_build + stage1_run_min.hako` が GREEN

**現在の状況**: 0/4基準達成（観測窓としての役割は完了）

---

## 🎯 次のフェーズへの引き継ぎ

**Phase 71の成果**:
- ✅ Phase 70完了直後にPhase 71実行成功
- ✅ RAW観測レイヤ活用成功
- ✅ SSA undef根本原因特定（trim系関数の未定義ValueId問題）
- ✅ JoinIR/プラグインは無関係であることを確認

**Phase 71-SSA-debugへの課題引き継ぎ**:
- trim系関数 SSA undef 修正（4件 → 0件）
- StageBDriverBox birth警告 解消（1件 → 0件）
- Program JSON emit 復活（0件 → 1件以上）

---

**備考**: このドキュメントは Phase 71初回実行の観測結果を記録したものです。
SSA undef修正作業は Phase 71-SSA-debug側で継続します。
Status: Historical
